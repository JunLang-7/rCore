use alloc::vec::Vec;
use easy_fs::BlockDevice;
use lazy_static::*;
use virtio_drivers::{Hal, VirtIOBlk, VirtIOHeader};

use crate::{
    mm::{
        FrameTracker, PageTable, PhysAddr, PhysPageNum, StepByOne, VirtAddr, frame_alloc,
        frame_dealloc, kernel_token,
    },
    sync::UPSafeCell,
};

/// The base address of control registers in VirtIO block device
const VIRTIO0: usize = 0x10001000;
/// VirtIOBlock device driver structure for virtio block device
pub struct VirtIOBlock(UPSafeCell<VirtIOBlk<'static, VirtioHal>>);

lazy_static! {
    static ref QUEUE_FRAMES: UPSafeCell<Vec<FrameTracker>> = unsafe { UPSafeCell::new(Vec::new()) };
}

impl VirtIOBlock {
    /// Create a VirtIOBlock driver with VIRTIO0 base address for virtio block device
    pub fn new() -> Self {
        unsafe {
            Self(UPSafeCell::new(
                VirtIOBlk::<VirtioHal>::new(&mut *(VIRTIO0 as *mut VirtIOHeader)).unwrap(),
            ))
        }
    }
}

impl BlockDevice for VirtIOBlock {
    fn read_block(&self, block_id: usize, buf: &mut [u8]) {
        self.0
            .exclusive_access()
            .read_block(block_id, buf)
            .expect("Error when reading VirtIOBlock");
    }

    fn write_block(&self, block_id: usize, buf: &[u8]) {
        self.0
            .exclusive_access()
            .write_block(block_id, buf)
            .expect("Error when writing VirtIOBlock");
    }
}

pub struct VirtioHal;

impl Hal for VirtioHal {
    fn dma_alloc(pages: usize) -> usize {
        let mut ppn_base = PhysPageNum(0);
        for i in 0..pages {
            let frame = frame_alloc().unwrap();
            if i == 0 {
                ppn_base = frame.ppn;
            }
            assert_eq!(frame.ppn.0, ppn_base.0 + i);
            QUEUE_FRAMES.exclusive_access().push(frame);
        }
        let pa: PhysAddr = ppn_base.into();
        pa.0
    }
    fn dma_dealloc(pa: usize, pages: usize) -> i32 {
        let pa = PhysAddr::from(pa);
        let mut ppn_base: PhysPageNum = pa.into();
        for _ in 0..pages {
            frame_dealloc(ppn_base);
            ppn_base.step();
        }
        0
    }
    fn phys_to_virt(addr: usize) -> usize {
        addr
    }
    fn virt_to_phys(addr: usize) -> usize {
        PageTable::from_token(kernel_token())
            .translate_va(VirtAddr::from(addr))
            .unwrap()
            .into()
    }
}
