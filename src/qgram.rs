use indexmap::IndexMap;
use indicatif::{ParallelProgressIterator, ProgressBar};
use log::trace;
use parking_lot::RwLock;
use rayon::prelude::*;
use std::{
    fmt::{Display, Formatter},
    fs::File,
    io::{prelude::*, BufReader},
    ops::{Deref, DerefMut},
    path::PathBuf,
    sync::Arc,
};

use crate::cli::ProgressBarBuilder;
use crate::errors::*;

/// A symbol, such as a q-gram
pub(crate) type Token = String;
/// Corresponds to a line number where a token appears.
pub(crate) type ID = usize;
/// Corresponds to a position in a string where a token appears.
pub(crate) type Loc = usize;

/**
A poistional q-gram is a `token`-`location` pair for a given string.
 */
#[derive(Clone, Debug)]
pub(crate) struct PosQGram {
    pub token: Token,
    pub loc: Loc,
}

impl PosQGram {
    fn from(t: Token, l: Loc) -> Self {
        Self { token: t, loc: l }
    }
}

impl Display for PosQGram {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.token, self.loc)
    }
}

/**
A collection of token-position pair, as an alternative representation of a string.

For a given string `s` and a integer-valued parameter `q`, q-gram of a string `s` is a set of substrings of length `q`.
The size of this collection is `s.len() - q`, in which individiual q-grams may not be unique.

When `q` is 1, then the 1-gram (unigram) set is simply a set of all characters in string `s`.
And we can represent `s` as a collection of index-unigram pairs, where the index indicates the position where the uniqgram appear in the `s`.
For example,

```text

    +---+---+---+---+---+
s   | H | e | l | l | o |  <=>  {(0, H), (1, l), (2, l), (3, l), (4, o)}  (index, uni-qgram) pairs
    +---+---+---+---+---+

```

We can also represent `s` as a collection of index-qgram pair for any q, though there seem to have redundancy. For example, when `q` is 2 and 3

```text
    +---+---+---+---+---+
s   | H | e | l | l | o |  <=>  {(0, He), (1, el), (2, ll), (3, lo)}  (index, bi-qgram) pairs
    +---+---+---+---+---+

                           <=>  {(0, Hel), (1, ell), (2, llo)}  (index, tri-qgram) pairs
```

Hence, we define positional q-gram array as a collection of token-location pair, where tokens are elements of a q-gram set of a string.
And it's a valid representation of the original string.

*/
#[derive(Debug)]
pub(crate) struct PosQGramArray {
    inner: Vec<PosQGram>,
}

impl PosQGramArray {
    pub fn new() -> Self {
        Self {
            inner: Vec::<PosQGram>::new(),
        }
    }

    // PosQGramArray is sorted in increasing order of location
    /**
    Given a string and a given `q`, generate a PosQGramArray.
     */
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
        inner.par_sort_by_key(|qgram| qgram.loc);

        Self { inner }
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

type InvertedList = Vec<(ID, Loc)>;
/**
An indexmap of inverted lists for each token.
 */
pub(crate) type InvertedIndex = IndexMap<Token, InvertedList>;

/**
This function reads an entire file into a string, count q-grams by parallel iterators, and returns a hashmap where the keys are q-gram tokens, and values are a vector of line-position pair.

It may look silly to do such a thing, but a file with 1 million lines where each line contains at most 200 characters isn't large. Suppose it's encoded in legacy encodings such as ASCII, or it only contains ASCII characters and is encoded in UTF-8, then each character takes one byte. And this file would takes around 200 megabytes, which is rather small. If the input file is larger, then simply divide the input to small slices in pre-processing.

# Args

* `doc`: A `&std::path::Path` type variable indicating the path, absolute or relative, to the document to be processed.
* `q`: A `usize` value used to generate the `q`-grams.

# Returns

* When succesful, returns a B-Tree map, where keys are numbers of occurences of all token, i.e. a q-gram, and values are vectors of tokens that have that number of occurences.

 */
pub(crate) fn generate_inverted_list(doc: &PathBuf, q: usize) -> Result<InvertedIndex> {
    let file: File = File::open(doc)?;
    let mut reader: BufReader<File> = BufReader::new(file);
    let mut buffer: String = String::new();
    // read the entire file into a string so we can use parallel iterator
    reader.read_to_string(&mut buffer)?;

    let ngram_map_protected: Arc<RwLock<InvertedIndex>> = Arc::new(RwLock::new(IndexMap::new()));

    let buffer_vec: Vec<Vec<u8>> = buffer
        .par_lines()
        // Convert each line to Vec<u8> type so we can use par_windows method on each line
        .map(Vec::from)
        // Convert the buffer into a Vec<Vec<u8>> so we can use enumerate on the file
        .collect();

    // progress bar
    let pbar: ProgressBar =
        ProgressBarBuilder::new(buffer.par_lines().count(), "Making InvertedIndex").build();

    buffer_vec
        .par_iter()
        .enumerate()
        .progress_with(pbar)
        .for_each(
            // Though each line is Vec<u8> type, enumerate() returns it as a slice of u8
            |(line_id, line_content)| {
                // Make a clone and the references count will increase by 1
                let guard = ngram_map_protected.clone();

                // `par_windows()` creates a parallel iterator on ovelapping slices of the input
                let slice: Vec<_> = line_content
                    .par_windows(q)
                    // convert u8 to &[str], and then String, so we can use enumerate method on each qgram
                    .map(|ngrams| {
                        std::str::from_utf8(ngrams)
                            .expect("Error when parsing ngrams")
                            .to_string()
                    })
                    .collect();

                slice.into_par_iter().enumerate().for_each(|(pos, key)| {
                    // Create a write lock
                    let mut map = guard.write();
                    map.entry(key).or_insert(Vec::new()).push((line_id, pos));
                });
            },
        );

    let mut ngram_map: InvertedIndex = Arc::try_unwrap(ngram_map_protected)
        .expect("Arc is weak")
        .into_inner();

    // sort each value by ID (line number)
    ngram_map.par_iter_mut().for_each(|(_, list)| {
        list.par_sort_unstable_by_key(|key| {
            key.0 //sort by id
        });
    });
    // sort keys by length of the value (InvertedList)
    ngram_map.par_sort_by(|k1, v1, k2, v2| match v1.len().cmp(&v2.len()) {
        std::cmp::Ordering::Equal => k1.cmp(k2),
        std::cmp::Ordering::Less => std::cmp::Ordering::Less,
        std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
    });

    trace!("{:?}", ngram_map);
    Ok(ngram_map)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn pos_qgram_array() {
        let pos_qgram = PosQGramArray::from("hello", 2);
        assert_eq!(format!("{}", &pos_qgram), "PosQGramArray { inner: [PosQGram { token: \"he\", loc: 0 }, PosQGram { token: \"el\", loc: 1 }, PosQGram { token: \"ll\", loc: 2 }, PosQGram { token: \"lo\", loc: 3 }] }");
    }

    #[test]
    fn qgram_counter() {
        let testfile: PathBuf = PathBuf::from("./testset/sample_test1.txt".to_string());
        let result: String = format!("{:?}", generate_inverted_list(&testfile, 2).unwrap());

        assert_eq!(
            &result,
            "{\"al\": [(3, 1)], \"ha\": [(3, 0)], \"la\": [(2, 3)], \"lo\": [(0, 3), (3, 3)], \"el\": [(0, 1), (1, 1), (2, 1)], \"he\": [(0, 0), (1, 0), (2, 0)], \"ll\": [(0, 2), (1, 2), (2, 2), (3, 2)]}"
        );
    }
}
