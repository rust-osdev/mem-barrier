use super::CpuBarrierKind;
use crate::BarrierType;

#[cfg(all(feature = "stdarch", feature = "nightly"))]
#[inline]
pub fn mem_barrier(kind: CpuBarrierKind, ty: BarrierType) {
    use core::arch::aarch64::{__dmb, __dsb, ISH, ISHLD, ISHST, LD, OSH, OSHLD, OSHST, ST, SY};

    // SAFETY: This is just a data synchronization barrier or data memory barrier.
    unsafe {
        match (kind, ty) {
            (CpuBarrierKind::Mmio, BarrierType::General) => __dsb(SY),
            (CpuBarrierKind::Mmio, BarrierType::Read) => __dsb(LD),
            (CpuBarrierKind::Mmio, BarrierType::Write) => __dsb(ST),
            (CpuBarrierKind::Smp, BarrierType::General) => __dmb(ISH),
            (CpuBarrierKind::Smp, BarrierType::Read) => __dmb(ISHLD),
            (CpuBarrierKind::Smp, BarrierType::Write) => __dmb(ISHST),
            (CpuBarrierKind::Dma, BarrierType::General) => __dmb(OSH),
            (CpuBarrierKind::Dma, BarrierType::Read) => __dmb(OSHLD),
            (CpuBarrierKind::Dma, BarrierType::Write) => __dmb(OSHST),
        }
    }
}

#[cfg(not(all(feature = "stdarch", feature = "nightly")))]
#[inline]
pub fn mem_barrier(kind: CpuBarrierKind, ty: BarrierType) {
    // SAFETY: This is just a data synchronization barrier or data memory barrier.
    unsafe {
        match (kind, ty) {
            (CpuBarrierKind::Mmio, BarrierType::General) => {
                core::arch::asm!("dsb sy", options(preserves_flags, nostack));
            }
            (CpuBarrierKind::Mmio, BarrierType::Read) => {
                core::arch::asm!("dsb ld", options(preserves_flags, nostack));
            }
            (CpuBarrierKind::Mmio, BarrierType::Write) => {
                core::arch::asm!("dsb st", options(preserves_flags, nostack));
            }
            (CpuBarrierKind::Smp, BarrierType::General) => {
                core::arch::asm!("dmb ish", options(preserves_flags, nostack));
            }
            (CpuBarrierKind::Smp, BarrierType::Read) => {
                core::arch::asm!("dmb ishld", options(preserves_flags, nostack));
            }
            (CpuBarrierKind::Smp, BarrierType::Write) => {
                core::arch::asm!("dmb ishst", options(preserves_flags, nostack));
            }
            (CpuBarrierKind::Dma, BarrierType::General) => {
                core::arch::asm!("dmb osh", options(preserves_flags, nostack));
            }
            (CpuBarrierKind::Dma, BarrierType::Read) => {
                core::arch::asm!("dmb oshld", options(preserves_flags, nostack));
            }
            (CpuBarrierKind::Dma, BarrierType::Write) => {
                core::arch::asm!("dmb oshst", options(preserves_flags, nostack));
            }
        }
    }
}
