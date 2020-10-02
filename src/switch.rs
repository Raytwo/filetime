use crate::FileTime;
use std::fs::{self, File, OpenOptions};
use std::io;
use std::path::Path;

use skyline::nn;

pub fn set_file_times(p: &Path, atime: FileTime, mtime: FileTime) -> io::Result<()> {
    let f = OpenOptions::new()
        .write(true)
        .open(p)?;
    set_file_handle_times(&f, Some(atime), Some(mtime))
}

pub fn set_file_mtime(p: &Path, mtime: FileTime) -> io::Result<()> {
    let f = OpenOptions::new()
        .write(true)
        .open(p)?;
    set_file_handle_times(&f, None, Some(mtime))
}

pub fn set_file_atime(p: &Path, atime: FileTime) -> io::Result<()> {
    let f = OpenOptions::new()
        .write(true)
        .open(p)?;
    set_file_handle_times(&f, Some(atime), None)
}

pub fn set_file_handle_times(
    f: &File,
    atime: Option<FileTime>,
    mtime: Option<FileTime>,
) -> io::Result<()> {
    return unsafe {
        use std::os::switch::ext::fs::FileExt;
        let mut timestamp = nn::fs::FileTimeStamp::default();
        let ret = nn::fs::GetFileTimeStampForDebug(&mut timestamp, skyline::c_str(f.path().unwrap().to_str().unwrap()));
        if ret != 0 {
            if let Some(mut atime) = atime {
                atime.seconds = timestamp.access.time as _;
            }
            if let Some(mut mtime) = mtime {
                mtime.seconds = timestamp.modify.time as _;
            }
            Ok(())
        } else {
            Err(io::Error::last_os_error())
        }
    };
}

pub fn set_symlink_file_times(p: &Path, atime: FileTime, mtime: FileTime) -> io::Result<()> {
    Err(io::Error::new(io::ErrorKind::Other, "Switch not implemented"))
}

pub fn from_last_modification_time(meta: &fs::Metadata) -> FileTime {
    unimplemented!()
}

pub fn from_last_access_time(meta: &fs::Metadata) -> FileTime {
    unimplemented!()
}

pub fn from_creation_time(meta: &fs::Metadata) -> Option<FileTime> {
    unimplemented!()
}