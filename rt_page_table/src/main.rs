#![no_std]
#![no_main]

use core::panic::PanicInfo;
use axhal::mem::{memory_regions, phys_to_virt};

#[macro_use]
extern crate axlog2;

#[cfg(target_arch = "riscv64")]
const TEST_ADDRESS: usize = 0x1000_1000;

#[cfg(target_arch = "x86_64")]
const TEST_ADDRESS: usize = 0xfec00000;

#[no_mangle]
pub extern "Rust" fn runtime_main(cpu_id: usize, _dtb_pa: usize) {
    axlog2::init("debug");
    info!("[rt_page_table]: ...");

    axhal::arch_init_early(cpu_id);
    axalloc::init();
    page_table::init();

    info!("Found physcial memory regions:");
    for r in memory_regions() {
        info!(
            "  [{:x?}, {:x?}) {} ({:?})",
            r.paddr,
            r.paddr + r.size,
            r.name,
            r.flags
        );
    }

    // Try to access virtio_mmio space.
    let va = phys_to_virt(TEST_ADDRESS.into()).as_usize();
    let ptr = va as *const u32;
    unsafe {
        info!("Try to access virtio_mmio [{:#X}]", *ptr);
    }

    info!("[rt_page_table]: ok!");
    axhal::misc::terminate();
}

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    arch_boot::panic(info)
}
