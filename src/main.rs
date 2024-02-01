#![no_std]
#![no_main]

extern crate alloc;
extern crate panic_halt;

use cortex_m_rt::entry;
use eerie::runtime;
use embedded_alloc::Heap;
use rtt_target::{rprintln, rtt_init_print};
use stm32f3xx_hal::prelude::*;

#[global_allocator]
static HEAP: Heap = Heap::empty();

#[entry]
fn main() -> ! {
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 4096;
        static mut HEAP_SPACE: MaybeUninit<[u8; HEAP_SIZE]> = MaybeUninit::uninit();
        unsafe { HEAP.init(HEAP_SPACE.as_ptr() as usize, HEAP_SIZE) };
    }

    rtt_init_print!();

    let mut driver_registry = runtime::hal::DriverRegistry::new();
    let options = runtime::api::InstanceOptions::new(&mut driver_registry).use_all_available_drivers();
    let instance = runtime::api::Instance::new(&options).unwrap();
    let device = instance
        .try_create_default_device("local-sync")
        .expect("Failed to create device");
    let session = runtime::api::Session::create_with_device(
        &instance,
        &runtime::api::SessionOptions::default(),
        &device,
        )
        .unwrap();


    rprintln!("Hello, world!");

    loop {}
}
