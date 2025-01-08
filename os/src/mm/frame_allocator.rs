use crate::config::MEMORY_END;
use alloc::vec::Vec;
use lazy_static::lazy_static;
use log::{error, info, warn};

use super::address::PhysPageNum;

trait FrameAllocator {
    fn new() -> Self;
    fn alloc(&mut self) -> Option<PhysPageNum>;
    fn dealloc(&mut self, ppn: PhysPageNum);
}

pub struct StackFrameAllocator {
    current: usize, // Free memory's starting PPN
    end: usize,     // Free memory's ending PPN
    recycled: Vec<usize>,
}

impl FrameAllocator for StackFrameAllocator {
    fn new() -> Self {
        Self {
            current: 0,
            end: 0,
            recycled: Vec::new(),
        }
    }

    fn alloc(&mut self) -> Option<PhysPageNum> {
        if let Some(last_recycled) = self.recycled.pop() {
            Some(last_recycled.into())
        } else {
            if self.current == self.end {
                // error!("[kernel] Cannot allocate new physical frame");
                None
            } else {
                self.current += 1;
                Some(self.current.into())
            }
        }
    }

    fn dealloc(&mut self, ppn: PhysPageNum) {
        let ppn = ppn.0;
        // validity check
        if ppn >= self.current || self.recycled.iter().find(|&v| *v == ppn).is_some() {
            // panic!("Frame ppn={:#x} has not been allocated!", ppn);
            warn!(
                "[kernel] Frame ppn={:#x} has not been allocated, but now trying to dealloc it!",
                ppn
            );
        } else {
            // recycle
            self.recycled.push(ppn);
        }
    }
}

impl StackFrameAllocator {
    pub fn init(&mut self, l: PhysPageNum, r: PhysPageNum) {
        self.current = l.0;
        self.end = r.0;
    }
}

use crate::{mm::address::PhysAddr, sync::UPSafeCell};
type FrameAllocatorImpl = StackFrameAllocator;
lazy_static! {
    pub static ref FRAME_ALLOCATOR: UPSafeCell<FrameAllocatorImpl> =
        unsafe { UPSafeCell::new(FrameAllocatorImpl::new()) };
}

pub fn init_frame_allocator() {
    extern "C" {
        fn ekernel();
    }
    let lppn = PhysAddr::from(ekernel as usize).ceil();
    let rppn = PhysAddr::from(MEMORY_END).floor();
    info!(
        "[kernel] frame allocate range [{:#x}, {:#x}]",
        PhysAddr::from(lppn).0,
        PhysAddr::from(rppn).0
    );
    FRAME_ALLOCATOR.exclusive_access().init(lppn, rppn);
}

#[derive(Debug)]
pub struct FrameTracker {
    pub ppn: PhysPageNum,
}

impl FrameTracker {
    pub fn new(ppn: PhysPageNum) -> Self {
        // page cleaning
        let bytes_array = ppn.get_bytes_array();
        for i in bytes_array {
            *i = 0;
        }
        Self { ppn }
    }
}

impl Drop for FrameTracker {
    fn drop(&mut self) {
        frame_dealloc(self.ppn);
    }
}

pub fn frame_alloc() -> Option<FrameTracker> {
    FRAME_ALLOCATOR
        .exclusive_access()
        .alloc()
        .map(|ppn| FrameTracker::new(ppn))
}

fn frame_dealloc(ppn: PhysPageNum) {
    FRAME_ALLOCATOR.exclusive_access().dealloc(ppn);
}

#[allow(unused)]
pub fn frame_allocator_test() {
    let mut v: Vec<FrameTracker> = Vec::new();
    for i in 0..5 {
        let frame = frame_alloc().unwrap();
        println!("{:?}", frame);
        v.push(frame);
    }
    v.clear();
    for i in 0..5 {
        let frame = frame_alloc().unwrap();
        println!("{:?}", frame);
        v.push(frame);
    }
    drop(v);
    println!("frame_allocator_test passed!");
}
