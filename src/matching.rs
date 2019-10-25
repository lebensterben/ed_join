use parking_lot::Mutex;
use rayon::prelude::*;
use std::{
    cmp::min,
    collections::HashSet,
    fs::File,
    io::{prelude::*, BufReader, BufWriter},
    path::PathBuf,
    sync::Arc,
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
pub(crate) fn min_edit_errors(qgram_array: &[PosQGram], q: usize) -> usize {
    let mut cnt = 0;
    let mut loc = 0;

    let mut array_clone: Vec<PosQGram> = vec![PosQGram::default(); qgram_array.len()];
    array_clone[..].clone_from_slice(qgram_array);
    // qgram_array was sorted in increasing order of frequency by CalcPrefix,
    // Now sort it according to location
    array_clone.par_sort_by_key(|qgram| qgram.loc);

    qgram_array.iter().for_each(|qgram| {
        if qgram.loc > loc {
            cnt += 1;
            loc = qgram.loc + q - 1;
        }
    });

    cnt
}

// Algorithm 3
// NOTE: PosQGram is not by default sorted in increasing frequency
/**
Given a set of q-grams, find the minimum length of prefix such that if all the q-grams in the prefix are mismatched, it will incur at least `tau + ` edit errors.

# Parameters
 * `qgram_array`: A PosQGramArray, i.e. a set of positional q-grams.
 * `q`: A positive integer as the tuning parameter for length of q-grams.
 * `tau`: A positive integer as the tuning parameter for threshold for matching.

# Return
The minimum length of prefix such that if all the q-grams in the prefix are mismatched, it will incur at least `tau + ` edit errors.
 */
fn calc_prefix_len(
    qgram_array: &mut PosQGramArray,
    inverted: &InvertedIndex,
    q: usize,
    tau: usize,
) -> usize {
    let mut left: usize = tau + 1;
    let mut right: usize = q * tau + 1;
    let mut mid: usize;
    let mut err: usize;
    let qgram_len: usize = qgram_array.len();

    // PosQGramArray is sorted in increasing order of location, but we need to sort it in increasing order of frequency
    qgram_array.par_sort_by_key(|qgram| inverted.get(&qgram.token).unwrap().len());

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
    trace!("CalcPrefix for `{}`, length: {}", &qgram_array, &left);
    left
}

// Algorithm 1
/**
Given a input file, and two tuning parameters, `q` and `tau`, find all matching pairs in the input file.

# Parameters
 * `doc`: A path to a input file.
 * `q`: A positive integer as the tuning parameter for length of q-grams.
 * `tau`: A positive integer as the tuning parameter for threshold for matching.

# Return
All matching pairs in the input file.
 */
pub fn ed_join(doc: &PathBuf, q: usize, tau: usize) -> Result<()> {
    let file: File = File::open(doc)?;
    let mut reader: BufReader<File> = BufReader::new(file);
    let mut buffer: String = String::new();
    reader.read_to_string(&mut buffer)?;
    // make it as Vec<Vec<u8>> so we can quickly index it by line id
    let buffer_vec: Vec<Vec<u8>> = buffer.par_lines().map(Vec::from).collect();
    #[cfg(feature = "cli")]
    trace!(
        "{:?}",
        buffer_vec
            .iter()
            .map(|x| std::str::from_utf8(x).unwrap().to_string())
            .collect::<Vec<_>>()
    );

    let out_name: PathBuf = PathBuf::from(
        format!(
            "{}_out_q{}_tau{}.{}",
            doc.file_stem().unwrap().to_str().unwrap(),
            q,
            tau,
            // note that extension may be empty
            doc.extension()
                .unwrap_or_else(|| std::ffi::OsStr::new("txt"))
                .to_str()
                .unwrap()
        )
        .to_string(),
    );
    let doc_out: File = File::create(&out_name).expect("Failed to Create File");
    let mut writer: BufWriter<File> = BufWriter::new(doc_out);
    let out_vec_protected: Arc<Mutex<Vec<(ID, Vec<(ID, usize)>)>>> =
        Arc::new(Mutex::new(Vec::new()));

    let inverted_index: InvertedIndex = generate_inverted_index(doc, q)?;
    #[cfg(feature = "cli")]
    info!("Inverted Index: {:?}", inverted_index);

    #[cfg(not(feature = "cli"))]
    let buffer_iter = buffer_vec.par_iter().enumerate();
    #[cfg(feature = "cli")]
    let buffer_iter;
    #[cfg(feature = "cli")]
    {
        // progress bar
        let pbar: ProgressBar =
            ProgressBarBuilder::new(buffer.par_lines().count(), "Processing").build();
        buffer_iter = buffer_vec.par_iter().enumerate().progress_with(pbar);
    }

    buffer_iter.for_each(|(line_id, line_vec)| {
        let line: &str = std::str::from_utf8(line_vec).unwrap();

        let candidates_protected: Arc<Mutex<HashSet<ID>>> = Arc::new(Mutex::new(HashSet::new()));
        let mut qgram_array: PosQGramArray = PosQGramArray::from(line, q);

        let prefix_len: usize = calc_prefix_len(&mut qgram_array, &inverted_index, q, tau);
        #[cfg(feature = "cli")]
        trace!("Prefix Len: {}", prefix_len);
        qgram_array.par_iter().take(prefix_len).for_each(|qgram| {
            let candidates_clone = candidates_protected.clone();

            let token_x: &Token = &qgram.token;
            let loc_x: &Loc = &qgram.loc;

            let inverted_list = &inverted_index[token_x];
            inverted_list.par_iter()
                // since this is self-join, filter to avoid duplication
                .filter(|qgram| qgram.0 > line_id)
                .for_each(|(id, loc)| {
                let mut candidates_guard = candidates_clone.lock();
                #[cfg(feature = "cli")]
                trace!(
                    "x: {} x.id: {} \n I-list of `{}`: {:?} \n y: {} y.id: {} contained: {} \n x.len: {} y.len: {} \n x.loc: {} y.loc: {}",
                    line,
                    line_id,
                    token_x,
                    inverted_list,
                    std::str::from_utf8(&buffer_vec[*id]).unwrap(),
                    id,
                    candidates_guard.contains(id),
                    line.len(),
                    std::str::from_utf8(&buffer_vec[*id]).unwrap().len(),
                    loc_x,
                    loc
                );
                if !candidates_guard.contains(id)
                    // length filter
                    && (std::str::from_utf8(&buffer_vec[*id]).unwrap().len() as isize - line.len() as isize).abs() <= tau as isize
                    // position filter
                    && (*loc_x as isize - *loc as isize).abs() <= tau as isize
                {
                    #[cfg(feature = "cli")]
                    trace!("insert `{}` for `{}`",
                           line,
                           std::str::from_utf8(&buffer_vec[*id]).unwrap());
                    candidates_guard.insert(*id);
                }
            });
        });
        let candidates_set: HashSet<ID> = Arc::try_unwrap(candidates_protected)
            .expect("Arc is weak")
            .into_inner();
        let candidates: Vec<&ID> = candidates_set
            .par_iter()
            .collect();

        #[cfg(feature = "cli")]
        info!("Candidate of `{}`: {:?}", line, candidates);

        let out_vec_clone = out_vec_protected.clone();

        if let Some(v) = verify(line_id, candidates, buffer_vec.clone(), &inverted_index, q, tau) {
            let mut out_vec_guard = out_vec_clone.lock();
            out_vec_guard.push(v);
        };
    });

    let mut out: Vec<(ID, Vec<(ID, usize)>)> = Arc::try_unwrap(out_vec_protected)
        .expect("Arc is weak")
        .into_inner();
    out.par_sort_by_key(|x| x.0);

    out.iter().for_each(|(id_x, pairs)| {
        pairs.iter().for_each(|(id_y, ed)| {
            writer
                .write_all(format!("{},{},{}\n", id_x, id_y, ed).as_bytes())
                .expect("Failed to write to output file.");
        })
    });

    Ok(())
}
