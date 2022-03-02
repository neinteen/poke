use crate::{helpers::*, Error};
use clap::{AppSettings, Parser};
use filetime::FileTime;

use std::{env, fs, time::SystemTime};

#[derive(Debug, Parser)]
#[clap(name = "Poke")]
#[clap(author = "neinteen")]
#[clap(version)]
#[clap(setting = AppSettings::ArgRequiredElseHelp)]
/// Inspired by the classic touch command.
pub struct Poke {
    /// change only the access time.
    #[clap(short)]
    pub(crate) access_time: bool,
    /// delete given file(s).
    #[clap(short,
        conflicts_with_all = &["access-time", "no-create", "date", "modification-time", "reference-file"])]
    pub(crate) bin: bool,
    /// if not found, don't create new file(s).
    #[clap(short = 'c', long)]
    pub(crate) no_create: bool,
    /// use this date string instead of current time.
    /// The Supported formats are:
    /// "1/01/2001 00:00",
    /// "1 jan(uary) 2001 0am",
    /// "last friday 2pm" and so on.
    /// for more information: https://crates.io/crates/chrono-english
    #[clap(short, long, conflicts_with = "reference-file")]
    pub(crate) date: Option<String>,
    /// change only the modification time.
    #[clap(short)]
    pub(crate) modification_time: bool,
    /// use this files time, instead of current time.
    #[clap(short, long, conflicts_with = "date")]
    pub(crate) reference_file: Option<String>,
    /// the file(s) to be modified.
    #[clap(required = true)]
    pub(crate) files: Vec<String>,
}
impl Poke {
    pub fn run(self) -> Result<(), Error> {
        // Get the current directory
        let current_dir = env::current_dir()?;

        // returns the filetime from the date, ref_file or timestamp.
        let timestamp = if let Some(ref date) = self.date {
            let time = parse_date(date)?;
            (time, time)
        } else if let Some(ref f) = self.reference_file {
            get_file_times(current_dir.join(f))?
        } else {
            let now = FileTime::from_system_time(SystemTime::now());
            (now, now)
        };

        for file in self.files {
            file_name_is_legal(&file)?;
            let path = current_dir.join(&file);

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
            }

            if !(self.date.is_none() && self.reference_file.is_none()) {
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
        }
        Ok(())
    }
}
