mod error;
mod helpers;

pub use error::Error;

use clap::{Arg, ArgMatches, Command};
use filetime::FileTime;
use helpers::*;

use std::{env, fs, path::PathBuf, time};

#[derive(Debug)]
pub(crate) struct Config {
    pub bin: bool,
    pub no_create: bool,
    pub a_time: bool,
    pub m_time: bool,
    pub ref_time: Option<(FileTime, FileTime)>,
    pub files: Vec<String>,
    pub current_dir: PathBuf,
}
impl TryFrom<ArgMatches> for Config {
    type Error = crate::Error;

    fn try_from(args: ArgMatches) -> Result<Self, Error> {
        let bin = args.is_present("bin");
        let no_create = args.is_present("no_create");
        let a_time = args.is_present("access_time");
        let m_time = args.is_present("modification_time");
        let current_dir = env::current_dir()?;
        // TODO: touch -t normally only changes the modification and access times.
        // It only changes the creation time if the target time is before the original creation time.
        let ref_time = if let Some(f) = args.value_of("reference_file") {
            Some(get_ref_file_times(current_dir.join(f))?)
        } else {
            None
        };
        // NOTE: This unwrap is safe since files can't be empty.
        // Should still be replaced.
        let files = args
            .values_of("files")
            .unwrap()
            .into_iter()
            .map(|f| f.into())
            .collect();

        Ok(Self {
            bin,
            no_create,
            a_time,
            m_time,
            ref_time,
            files,
            current_dir,
        })
    }
}
impl Config {
    pub fn run(self) -> Result<(), Error> {
        self.files.iter().try_for_each(|f| poke(&f, &self))?;
        Ok(())
    }
}
fn main() {
    let args = Command::new("poke")
        .version("v0.1.1")
        .author("neinteen")
        .about("Touch-like application mainly used for creating files quickly on windows")
        .arg(
            Arg::new("bin")
                .short('b')
                .help("deletes files. incompatible with any other command.")
                .conflicts_with_all(&[
                    "access_time",
                    "no_create",
                    "date_string",
                    "modification_time",
                    "reference_file",
                ])
                .takes_value(false),
        )
        .arg(
            Arg::new("access_time")
                .short('a')
                .help("change only access time.")
                .takes_value(false),
        )
        .arg(
            Arg::new("no_create")
                .long("no-create")
                .short('c')
                .help("do not create any files.")
                .takes_value(false),
        )
        .arg(
            Arg::new("date_string")
                .long("date")
                .short('d')
                .help("parse time string and use it instead of current time. (NOT IMPLEMENTED)")
                .takes_value(true),
        )
        .arg(
            Arg::new("modification_time")
                .short('m')
                .help("change only modification time.")
                .conflicts_with("access_time")
                .takes_value(false),
        )
        .arg(
            Arg::new("reference_file")
                .long("reference")
                .short('r')
                .help("use this file's times instead of current time.")
                .conflicts_with("date_string")
                .takes_value(true),
        )
        .arg(
            Arg::new("files")
                .multiple_occurrences(true)
                .value_delimiter(' ')
                .required(true),
        )
        .arg_required_else_help(true)
        .get_matches();

    match Config::try_from(args) {
        Ok(cfg) => {
            if let Err(e) = cfg.run() {
                println!("{e}")
            }
        }
        Err(e) => {
            println!("Error parsing arguments: {e}")
        }
    }
}

fn poke(file: &str, cfg: &Config) -> Result<(), Error> {
    file_name_is_legal(file)?;
    let path = cfg.current_dir.join(file);
    if cfg.bin {
        delete_file(path.as_path())?;
        return Ok(());
    }

    let exists = file_exists(path.as_path())?;

    if !exists {
        if cfg.no_create {
            println!("The specified file doesn't exist but the --no-create flag was passed.");
            return Ok(());
        }
        fs::File::create(&path)?;

        if cfg.ref_time.is_none()
        /* && Time string none */
        {
            return Ok(());
        }
    }

    let a_time: FileTime;
    let m_time: FileTime;

    if let Some(ref_time) = cfg.ref_time {
        a_time = ref_time.0;
        m_time = ref_time.1;
    } else {
        let now = FileTime::from_system_time(time::SystemTime::now());
        a_time = now.clone();
        m_time = now;
    }

    let handle = fs::OpenOptions::new().write(true).open(path.as_path())?;
    filetime::set_file_handle_times(
        &handle,
        if !cfg.m_time { Some(a_time) } else { None },
        if !cfg.a_time { Some(m_time) } else { None },
    )?;
    Ok(())
}
