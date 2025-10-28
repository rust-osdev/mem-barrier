#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]

use mem_barrier::{BarrierKind, BarrierType, mem_barrier};

fn main() {
    for kind in [
        BarrierKind::Mmio,
        BarrierKind::Smp,
        BarrierKind::Dma,
        BarrierKind::Compiler,
    ] {
        for ty in [BarrierType::General, BarrierType::Read, BarrierType::Write] {
            mem_barrier(kind, ty);
        }
    }
}

#[cfg(target_os = "none")]
#[unsafe(no_mangle)]
extern "C" fn _start() -> ! {
    main();

    loop {}
}

#[cfg(target_os = "none")]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
