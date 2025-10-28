use super::CpuBarrierKind;
use crate::BarrierType;

#[inline]
pub fn mem_barrier(kind: CpuBarrierKind, ty: BarrierType) {
    // SAFETY: This is just a memory ordering fence.
    unsafe {
        match (kind, ty) {
            (CpuBarrierKind::Mmio | CpuBarrierKind::Dma, BarrierType::General) => {
                core::arch::asm!("fence iorw, iorw", options(preserves_flags, nostack));
            }
            (CpuBarrierKind::Mmio | CpuBarrierKind::Dma, BarrierType::Read) => {
                core::arch::asm!("fence ir, ir", options(preserves_flags, nostack));
            }
            (CpuBarrierKind::Mmio | CpuBarrierKind::Dma, BarrierType::Write) => {
                core::arch::asm!("fence ow, ow", options(preserves_flags, nostack));
            }
            (CpuBarrierKind::Smp, BarrierType::General) => {
                core::arch::asm!("fence rw, rw", options(preserves_flags, nostack));
            }
            (CpuBarrierKind::Smp, BarrierType::Read) => {
                core::arch::asm!("fence r, r", options(preserves_flags, nostack));
            }
            (CpuBarrierKind::Smp, BarrierType::Write) => {
                core::arch::asm!("fence w, w", options(preserves_flags, nostack));
            }
        }
    }
}
