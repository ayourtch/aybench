use std::arch::asm;

// https://developer.arm.com/documentation/ddi0595/2021-12/AArch64-Registers/CNTVCT-EL0--Counter-timer-Virtual-Count-register
// It's not quite the rdtsc but close enough.
#[cfg(any(target_arch = "arm64", target_arch = "aarch64"))]
pub fn rdtscp() -> u64 {
    let mut val: u64;
    unsafe {
        asm!(
            "mrs {0}, cntvct_el0",
            out(reg) val,
            options(nostack)
        );
    }

    val
}

/* https://lukas-prokop.at/articles/2021-11-10-rdtsc-with-rust-asm */
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn rdtscp() -> u64 {
    let eax: u32;
    let ecx: u32;
    let edx: u32;
    {
        unsafe {
            asm!(
              "rdtscp",
              lateout("eax") eax,
              lateout("ecx") ecx,
              lateout("edx") edx,
              options(nomem, nostack)
            );
        }
    }

    let counter: u64 = (edx as u64) << 32 | eax as u64;
    counter
}
