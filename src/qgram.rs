use crossbeam_channel::unbounded;
use rayon::prelude::*;
use std::{
    cmp::Ordering,
    collections::HashMap,
    fmt::{Display, Formatter},
    fs::File,
    io::{prelude::*, BufReader},
    ops::{Deref, DerefMut},
    path::PathBuf,
};

use crate::errors::*;

/// A symbol, such as a q-gram
pub(crate) type Token = String;
/// Corresponds to a line number where a token appears.
pub(crate) type ID = usize;
/// Corresponds to a position in a string where a token appears.
pub(crate) type Loc = usize;

/// A poistional q-gram is a `token`-`location` pair for a given string.
#[derive(Clone, Debug, Default)]
pub struct PosQGram {
    pub token: Token,
    pub loc: Loc,
}

impl PosQGram {
    fn from(t: Token, l: Loc) -> Self {
        Self { token: t, loc: l }
    }

    pub fn cmp(&self, other: &Self, inverted: InvertedIndex) -> Ordering {
        let len_a: usize = inverted.get(&self.token).unwrap().1;
        let len_b: usize = inverted.get(&other.token).unwrap().1;
        match len_a.cmp(&len_b) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => self.token.as_bytes().cmp(&other.token.as_bytes()),
        }
    }
}

impl Display for PosQGram {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.token, self.loc)
    }
}

/// A collection of token-position pair, as an alternative representation of a string.
///
/// For a given string `s` and a integer-valued parameter `q`, q-gram of a string `s` is a set of substrings of length `q`.
/// The size of this collection is `s.len() - q`, in which individiual q-grams may not be unique.
///
/// When `q` is 1, then the 1-gram (unigram) set is simply a set of all characters in string `s`.
/// And we can represent `s` as a collection of index-unigram pairs, where the index indicates the position where the uniqgram appear in the `s`.
/// For example,
///
/// ```text
///
///     +---+---+---+---+---+
/// s   | H | e | l | l | o |  <=>  {(0, H), (1, l), (2, l), (3, l), (4, o)}  (index, uni-qgram) pairs
///     +---+---+---+---+---+
///
/// ```
///
/// We can also represent `s` as a collection of index-qgram pair for any q, though there seem to have redundancy. For example, when `q` is 2 and 3
///
/// ```text
///     +---+---+---+---+---+
/// s   | H | e | l | l | o |  <=>  {(0, He), (1, el), (2, ll), (3, lo)}  (index, bi-qgram) pairs
///     +---+---+---+---+---+
///
///                            <=>  {(0, Hel), (1, ell), (2, llo)}  (index, tri-qgram) pairs
/// ```
///
/// Hence, we define positional q-gram array as a collection of token-location pair, where tokens are elements of a q-gram set of a string.
/// And it's a valid representation of the original string.
#[derive(Debug)]
pub struct PosQGramArray {
    pub inner: Vec<PosQGram>,
}

impl PosQGramArray {
    pub fn new() -> Self {
        Self {
            inner: Vec::<PosQGram>::new(),
        }
    }

    /// Convenient method for converting a vector of PosQGram to PosQGramArray
    pub fn from_vec(inner: Vec<PosQGram>) -> Self {
        Self { inner }
    }

    /// Given a string and a given `q`, generate a PosQGramArray.
    // NOTE: The position QGramArray is sorted in increasing order of location.
    pub fn from(s: &str, q: usize) -> Self {
        let slice: Vec<String> = Vec::from(s)
            .par_windows(q)
            .map(|ngrams| {
                std::str::from_utf8(ngrams)
                    .expect("Error when parsing ngrams")
                    .to_string()
            })
            .collect();

        let mut inner: Vec<PosQGram> = Vec::new();

        slice.into_iter().enumerate().for_each(|(pos, key)| {
            inner.push(PosQGram::from(key.to_string(), pos));
        });
        // sort in increasing order of location
        inner.par_sort_unstable_by_key(|qgram| qgram.loc);

        Self { inner }
    }

    /// Actually, it's sorted in the following hierarchical order:
    ///
    /// - Firstly, in decreasing order of frequency
    /// - Secondly, in lexicographical order of token name
    pub fn sort_by_frequency(&mut self, inverted: &InvertedIndex) {
        self.par_sort_unstable_by(|a, b| {
            let len_a: usize = inverted.get(&a.token).unwrap().1;
            let len_b: usize = inverted.get(&b.token).unwrap().1;
            match len_a.cmp(&len_b) {
                Ordering::Greater => Ordering::Greater,
                Ordering::Less => Ordering::Less,
                Ordering::Equal => a.token.as_bytes().cmp(&b.token.as_bytes()),
            }
        });
    }

    pub fn sort_by_location(&mut self) {
        self.par_sort_unstable_by(|a, b| a.loc.cmp(&b.loc))
    }
}

impl Deref for PosQGramArray {
    type Target = Vec<PosQGram>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for PosQGramArray {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Display for PosQGramArray {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut r: String = String::new();

        self.inner.iter().for_each(|qgram| {
            r.push_str(&format!("{}, ", qgram));
        });
        r.pop();
        r.pop();
        write!(f, "[{}]", r)
    }
}

/// An InvertedList is a vector of ID-location pair, where ID is the line number where a certain token appears,
/// and location is the index of that line where the token appear.
pub type InvertedList = Vec<(ID, Loc)>;

/// An indexmap of inverted lists for each token. The keys are Token, while the values are a tuple of  InvertedList and usize.
///
/// - When it's self-join, the usize is the total number of occurences of the token, and the InvertedList is for the document.
/// - When it's not self-join, the usize is still the total number of occurences of the token, while the InvertedList is only for the second document.
///
pub type InvertedIndex = HashMap<Token, (InvertedList, usize)>;

/// This function reads an entire file into a string, count q-grams by parallel iterators,
/// and returns a hashmap where the keys are q-gram tokens, and values are a vector of line-position pair.
///
/// NOTE: If the input file is larger, then simply divide the input to small slices in pre-processing. This would be implemented in the future.
///
/// # Args
///
/// * `doc_x` and `doc_y`: Path, absolute or relative, to documents to be processed.
/// * `file_x_len` and `file_y_len`: Number of lines for each file. This is used
/// * `q`: A tuning parameter used to generate the `q`-grams.
///
/// # Returns
///
/// * When succesful, returns a B-Tree map, where keys are numbers of occurences of all token, i.e. a q-gram,
///   and values are vectors of tokens that have that number of occurences.
pub fn generate_inverted_index(
    doc_x: &PathBuf,
    doc_y: &PathBuf,
    q: usize,
) -> Result<InvertedIndex> {
    let reader_y: BufReader<File> = BufReader::new(File::open(doc_y)?);
    let mut ngram_map: InvertedIndex = HashMap::new();

    // first collect ngrams for document_y
    let (map_y_s, map_y_r) = unbounded::<(Token, (ID, Loc))>();
    reader_y
        .lines()
        .enumerate()
        .for_each(|(line_id, line_result)| {
            let map_y_s_clone = map_y_s.clone();
            // `par_windows()` creates a parallel iterator on ovelapping slices of the input
            let slice: Vec<_> = Vec::from(line_result.unwrap())
                .par_windows(q)
                // convert u8 to &[str], and then String, so we can use enumerate method on each qgram
                .map(|qgrams| {
                    std::str::from_utf8(qgrams)
                        .expect("Error when parsing ngrams")
                        .to_string()
                })
                .collect();
            slice.into_par_iter().enumerate().for_each(|(pos, key)| {
                map_y_s_clone.send((key, (line_id, pos))).unwrap();
            });
        });
    drop(map_y_s);

    while let Ok((key, (line_id, pos))) = map_y_r.recv() {
        ngram_map
            .entry(key)
            .or_insert((Vec::new(), 0))
            .0
            .push((line_id, pos));
    }
    drop(map_y_r);

    // then count the occurences for doc_y only, and store it in the second slot
    ngram_map
        .values_mut()
        .par_bridge()
        .for_each(|value| value.1 = value.0.len());

    // Only process doc_x when it's not self-join
    // but only add the count to the second slot of the value
    // And the channel only sends the Token
    if doc_x != doc_y {
        let reader_x: BufReader<File> = BufReader::new(File::open(doc_x)?);
        let (map_x_s, map_x_r) = unbounded::<Token>();

        reader_x.lines().for_each(|line_result| {
            let map_x_s_clone = map_x_s.clone();
            let slice: Vec<_> = Vec::from(line_result.unwrap())
                .par_windows(q)
                .map(|qgrams| {
                    std::str::from_utf8(qgrams)
                        .expect("Error when parsing ngrams")
                        .to_string()
                })
                .collect();

            slice.into_par_iter().for_each(|key| {
                map_x_s_clone.send(key).unwrap();
            });
        });
        drop(map_x_s);

        while let Ok(key) = map_x_r.recv() {
            let (_list_y, count) = ngram_map.entry(key).or_insert((Vec::new(), 0));
            *count += 1;
        }
        drop(map_x_r);
    }

    // sort values by ID (line number)
    ngram_map.par_iter_mut().for_each(|(_, (list_y, _count))| {
        list_y.par_sort_unstable_by_key(|(id_y, _loc_y)| {
            *id_y //sort by id
        });
    });

    Ok(ngram_map)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn pos_qgram_array() {
        let pos_qgram = PosQGramArray::from("hello", 2);
        assert_eq!(
            format!("{}", &pos_qgram),
            "[(he, 0), (el, 1), (ll, 2), (lo, 3)]"
        );
    }

    #[test]
    fn qgram_counter() {
        let testfile: PathBuf = PathBuf::from("./testset/sample_test1.txt".to_string());
        let result: String = format!(
            "{:?}",
            generate_inverted_index(&testfile, &testfile, 2)
                .unwrap()
                .get("he")
        );

        assert_eq!(result, format!("{:?}", Some(([(0, 0), (1, 0), (2, 0)], 3))));
    }
}
