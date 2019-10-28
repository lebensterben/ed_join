use clap::App;
use dialoguer::{theme::ColorfulTheme, Confirmation, Input};
use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};
use std::{
    cmp,
    fs::File,
    io::{prelude::*, BufReader},
    path::PathBuf,
};

use crate::errors::*;

pub(crate) struct ProgressBarBuilder<'a> {
    count: u64,
    messege: &'a str,
}

impl<'a> ProgressBarBuilder<'a> {
    pub fn new(count: usize, messege: &'a str) -> Self {
        Self {
            count: count as u64,
            messege,
        }
    }

    pub fn build(&self) -> ProgressBar {
        let pbar = ProgressBar::new(self.count);
        pbar.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{msg} {spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})",
                ).progress_chars("#>-")
        );
        pbar.set_draw_target(ProgressDrawTarget::stdout());
        pbar.set_message(self.messege);

        pbar
    }
}

#[allow(dead_code)]
pub(crate) struct Config {
    pub doc_x: PathBuf,
    pub doc_y: PathBuf,
    pub q: usize,
    pub tau: usize,
}

#[allow(dead_code)]
fn input_file_validator(f: &str) -> Result<PathBuf> {
    if PathBuf::from(&f).is_file() {
        Ok(PathBuf::from(&f))
    } else {
        bail!(ErrorKind::InputFileNotReadable(f.to_string()));
    }
}

fn q_validator(v: &str, min_line_len: usize) -> Result<usize> {
    #[allow(dead_code)]
    match v.parse::<usize>() {
        Ok(q) if q >= 1 && q <= min_line_len => Ok(q),
        Ok(q) if q < 1 => bail!(ErrorKind::QTooSmall(q)),
        Ok(q) if q > min_line_len => bail!(ErrorKind::QTooLarge(q, min_line_len)),
        Ok(_) => unreachable!(),
        Err(_) => bail!("Not a valid integer: q = {}", v),
    }
}

#[allow(dead_code)]
fn tau_validator(v: &str) -> Result<usize> {
    match v.parse::<usize>() {
        Ok(t) if t >= 1 => Ok(t),
        Ok(t) => bail!(ErrorKind::TauTooSmall(t)),
        Err(_) => bail!("Not a vlid integer: t = {}", v),
    }
}

#[allow(dead_code)]
fn calc_min_line_len(doc_x: &PathBuf, doc_y: &PathBuf) -> usize {
    cmp::min(
        BufReader::new(File::open(doc_x).unwrap())
            .lines()
            .map(|line| line.unwrap().len())
            .min()
            .unwrap(),
        BufReader::new(File::open(doc_y).unwrap())
            .lines()
            .map(|line| line.unwrap().len())
            .min()
            .unwrap(),
    )
}

#[allow(dead_code)]
pub(crate) fn parse_config() -> Result<Config> {
    let matches = App::new("EdJoin")
        .author(clap::crate_authors!())
        .version(clap::crate_version!())
        .about("String Similarity Join with Ed-Join Algorithm")
        .usage(
            "\
             ed-join FILE_1 [FILE_2] [-q Q] [-t TAU]",
        )
        .args_from_usage(
            "\
            <doc_x> 'File which matches are generated for' \n
            [doc_y] '(Optional) File which matches come from' \n
            [q] -q [INTEGER] '`q` as used in `q-gram`' \n
            [tau] -t [INTEGER] '`tau` as threshold for matching' \n
            [interactive] -i, --interactive 'Interactive mode' ",
        )
        .get_matches();

    println!("Ed-Join by Lucius Hu");

    // Get `doc_x` from user input or fallback t
    // Throw an error if user-provided file is not readable
    let doc_x: PathBuf = input_file_validator(matches.value_of("doc_x").unwrap())?;

    // Get `doc_y` from user input or fallback to default as `doc_x`
    // Throw an error if user-provided file is not readable
    let doc_y: PathBuf;
    if matches.is_present("doc_y") {
        doc_y = input_file_validator(matches.value_of("doc_y").unwrap())?;
    } else {
        doc_y = doc_x.clone();
    }

    // Get `q` from user input or fallback to default value 1
    // Throw an error if user-provided value is not a valid positive integer
    let mut min_line_len = calc_min_line_len(&doc_x, &doc_y);
    let mut q: usize = q_validator(
        matches.value_of("q").unwrap_or(&min_line_len.to_string()),
        min_line_len,
    )?;

    // Get `tau` from user input or fallback to default value 2
    // Throw an error if user-provided value is not a valid positive integer
    let mut tau: usize = tau_validator(matches.value_of("tau").unwrap_or("2"))?;

    let theme: ColorfulTheme = ColorfulTheme::default();

    if matches.is_present("interactive")
        && !Confirmation::with_theme(&theme)
            .with_text(
                &format!(
                    "Do you want to accept those values? \nFile_1: {}\nFile_2: {}\nq = {}, tau = {}: ",
                    &doc_x.to_str().unwrap(),
                    &doc_y.to_str().unwrap(),
                    q,
                    tau,
                )
                .to_string(),
            )
            .interact()?
    {
        loop {
            let doc_x_t: std::io::Result<String> = Input::with_theme(&theme)
                    .with_prompt("File which matches are generated for")
                    .validate_with(|f: &str| -> Result<()> {
                        if std::path::Path::new(f).is_file() {
                            Ok(())
                        } else {
                            bail!(ErrorKind::InputFileNotReadable(f.to_string()))
                        }
                    })
                    .interact();
            let doc_x = PathBuf::from(&doc_x_t.unwrap());

            let doc_y = PathBuf::from(
                Input::with_theme(&theme)
                    .with_prompt("File which matches come from")
                    .default(doc_x.to_str().unwrap().to_string())
                    .validate_with(|f: &str| -> Result<()> {
                        if std::path::Path::new(f).is_file() {
                            Ok(())
                        } else {
                            bail!(ErrorKind::InputFileNotReadable(f.to_string()))
                        }
                    })
                    .interact()?,
            );


            min_line_len = calc_min_line_len(&doc_x.clone(), &doc_y.clone());
            q = Input::with_theme(&theme)
                .with_prompt("q")
                .default(min_line_len)
                .validate_with(move |v: &str| -> Result<()> {
                    match v.parse::<usize>() {
                        Ok(q) if q >= 1 && q <= min_line_len => Ok(()),
                        Ok(q) if q < 1 => bail!(ErrorKind::QTooSmall(q)),
                        Ok(q) if q > min_line_len => bail!(ErrorKind::QTooLarge(q, min_line_len)),
                        Ok(_) => unreachable!(),
                        Err(_) => bail!("Not a valid integer: q = {}", v),
                    }
                })
                .interact()?;

            tau = Input::with_theme(&theme)
                .with_prompt("tau")
                .default(2)
                .validate_with(|v: &str| -> Result<()> {
                    match v.parse::<usize>() {
                        Ok(t) if t >= 1 => Ok(()),
                        Ok(t) => bail!(ErrorKind::TauTooSmall(t)),
                        Err(_) => bail!("Not a vlid integer: t = {}", v),
                    }
                })
                .interact()?;

            if !Confirmation::with_theme(&theme)
                .with_text(
                    &format!(
                        "Do you want to accept those values? \nFile_1: {}\nFile_2: {}\n, q = {}, tau = {}: ",
                        &doc_x.to_str().unwrap(),
                        &doc_y.to_str().unwrap(),
                        q,
                        tau,
                    )
                    .to_string(),
                )
                .interact()?
            {
                break;
            }
        }
    }
    Ok(Config {
        doc_x,
        doc_y,
        q,
        tau,
    })
}
