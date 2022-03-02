use chrono::Local;
use chrono_english::{parse_date_string, Dialect::Uk};
use filetime::FileTime;
use std::{fs, io::ErrorKind::NotFound, path::Path};

use crate::Error;

/// Checks if file exits, but doesn't error if the file wasn't found.
pub fn file_exists(path: impl AsRef<Path>) -> Result<bool, Error> {
    if let Err(e) = path.as_ref().metadata() {
        if e.kind() != NotFound {
            return Err(e.into());
        }
        return Ok(false);
    }
    return Ok(true);
}
/// Deletes a file, but doesn't error if the file wasn't found.
pub fn delete_file(path: impl AsRef<Path>) -> Result<(), Error> {
    if let Err(e) = fs::remove_file(path) {
        if e.kind() != NotFound {
            return Err(e.into());
        }
    }
    Ok(())
}

/// Parses a chrono_english formatted string into a FileTime.
pub fn parse_date(date: &str) -> Result<FileTime, Error> {
    Ok(FileTime::from_system_time(
        parse_date_string(date, Local::now(), Uk)?.into(),
    ))
}

/// Returns the Access and Modification time of a file.
pub fn get_file_times(path: impl AsRef<Path>) -> Result<(FileTime, FileTime), Error> {
    match fs::metadata(path.as_ref()) {
        Ok(meta) => Ok((
            FileTime::from_last_access_time(&meta),
            FileTime::from_last_modification_time(&meta),
        )),
        Err(e) => {
            if e.kind() == NotFound {
                Err(crate::Error::FileNotFound(
                    path.as_ref().to_string_lossy().to_string(),
                ))
            } else {
                Err(e.into())
            }
        }
    }
}

/// Simple check for illegal characters (windows/unix).
pub fn file_name_is_legal(name: &str) -> Result<(), Error> {
    #[cfg(target_family = "windows")]
    static LEGAL_CHARS: [char; 9] = ['<', '>', ':', '"', '/', '\\', '|', '?', '*'];
    #[cfg(target_family = "unix")]
    static LEGAL_CHARS: [char; 3] = ['/', ':', '\0'];

    #[cfg(target_family = "windows")]
    if let "CON" | "PRN" | "AUX" | "NUL" | "COM1" | "COM2" | "COM3" | "COM4" | "COM5" | "COM6"
    | "COM7" | "COM8" | "COM9" | "LPT1" | "LPT2" | "LPT3" | "LPT4" | "LPT5" | "LPT6"
    | "LPT7" | "LPT8" | "LPT9" = name
    {
        return Err(crate::Error::IllegalName(name.into()));
    }

    // NOTE: (Windows) Technically these are illegal too: 0-31 (ASCII control characters)
    for c in LEGAL_CHARS {
        if name.contains(c) {
            return Err(crate::Error::IllegalCharacter(c));
        }
    }
    Ok(())
}
