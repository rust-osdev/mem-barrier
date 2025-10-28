//! Cross-architecture, no-std memory barriers.
//!
//! When compiling with optimizations, the compiler may try to improve performance by reordering independent memory accesses and instructions.
//! Modern CPUs use similar techniques for improving performance, such as out-of-order execution.
//! Memory barriers affect both the compiler and the CPU by restricting reordering of certain memory operations across these barriers respective to other CPUs or devices, allowing proper communication with them.
//!
//! To insert a memory barrier, use the [`mem_barrier`] function.
//!
//! The memory barriers provided by this crate are similar to the [Linux kernel memory barriers].
//! For more details on _that_ API, also see the [_Linux Kernel Memory Consistency Model_ (LKMM)].
//!
//! [Linux kernel memory barriers]: https://www.kernel.org/doc/html/latest/core-api/wrappers/memory-barriers.html
//! [_Linux Kernel Memory Consistency Model_ (LKMM)]: https://www.kernel.org/doc/html/latest/dev-tools/lkmm/index.html
//!
//! # Examples
//!
//! ```
//! use mem_barrier::{BarrierKind, BarrierType, mem_barrier};
//!
//! mem_barrier(BarrierKind::Mmio, BarrierType::General);
//! ```
//!
//! # Supported architectures
//!
//! | Architecture | `target_arch` | Supported |
//! | ------------ | ------------- | --------- |
//! | AArch64      | `aarch64`     | ✅        |
//! | RISC-V RV32  | `riscv32`     | ✅        |
//! | RISC-V RV64  | `riscv64`     | ✅        |
//! | x86          | `x86`         | ✅        |
//! | x86-64       | `x86_64`      | ✅        |
//!
//! # Cargo features
//!
//! This crate has the following Cargo features:
//! - `nightly`—Disabled by default, this feature enables memory barrier implementations based on unstable, nightly-only Rust features.
//! - `stdarch`—Enabled by default, this feature enables memory barrier implementations based on [`core::arch`] intrinsics.
//!   If available, these intrinsics replace the fallback implementations based on inline assembly.
//!
//! # Related crates
//!
//! Several crates provide alternative approaches to memory barriers:
//!
//! - [membarrier](https://docs.rs/membarrier) provides OS-based process-wide memory barriers.
//! - [mbarrier](https://docs.rs/mbarrier) closely resembles Linux kernel memory barriers by providing `mb`, `rmb`, and `wmb`.
//!   It also provides `smp_mb`, `smp_rmb`, and `smp_wmb`, which either are the same as the non-SMP functions or fall back to compiler fences, depending on a crate feature.
//!   It does not, however, distinguish between MMIO-suitable, SMP-suitable, and DMA-suitable memory barriers.

#![no_std]
#![cfg_attr(
    all(target_arch = "aarch64", feature = "stdarch", feature = "nightly"),
    feature(stdarch_arm_barrier)
)]

mod arch;

/// The kind of a memory barrier.
///
/// This enum determines the strength or flavor of the memory barrier.
///
/// # Current implementation
///
/// On x86, this does not affect instruction generation.
#[derive(Default, PartialEq, Eq, Clone, Copy, Debug)]
#[non_exhaustive]
pub enum BarrierKind {
    /// MMIO.
    ///
    /// This is the strongest kind of barrier.
    /// It enforces ordering on memory accesses as well as on MMIO-based device I/O.
    ///
    /// # Corresponding functions
    ///
    /// This kind of barrier corresponds to the _mandatory_ `mb`, `rmb`, and `wmb` Linux functions.
    ///
    /// # Current implementation
    ///
    /// On Arm, this runs a [DSB] instruction; see _[Data Synchronization Barrier]_.
    ///
    /// [DSB]: https://developer.arm.com/documentation/ddi0602/2025-09/Base-Instructions/DSB--Data-synchronization-barrier-
    /// [Data Synchronization Barrier]: https://developer.arm.com/documentation/102336/0100/Data-Synchronization-Barrier
    #[doc(alias = "mb")]
    #[doc(alias = "rmb")]
    #[doc(alias = "wmb")]
    #[default]
    Mmio,

    /// SMP.
    ///
    /// This kind of barrier enforces ordering on memory across an SMP system.
    /// It is also suitable to enforce ordering on memory shared from a single-CPU VM guest with an SMP host.
    ///
    /// # Corresponding functions
    ///
    /// This kind of barrier corresponds to the _VM-guest_-flavoured `virt_mb`, `virt_rmb`, and `virt_wmb` Linux functions.
    /// Those functions are equivalent to the _SMP_-flavoured `smp_mb`, `smp_rmb`, and `smp_wmb` Linux functions when SMP support is turned on.
    ///
    /// Note that this kind of barrier does not change its behavior based on the build configuration.
    ///
    /// # Current implementation
    ///
    /// On Arm, this runs a [DMB] instruction; see _[Data Memory Barrier]_.
    ///
    /// [DMB]: https://developer.arm.com/documentation/ddi0602/2025-09/Base-Instructions/DMB--Data-memory-barrier-
    /// [Data Memory Barrier]: https://developer.arm.com/documentation/102336/0100/Data-Memory-Barrier
    #[doc(alias = "smp_mb")]
    #[doc(alias = "smp_rmb")]
    #[doc(alias = "smp_wmb")]
    #[doc(alias = "virt_mb")]
    #[doc(alias = "virt_rmb")]
    #[doc(alias = "virt_wmb")]
    Smp,

    /// DMA.
    ///
    /// This kind of barrier enforces ordering on memory accessed by the CPU and DMA-capable devices.
    ///
    /// # Corresponding functions
    ///
    /// This kind of barrier corresponds to the _DMA_-flavoured `dma_mb`, `dma_rmb`, and `dma_wmb` Linux functions.
    ///
    /// # Current implementation
    ///
    /// On Arm, this runs a [DMB] instruction; see _[Data Memory Barrier]_.
    ///
    /// [DMB]: https://developer.arm.com/documentation/ddi0602/2025-09/Base-Instructions/DMB--Data-memory-barrier-
    /// [Data Memory Barrier]: https://developer.arm.com/documentation/102336/0100/Data-Memory-Barrier
    ///
    /// # Examples
    ///
    /// This example is inspired by the [Linux `dma_rmb` and `dma_wmb` example].
    ///
    /// [Linux `dma_rmb` and `dma_wmb` example]: https://www.kernel.org/doc/html/latest/core-api/wrappers/memory-barriers.html
    ///
    /// ```
    /// # struct Desc {
    /// #     device_owns_memory: bool,
    /// #     data: *mut [u8],
    /// # }
    /// #
    /// # impl Desc {
    /// #     fn device_owns_memory(&self) -> bool {
    /// #         self.device_owns_memory
    /// #     }
    /// #
    /// #     fn set_device_owns_memory(&mut self, device_owns_memory: bool) {
    /// #         self.device_owns_memory = device_owns_memory;
    /// #     }
    /// #
    /// #     fn data(&self) -> *mut [u8] {
    /// #         self.data
    /// #     }
    /// #
    /// #     fn set_data(&mut self, data: *mut [u8]) {
    /// #         self.data = data;
    /// #     }
    /// # }
    /// #
    /// # struct Device;
    /// #
    /// # impl Device {
    /// #     fn notify(&mut self) {}
    /// # }
    /// #
    /// # let data = core::ptr::slice_from_raw_parts_mut(core::ptr::null_mut(), 0);
    /// # let mut desc = Desc { device_owns_memory: false, data };
    /// # let mut device = Device;
    /// # let mut read_data = data;
    /// # let mut write_data = data;
    /// #
    /// use mem_barrier::{BarrierKind, BarrierType, mem_barrier};
    ///
    /// if !desc.device_owns_memory() {
    ///     // Don't read until we own the descriptor.
    ///     mem_barrier(BarrierKind::Dma, BarrierType::Read);
    ///
    ///     // Read/modify data
    ///     read_data = desc.data();
    ///     desc.set_data(write_data);
    ///
    ///     // Flush modifications.
    ///     mem_barrier(BarrierKind::Dma, BarrierType::Write);
    ///
    ///     // Give the descriptor ownership back to the device.
    ///     desc.set_device_owns_memory(true);
    ///
    ///     // Notify the device.
    ///     device.notify();
    /// }
    /// ```
    #[doc(alias = "dma_mb")]
    #[doc(alias = "dma_rmb")]
    #[doc(alias = "dma_wmb")]
    Dma,

    /// Compiler.
    ///
    /// This kind of barrier does not run any CPU instructions.
    /// Instead, it only prevents the compiler from moving memory accesses through the barrier.
    ///
    /// # Corresponding functions
    ///
    /// This kind of barrier corresponds to the `barrier` Linux function.
    Compiler,
}

/// The type of a memory barrier.
///
/// This enum determines which type of memory accesses are ordered: read, write, or both (general).
#[derive(Default, PartialEq, Eq, Clone, Copy, Debug)]
#[non_exhaustive]
pub enum BarrierType {
    /// General.
    ///
    /// This type of barrier orders both read and write memory accesses.
    ///
    /// # Corresponding functions
    ///
    /// This type of barrier corresponds to the `*_mb` family of Linux functions.
    ///
    /// # Current implementation
    ///
    /// On x86, this runs an [MFENCE] instruction.
    ///
    /// [MFENCE]: https://www.felixcloutier.com/x86/mfence
    #[doc(alias = "mb")]
    #[doc(alias = "smp_mb")]
    #[doc(alias = "virt_mb")]
    #[doc(alias = "dma_mb")]
    #[default]
    General,

    /// Read.
    ///
    /// This type of barrier orders read memory accesses.
    ///
    /// # Corresponding functions
    ///
    /// This type of barrier corresponds to the `*_rmb` family of Linux functions.
    ///
    /// # Current implementation
    ///
    /// On x86, this runs an [LFENCE] instruction.
    ///
    /// [LFENCE]: https://www.felixcloutier.com/x86/lfence
    #[doc(alias = "rmb")]
    #[doc(alias = "smp_rmb")]
    #[doc(alias = "virt_rmb")]
    #[doc(alias = "dma_rmb")]
    Read,

    /// Write.
    ///
    /// This type of barrier orders write memory accesses and corresponds to the `*_wmb` family of Linux functions.
    ///
    /// # Current implementation
    ///
    /// On x86, this runs an [SFENCE] instruction.
    ///
    /// [SFENCE]: https://www.felixcloutier.com/x86/sfence
    #[doc(alias = "wmb")]
    #[doc(alias = "smp_wmb")]
    #[doc(alias = "virt_wmb")]
    #[doc(alias = "dma_wmb")]
    Write,
}

/// A memory barrier.
///
/// This function runs the appropriate CPU instructions for enforcing memory ordering according to the provided [`BarrierKind`] and [`BarrierType`].
///
/// # Current implementation
///
/// On RISC-V, this runs a [FENCE] instruction.
///
/// [FENCE]: https://docs.riscv.org/reference/isa/unpriv/rv32.html#fence
#[inline]
pub fn mem_barrier(kind: BarrierKind, ty: BarrierType) {
    let cpu_barrier_kind = match kind {
        BarrierKind::Mmio => arch::CpuBarrierKind::Mmio,
        BarrierKind::Smp => arch::CpuBarrierKind::Smp,
        BarrierKind::Dma => arch::CpuBarrierKind::Dma,
        BarrierKind::Compiler => {
            compiler_barrier();
            return;
        }
    };

    arch::mem_barrier(cpu_barrier_kind, ty);
}

#[inline]
fn compiler_barrier() {
    // SAFETY: This asm invocation is empty.
    unsafe {
        core::arch::asm!("", options(preserves_flags, nostack));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_barrier() {
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
}
