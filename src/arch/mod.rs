cfg_if::cfg_if! {
    if #[cfg(target_arch = "aarch64")] {
        mod aarch64;
        pub use self::aarch64::*;
    } else if #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))] {
        mod riscv;
        pub use self::riscv::*;
    } else if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
        mod x86;
        pub use self::x86::*;
    }
}

pub enum CpuBarrierKind {
    Mmio,
    Smp,
    Dma,
}
