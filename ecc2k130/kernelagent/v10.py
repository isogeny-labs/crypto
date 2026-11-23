"""v10: the whole walk step in one kernel.

The step is ~30 launches and an empty kernel on the same grid costs ~13 us, so
at 0.435 ms the shape of the cost is launch overhead, not arithmetic. Every
operation in the step is row-local: sigma is a per-row bit permutation, the
multiply is a per-row clmad product, the xors are per-row. So all fourteen
slots can live in registers inside one program and the step needs one launch.

    python3 v10.py     # verify against the reference and sweep (R, warps)
"""
import json
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


@triton.jit
def _popc(v):
    v = v - ((v >> 1) & 0x5555555555555555)
    v = (v & 0x3333333333333333) + ((v >> 2) & 0x3333333333333333)
    v = (v + (v >> 4)) & 0x0F0F0F0F0F0F0F0F
    v = v + (v >> 8)
    v = v + (v >> 16)
    v = v + (v >> 32)
    return v & 0x7F


@triton.jit
def _pack3(bit, t, R: tl.constexpr, T: tl.constexpr):
    """(R, T) bit tile -> three (R,) limbs."""
    sh = (t % 64).to(tl.int64)
    w = bit << sh[None, :]
    l0 = tl.sum(tl.where((t < 64)[None, :], w, 0), axis=1)
    l1 = tl.sum(tl.where(((t >= 64) & (t < 128))[None, :], w, 0), axis=1)
    l2 = tl.sum(tl.where((t >= 128)[None, :], w, 0), axis=1)
    return l0, l1, l2


@triton.jit
def _sigma_k(a0, a1, a2, table, k, R: tl.constexpr, T: tl.constexpr):
    """out bit t = source bit table[k][t], read straight out of the limbs."""
    t = tl.arange(0, T)
    code = tl.load(table + k * 131 + t, mask=t < 131, other=0)
    li = code >> 6
    sh = (code & 63).to(tl.int64)
    a = tl.where(li[None, :] == 0, a0[:, None],
                 tl.where(li[None, :] == 1, a1[:, None], a2[:, None]))
    bit = (a >> sh[None, :]) & 1
    bit = tl.where((t < 131)[None, :], bit, 0)
    return _pack3(bit, t, R, T)


@triton.jit
def _sigma_row(a0, a1, a2, table, j, R: tl.constexpr, T: tl.constexpr):
    """Same, with the permutation row chosen per row (the walk's j)."""
    t = tl.arange(0, T)
    code = tl.load(table + j[:, None] * 131 + t[None, :],
                   mask=(t < 131)[None, :], other=0)
    li = code >> 6
    sh = (code & 63).to(tl.int64)
    a = tl.where(li == 0, a0[:, None], tl.where(li == 1, a1[:, None], a2[:, None]))
    bit = (a >> sh) & 1
    bit = tl.where((t < 131)[None, :], bit, 0)
    return _pack3(bit, t, R, T)


@triton.jit
def _rev64(v):
    v = ((v >> 1) & 0x5555555555555555) | ((v & 0x5555555555555555) << 1)
    v = ((v >> 2) & 0x3333333333333333) | ((v & 0x3333333333333333) << 2)
    v = ((v >> 4) & 0x0F0F0F0F0F0F0F0F) | ((v & 0x0F0F0F0F0F0F0F0F) << 4)
    v = ((v >> 8) & 0x00FF00FF00FF00FF) | ((v & 0x00FF00FF00FF00FF) << 8)
    v = ((v >> 16) & 0x0000FFFF0000FFFF) | ((v & 0x0000FFFF0000FFFF) << 16)
    return (v >> 32) | (v << 32)


@triton.jit
def _rev131(x0, x1, x2):
    r0 = _rev64(x0)
    r1 = _rev64(x1)
    r2 = _rev64(x2)
    return ((r2 >> 61) | (r1 << 3), (r1 >> 61) | (r0 << 3), r0 >> 61)


@triton.jit
def _mul(a0, a1, a2, b0, b1, b2, R: tl.constexpr):
    """131-bit normal-basis product: symmetric lift, 5x5 clmad, fold mod 2^263-1."""
    u0 = a0.to(tl.uint64, bitcast=True)
    u1 = a1.to(tl.uint64, bitcast=True)
    u2 = a2.to(tl.uint64, bitcast=True) & 7
    v0 = b0.to(tl.uint64, bitcast=True)
    v1 = b1.to(tl.uint64, bitcast=True)
    v2 = b2.to(tl.uint64, bitcast=True) & 7

    ra0, ra1, ra2 = _rev131(u0, u1, u2)
    rb0, rb1, rb2 = _rev131(v0, v1, v2)

    la0 = u0 << 1
    la1 = (u1 << 1) | (u0 >> 63)
    la2 = (u2 << 1) | (u1 >> 63) | (ra0 << 4)
    la3 = (ra0 >> 60) | (ra1 << 4)
    la4 = (ra1 >> 60) | ((ra2 & 7) << 4)
    lb0 = v0 << 1
    lb1 = (v1 << 1) | (v0 >> 63)
    lb2 = (v2 << 1) | (v1 >> 63) | (rb0 << 4)
    lb3 = (rb0 >> 60) | (rb1 << 4)
    lb4 = (rb1 >> 60) | ((rb2 & 7) << 4)

    zero = tl.zeros((R,), tl.uint64)
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
    return ((x0 ^ nb).to(tl.int64, bitcast=True),
            (x1 ^ nb).to(tl.int64, bitcast=True),
            ((x2 ^ nb) & 7).to(tl.int64, bitcast=True))


@triton.jit
def _walk_kernel(x_ptr, y_ptr, out_ptr, table, n_rows, R: tl.constexpr, T: tl.constexpr):
    pid = tl.program_id(0)
    rows = pid * R + tl.arange(0, R)
    m = rows < n_rows

    xb0 = tl.load(x_ptr + rows * 3 + 0, mask=m, other=0)
    xb1 = tl.load(x_ptr + rows * 3 + 1, mask=m, other=0)
    xb2 = tl.load(x_ptr + rows * 3 + 2, mask=m, other=0) & 7
    yb0 = tl.load(y_ptr + rows * 3 + 0, mask=m, other=0)
    yb1 = tl.load(y_ptr + rows * 3 + 1, mask=m, other=0)
    yb2 = tl.load(y_ptr + rows * 3 + 2, mask=m, other=0) & 7

    hw = _popc(xb0) + _popc(xb1) + _popc(xb2)
    j = 3 + ((hw >> 1) & 7)

    x20, x21, x22 = _sigma_row(xb0, xb1, xb2, table, j, R, T)
    y20, y21, y22 = _sigma_row(yb0, yb1, yb2, table, j, R, T)
    d0, d1, d2 = xb0 ^ x20, xb1 ^ x21, xb2 ^ x22

    a0, a1, a2 = d0, d1, d2
    s0, s1, s2 = _sigma_k(a0, a1, a2, table, 1, R, T)
    a0, a1, a2 = _mul(s0, s1, s2, a0, a1, a2, R)
    s0, s1, s2 = _sigma_k(a0, a1, a2, table, 2, R, T)
    a0, a1, a2 = _mul(s0, s1, s2, a0, a1, a2, R)
    s0, s1, s2 = _sigma_k(a0, a1, a2, table, 4, R, T)
    a0, a1, a2 = _mul(s0, s1, s2, a0, a1, a2, R)
    s0, s1, s2 = _sigma_k(a0, a1, a2, table, 8, R, T)
    a0, a1, a2 = _mul(s0, s1, s2, a0, a1, a2, R)
    s0, s1, s2 = _sigma_k(a0, a1, a2, table, 16, R, T)
    a0, a1, a2 = _mul(s0, s1, s2, a0, a1, a2, R)
    s0, s1, s2 = _sigma_k(a0, a1, a2, table, 32, R, T)
    a0, a1, a2 = _mul(s0, s1, s2, a0, a1, a2, R)
    s0, s1, s2 = _sigma_k(a0, a1, a2, table, 1, R, T)
    a0, a1, a2 = _mul(s0, s1, s2, d0, d1, d2, R)
    s0, s1, s2 = _sigma_k(a0, a1, a2, table, 65, R, T)
    a0, a1, a2 = _mul(s0, s1, s2, a0, a1, a2, R)
    s0, s1, s2 = _sigma_k(a0, a1, a2, table, 1, R, T)      # s = d^-1

    t0, t1, t2 = yb0 ^ y20, yb1 ^ y21, yb2 ^ y22
    l0, l1, l2 = _mul(t0, t1, t2, s0, s1, s2, R)           # lam
    q0, q1, q2 = _sigma_k(l0, l1, l2, table, 1, R, T)
    x30, x31, x32 = q0 ^ l0 ^ d0, q1 ^ l1 ^ d1, q2 ^ l2 ^ d2
    u0, u1, u2 = xb0 ^ x30, xb1 ^ x31, xb2 ^ x32
    w0, w1, w2 = _mul(l0, l1, l2, u0, u1, u2, R)
    y30, y31, y32 = w0 ^ x30 ^ yb0, w1 ^ x31 ^ yb1, w2 ^ x32 ^ yb2

    tl.store(out_ptr + rows * 6 + 0, x30, mask=m)
    tl.store(out_ptr + rows * 6 + 1, x31, mask=m)
    tl.store(out_ptr + rows * 6 + 2, x32, mask=m)
    tl.store(out_ptr + rows * 6 + 3, y30, mask=m)
    tl.store(out_ptr + rows * 6 + 4, y31, mask=m)
    tl.store(out_ptr + rows * 6 + 5, y32, mask=m)


def kernel_function(x, y, r=4, warps=4, t=256):
    assert x.is_cuda and y.is_cuda
    assert x.dtype == torch.int64 and y.dtype == torch.int64
    assert x.shape == y.shape and x.dim() == 2 and x.shape[1] == 3
    n = x.shape[0]
    out = torch.empty((n, 2, 3), dtype=torch.int64, device=x.device)
    if n == 0:
        return out
    table = K.get_table(x.device)
    _walk_kernel[((n + r - 1) // r,)](
        x, y, out, table, n, R=r, T=t, num_warps=warps,
        num_stages=1)
    return out


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
    torch.manual_seed(97)
    n = 1 << 16
    x = wsp._random_elements(n).cuda()
    y = wsp._random_elements(n).cuda()
    rep = {"gpu": torch.cuda.get_device_name(), "rows": n}
    want = K.kernel_function(x, y)

    best = None
    for r in (1, 2, 4, 8):
        for w in (1, 2, 4):
            try:
                got = kernel_function(x, y, r=r, warps=w)
                ok = bool(torch.equal(got, want))
                ms = median_ms(lambda _r=r, _w=w: kernel_function(x, y, r=_r, warps=_w))
                rec = {"r": r, "w": w, "exact": ok, "ms": ms, "updates_per_s": n / ms * 1e3}
            except Exception as exc:
                rec = {"r": r, "w": w, "error": "%s: %s" % (type(exc).__name__, str(exc)[:120])}
            print(json.dumps(rec), flush=True)
            rep.setdefault("variants", []).append(rec)
            if rec.get("exact") and (best is None or rec["ms"] < best["ms"]):
                best = rec
    rep["best"] = best
    if best:
        w2 = K.kernel_function(x, y)
        rep["exact_65k"] = bool(torch.equal(kernel_function(x, y, r=best["r"], warps=best["w"]), w2))
    print(json.dumps({"best": best}))
    (pathlib.Path("/results") if pathlib.Path("/results").is_dir() else HERE).joinpath("v10.json").write_text(
        json.dumps(rep, indent=2))
    return 0 if best else 1


if __name__ == "__main__":
    raise SystemExit(main())
