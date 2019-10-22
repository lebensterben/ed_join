use edit_distance::edit_distance;
use parking_lot::Mutex;
use rayon::prelude::*;
use std::{collections::HashMap, sync::Arc};

use crate::matching::min_edit_errors;
use crate::qgram::*;

type RightError = usize;
type SuffixSumArray = Vec<(Loc, RightError)>;

// Algorithm 8
/**
Given two sorted q-gram arrays, in increasing order of location, find the set of loosely mismatching q-grams and the number of strictly mismatching q-grams.

# Parameters
 * `x` and `y`: PosQGramArrays as `source` and `target` for matching.
 * `invert`: The inverted index.
 * `tau`: A positive integer as the tuning parameter for threshold for matching.

# Return
 * A set of loosely mismatching q-grams from `x` to `y`.
 * The number of strictly mismatching q-grams from `x` to `y`.
 */
fn compare_qgrams(
    x: &PosQGramArray,
    y: &PosQGramArray,
    invert: &InvertedIndex,
    tau: usize,
) -> (PosQGramArray, usize) {
    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut epsilon: usize = 0;
    let mut loose_mismatch: PosQGramArray = PosQGramArray::new();

    let comparator = |x: &PosQGramArray,
                      y: &PosQGramArray,
                      i: &mut usize,
                      j: usize,
                      epsilon: &mut usize,
                      loose_mismatch: &mut PosQGramArray| {
        if ((*i >= 1) && (x[*i].token != x[*i - 1].token))
            || ((j >= 1) && (x[*i].token != y[j - 1].token))
            || ((j >= 1) && ((x[*i].loc as isize - y[j - 1].loc as isize).abs() > tau as isize))
        {
            loose_mismatch.push(x[*i].clone());
        }
        *i += 1;
        *epsilon += 1;
    };

    while i < x.len() && j < y.len() {
        if x[i].token == y[j].token {
            if (x[i].loc as isize - y[j].loc as isize).abs() <= tau as isize {
                i += 1;
                j += 1;
            } else if x[i].loc < y[j].loc {
                comparator(x, y, &mut i, j, &mut epsilon, &mut loose_mismatch);
            } else {
                j += 1;
            }
        } else if (invert.get(&x[i].token).unwrap().len() < invert.get(&y[j].token).unwrap().len())
            || ((invert.get(&x[i].token).unwrap().len() == invert.get(&y[j].token).unwrap().len())
                & ((x[i].token.as_bytes()) < (y[j].token.as_bytes())))
        {
            comparator(x, y, &mut i, j, &mut epsilon, &mut loose_mismatch);
        } else {
            j += 1;
        }
    }
    while i < x.len() {
        comparator(x, y, &mut i, j, &mut epsilon, &mut loose_mismatch);
    }

    loose_mismatch.par_sort_by_key(|qgram| qgram.loc);

    (loose_mismatch, epsilon)
}

// Based on Algorithm 2
/**
Given a set of q-grams, find the minimum number of edit operations in the suffix that destroys all q-grams.

# Parameters
 * `qgram_array`: A PosQGramArray, i.e. a set of positional q-grams.
 * `q`: A positive integer as the tuning parameter for length of q-grams.

# Return
The minimum number of edit operations on the suffix that destroy all q-grams.
 */
fn sum_right_errors(qgram_array: &mut PosQGramArray, q: usize) -> Option<SuffixSumArray> {
    if qgram_array.len() == 0 {
        None
    } else {
        qgram_array.reverse();
        let mut cnt: usize = 0;
        let mut loc: usize = qgram_array[0].loc + 1;

        let mut suffix_sum: SuffixSumArray = Vec::new();

        qgram_array.iter().for_each(|qgram| {
            if qgram.loc < loc {
                cnt += 1;
                suffix_sum.push((qgram.loc, cnt));
                if qgram.loc + 1 >= q {
                    loc = qgram.loc + 1 - q;
                } else {
                    loc = 0;
                }
            }
        });

        qgram_array.reverse();
        Some(suffix_sum)
    }
}

fn frequency_histogram(s: &str) -> HashMap<char, usize> {
    let mut map: HashMap<char, usize> = HashMap::new();

    s.chars().for_each(|c| {
        map.entry(c).and_modify(|v| *v += 1).or_insert(1);
    });

    map
}

// Algorithm 6
/**
Given two strings, calculate their L1 distance.

# Parameters
 * `s` and `t`: (Sub-)String that is under probing window.
 * `lo` and `hi`: Indicates the start and end point of the probing window.

# Return
L1 distance of the two given strings with given probing window.
 */
fn l1_distance(s: &str, t: &str, lo: usize, hi: usize) -> usize {
    let h_s: HashMap<char, usize> = frequency_histogram(&s[lo..hi]);
    let h_t: HashMap<char, usize> = frequency_histogram(&t[lo..hi]);

    let mut keys: Vec<&char> = h_s.keys().collect::<Vec<&char>>();
    keys.append(&mut h_t.keys().collect::<Vec<&char>>());
    keys.par_sort();
    keys.dedup();

    let mut v_s: Vec<usize> = Vec::with_capacity(keys.len());
    let mut v_t: Vec<usize> = Vec::with_capacity(keys.len());

    keys.iter().for_each(|k| {
        v_s.push(*h_s.get(k).unwrap_or(&0));
        v_t.push(*h_t.get(k).unwrap_or(&0));
    });

    let distance: usize = v_s
        .par_iter()
        .zip_eq(v_t.par_iter())
        .map(|(a, b)| (*a as isize - *b as isize).abs() as usize)
        .sum();
    distance
}

// Algorithm 5
/**
Content-based mismatch filtering by combining L1-distance and minimum edit errors in the suffix to the probing window.

# Parameters
 * `s` and `t`: (Sub-)String that is under probing window.
 * `mismatch`: A PosQGramArray with loosely mismatching q-grams from `s` to `t`.
 * `suffix_sum`: A condensed suffix sum array.
 * `q`: A positive integer as the tuning parameter for length of q-grams.
 * `tau`: A positive integer as the tuning parameter for threshold for matching.

# Return
A lower bound of the edit distance from `s` to `t`.
 */
fn content_filter(
    s: &str,
    t: &str,
    mismatch: PosQGramArray,
    suffix_sum: SuffixSumArray,
    q: usize,
    tau: usize,
) -> Option<usize> {
    let mut i: usize = 1;
    let mut j: usize = 0;
    let mut epsilon: usize;

    let epsi = |s, t, mismatch: &PosQGramArray, q, ii: usize, jj: usize| {
        let l1 = l1_distance(s, t, mismatch[jj].loc, mismatch[ii - 1].loc + q - 1);
        let right_error = suffix_sum
            .par_iter()
            .find_first(|e| e.0 >= mismatch[ii - 1].loc + q) // e is a PosQGram, e.0 is location
            .unwrap_or(&(0, 0)) // returns (Loc, RightError)
            .1; // returns RightError
        l1 / 2 + right_error // NOTE: I believe author had a typo here and I fixed it
    };

    // otherwise index is out-of-bound
    if mismatch.len() >= 2 {
        while i < mismatch.len() {
            if mismatch[i].loc - mismatch[i - 1].loc > 1 {
                epsilon = epsi(s, t, &mismatch, q, i, j);
                if epsilon > tau {
                    return Some(2 * tau + 1);
                }
                j = i;
            }
            i += 1;
        }

        let epsilon = epsi(s, t, &mismatch, q, i, j);
        Some(epsilon)
    } else {
        None
    }
}

// Algorithm 7
/**
Given a string and a set of possible candidates for matching, verify whether each of the candidate is valid by various filters, and eventually output all matched candidates and corresponding edit distance.

# Parameters
 * `line_id`: Corresponds to the line number where the string appears in input file.
 * `candidates_id`: A collection of line number where each candidate appears in input file.
 * `buffer`: A vector contains the input file.
 * `inverted`: The inverted index.
 * `q`: A positive integer as the tuning parameter for length of q-grams.
 * `tau`: A positive integer as the tuning parameter for threshold for matching.

# Return
Return only verified matched paris from the candidates set.
 */
pub(crate) fn verify(
    line_id: ID,
    candidates_id: &Vec<&ID>,
    buffer: Vec<Vec<u8>>,
    inverted: &InvertedIndex,
    q: usize,
    tau: usize,
) -> Option<(ID, Vec<(ID, usize)>)> {
    let line: &str = std::str::from_utf8(&buffer[line_id]).unwrap();
    let candidates: Vec<(&usize, &str)> = candidates_id
        .par_iter()
        .map(|id| (*id, std::str::from_utf8(&buffer[**id]).unwrap()))
        .collect();
    let mut x = PosQGramArray::from(line, q);
    // PosQGramArray is only sorted in increasing order of location, now sort it in increasing order of frequency
    x.par_sort_by_key(|qgram| inverted.clone().entry(qgram.token.to_string()).index());
    let out_protected: Arc<Mutex<Vec<(ID, usize)>>> = Arc::new(Mutex::new(Vec::new()));

    #[cfg(not(feature = "cli"))]
    let candidates_iter = candidates.par_iter();
    #[cfg(feature = "cli")]
    let candidates_iter;
    #[cfg(feature = "cli")]
    {
        use crate::cli::ProgressBarBuilder;
        use indicatif::{ParallelProgressIterator, ProgressBar};
        // progress bar
        let pbar: ProgressBar =
            ProgressBarBuilder::new(candidates.len(), "Verifying Candidates").build();
        candidates_iter = candidates.par_iter().progress_with(pbar);
    }

    candidates_iter.for_each(|(id, candidate)| {
        #[cfg(feature = "cli")]
        trace!("Match `{}` against `{}`", line, candidate);

        let out_clone = out_protected.clone();
        let mut y = PosQGramArray::from(candidate, q);
        // PosQGramArray is only sorted in increasing order of location, now sort it in increasing order of frequency
        y.par_sort_by_key(|qgram| inverted.clone().entry(qgram.token.to_string()).index());

        let (mut loose_mismatch, epsilon_1) = compare_qgrams(&x, &y, &inverted, q);
        #[cfg(feature = "cli")]
        trace!(
            "x: {}\n y: {}\n Loosely-Mismatch: {}\n # of Strongly Mismatch: {}",
            x,
            y,
            loose_mismatch,
            epsilon_1
        );

        // count filtering
        #[cfg(feature = "cli")]
        trace!(
            "Count filtering on {}: epsilon_1 = {}",
            candidate,
            epsilon_1
        );
        if epsilon_1 <= q * tau {
            // loose_mismatch is a PosQGramArray, which is generated from &x, &y, who were sorted in increasing order of frequency
            // now sort it in increasing order of location
            loose_mismatch.par_sort_by_key(|qgram| qgram.loc);
            let epsilon_2 = min_edit_errors(&loose_mismatch, q);

            // location-based filtering
            #[cfg(feature = "cli")]
            trace!(
                "Location-based filtering on {}: epsilon_2 = {}",
                candidate,
                epsilon_2
            );
            if epsilon_2 <= tau {
                if let Some(right_error) = sum_right_errors(&mut loose_mismatch, q) {
                    let suffix_sum_array: SuffixSumArray = right_error;
                    #[cfg(feature = "cli")]
                    trace!("Suffix Sum Array: {:?}", suffix_sum_array);
                    let epsilon_3 =
                        content_filter(line, candidate, loose_mismatch, suffix_sum_array, q, tau);

                    // content-based filtering
                    if let Some(v) = epsilon_3 {
                        #[cfg(feature = "cli")]
                        trace!(
                            "Content-based filtering on {}: epsilon_3 = {}",
                            candidate,
                            v,
                        );
                        // NOTE: I believe author made a mistake here
                        if v <= tau {
                            let ed: usize = edit_distance(&line, candidate);
                            #[cfg(feature = "cli")]
                            trace!("Ed of `{}` against `{}`", line, candidate);
                            if ed <= tau {
                                #[cfg(feature = "cli")]
                                trace!("Add `{}` to matched set of `{}`", candidate, &line);
                                let mut out_guard = out_clone.lock();
                                out_guard.push((**id, ed));
                            }
                        }
                    } else {
                        // when mismatch is empty, cannot apply content filter, go to this branch
                        let ed: usize = edit_distance(&line, candidate);
                        #[cfg(feature = "cli")]
                        trace!("Ed of `{}` against `{}`", line, candidate);
                        if ed <= tau {
                            #[cfg(feature = "cli")]
                            trace!("Add `{}` to matched set of `{}`", candidate, &line);
                            let mut out_guard = out_clone.lock();
                            out_guard.push((**id, ed));
                        }
                    }
                } else {
                    // when mismatch is empty, sum_right_errors is empty, go to this branch
                    let ed: usize = edit_distance(&line, candidate);
                    #[cfg(feature = "cli")]
                    trace!("Ed of `{}` against `{}`", line, candidate);
                    if ed <= tau {
                        #[cfg(feature = "cli")]
                        trace!("Add `{}` to matched set of `{}`", candidate, &line);
                        let mut out_guard = out_clone.lock();
                        out_guard.push((**id, ed));
                    }
                }
            }
        }
    });

    let mut out: Vec<(ID, usize)> = Arc::try_unwrap(out_protected)
        .expect("Arc is weak")
        .into_inner();
    out.par_sort_by_key(|x| x.0);

    if !out.is_empty() {
        Some((line_id, out))
    } else {
        None
    }
}
