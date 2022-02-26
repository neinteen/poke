use std::path::Path;
use std::{fs, io};

use filetime::FileTime;

pub fn file_exists(path: impl AsRef<Path>) -> Result<bool, io::Error> {
    match path.as_ref().metadata() {
        Ok(_) => Ok(true),
        Err(e) => {
            if e.kind() == io::ErrorKind::NotFound {
                Ok(false)
            } else {
                Err(e)
            }
        }
    }
}

pub fn delete_file(path: impl AsRef<Path>) -> Result<(), io::Error> {
    match fs::remove_file(path) {
        Ok(_) => Ok(()),
        Err(e) => {
            if e.kind() == io::ErrorKind::NotFound {
                Ok(())
            } else {
                Err(e)
            }
        }
    }
}

pub fn get_ref_file_times(path: impl AsRef<Path>) -> Result<(FileTime, FileTime), crate::Error> {
    match fs::metadata(path.as_ref()) {
        Ok(meta) => Ok((
            FileTime::from_last_access_time(&meta),
            FileTime::from_last_modification_time(&meta),
        )),
        Err(e) => {
            if e.kind() == io::ErrorKind::NotFound {
                Err(crate::Error::FileNotFound("".to_string()))
            } else {
                Err(e.into())
            }
        }
    }
}

pub fn file_name_is_legal(name: &str) -> Result<(), crate::Error> {
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
