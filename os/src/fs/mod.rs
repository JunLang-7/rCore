//! File trait & inode(dir, file, pipe, stdin, stdout)

mod inode;
mod stdio;
use crate::mm::UserBuffer;

pub use inode::{OpenFlags, list_apps, open_file};
pub use stdio::{Stdin, Stdout};

/// trait File for all file types
pub trait File: Send + Sync {
    /// the file readable?
    fn readable(&self) -> bool;
    /// the file writable?
    fn writable(&self) -> bool;
    /// read from file to buffer, return number of bytes read
    fn read(&self, buf: UserBuffer) -> usize;
    /// write to file from buffer, return number of bytes written
    fn write(&self, buf: UserBuffer) -> usize;
}
