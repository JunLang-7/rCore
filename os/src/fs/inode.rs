use alloc::{sync::Arc, vec::Vec};
use easy_fs::{EasyFileSystem, Inode};
use lazy_static::*;

use super::File;
use crate::{
    drivers::BLOCK_DEVICE,
    fs::{Stat, StatMode},
    mm::UserBuffer,
    sync::UPSafeCell,
};

/// inode in memory
/// A wrapper around a file system inode
/// to implement the File trait atop
pub struct OSInode {
    readable: bool,
    writable: bool,
    inner: UPSafeCell<OSInodeInner>,
}

/// The OSInode inner structure
pub struct OSInodeInner {
    offset: usize,
    inode: Arc<Inode>,
}

impl OSInode {
    /// Create a new OSInode
    pub fn new(readable: bool, writable: bool, inode: Arc<Inode>) -> Self {
        Self {
            readable,
            writable,
            inner: unsafe { UPSafeCell::new(OSInodeInner { offset: 0, inode }) },
        }
    }
    /// read all data from the inode
    pub fn read_all(&self) -> Vec<u8> {
        let mut inner = self.inner.exclusive_access();
        let mut buffer: Vec<u8> = Vec::with_capacity(512);
        buffer.resize(512, 0);
        let mut v: Vec<u8> = Vec::new();
        loop {
            let len = inner.inode.read_at(inner.offset, &mut buffer);
            if len == 0 {
                break;
            }
            inner.offset += len;
            v.extend_from_slice(&buffer[..len]);
        }
        v
    }

    /// Collect metadata for this inode
    pub fn metadata(&self) -> Stat {
        let inode = {
            let inner = self.inner.exclusive_access();
            inner.inode.clone()
        };
        let inode_id = inode.inode_id();
        let mode = if inode.is_dir() {
            StatMode::DIR
        } else {
            StatMode::FILE
        };
        let nlink = ROOT_INODE.link_count(inode_id);
        Stat {
            dev: 0,
            ino: inode_id as u64,
            mode,
            nlink,
            pad: [0; 7],
        }
    }
}

impl File for OSInode {
    fn readable(&self) -> bool {
        self.readable
    }
    fn writable(&self) -> bool {
        self.writable
    }
    fn read(&self, mut buf: UserBuffer) -> usize {
        let mut inner = self.inner.exclusive_access();
        let mut total_read_size = 0usize;
        for slice in buf.buffers.iter_mut() {
            let read_size = inner.inode.read_at(inner.offset, *slice);
            if read_size == 0 {
                break;
            }
            inner.offset += read_size;
            total_read_size += read_size;
        }
        total_read_size
    }
    fn write(&self, mut buf: UserBuffer) -> usize {
        let mut inner = self.inner.exclusive_access();
        let mut total_write_size = 0usize;
        for slice in buf.buffers.iter_mut() {
            let write_size = inner.inode.write_at(inner.offset, *slice);
            assert_eq!(write_size, slice.len());
            inner.offset += write_size;
            total_write_size += write_size;
        }
        total_write_size
    }
    fn stat(&self, stat: &mut Stat) -> isize {
        *stat = self.metadata();
        0
    }
}

lazy_static! {
    pub static ref ROOT_INODE: Arc<Inode> = {
        let efs = EasyFileSystem::open(BLOCK_DEVICE.clone());
        Arc::new(EasyFileSystem::root_inode(&efs))
    };
}

/// list all apps in the root directory
pub fn list_apps() {
    println!("/**** APPS ****/");
    for app in ROOT_INODE.ls() {
        println!("{}", app);
    }
    println!("/**************/");
}

bitflags! {
    /// The flags argument for open() system call is constructed by ORing together zero or more of the following values.
    pub struct OpenFlags: u32 {
        /// read only
        const RDONLY = 0;
        /// write only
        const WRONLY = 1 << 0;
        /// read and write
        const RDWR = 1 << 1;
        /// create new file
        const CREATE = 1 << 9;
        /// truncate file to zero length
        const TRUNC = 1 << 10;
    }
}

impl OpenFlags {
    /// Do not check validity for simplicity
    /// Return (readable, writable)
    pub fn read_write(&self) -> (bool, bool) {
        if self.is_empty() {
            (true, false)
        } else if self.contains(OpenFlags::WRONLY) {
            (false, true)
        } else {
            (true, true)
        }
    }
}

/// Open a file
pub fn open_file(name: &str, flags: OpenFlags) -> Option<Arc<OSInode>> {
    let (readable, writable) = flags.read_write();
    if flags.contains(OpenFlags::CREATE) {
        if let Some(inode) = ROOT_INODE.find(name) {
            // clear size
            inode.clear();
            Some(Arc::new(OSInode::new(readable, writable, inode)))
        } else {
            // create file
            ROOT_INODE
                .create(name)
                .map(|inode| Arc::new(OSInode::new(readable, writable, inode)))
        }
    } else {
        ROOT_INODE.find(name).map(|inode| {
            if flags.contains(OpenFlags::TRUNC) {
                inode.clear();
            }
            Arc::new(OSInode::new(readable, writable, inode))
        })
    }
}

/// Create a hard link to an existing file located at root directory
pub fn link_file(old_name: &str, new_name: &str) -> Result<(), ()> {
    if old_name == new_name {
        return Err(());
    }
    let old_inode = ROOT_INODE.find(old_name).ok_or(())?;
    if old_inode.is_dir() {
        return Err(());
    }
    ROOT_INODE
        .add_dirent(new_name, old_inode.inode_id())
        .ok_or(())
        .map(|_| ())
}

/// Remove a directory entry from the root directory
pub fn unlink_file(name: &str) -> Result<(), ()> {
    // prevent unlinking of non-existent files or directories
    let target = ROOT_INODE.find(name).ok_or(())?;
    if target.is_dir() {
        return Err(());
    }
    let inode_id = target.inode_id();
    let removed = ROOT_INODE.remove_dirent(name).ok_or(())?;
    debug_assert_eq!(removed, inode_id);
    if ROOT_INODE.link_count(inode_id) == 0 {
        target.dealloc();
    }
    Ok(())
}
