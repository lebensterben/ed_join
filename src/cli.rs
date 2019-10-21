use clap::App;
use dialoguer::{theme::ColorfulTheme, Confirmation, Input};
use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};
use std::{marker::PhantomData, path::PathBuf};

use crate::errors::*;

pub(crate) struct ProgressBarBuilder<'a, T> {
    count: u64,
    messege: &'a str,
    phantom: PhantomData<T>,
}

impl<'a, T> ProgressBarBuilder<'a, T> {
    pub fn new(count: T, messege: &'a str) -> Self
    where
        T: std::convert::TryInto<u64> + std::fmt::Debug,
    {
        Self {
            count: count.try_into().ok().unwrap(),
            messege,
            phantom: PhantomData,
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

pub(crate) struct Config {
    pub filepath: PathBuf,
    pub q: usize,
    pub tau: usize,
}

fn input_file_validator(f: &str) -> Result<PathBuf> {
    if PathBuf::from(&f).is_file() {
        Ok(PathBuf::from(&f))
    } else {
        bail!(ErrorKind::InputFileNotReadable(f.to_string()));
    }
}

fn q_validator(v: &str) -> Result<usize> {
    match v.parse::<usize>() {
        Ok(q) if q >= 1 && q <= 10 => Ok(q),
        Ok(_) | Err(_) => bail!(ErrorKind::InvalidParameterQ(v.to_string())),
    }
}

fn tau_validator(v: &str) -> Result<usize> {
    match v.parse::<usize>() {
        Ok(t) if t >= 1 => Ok(t),
        Ok(_) | Err(_) => bail!(ErrorKind::InvalidParameterTau(v.to_string())),
    }
}

pub(crate) fn parse_config() -> Result<Config> {
    let matches = App::new("EdJoin")
        .author("Lucius Hu")
        .version(clap::crate_version!())
        .about("String Similarity Join with Ed-Join Algorithm")
        .usage(
            "\
             ed_join [-f filepath] [-q tau] [-t tau]",
        )
        .args_from_usage(
            "\
            [filepath] -f --file=[INPUT FILE] 'Specify the file to be processed' \n
            [q] -q [INTEGER] 'Specify the parameter `q` as used in `q-gram`' \n
            [tau] -t [INTEGER] 'Specify the parameter `tau`' \n
            [interactive] -i, --interactive 'Interactive mode' ",
        )
        .get_matches();

    println!("Ed-Join by Lucius Hu <orctarorga@gmail.com>");

    // Get `filepath` from user input or fallback to default "./testset/sample_test1.txt"
    // Throw an error if user-provided file is not readable, or when use default but default is not readable
    let mut filepath: PathBuf = input_file_validator(
        matches
            .value_of("filepath")
            .unwrap_or("./testset/sample_test1.txt"),
    )?;

    // Get `q` from user input or fallback to default value 2
    // Throw an error if user-provided value is not a valid positive integer
    let mut q: usize = q_validator(matches.value_of("q").unwrap_or("2"))?;

    // Get `tau` from user input or fallback to default value 2
    // Throw an error if user-provided value is not a valid positive integer
    let mut tau: usize = tau_validator(matches.value_of("tau").unwrap_or("2"))?;

    let theme: ColorfulTheme = ColorfulTheme::default();

    if matches.is_present("interactive")
        && !Confirmation::with_theme(&theme)
            .with_text(
                &format!(
                    "Do you want to accept those values? [Filepath: {}, q = {}, tau = {}]: ",
                    filepath.to_str().unwrap(),
                    q,
                    tau,
                )
                .to_string(),
            )
            .interact()?
    {
        loop {
            filepath = PathBuf::from(
                Input::with_theme(&theme)
                    .with_prompt("File to be processed")
                    .default("./testset/sample_test1.txt".to_string())
                    .validate_with(|f: &str| -> Result<()> {
                        if std::path::Path::new(f).is_file() {
                            Ok(())
                        } else {
                            bail!(ErrorKind::InputFileNotReadable(f.to_string()))
                        }
                    })
                    .interact()?,
            );

            q = Input::with_theme(&theme)
                .with_prompt("q")
                .default(2)
                .validate_with(|q: &str| -> Result<()> {
                    match q.parse::<usize>() {
                        Ok(q) if q >= 1 && q <= 10 => Ok(()),
                        Ok(_) | Err(_) => bail!(ErrorKind::InvalidParameterQ(q.to_string())),
                    }
                })
                .interact()?;

            tau = Input::with_theme(&theme)
                .with_prompt("tau")
                .default(2)
                .validate_with(|t: &str| -> Result<()> {
                    match t.parse::<usize>() {
                        Ok(t) if t >= 1 => Ok(()),
                        Ok(_) | Err(_) => bail!(ErrorKind::InvalidParameterTau(t.to_string())),
                    }
                })
                .interact()?;

            if !Confirmation::with_theme(&theme)
                .with_text(
                    &format!(
                        "Do you want to accept those values? [Filepath: {}, q = {}, tau = {}]: ",
                        filepath.to_str().unwrap(),
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
    Ok(Config { filepath, q, tau })
}
