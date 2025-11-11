//! x86-64 Linux system calls.

use super::Reg;
use core::arch::asm;

#[cfg(target_pointer_width = "32")]
compile_error!("x32 is not yet supported");

#[inline(always)]
pub unsafe fn syscall0_readonly<R: From<Reg>>(nr: u32) -> R {
    let r0;
    asm!(
        "syscall",
        inlateout("rax") nr as *mut () => r0,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags, readonly)
    );
    R::from(Reg(r0))
}

#[inline(always)]
pub unsafe fn syscall1<R: From<Reg>>(nr: u32, a0: impl Into<Reg>) -> R {
    let r0;
    asm!(
        "syscall",
        inlateout("rax") nr as *mut () => r0,
        in("rdi") a0.into().0,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags)
    );
    R::from(Reg(r0))
}

#[inline(always)]
pub unsafe fn syscall1_readonly<R: From<Reg>>(nr: u32, a0: impl Into<Reg>) -> R {
    let r0;
    asm!(
        "syscall",
        inlateout("rax") nr as *mut () => r0,
        in("rdi") a0.into().0,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags, readonly)
    );
    R::from(Reg(r0))
}

#[inline(always)]
pub unsafe fn syscall1_noreturn<R: From<Reg>>(nr: u32, a0: impl Into<Reg>) -> ! {
    asm!(
        "syscall",
        in("rax") nr as *mut (),
        in("rdi") a0.into().0,
        options(nostack, noreturn)
    )
}

#[inline(always)]
pub unsafe fn syscall2<R: From<Reg>>(nr: u32, a0: impl Into<Reg>, a1: impl Into<Reg>) -> R {
    let r0;
    asm!(
        "syscall",
        inlateout("rax") nr as *mut () => r0,
        in("rdi") a0.into().0,
        in("rsi") a1.into().0,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags)
    );
    R::from(Reg(r0))
}

#[inline(always)]
pub unsafe fn syscall2_readonly<R: From<Reg>>(
    nr: u32,
    a0: impl Into<Reg>,
    a1: impl Into<Reg>,
) -> R {
    let r0;
    asm!(
        "syscall",
        inlateout("rax") nr as *mut () => r0,
        in("rdi") a0.into().0,
        in("rsi") a1.into().0,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags, readonly)
    );
    R::from(Reg(r0))
}

#[inline(always)]
pub unsafe fn syscall3<R: From<Reg>>(
    nr: u32,
    a0: impl Into<Reg>,
    a1: impl Into<Reg>,
    a2: impl Into<Reg>,
) -> R {
    let r0;
    asm!(
        "syscall",
        inlateout("rax") nr as *mut () => r0,
        in("rdi") a0.into().0,
        in("rsi") a1.into().0,
        in("rdx") a2.into().0,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags)
    );
    R::from(Reg(r0))
}

#[inline(always)]
pub unsafe fn syscall3_readonly<R: From<Reg>>(
    nr: u32,
    a0: impl Into<Reg>,
    a1: impl Into<Reg>,
    a2: impl Into<Reg>,
) -> R {
    let r0;
    asm!(
        "syscall",
        inlateout("rax") nr as *mut () => r0,
        in("rdi") a0.into().0,
        in("rsi") a1.into().0,
        in("rdx") a2.into().0,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags, readonly)
    );
    R::from(Reg(r0))
}

#[inline(always)]
pub unsafe fn syscall4<R: From<Reg>>(
    nr: u32,
    a0: impl Into<Reg>,
    a1: impl Into<Reg>,
    a2: impl Into<Reg>,
    a3: impl Into<Reg>,
) -> R {
    let r0;
    asm!(
        "syscall",
        inlateout("rax") nr as *mut () => r0,
        in("rdi") a0.into().0,
        in("rsi") a1.into().0,
        in("rdx") a2.into().0,
        in("r10") a3.into().0,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags)
    );
    R::from(Reg(r0))
}

#[inline(always)]
pub unsafe fn syscall4_readonly<R: From<Reg>>(
    nr: u32,
    a0: impl Into<Reg>,
    a1: impl Into<Reg>,
    a2: impl Into<Reg>,
    a3: impl Into<Reg>,
) -> R {
    let r0;
    asm!(
        "syscall",
        inlateout("rax") nr as *mut () => r0,
        in("rdi") a0.into().0,
        in("rsi") a1.into().0,
        in("rdx") a2.into().0,
        in("r10") a3.into().0,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags, readonly)
    );
    R::from(Reg(r0))
}

#[inline(always)]
pub unsafe fn syscall5<R: From<Reg>>(
    nr: u32,
    a0: impl Into<Reg>,
    a1: impl Into<Reg>,
    a2: impl Into<Reg>,
    a3: impl Into<Reg>,
    a4: impl Into<Reg>,
) -> R {
    let r0;
    asm!(
        "syscall",
        inlateout("rax") nr as *mut () => r0,
        in("rdi") a0.into().0,
        in("rsi") a1.into().0,
        in("rdx") a2.into().0,
        in("r10") a3.into().0,
        in("r8") a4.into().0,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags)
    );
    R::from(Reg(r0))
}

#[inline(always)]
pub unsafe fn syscall5_readonly<R: From<Reg>>(
    nr: u32,
    a0: impl Into<Reg>,
    a1: impl Into<Reg>,
    a2: impl Into<Reg>,
    a3: impl Into<Reg>,
    a4: impl Into<Reg>,
) -> R {
    let r0;
    asm!(
        "syscall",
        inlateout("rax") nr as *mut () => r0,
        in("rdi") a0.into().0,
        in("rsi") a1.into().0,
        in("rdx") a2.into().0,
        in("r10") a3.into().0,
        in("r8") a4.into().0,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags, readonly)
    );
    R::from(Reg(r0))
}

#[inline(always)]
pub unsafe fn syscall6<R: From<Reg>>(
    nr: u32,
    a0: impl Into<Reg>,
    a1: impl Into<Reg>,
    a2: impl Into<Reg>,
    a3: impl Into<Reg>,
    a4: impl Into<Reg>,
    a5: impl Into<Reg>,
) -> R {
    let r0;
    asm!(
        "syscall",
        inlateout("rax") nr as *mut () => r0,
        in("rdi") a0.into().0,
        in("rsi") a1.into().0,
        in("rdx") a2.into().0,
        in("r10") a3.into().0,
        in("r8") a4.into().0,
        in("r9") a5.into().0,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags)
    );
    R::from(Reg(r0))
}

#[inline(always)]
pub unsafe fn syscall6_readonly<R: From<Reg>>(
    nr: u32,
    a0: impl Into<Reg>,
    a1: impl Into<Reg>,
    a2: impl Into<Reg>,
    a3: impl Into<Reg>,
    a4: impl Into<Reg>,
    a5: impl Into<Reg>,
) -> R {
    let r0;
    asm!(
        "syscall",
        inlateout("rax") nr as *mut () => r0,
        in("rdi") a0.into().0,
        in("rsi") a1.into().0,
        in("rdx") a2.into().0,
        in("r10") a3.into().0,
        in("r8") a4.into().0,
        in("r9") a5.into().0,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags, readonly)
    );
    R::from(Reg(r0))
}
