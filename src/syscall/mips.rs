//! mipsel Linux system calls.
//!
//! On mipsel, Linux indicates success or failure using `$a3` rather
//! than by returning a negative error code as most other architectures do.
//!
//! Mips-family platforms have a special calling convention for `__NR_pipe`,
//! however we use `__NR_pipe2` instead to avoid having to implement it.

use super::Reg;
use core::arch::asm;

#[inline(always)]
pub unsafe fn syscall0_readonly<R: From<Reg>>(nr: SyscallNumber) -> R {
    let x0;
    let err: usize;
    asm!(
        "syscall",
        inlateout("$2" /*$v0*/) nr as usize => x0,
        lateout("$7" /*$a3*/) err,
        lateout("$8" /*$t0*/) _,
        lateout("$9" /*$t1*/) _,
        lateout("$10" /*$t2*/) _,
        lateout("$11" /*$t3*/) _,
        lateout("$12" /*$t4*/) _,
        lateout("$13" /*$t5*/) _,
        lateout("$14" /*$t6*/) _,
        lateout("$15" /*$t7*/) _,
        lateout("$24" /*$t8*/) _,
        lateout("$25" /*$t9*/) _,
        options(nostack, preserves_flags, readonly)
    );
    R::from(Reg(if err != 0 {
        x0.wrapping_neg() as *mut _
    } else {
        x0 as *mut _
    }))
}

#[inline(always)]
pub unsafe fn syscall1<R: From<Reg>>(nr: u32, a0: impl Into<Reg>) -> R {
    let x0;
    let err: usize;
    asm!(
        "syscall",
        inlateout("$2" /*$v0*/) nr as usize => x0,
        in("$4" /*$a0*/) a0.0,
        lateout("$7" /*$a3*/) err,
        lateout("$8" /*$t0*/) _,
        lateout("$9" /*$t1*/) _,
        lateout("$10" /*$t2*/) _,
        lateout("$11" /*$t3*/) _,
        lateout("$12" /*$t4*/) _,
        lateout("$13" /*$t5*/) _,
        lateout("$14" /*$t6*/) _,
        lateout("$15" /*$t7*/) _,
        lateout("$24" /*$t8*/) _,
        lateout("$25" /*$t9*/) _,
        options(nostack, preserves_flags)
    );
    R::from(Reg(if err != 0 {
        x0.wrapping_neg() as *mut _
    } else {
        x0 as *mut _
    }))
}

#[inline(always)]
pub unsafe fn syscall1_readonly<R: From<Reg>>(nr: u32, a0: impl Into<Reg>) -> R {
    let x0;
    let err: usize;
    asm!(
        "syscall",
        inlateout("$2" /*$v0*/) nr as usize => x0,
        in("$4" /*$a0*/) a0.0,
        lateout("$7" /*$a3*/) err,
        lateout("$8" /*$t0*/) _,
        lateout("$9" /*$t1*/) _,
        lateout("$10" /*$t2*/) _,
        lateout("$11" /*$t3*/) _,
        lateout("$12" /*$t4*/) _,
        lateout("$13" /*$t5*/) _,
        lateout("$14" /*$t6*/) _,
        lateout("$15" /*$t7*/) _,
        lateout("$24" /*$t8*/) _,
        lateout("$25" /*$t9*/) _,
        options(nostack, preserves_flags, readonly)
    );
    R::from(Reg(if err != 0 {
        x0.wrapping_neg() as *mut _
    } else {
        x0 as *mut _
    }))
}

#[inline(always)]
pub unsafe fn syscall1_noreturn<R: From<Reg>>(nr: u32, a0: impl Into<Reg>) -> ! {
    asm!(
        "syscall",
        in("$2" /*$v0*/) nr,
        in("$4" /*$a0*/) a0.0,
        options(nostack, noreturn)
    )
}

#[inline(always)]
pub unsafe fn syscall2<R: From<Reg>>(nr: u32, a0: impl Into<Reg>, a1: impl Into<Reg>) -> R {
    let x0;
    let err: usize;
    asm!(
        "syscall",
        inlateout("$2" /*$v0*/) nr as usize => x0,
        in("$4" /*$a0*/) a0.0,
        in("$5" /*$a1*/) a1.0,
        lateout("$7" /*$a3*/) err,
        lateout("$8" /*$t0*/) _,
        lateout("$9" /*$t1*/) _,
        lateout("$10" /*$t2*/) _,
        lateout("$11" /*$t3*/) _,
        lateout("$12" /*$t4*/) _,
        lateout("$13" /*$t5*/) _,
        lateout("$14" /*$t6*/) _,
        lateout("$15" /*$t7*/) _,
        lateout("$24" /*$t8*/) _,
        lateout("$25" /*$t9*/) _,
        options(nostack, preserves_flags)
    );
    R::from(Reg(if err != 0 {
        x0.wrapping_neg() as *mut _
    } else {
        x0 as *mut _
    }))
}

#[inline(always)]
pub unsafe fn syscall2_readonly<R: From<Reg>>(
    nr: u32,
    a0: impl Into<Reg>,
    a1: impl Into<Reg>,
) -> R {
    let x0;
    let err: usize;
    asm!(
        "syscall",
        inlateout("$2" /*$v0*/) nr as usize => x0,
        in("$4" /*$a0*/) a0.0,
        in("$5" /*$a1*/) a1.0,
        lateout("$7" /*$a3*/) err,
        lateout("$8" /*$t0*/) _,
        lateout("$9" /*$t1*/) _,
        lateout("$10" /*$t2*/) _,
        lateout("$11" /*$t3*/) _,
        lateout("$12" /*$t4*/) _,
        lateout("$13" /*$t5*/) _,
        lateout("$14" /*$t6*/) _,
        lateout("$15" /*$t7*/) _,
        lateout("$24" /*$t8*/) _,
        lateout("$25" /*$t9*/) _,
        options(nostack, preserves_flags, readonly)
    );
    R::from(Reg(if err != 0 {
        x0.wrapping_neg() as *mut _
    } else {
        x0 as *mut _
    }))
}

#[inline(always)]
pub unsafe fn syscall3<R: From<Reg>>(
    nr: u32,
    a0: impl Into<Reg>,
    a1: impl Into<Reg>,
    a2: impl Into<Reg>,
) -> R {
    let x0;
    let err: usize;
    asm!(
        "syscall",
        inlateout("$2" /*$v0*/) nr as usize => x0,
        in("$4" /*$a0*/) a0.0,
        in("$5" /*$a1*/) a1.0,
        in("$6" /*$a2*/) a2.0,
        lateout("$7" /*$a3*/) err,
        lateout("$8" /*$t0*/) _,
        lateout("$9" /*$t1*/) _,
        lateout("$10" /*$t2*/) _,
        lateout("$11" /*$t3*/) _,
        lateout("$12" /*$t4*/) _,
        lateout("$13" /*$t5*/) _,
        lateout("$14" /*$t6*/) _,
        lateout("$15" /*$t7*/) _,
        lateout("$24" /*$t8*/) _,
        lateout("$25" /*$t9*/) _,
        options(nostack, preserves_flags)
    );
    R::from(Reg(if err != 0 {
        x0.wrapping_neg() as *mut _
    } else {
        x0 as *mut _
    }))
}

#[inline(always)]
pub unsafe fn syscall3_readonly<R: From<Reg>>(
    nr: u32,
    a0: impl Into<Reg>,
    a1: impl Into<Reg>,
    a2: impl Into<Reg>,
) -> R {
    let x0;
    let err: usize;
    asm!(
        "syscall",
        inlateout("$2" /*$v0*/) nr as usize => x0,
        in("$4" /*$a0*/) a0.0,
        in("$5" /*$a1*/) a1.0,
        in("$6" /*$a2*/) a2.0,
        lateout("$7" /*$a3*/) err,
        lateout("$8" /*$t0*/) _,
        lateout("$9" /*$t1*/) _,
        lateout("$10" /*$t2*/) _,
        lateout("$11" /*$t3*/) _,
        lateout("$12" /*$t4*/) _,
        lateout("$13" /*$t5*/) _,
        lateout("$14" /*$t6*/) _,
        lateout("$15" /*$t7*/) _,
        lateout("$24" /*$t8*/) _,
        lateout("$25" /*$t9*/) _,
        options(nostack, preserves_flags, readonly)
    );
    R::from(Reg(if err != 0 {
        x0.wrapping_neg() as *mut _
    } else {
        x0 as *mut _
    }))
}

#[inline(always)]
pub unsafe fn syscall4<R: From<Reg>>(
    nr: u32,
    a0: impl Into<Reg>,
    a1: impl Into<Reg>,
    a2: impl Into<Reg>,
    a3: impl Into<Reg>,
) -> R {
    let x0;
    let err: usize;
    asm!(
        "syscall",
        inlateout("$2" /*$v0*/) nr as usize => x0,
        in("$4" /*$a0*/) a0.0,
        in("$5" /*$a1*/) a1.0,
        in("$6" /*$a2*/) a2.0,
        inlateout("$7" /*$a3*/) a3.0 as usize => err,
        lateout("$8" /*$t0*/) _,
        lateout("$9" /*$t1*/) _,
        lateout("$10" /*$t2*/) _,
        lateout("$11" /*$t3*/) _,
        lateout("$12" /*$t4*/) _,
        lateout("$13" /*$t5*/) _,
        lateout("$14" /*$t6*/) _,
        lateout("$15" /*$t7*/) _,
        lateout("$24" /*$t8*/) _,
        lateout("$25" /*$t9*/) _,
        options(nostack, preserves_flags)
    );
    R::from(Reg(if err != 0 {
        x0.wrapping_neg() as *mut _
    } else {
        x0 as *mut _
    }))
}

#[inline(always)]
pub unsafe fn syscall4_readonly<R: From<Reg>>(
    nr: u32,
    a0: impl Into<Reg>,
    a1: impl Into<Reg>,
    a2: impl Into<Reg>,
    a3: impl Into<Reg>,
) -> R {
    let x0;
    let err: usize;
    asm!(
        "syscall",
        inlateout("$2" /*$v0*/) nr as usize => x0,
        in("$4" /*$a0*/) a0.0,
        in("$5" /*$a1*/) a1.0,
        in("$6" /*$a2*/) a2.0,
        inlateout("$7" /*$a3*/) a3.0 as usize => err,
        lateout("$8" /*$t0*/) _,
        lateout("$9" /*$t1*/) _,
        lateout("$10" /*$t2*/) _,
        lateout("$11" /*$t3*/) _,
        lateout("$12" /*$t4*/) _,
        lateout("$13" /*$t5*/) _,
        lateout("$14" /*$t6*/) _,
        lateout("$15" /*$t7*/) _,
        lateout("$24" /*$t8*/) _,
        lateout("$25" /*$t9*/) _,
        options(nostack, preserves_flags, readonly)
    );
    R::from(Reg(if err != 0 {
        x0.wrapping_neg() as *mut _
    } else {
        x0 as *mut _
    }))
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
    let x0;
    let err: usize;
    asm!(
        ".set noat",
        "subu $sp, 32",
        "sw {}, 16($sp)",
        "syscall",
        "addu $sp, 32",
        ".set at",
        in(reg) a4.0,
        inlateout("$2" /*$v0*/) nr as usize => x0,
        in("$4" /*$a0*/) a0.0,
        in("$5" /*$a1*/) a1.0,
        in("$6" /*$a2*/) a2.0,
        inlateout("$7" /*$a3*/) a3.0 as usize => err,
        lateout("$8" /*$t0*/) _,
        lateout("$9" /*$t1*/) _,
        lateout("$10" /*$t2*/) _,
        lateout("$11" /*$t3*/) _,
        lateout("$12" /*$t4*/) _,
        lateout("$13" /*$t5*/) _,
        lateout("$14" /*$t6*/) _,
        lateout("$15" /*$t7*/) _,
        lateout("$24" /*$t8*/) _,
        lateout("$25" /*$t9*/) _,
        options(preserves_flags)
    );
    R::from(Reg(if err != 0 {
        x0.wrapping_neg() as *mut _
    } else {
        x0 as *mut _
    }))
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
    let x0;
    let err: usize;
    asm!(
        ".set noat",
        "subu $sp, 32",
        "sw {}, 16($sp)",
        "syscall",
        "addu $sp, 32",
        ".set at",
        in(reg) a4.0,
        inlateout("$2" /*$v0*/) nr as usize => x0,
        in("$4" /*$a0*/) a0.0,
        in("$5" /*$a1*/) a1.0,
        in("$6" /*$a2*/) a2.0,
        inlateout("$7" /*$a3*/) a3.0 as usize => err,
        lateout("$8" /*$t0*/) _,
        lateout("$9" /*$t1*/) _,
        lateout("$10" /*$t2*/) _,
        lateout("$11" /*$t3*/) _,
        lateout("$12" /*$t4*/) _,
        lateout("$13" /*$t5*/) _,
        lateout("$14" /*$t6*/) _,
        lateout("$15" /*$t7*/) _,
        lateout("$24" /*$t8*/) _,
        lateout("$25" /*$t9*/) _,
        options(preserves_flags, readonly)
    );
    R::from(Reg(if err != 0 {
        x0.wrapping_neg() as *mut _
    } else {
        x0 as *mut _
    }))
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
    let x0;
    let err: usize;
    asm!(
        ".set noat",
        "subu $sp, 32",
        "sw {}, 16($sp)",
        "sw {}, 20($sp)",
        "syscall",
        "addu $sp, 32",
        ".set at",
        in(reg) a4.0,
        in(reg) a5.0,
        inlateout("$2" /*$v0*/) nr as usize => x0,
        in("$4" /*$a0*/) a0.0,
        in("$5" /*$a1*/) a1.0,
        in("$6" /*$a2*/) a2.0,
        inlateout("$7" /*$a3*/) a3.0 as usize => err,
        lateout("$8" /*$t0*/) _,
        lateout("$9" /*$t1*/) _,
        lateout("$10" /*$t2*/) _,
        lateout("$11" /*$t3*/) _,
        lateout("$12" /*$t4*/) _,
        lateout("$13" /*$t5*/) _,
        lateout("$14" /*$t6*/) _,
        lateout("$15" /*$t7*/) _,
        lateout("$24" /*$t8*/) _,
        lateout("$25" /*$t9*/) _,
        options(preserves_flags)
    );
    R::from(Reg(if err != 0 {
        x0.wrapping_neg() as *mut _
    } else {
        x0 as *mut _
    }))
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
    let x0;
    let err: usize;
    asm!(
        ".set noat",
        "subu $sp, 32",
        "sw {}, 16($sp)",
        "sw {}, 20($sp)",
        "syscall",
        "addu $sp, 32",
        ".set at",
        in(reg) a4.0,
        in(reg) a5.0,
        inlateout("$2" /*$v0*/) nr as usize => x0,
        in("$4" /*$a0*/) a0.0,
        in("$5" /*$a1*/) a1.0,
        in("$6" /*$a2*/) a2.0,
        inlateout("$7" /*$a3*/) a3.0 as usize => err,
        lateout("$8" /*$t0*/) _,
        lateout("$9" /*$t1*/) _,
        lateout("$10" /*$t2*/) _,
        lateout("$11" /*$t3*/) _,
        lateout("$12" /*$t4*/) _,
        lateout("$13" /*$t5*/) _,
        lateout("$14" /*$t6*/) _,
        lateout("$15" /*$t7*/) _,
        lateout("$24" /*$t8*/) _,
        lateout("$25" /*$t9*/) _,
        options(preserves_flags, readonly)
    );
    R::from(Reg(if err != 0 {
        x0.wrapping_neg() as *mut _
    } else {
        x0 as *mut _
    }))
}

#[inline(always)]
pub unsafe fn syscall7_readonly<R: From<Reg>>(
    nr: u32,
    a0: impl Into<Reg>,
    a1: impl Into<Reg>,
    a2: impl Into<Reg>,
    a3: impl Into<Reg>,
    a4: impl Into<Reg>,
    a5: impl Into<Reg>,
    a6: impl Into<Reg>,
) -> R {
    let x0;
    let err: usize;
    asm!(
        ".set noat",
        "subu $sp, 32",
        "sw {}, 16($sp)",
        "sw {}, 20($sp)",
        "sw {}, 24($sp)",
        "syscall",
        "addu $sp, 32",
        ".set at",
        in(reg) a4.0,
        in(reg) a5.0,
        in(reg) a6.0,
        inlateout("$2" /*$v0*/) nr as usize => x0,
        in("$4" /*$a0*/) a0.0,
        in("$5" /*$a1*/) a1.0,
        in("$6" /*$a2*/) a2.0,
        inlateout("$7" /*$a3*/) a3.0 as usize => err,
        lateout("$8" /*$t0*/) _,
        lateout("$9" /*$t1*/) _,
        lateout("$10" /*$t2*/) _,
        lateout("$11" /*$t3*/) _,
        lateout("$12" /*$t4*/) _,
        lateout("$13" /*$t5*/) _,
        lateout("$14" /*$t6*/) _,
        lateout("$15" /*$t7*/) _,
        lateout("$24" /*$t8*/) _,
        lateout("$25" /*$t9*/) _,
        options(preserves_flags, readonly)
    );
    R::from(Reg(if err != 0 {
        x0.wrapping_neg() as *mut _
    } else {
        x0 as *mut _
    }))
}
