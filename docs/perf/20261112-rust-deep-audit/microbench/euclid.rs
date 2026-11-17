fn euclid_u32_x86(m: u32, a: u32) -> (u32, u32, bool) {
    let (mut r0, mut r1, mut t0, mut t1) = (m, a, 0u32, 1u32);
    let neg: u32;
    // SAFETY: every `div` divides by a remainder just tested nonzero
    // (`r1` at entry and after half B, `r0` after half A), with `edx`
    // cleared, so the quotient fits 32 bits.  The coefficient updates do
    // not overflow: every magnitude stays at most `m` (`invmod_u64`).
    unsafe {
        std::arch::asm!(
            "test {r1:e}, {r1:e}",
            "jz 3f",
            "2:",
            // half A: q = r0 / r1, r0 = r0 mod r1, t0 += q t1
            "mov eax, {r0:e}",
            "xor edx, edx",
            "div {r1:e}",
            "mov {r0:e}, edx",
            "imul eax, {t1:e}",
            "add {t0:e}, eax",
            "test edx, edx",
            "jz 4f",
            // half B: q = r1 / r0, r1 = r1 mod r0, t1 += q t0
            "mov eax, {r1:e}",
            "xor edx, edx",
            "div {r0:e}",
            "mov {r1:e}, edx",
            "imul eax, {t0:e}",
            "add {t1:e}, eax",
            "test edx, edx",
            "jnz 2b",
            "3:",
            "mov {neg:e}, 1",
            "jmp 5f",
            "4:",
            "xor {neg:e}, {neg:e}",
            "5:",
            r0 = inout(reg) r0,
            r1 = inout(reg) r1,
            t0 = inout(reg) t0,
            t1 = inout(reg) t1,
            neg = out(reg) neg,
            out("eax") _,
            out("edx") _,
            options(pure, nomem, nostack),
        );
    }
    if neg != 0 {
        (r0, t0, true)
    } else {
        (r1, t1, false)
    }
}
