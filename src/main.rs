mod error;
mod helpers;

pub use error::Error;

use clap::{AppSettings, Parser};
use filetime::FileTime;
use helpers::*;

use std::{env, fs, time};

#[derive(Debug, Parser)]
#[clap(name = "Poke")]
#[clap(author = "neinteen")]
#[clap(setting = AppSettings::ArgRequiredElseHelp)]
/// A Simple program that aims to replicate the functionality of touch, and expand on it.
pub struct Poke {
    /// change only the access time.
    #[clap(short)]
    pub(crate) access_time: bool,
    /// delete given files.
    #[clap(short,
        conflicts_with_all = &["access-time", "no-create", "date", "modification-time", "reference-file", "timestamp"])]
    pub(crate) bin: bool,
    /// don't create new file, if the given file wasn't found.
    #[clap(short = 'c', long)]
    pub(crate) no_create: bool,
    /// use this time string instead of current time. (coming in 0.1.3)
    #[clap(short, long, conflicts_with_all = &["reference-file", "timestamp"])]
    pub(crate) date: Option<String>,
    /// change only the modification time.
    #[clap(short)]
    pub(crate) modification_time: bool,
    /// use this files time, instead of current time. (coming in 0.1.3)
    #[clap(short, long, conflicts_with_all = &["date", "timestamp"])]
    pub(crate) reference_file: Option<String>,
    /// use this timestamp, instead of current time. (coming in 0.1.3)
    #[clap(short, long, conflicts_with_all = &["date", "reference-file"])]
    pub(crate) timestamp: Option<String>,
    /// single or multiple files.
    #[clap(required = true)]
    pub(crate) files: Vec<String>,
}
impl Poke {
    pub fn run(self) -> Result<(), Error> {
        let current_dir = env::current_dir()?;
        let timestamp = if let Some(f) = &self.reference_file {
            get_ref_file_times(current_dir.join(f))?
        } else {
            let now = FileTime::from_system_time(time::SystemTime::now());
            (now, now)
        };

        for file in self.files {
            file_name_is_legal(&file)?;
            let path = current_dir.join(file);

            if self.bin {
                delete_file(path.as_path())?;
                return Ok(());
            }

            if !file_exists(path.as_path())? {
                if self.no_create {
                    println!(
                        "The specified file doesn't exist but the --no-create flag was passed."
                    );
                    return Ok(());
                }
                fs::File::create(&path)?;

                if self.date.is_none() && self.reference_file.is_none() && self.timestamp.is_none()
                {
                    return Ok(());
                }
            }

            let handle = fs::OpenOptions::new().write(true).open(path.as_path())?;
            filetime::set_file_handle_times(
                &handle,
                if !self.modification_time {
                    Some(timestamp.0)
                } else {
                    None
                },
                if !self.access_time {
                    Some(timestamp.1)
                } else {
                    None
                },
            )?;
        }
        Ok(())
    }
}
// TODO: VERSION 0.1.3
// FIXME: Access Time not working. Generally times are acting strange. Consider writing own crate or switching.
// TODO: Implement the 3 missing features.

// TODO: FUTURE VERSIONS
// TODO: Move files command
// TODO: support ../this/syntax
fn main() {
    if let Err(e) = Poke::parse().run() {
        println!("{e}");
    }
}
