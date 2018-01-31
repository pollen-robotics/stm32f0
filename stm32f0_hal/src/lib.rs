#![no_std]
#![cfg_attr(feature = "use_alloc", feature(global_allocator))]

extern crate cortex_m;
extern crate embedded_hal as hal;

pub extern crate stm32f0x2;

pub mod delay;
pub mod gpio;
pub mod flash;
pub mod rcc;
pub mod time;

#[cfg(feature = "use_alloc")]
extern crate alloc_cortex_m as heap;
#[cfg(feature = "use_alloc")]
#[global_allocator]
static ALLOCATOR: heap::CortexMHeap = heap::CortexMHeap::empty();
#[cfg(feature = "use_alloc")]
pub mod allocator {
    extern "C" {
        static mut _sheap: u32;
    }

    pub fn setup(heap_size: usize) {
        let heap_start = unsafe { &mut _sheap as *mut u32 as usize };
        unsafe {
            ::ALLOCATOR.init(heap_start, heap_size);
        }
    }
}
