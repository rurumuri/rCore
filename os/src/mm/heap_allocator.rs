/*
    enabling dynamic memory allocating IN KERNEL, and then Rust lib `alloc` including `Box`, `Vec`, ... is available.
*/

use crate::config::KERNEL_HEAP_SIZE;
use buddy_system_allocator::LockedHeap;
use log::info;

#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap = LockedHeap::empty();

static mut HEAP_SPACE: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];

pub fn init_heap() {
    let mut ha = HEAP_ALLOCATOR.lock();
    unsafe {
        ha.init(HEAP_SPACE.as_ptr() as usize, KERNEL_HEAP_SIZE);
        info!(
            "[kernel] kernel heap start at {:#x}",
            HEAP_SPACE.as_ptr() as usize
        );
        info!(
            "[kernel] kernel heap end at {:#x}",
            HEAP_SPACE.as_ptr() as usize + KERNEL_HEAP_SIZE
        );
    }
}

#[alloc_error_handler]
pub fn handle_alloc_error(layout: core::alloc::Layout) -> ! {
    panic!("Heap allocation error, layout = {:?}", layout);
}

#[allow(unused)]
#[no_mangle]
pub fn heap_test() {
    use alloc::boxed::Box;
    use alloc::vec::Vec;
    extern "C" {
        fn sbss();
        fn ebss();
    }
    let bss_range = sbss as usize..ebss as usize;

    // check if dynamic alloced memory is in bss range.
    // if `HEAP_SPACE` is `static` without `mut`, `a` will be in `.rodata`
    let a = Box::new(5);
    println!("a at {:#x}", &(a.as_ref() as *const _ as usize));
    assert_eq!(*a, 5);
    assert!(bss_range.contains(&(a.as_ref() as *const _ as usize)));
    drop(a);

    let mut v: Vec<usize> = Vec::new();
    let n = 1024; // change elements number here
    for i in 0..n {
        v.push(i);
    }
    for i in 0..n {
        assert_eq!(v[i], i);
    }
    assert!(bss_range.contains(&(v.as_ptr() as usize)));
    drop(v);

    println!("heap_test passed!");
}
