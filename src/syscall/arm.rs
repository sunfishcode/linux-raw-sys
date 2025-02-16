//! arm Linux system calls.

use super::Reg;
use core::arch::asm;

#[inline(always)]
pub(in crate::backend) unsafe fn syscall0_readonly<R: From<Reg>>(nr: u32) -> R {
    let r0;
    asm!(
        "svc 0",
        in("r7") nr.to_asm(),
        lateout("r0") r0,
        options(nostack, preserves_flags, readonly)
    );
    R::from(Reg(r0))
}

#[inline(always)]
pub(in crate::backend) unsafe fn syscall1<R: From<Reg>>(nr: u32, a0: impl Into<Reg>) -> R {
    let r0;
    asm!(
        "svc 0",
        in("r7") nr.to_asm(),
        inlateout("r0") a0.into().0 => r0,
        options(nostack, preserves_flags)
    );
    R::from(Reg(r0))
}

#[inline(always)]
pub(in crate::backend) unsafe fn syscall1_readonly<R: From<Reg>>(nr: u32, a0: impl Into<Reg>) -> R {
    let r0;
    asm!(
        "svc 0",
        in("r7") nr.to_asm(),
        inlateout("r0") a0.into().0 => r0,
        options(nostack, preserves_flags, readonly)
    );
    R::from(Reg(r0))
}

#[inline(always)]
pub(in crate::backend) unsafe fn syscall1_noreturn<R: From<Reg>>(nr: u32, a0: impl Into<Reg>) -> ! {
    asm!(
        "svc 0",
        in("r7") nr.to_asm(),
        in("r0") a0.into().0,
        options(nostack, noreturn)
    )
}

#[inline(always)]
pub(in crate::backend) unsafe fn syscall2<R: From<Reg>>(
    nr: u32,
    a0: impl Into<Reg>,
    a1: impl Into<Reg>,
) -> R {
    let r0;
    asm!(
        "svc 0",
        in("r7") nr.to_asm(),
        inlateout("r0") a0.into().0 => r0,
        in("r1") a1.into().0,
        options(nostack, preserves_flags)
    );
    R::from(Reg(r0))
}

#[inline(always)]
pub(in crate::backend) unsafe fn syscall2_readonly<R: From<Reg>>(
    nr: u32,
    a0: impl Into<Reg>,
    a1: impl Into<Reg>,
) -> R {
    let r0;
    asm!(
        "svc 0",
        in("r7") nr.to_asm(),
        inlateout("r0") a0.into().0 => r0,
        in("r1") a1.into().0,
        options(nostack, preserves_flags, readonly)
    );
    R::from(Reg(r0))
}

#[inline(always)]
pub(in crate::backend) unsafe fn syscall3<R: From<Reg>>(
    nr: u32,
    a0: impl Into<Reg>,
    a1: impl Into<Reg>,
    a2: impl Into<Reg>,
) -> R {
    let r0;
    asm!(
        "svc 0",
        in("r7") nr.to_asm(),
        inlateout("r0") a0.into().0 => r0,
        in("r1") a1.into().0,
        in("r2") a2.into().0,
        options(nostack, preserves_flags)
    );
    R::from(Reg(r0))
}

#[inline(always)]
pub(in crate::backend) unsafe fn syscall3_readonly<R: From<Reg>>(
    nr: u32,
    a0: impl Into<Reg>,
    a1: impl Into<Reg>,
    a2: impl Into<Reg>,
) -> R {
    let r0;
    asm!(
        "svc 0",
        in("r7") nr.to_asm(),
        inlateout("r0") a0.into().0 => r0,
        in("r1") a1.into().0,
        in("r2") a2.into().0,
        options(nostack, preserves_flags, readonly)
    );
    R::from(Reg(r0))
}

#[inline(always)]
pub(in crate::backend) unsafe fn syscall4<R: From<Reg>>(
    nr: u32,
    a0: impl Into<Reg>,
    a1: impl Into<Reg>,
    a2: impl Into<Reg>,
    a3: impl Into<Reg>,
) -> R {
    let r0;
    asm!(
        "svc 0",
        in("r7") nr.to_asm(),
        inlateout("r0") a0.into().0 => r0,
        in("r1") a1.into().0,
        in("r2") a2.into().0,
        in("r3") a3.into().0,
        options(nostack, preserves_flags)
    );
    R::from(Reg(r0))
}

#[inline(always)]
pub(in crate::backend) unsafe fn syscall4_readonly<R: From<Reg>>(
    nr: u32,
    a0: impl Into<Reg>,
    a1: impl Into<Reg>,
    a2: impl Into<Reg>,
    a3: impl Into<Reg>,
) -> R {
    let r0;
    asm!(
        "svc 0",
        in("r7") nr.to_asm(),
        inlateout("r0") a0.into().0 => r0,
        in("r1") a1.into().0,
        in("r2") a2.into().0,
        in("r3") a3.into().0,
        options(nostack, preserves_flags, readonly)
    );
    R::from(Reg(r0))
}

#[inline(always)]
pub(in crate::backend) unsafe fn syscall5<R: From<Reg>>(
    nr: u32,
    a0: impl Into<Reg>,
    a1: impl Into<Reg>,
    a2: impl Into<Reg>,
    a3: impl Into<Reg>,
    a4: impl Into<Reg>,
) -> R {
    let r0;
    asm!(
        "svc 0",
        in("r7") nr.to_asm(),
        inlateout("r0") a0.into().0 => r0,
        in("r1") a1.into().0,
        in("r2") a2.into().0,
        in("r3") a3.into().0,
        in("r4") a4.into().0,
        options(nostack, preserves_flags)
    );
    R::from(Reg(r0))
}

#[inline(always)]
pub(in crate::backend) unsafe fn syscall5_readonly<R: From<Reg>>(
    nr: u32,
    a0: impl Into<Reg>,
    a1: impl Into<Reg>,
    a2: impl Into<Reg>,
    a3: impl Into<Reg>,
    a4: impl Into<Reg>,
) -> R {
    let r0;
    asm!(
        "svc 0",
        in("r7") nr.to_asm(),
        inlateout("r0") a0.into().0 => r0,
        in("r1") a1.into().0,
        in("r2") a2.into().0,
        in("r3") a3.into().0,
        in("r4") a4.into().0,
        options(nostack, preserves_flags, readonly)
    );
    R::from(Reg(r0))
}

#[inline(always)]
pub(in crate::backend) unsafe fn syscall6<R: From<Reg>>(
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
        "svc 0",
        in("r7") nr.to_asm(),
        inlateout("r0") a0.into().0 => r0,
        in("r1") a1.into().0,
        in("r2") a2.into().0,
        in("r3") a3.into().0,
        in("r4") a4.into().0,
        in("r5") a5.into().0,
        options(nostack, preserves_flags)
    );
    R::from(Reg(r0))
}

#[inline(always)]
pub(in crate::backend) unsafe fn syscall6_readonly<R: From<Reg>>(
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
        "svc 0",
        in("r7") nr.to_asm(),
        inlateout("r0") a0.into().0 => r0,
        in("r1") a1.into().0,
        in("r2") a2.into().0,
        in("r3") a3.into().0,
        in("r4") a4.into().0,
        in("r5") a5.into().0,
        options(nostack, preserves_flags, readonly)
    );
    R::from(Reg(r0))
}
