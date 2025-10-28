use super::CpuBarrierKind;
use crate::BarrierType;

#[cfg(feature = "stdarch")]
#[inline]
pub fn mem_barrier(_kind: CpuBarrierKind, ty: BarrierType) {
    #[cfg(target_arch = "x86")]
    use core::arch::x86::{_mm_lfence, _mm_mfence, _mm_sfence};
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::{_mm_lfence, _mm_mfence, _mm_sfence};

    // SAFETY: This is just a memory fence.
    unsafe {
        match ty {
            BarrierType::General => {
                _mm_mfence();
            }
            BarrierType::Read => {
                _mm_lfence();
            }
            BarrierType::Write => {
                _mm_sfence();
            }
        }
    }
}

#[cfg(not(feature = "stdarch"))]
#[inline]
pub fn mem_barrier(_kind: CpuBarrierKind, ty: BarrierType) {
    // SAFETY: This is just a memory fence.
    unsafe {
        match ty {
            BarrierType::General => {
                core::arch::asm!("mfence", options(preserves_flags, nostack));
            }
            BarrierType::Read => {
                core::arch::asm!("lfence", options(preserves_flags, nostack));
            }
            BarrierType::Write => {
                core::arch::asm!("sfence", options(preserves_flags, nostack));
            }
        }
    }
}
