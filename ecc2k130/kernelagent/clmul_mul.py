"""Is a clmad product actually faster than the bit-pair convolution, all-in?

The scratch slots hold one byte per bit, which suits sigma (a bit permutation)
but not clmad (which wants packed limbs). So a clmad product needs
    bits -> limbs (pack), lift, 5x5 clmad, fold, limbs -> bits (unpack)
and the packs are two extra passes per multiply.

This measures, on a Blackwell card:
  * correctness of the clmad product against kernel_v5's convolution;
  * the cost of the whole chain vs the convolution alone.
"""
import json
import os
import pathlib
import statistics
import sys

import torch
import triton
import triton.language as tl

HERE = pathlib.Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
import walk_step_problem as wsp  # noqa: E402
import kernel_v5 as K  # noqa: E402

ASM = tl.constexpr("clmad.lo.u64 $0, $2, $3, 0;\n\tclmad.hi.u64 $1, $2, $3, 0;")
SLOT = 264


@triton.jit
def _pack_bits_kernel(scratch, limbs, src, n_rows, STRIDE: tl.constexpr):
    """slot bytes (0/1) -> 3 int64 limbs, one row per program."""
    pid = tl.program_id(0)
    base = scratch + pid * STRIDE + src * 264
    offs = tl.arange(0, 256)
    m = (offs < 131) & (pid < n_rows)
    b = tl.load(base + offs, mask=m, other=0).to(tl.int64)
    sh = (offs % 64).to(tl.int64)
    v = tl.where(m, b << sh, 0)
    l0 = tl.sum(tl.where(offs < 64, v, 0), axis=0)
    l1 = tl.sum(tl.where((offs >= 64) & (offs < 128), v, 0), axis=0)
    l2 = tl.sum(tl.where(offs >= 128, v, 0), axis=0)
    tl.store(limbs + pid * 3 + 0, l0)
    tl.store(limbs + pid * 3 + 1, l1)
    tl.store(limbs + pid * 3 + 2, l2)


@triton.jit
def _unpack_bits_kernel(limbs, scratch, dst, n_rows, STRIDE: tl.constexpr):
    """3 int64 limbs -> slot bytes, one row per program."""
    pid = tl.program_id(0)
    offs = tl.arange(0, 256)
    m = (offs < 131) & (pid < n_rows)
    l0 = tl.load(limbs + pid * 3 + 0)
    l1 = tl.load(limbs + pid * 3 + 1)
    l2 = tl.load(limbs + pid * 3 + 2)
    li = offs // 64
    sh = (offs % 64).to(tl.int64)
    v = tl.where(li == 0, l0, tl.where(li == 1, l1, l2))
    tl.store(scratch + pid * STRIDE + dst * 264 + offs, ((v >> sh) & 1).to(tl.int8), mask=m)


@triton.jit
def _rev131(x0, x1, x2):
    v = x0
    v = ((v >> 1) & 0x5555555555555555) | ((v & 0x5555555555555555) << 1)
    v = ((v >> 2) & 0x3333333333333333) | ((v & 0x3333333333333333) << 2)
    v = ((v >> 4) & 0x0F0F0F0F0F0F0F0F) | ((v & 0x0F0F0F0F0F0F0F0F) << 4)
    v = ((v >> 8) & 0x00FF00FF00FF00FF) | ((v & 0x00FF00FF00FF00FF) << 8)
    v = ((v >> 16) & 0x0000FFFF0000FFFF) | ((v & 0x0000FFFF0000FFFF) << 16)
    r0 = (v >> 32) | (v << 32)
    v = x1
    v = ((v >> 1) & 0x5555555555555555) | ((v & 0x5555555555555555) << 1)
    v = ((v >> 2) & 0x3333333333333333) | ((v & 0x3333333333333333) << 2)
    v = ((v >> 4) & 0x0F0F0F0F0F0F0F0F) | ((v & 0x0F0F0F0F0F0F0F0F) << 4)
    v = ((v >> 8) & 0x00FF00FF00FF00FF) | ((v & 0x00FF00FF00FF00FF) << 8)
    v = ((v >> 16) & 0x0000FFFF0000FFFF) | ((v & 0x0000FFFF0000FFFF) << 16)
    r1 = (v >> 32) | (v << 32)
    v = x2
    v = ((v >> 1) & 0x5555555555555555) | ((v & 0x5555555555555555) << 1)
    v = ((v >> 2) & 0x3333333333333333) | ((v & 0x3333333333333333) << 2)
    v = ((v >> 4) & 0x0F0F0F0F0F0F0F0F) | ((v & 0x0F0F0F0F0F0F0F0F) << 4)
    v = ((v >> 8) & 0x00FF00FF00FF00FF) | ((v & 0x00FF00FF00FF00FF) << 8)
    v = ((v >> 16) & 0x0000FFFF0000FFFF) | ((v & 0x0000FFFF0000FFFF) << 16)
    r2 = (v >> 32) | (v << 32)
    # reverse of the 192-bit register, then shift down by 61 to keep 131 bits
    y0 = (r2 >> 61) | (r1 << 3)
    y1 = (r1 >> 61) | (r0 << 3)
    y2 = r0 >> 61
    return y0, y1, y2


@triton.jit
def _mul_clmad_kernel(a, b, o, BLOCK: tl.constexpr):
    """131-bit normal-basis product from clmad; one row per lane."""
    r = tl.program_id(0) * BLOCK + tl.arange(0, BLOCK)
    a0 = tl.load(a + r * 3 + 0).to(tl.uint64, bitcast=True)
    a1 = tl.load(a + r * 3 + 1).to(tl.uint64, bitcast=True)
    a2 = tl.load(a + r * 3 + 2).to(tl.uint64, bitcast=True) & 7
    b0 = tl.load(b + r * 3 + 0).to(tl.uint64, bitcast=True)
    b1 = tl.load(b + r * 3 + 1).to(tl.uint64, bitcast=True)
    b2 = tl.load(b + r * 3 + 2).to(tl.uint64, bitcast=True) & 7

    ra0, ra1, ra2 = _rev131(a0, a1, a2)
    rb0, rb1, rb2 = _rev131(b0, b1, b2)

    la0 = a0 << 1
    la1 = (a1 << 1) | (a0 >> 63)
    la2 = (a2 << 1) | (a1 >> 63) | (ra0 << 4)
    la3 = (ra0 >> 60) | (ra1 << 4)
    la4 = (ra1 >> 60) | ((ra2 & 7) << 4)
    lb0 = b0 << 1
    lb1 = (b1 << 1) | (b0 >> 63)
    lb2 = (b2 << 1) | (b1 >> 63) | (rb0 << 4)
    lb3 = (rb0 >> 60) | (rb1 << 4)
    lb4 = (rb1 >> 60) | ((rb2 & 7) << 4)

    zero = tl.zeros((BLOCK,), tl.uint64)
    c0 = zero
    c1 = zero
    c2 = zero
    c3 = zero
    c4 = zero
    c5 = zero
    c6 = zero
    c7 = zero
    c8 = zero
    lo, hi = tl.inline_asm_elementwise(ASM, "=l,=l,l,l", (la0, lb0), dtype=(tl.uint64, tl.uint64), is_pure=True, pack=1)
    c0 = c0 ^ lo
    c1 = c1 ^ hi
    lo, hi = tl.inline_asm_elementwise(ASM, "=l,=l,l,l", (la0, lb1), dtype=(tl.uint64, tl.uint64), is_pure=True, pack=1)
    c1 = c1 ^ lo
    c2 = c2 ^ hi
    lo, hi = tl.inline_asm_elementwise(ASM, "=l,=l,l,l", (la0, lb2), dtype=(tl.uint64, tl.uint64), is_pure=True, pack=1)
    c2 = c2 ^ lo
    c3 = c3 ^ hi
    lo, hi = tl.inline_asm_elementwise(ASM, "=l,=l,l,l", (la0, lb3), dtype=(tl.uint64, tl.uint64), is_pure=True, pack=1)
    c3 = c3 ^ lo
    c4 = c4 ^ hi
    lo, hi = tl.inline_asm_elementwise(ASM, "=l,=l,l,l", (la0, lb4), dtype=(tl.uint64, tl.uint64), is_pure=True, pack=1)
    c4 = c4 ^ lo
    c5 = c5 ^ hi
    lo, hi = tl.inline_asm_elementwise(ASM, "=l,=l,l,l", (la1, lb0), dtype=(tl.uint64, tl.uint64), is_pure=True, pack=1)
    c1 = c1 ^ lo
    c2 = c2 ^ hi
    lo, hi = tl.inline_asm_elementwise(ASM, "=l,=l,l,l", (la1, lb1), dtype=(tl.uint64, tl.uint64), is_pure=True, pack=1)
    c2 = c2 ^ lo
    c3 = c3 ^ hi
    lo, hi = tl.inline_asm_elementwise(ASM, "=l,=l,l,l", (la1, lb2), dtype=(tl.uint64, tl.uint64), is_pure=True, pack=1)
    c3 = c3 ^ lo
    c4 = c4 ^ hi
    lo, hi = tl.inline_asm_elementwise(ASM, "=l,=l,l,l", (la1, lb3), dtype=(tl.uint64, tl.uint64), is_pure=True, pack=1)
    c4 = c4 ^ lo
    c5 = c5 ^ hi
    lo, hi = tl.inline_asm_elementwise(ASM, "=l,=l,l,l", (la1, lb4), dtype=(tl.uint64, tl.uint64), is_pure=True, pack=1)
    c5 = c5 ^ lo
    c6 = c6 ^ hi
    lo, hi = tl.inline_asm_elementwise(ASM, "=l,=l,l,l", (la2, lb0), dtype=(tl.uint64, tl.uint64), is_pure=True, pack=1)
    c2 = c2 ^ lo
    c3 = c3 ^ hi
    lo, hi = tl.inline_asm_elementwise(ASM, "=l,=l,l,l", (la2, lb1), dtype=(tl.uint64, tl.uint64), is_pure=True, pack=1)
    c3 = c3 ^ lo
    c4 = c4 ^ hi
    lo, hi = tl.inline_asm_elementwise(ASM, "=l,=l,l,l", (la2, lb2), dtype=(tl.uint64, tl.uint64), is_pure=True, pack=1)
    c4 = c4 ^ lo
    c5 = c5 ^ hi
    lo, hi = tl.inline_asm_elementwise(ASM, "=l,=l,l,l", (la2, lb3), dtype=(tl.uint64, tl.uint64), is_pure=True, pack=1)
    c5 = c5 ^ lo
    c6 = c6 ^ hi
    lo, hi = tl.inline_asm_elementwise(ASM, "=l,=l,l,l", (la2, lb4), dtype=(tl.uint64, tl.uint64), is_pure=True, pack=1)
    c6 = c6 ^ lo
    c7 = c7 ^ hi
    lo, hi = tl.inline_asm_elementwise(ASM, "=l,=l,l,l", (la3, lb0), dtype=(tl.uint64, tl.uint64), is_pure=True, pack=1)
    c3 = c3 ^ lo
    c4 = c4 ^ hi
    lo, hi = tl.inline_asm_elementwise(ASM, "=l,=l,l,l", (la3, lb1), dtype=(tl.uint64, tl.uint64), is_pure=True, pack=1)
    c4 = c4 ^ lo
    c5 = c5 ^ hi
    lo, hi = tl.inline_asm_elementwise(ASM, "=l,=l,l,l", (la3, lb2), dtype=(tl.uint64, tl.uint64), is_pure=True, pack=1)
    c5 = c5 ^ lo
    c6 = c6 ^ hi
    lo, hi = tl.inline_asm_elementwise(ASM, "=l,=l,l,l", (la3, lb3), dtype=(tl.uint64, tl.uint64), is_pure=True, pack=1)
    c6 = c6 ^ lo
    c7 = c7 ^ hi
    lo, hi = tl.inline_asm_elementwise(ASM, "=l,=l,l,l", (la3, lb4), dtype=(tl.uint64, tl.uint64), is_pure=True, pack=1)
    c7 = c7 ^ lo
    c8 = c8 ^ hi
    lo, hi = tl.inline_asm_elementwise(ASM, "=l,=l,l,l", (la4, lb0), dtype=(tl.uint64, tl.uint64), is_pure=True, pack=1)
    c4 = c4 ^ lo
    c5 = c5 ^ hi
    lo, hi = tl.inline_asm_elementwise(ASM, "=l,=l,l,l", (la4, lb1), dtype=(tl.uint64, tl.uint64), is_pure=True, pack=1)
    c5 = c5 ^ lo
    c6 = c6 ^ hi
    lo, hi = tl.inline_asm_elementwise(ASM, "=l,=l,l,l", (la4, lb2), dtype=(tl.uint64, tl.uint64), is_pure=True, pack=1)
    c6 = c6 ^ lo
    c7 = c7 ^ hi
    lo, hi = tl.inline_asm_elementwise(ASM, "=l,=l,l,l", (la4, lb3), dtype=(tl.uint64, tl.uint64), is_pure=True, pack=1)
    c7 = c7 ^ lo
    c8 = c8 ^ hi
    lo, hi = tl.inline_asm_elementwise(ASM, "=l,=l,l,l", (la4, lb4), dtype=(tl.uint64, tl.uint64), is_pure=True, pack=1)
    c8 = c8 ^ lo

    f0 = c0 ^ ((c4 >> 7) | (c5 << 57))
    f1 = c1 ^ ((c5 >> 7) | (c6 << 57))
    f2 = c2 ^ ((c6 >> 7) | (c7 << 57))
    f3 = c3 ^ ((c7 >> 7) | (c8 << 57))

    x0 = (f0 >> 1) | (f1 << 63)
    x1 = (f1 >> 1) | (f2 << 63)
    x2 = ((f2 >> 1) | (f3 << 63)) & 7
    nb = (f0 & 1) * 0xFFFFFFFFFFFFFFFF
    tl.store(o + r * 3 + 0, (x0 ^ nb).to(tl.int64, bitcast=True))
    tl.store(o + r * 3 + 1, (x1 ^ nb).to(tl.int64, bitcast=True))
    tl.store(o + r * 3 + 2, ((x2 ^ nb) & 7).to(tl.int64, bitcast=True))


def median_ms(fn, reps=20):
    fn()
    torch.cuda.synchronize()
    ts = []
    for _ in range(reps):
        e0, e1 = torch.cuda.Event(True), torch.cuda.Event(True)
        e0.record()
        fn()
        e1.record()
        torch.cuda.synchronize()
        ts.append(e0.elapsed_time(e1))
    return statistics.median(ts)


def main():
    torch.manual_seed(5)
    dev = "cuda"
    N = 1 << 16
    rep = {"gpu": torch.cuda.get_device_name(), "rows": N}

    x = wsp._random_elements(N).cuda()
    y = wsp._random_elements(N).cuda()
    npad = N
    scratch = torch.empty((npad, K.STRIDE), dtype=torch.int8, device=dev)
    table = K.get_table(dev)
    g = (npad,)
    K._unpack_kernel[g](x, y, scratch, N, STRIDE=K.STRIDE, num_warps=4)   # fills slots 0,1
    K._frob_kernel[g](scratch, table, STRIDE=K.STRIDE, num_warps=4)       # slots 2,3
    K._xor_kernel[g](scratch, 0, 2, 4, STRIDE=K.STRIDE, num_warps=4)      # slot 4 = d

    limbs_a = torch.empty((npad, 3), dtype=torch.int64, device=dev)
    limbs_b = torch.empty((npad, 3), dtype=torch.int64, device=dev)
    limbs_o = torch.empty((npad, 3), dtype=torch.int64, device=dev)

    # correctness: clmad product of (slot 6? use y2 and d) vs the convolution
    _pack_bits_kernel[g](scratch, limbs_a, 1, N, STRIDE=K.STRIDE, num_warps=4)
    _pack_bits_kernel[g](scratch, limbs_b, 4, N, STRIDE=K.STRIDE, num_warps=4)
    _mul_clmad_kernel[(npad // 128,)](limbs_a, limbs_b, limbs_o, BLOCK=128)
    K._mul_kernel[(npad // 4,)](scratch, 1, 4, 12, R=4, ITILE=8, STRIDE=K.STRIDE, num_warps=4)
    ref_bits = scratch[:, 12 * SLOT:12 * SLOT + 131].to(torch.int64)
    _unpack_bits_kernel[g](limbs_o, scratch, 13, N, STRIDE=K.STRIDE, num_warps=4)
    got_bits = scratch[:, 13 * SLOT:13 * SLOT + 131].to(torch.int64)
    rep["clmad_product_exact"] = bool(torch.equal(got_bits, ref_bits))
    sh = torch.arange(64, device=dev)
    ybits = ((y.unsqueeze(-1) >> sh) & 1).reshape(N, 192)[:, :131].to(torch.int64)
    pos = torch.arange(131, device=dev)
    want_limbs = torch.stack([
        (ybits << (pos % 64)).masked_fill(pos >= 64, 0).sum(1),
        (ybits << (pos % 64)).masked_fill((pos < 64) | (pos >= 128), 0).sum(1),
        (ybits << (pos % 64)).masked_fill(pos < 128, 0).sum(1)], dim=1)
    rep["pack_ok"] = bool(torch.equal(limbs_a, want_limbs))
    if not rep["pack_ok"]:
        bad = (limbs_a != want_limbs).any(1).nonzero().flatten()[:3].tolist()
        rep["pack_bad_rows"] = bad
        if bad:
            k = bad[0]
            rep["pack_row"] = [int(v) for v in limbs_a[k].tolist()]
            rep["pack_want"] = [int(v) for v in want_limbs[k].tolist()]

    # cost: convolution alone vs pack+pack+clmad+unpack
    conv_ms = median_ms(lambda: K._mul_kernel[(npad // 4,)](scratch, 1, 4, 12, R=4, ITILE=8,
                                                           STRIDE=K.STRIDE, num_warps=4))
    pack_ms = median_ms(lambda: (_pack_bits_kernel[g](scratch, limbs_a, 1, N, STRIDE=K.STRIDE, num_warps=4),
                                 _pack_bits_kernel[g](scratch, limbs_b, 4, N, STRIDE=K.STRIDE, num_warps=4)))
    mul_ms = median_ms(lambda: _mul_clmad_kernel[(npad // 128,)](limbs_a, limbs_b, limbs_o, BLOCK=128))
    unp_ms = median_ms(lambda: _unpack_bits_kernel[g](limbs_o, scratch, 13, N, STRIDE=K.STRIDE, num_warps=4))
    rep.update({"conv_ms": conv_ms, "clmad_mul_ms": mul_ms, "pack2_ms": pack_ms,
                "unpack_ms": unp_ms, "chain_ms": pack_ms + mul_ms + unp_ms,
                "speedup_chain_vs_conv": conv_ms / (pack_ms + mul_ms + unp_ms)})

    print(json.dumps(rep))
    (pathlib.Path("/results") if pathlib.Path("/results").is_dir() else HERE).joinpath("clmul_mul.json").write_text(
        json.dumps(rep, indent=2))
    return 0 if rep["clmad_product_exact"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
