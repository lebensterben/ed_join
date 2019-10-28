use crossbeam_channel::unbounded;
use rayon::prelude::*;
use std::{
    cmp::min,
    fs::File,
    io::{prelude::*, BufReader, BufWriter},
    path::PathBuf,
};

use crate::errors::*;
use crate::qgram::*;
use crate::verification::*;

#[cfg(feature = "cli")]
use crate::cli::ProgressBarBuilder;
#[cfg(feature = "cli")]
use indicatif::{ParallelProgressIterator, ProgressBar};

// Algorithm 2
// NOTE: PosQGram is not by default sorted in increasing frequency
/**
Given a set of q-grams, find the minimum number of edit operations that destroys all q-grams.

# Parameters
 * `qgram_array`: A PosQGramArray, i.e. a set of positional q-grams.
 * `q`: A positive integer as the tuning parameter for length of q-grams.

# Return
The minimum number of edit operations that destroy all q-grams in the given set.
 */
pub fn min_edit_errors(qgram_array: &[PosQGram], q: usize) -> usize {
    let mut cnt = 0;
    let mut loc = 0;

    let mut array_clone: Vec<PosQGram> = vec![PosQGram::default(); qgram_array.len()];
    array_clone[..].clone_from_slice(qgram_array);

    // qgram_array was sorted in increasing order before calling CalcPrefix,
    // Now sort it according to location
    array_clone.par_sort_unstable_by_key(|qgram| qgram.loc);

    qgram_array.iter().for_each(|qgram| {
        if qgram.loc > loc {
            cnt += 1;
            loc = qgram.loc + q - 1;
        }
    });

    cnt
}

// Algorithm 3
/**
Given a set of q-grams, find the minimum length of prefix such that if all the q-grams in the prefix are mismatched, it will incur at least `tau + ` edit errors.

# Parameters
 * `qgram_array`: A PosQGramArray, i.e. a set of positional q-grams.
 * `q`: A positive integer as the tuning parameter for length of q-grams.
 * `tau`: A positive integer as the tuning parameter for threshold for matching.

# Return
The minimum length of prefix such that if all the q-grams in the prefix are mismatched, it will incur at least `tau + ` edit errors.
 */
pub fn calc_prefix_len(qgram_array: &mut PosQGramArray, q: usize, tau: usize) -> usize {
    let mut left: usize = tau + 1;
    let mut right: usize = q * tau + 1;
    let mut mid: usize;
    let mut err: usize;
    let qgram_len: usize = qgram_array.len();

    while left < right {
        mid = (left + right) / 2; // usize automatically floored
        err = min_edit_errors(&qgram_array[0..min(mid, qgram_len)], q);
        if err <= tau {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    left = std::cmp::min(left, qgram_array.len());
    #[cfg(feature = "cli")]
    trace!(
        "CalcPrefix for `{}`: prefix length = {}",
        &qgram_array,
        &left
    );
    left
}

// Algorithm 1
/**
Given two input files, `doc_x` and `doc_y`, and two parameters, `q` and `tau`, find all records in `doc_y` that match records in `doc_x` such that are matched pairs have edit-distance smaller or equal to `tau`.

# Parameters
 * `doc_x` and `doc_y`: Paths to a input files, in which we process each record in `doc_x` and looking for valid matches in `doc_y`
 * `q`: A positive integer as the tuning parameter for length of q-grams. Large `q` reduces the amount of tokens in pre-matching, but makes filtering less effective. Small `q` generates large amount of tokens for filtering, the output of filtering are more likely to be valid matches, but this prolongs the time on filtering.
 * `tau`: A positive integer as the tuning parameter for threshold for matching.

# Return
All matching pairs. This would be stored in a output file automatically under the same directory of the first input file.
 */
pub fn ed_join(doc_x: &PathBuf, doc_y: &PathBuf, q: usize, tau: usize) -> Result<()> {
    // `doc_x` is read by a BufReader, line by line
    let file_x: File = File::open(doc_x)?;
    let reader_x: BufReader<File> = BufReader::new(file_x);

    // Read entire `doc_y` into a vector to reduce IO
    let file_y: File = File::open(doc_x)?;
    let mut reader_y: BufReader<File> = BufReader::new(file_y);
    let mut y_buffer: String = String::new();
    reader_y.read_to_string(&mut y_buffer)?;
    let y_vec: Vec<Vec<u8>> = y_buffer.par_lines().map(Vec::from).collect();

    let out_name: PathBuf = PathBuf::from(
        format!(
            "{}_out_q{}_tau{}.{}",
            doc_x.file_stem().unwrap().to_str().unwrap(),
            q,
            tau,
            // note that extension may be empty
            doc_x
                .extension()
                .unwrap_or_else(|| std::ffi::OsStr::new("txt"))
                .to_str()
                .unwrap()
        )
        .to_string(),
    );
    let doc_out: File = File::create(&out_name).expect("Failed to Create File");
    let mut writer: BufWriter<File> = BufWriter::new(doc_out);
    let mut output_vec: Vec<(ID, Vec<(ID, usize)>)> = Vec::new();
    let (output_s, output_r) = unbounded::<Vec<(ID, Vec<(ID, usize)>)>>();

    let inverted_index: InvertedIndex = generate_inverted_index(doc_x, doc_y, q)?;
    #[cfg(feature = "cli")]
    debug!("InvertedList: {:?}", &inverted_index);

    #[cfg(not(feature = "cli"))]
    let file_x_iter = reader_x.lines().enumerate().par_bridge();
    #[cfg(feature = "cli")]
    let file_x_iter;
    #[cfg(feature = "cli")]
    {
        // progress bar
        let file_x_len: usize = BufReader::new(File::open(doc_x)?).lines().count();
        let pbar: ProgressBar = ProgressBarBuilder::new(file_x_len, "Processing").build();
        file_x_iter = reader_x
            .lines()
            .enumerate()
            .par_bridge()
            .progress_with(pbar);
    }

    file_x_iter.for_each(|(x_id, line_x)| {
        let x_content = String::from(line_x.unwrap());
        #[cfg(feature = "cli")]
        trace!(
            "=====================\nCurrent line {}: {}",
            x_id,
            x_content
        );

        let mut qgram_array_x: PosQGramArray = PosQGramArray::from(&x_content, q);
        // PosQGramArray is sorted in increasing order of location, but we need to sort it in increasing order of frequency
        // to calculate the prefix length, which is stored in the secod slot of InvertedList
        qgram_array_x.sort_by_frequency(&inverted_index);

        // calculate a prefix length between `tau + 1` and `q * tau + 1`, by `calc_prefix_len()`
        let prefix_len: usize = calc_prefix_len(&mut qgram_array_x, q, tau);

        let mut candidates: Vec<ID> = qgram_array_x
            .par_iter()
            .take(prefix_len)
            .flat_map(|qgram| {
                let token_x: Token = qgram.token.clone();
                let loc_x: Loc = qgram.loc;

                // NOTE, the first slot is the inverted list of document y
                let inverted_list: &Vec<(ID, Loc)> = &inverted_index[&token_x].0;
                #[cfg(feature = "cli")]
                trace!(
                    "**************\nI-list of `{}`: {:?}",
                    token_x,
                    inverted_list,
                );

                let mut filtered: Vec<ID> = inverted_list
                    .par_iter()
                    .filter(|(y_id, _loc_y)| {
                        // only consider line id greater than current line when self-join
                        // If doc_x != doc_y => false && not_evaluated || true => true
                        // If doc_x == doc_y => true && (*id > x_id) || false => (*id > x_id)
                        (doc_x == doc_y) && (*y_id > x_id) || (doc_x != doc_y)
                    })
                    .filter(|(y_id, loc_y)| {
                        // length filter
                        (y_vec[*y_id].len() as isize - x_content.len() as isize).abs() <= tau as isize
                        // position filter
                            && (loc_x as isize - *loc_y as isize).abs() <= tau as isize
                    })
                    .map(|pair| pair.0)
                    .collect();
                filtered.par_sort_unstable();
                filtered.dedup();
                filtered
            })
            .collect();
        candidates.par_sort_unstable();
        candidates.dedup();

        #[cfg(feature = "cli")]
        debug!("Candidate of `{}: {}`: {:?}", x_id, x_content, candidates);

        let mut verified: Vec<(ID, Vec<(ID, usize)>)> = candidates
            .par_iter()
            .map(|y_id| {
                let y_content = std::str::from_utf8(&y_vec[*y_id]).unwrap();
                let qgram_array_y = PosQGramArray::from(&y_content, q);
                (y_id, y_content, qgram_array_y)
            })
            .filter_map(|(y_id, y_content, mut qgram_array_y)|{
                verify(
                    qgram_array_x.to_vec(),
                    x_id,
                    &x_content,
                    &mut qgram_array_y,
                    *y_id,
                    &y_content,
                    &inverted_index,
                    q,
                    tau,
                )
            })
            .collect();
        verified.par_iter_mut().for_each(|(_x_id, yvec)| yvec.par_sort_unstable_by(|(a_id, _a_ed), (b_id, _b_ed)| a_id.cmp(&b_id)));

        output_s.send(verified).unwrap();

    });
    drop(output_s);

    while let Ok(mut v) = output_r.recv() {
        output_vec.append(&mut v);
    }
    drop(output_r);

    // sort by line id of doc_x, i.e. the first slot
    output_vec.par_sort_by_key(|x| x.0);

    #[cfg(feature = "cli")]
    debug!("Mathes: {:?}", output_vec);

    output_vec.iter().for_each(|(id_x, pairs)| {
        // first sort the pairs, which is a vector of ID and edit-distance,
        // by ID, that is the ID from doc_y
        pairs.iter().for_each(|(id_y, ed)| {
            writer
                .write_all(format!("{},{},{}\n", id_x, id_y, ed).as_bytes())
                .expect("Failed to write to output file.");
        })
    });

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::qgram::{PosQGram, PosQGramArray};

    #[test]
    fn test_min_edit_error() {
        let qgram_array: PosQGramArray = PosQGramArray::from("hello", 2);
        assert_eq!(min_edit_errors(&qgram_array, 2), 2);
    }

    #[test]
    fn test_calc_prefix_len() {
        let mut qgram_array: PosQGramArray = PosQGramArray::from_vec(vec![
            PosQGram {
                token: "lo".to_string(),
                loc: 3,
            },
            PosQGram {
                token: "he".to_string(),
                loc: 0,
            },
            PosQGram {
                token: "el".to_string(),
                loc: 1,
            },
            PosQGram {
                token: "ll".to_string(),
                loc: 2,
            },
        ]);
        let result = calc_prefix_len(&mut qgram_array, 2, 2);
        assert_eq!(result, 4);
    }
}
