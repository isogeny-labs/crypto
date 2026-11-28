"""v11: sigma in a 128-lane chunk plus a 3-bit tail.

Every sigma processes a 256-lane tile to produce 131 bits, so 49% of its lanes
are wasted — the same waste the multiply had before v5. Splitting sigma into a
128-lane main chunk (bits 0..127 -> limbs 0 and 1) plus a 4-lane tail (bits
128..130 -> limb 2) keeps ~99% of the lanes useful.

    python3 v11.py
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
import clmul_mul as CM  # noqa: E402
import v8  # noqa: E402

NSLOT = 14
SIGMA_R, SIGMA_W = 16, 1
FROB_R, FROB_W = 8, 1
MUL_B, MUL_W = 128, 4


@triton.jit
def _bits(a0, a1, a2, code, R: tl.constexpr):
    """Pick the source bits named by `code` out of the three limbs.

    `code` is limb*64 + shift, either a (T,) row or an (R, T) per-row table.
    """
    li = code >> 6
    sh = (code & 63).to(tl.int64)
    a = tl.where(li == 0, a0[:, None], tl.where(li == 1, a1[:, None], a2[:, None]))
    return (a >> sh) & 1


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
def _sigma_slot(limbs, table, src, dst, k, n_rows, LSTRIDE, R: tl.constexpr):
    pid = tl.program_id(0)
    rows = pid * R + tl.arange(0, R)
    rm = rows < n_rows
    base = limbs + src * LSTRIDE + rows * 3
    a0 = tl.load(base + 0, mask=rm, other=0)
    a1 = tl.load(base + 1, mask=rm, other=0)
    a2 = tl.load(base + 2, mask=rm, other=0)

    t = tl.arange(0, 128)
    bit = _bits(a0, a1, a2, tl.load(table + k * 131 + t), R)
    w = bit << (t % 64).to(tl.int64)[None, :]
    l0 = tl.sum(tl.where((t < 64)[None, :], w, 0), axis=1)
    l1 = tl.sum(tl.where((t >= 64)[None, :], w, 0), axis=1)

    tt = 128 + tl.arange(0, 4)
    b2 = _bits(a0, a1, a2, tl.load(table + k * 131 + tt, mask=tt < 131, other=0), R)
    b2 = tl.where((tt < 131)[None, :], b2, 0)
    l2 = tl.sum(b2 << (tt - 128).to(tl.int64)[None, :], axis=1)

    o = limbs + dst * LSTRIDE + rows * 3
    tl.store(o + 0, l0, mask=rm)
    tl.store(o + 1, l1, mask=rm)
    tl.store(o + 2, l2, mask=rm)


@triton.jit
def _sigma_j_slot(limbs, table, src, dst, j, n_rows, LSTRIDE, R: tl.constexpr):
    """sigma^j with a per-row permutation row."""
    pid = tl.program_id(0)
    rows = pid * R + tl.arange(0, R)
    rm = rows < n_rows
    b = limbs + src * LSTRIDE + rows * 3
    b0 = tl.load(b + 0, mask=rm, other=0)
    b1 = tl.load(b + 1, mask=rm, other=0)
    b2 = tl.load(b + 2, mask=rm, other=0)

    t = tl.arange(0, 128)
    code = tl.load(table + j[:, None] * 131 + t[None, :])
    bit = _bits(b0, b1, b2, code, R)
    w = bit << (t % 64).to(tl.int64)[None, :]
    l0 = tl.sum(tl.where((t < 64)[None, :], w, 0), axis=1)
    l1 = tl.sum(tl.where((t >= 64)[None, :], w, 0), axis=1)

    tt = 128 + tl.arange(0, 4)
    c2 = tl.load(table + j[:, None] * 131 + tt[None, :], mask=(tt < 131)[None, :], other=0)
    q2 = _bits(b0, b1, b2, c2, R)
    q2 = tl.where((tt < 131)[None, :], q2, 0)
    l2 = tl.sum(q2 << (tt - 128).to(tl.int64)[None, :], axis=1)

    o = limbs + dst * LSTRIDE + rows * 3
    tl.store(o + 0, l0, mask=rm)
    tl.store(o + 1, l1, mask=rm)
    tl.store(o + 2, l2, mask=rm)


@triton.jit
def _frob_slot(limbs, table, n_rows, LSTRIDE, R: tl.constexpr):
    """j = 3 + ((HW(x) >> 1) & 7) per row; slots 2, 3 = sigma^j(x), sigma^j(y)."""
    pid = tl.program_id(0)
    rows = pid * R + tl.arange(0, R)
    rm = rows < n_rows
    xb = limbs + rows * 3
    hw = (_popc(tl.load(xb + 0, mask=rm, other=0))
          + _popc(tl.load(xb + 1, mask=rm, other=0))
          + _popc(tl.load(xb + 2, mask=rm, other=0) & 7))
    j = 3 + ((hw >> 1) & 7)
    _sigma_j_slot(limbs, table, 0, 2, j, n_rows, LSTRIDE, R)
    _sigma_j_slot(limbs, table, 1, 3, j, n_rows, LSTRIDE, R)


def kernel_function(x, y, ws=None):
    assert x.is_cuda and y.is_cuda
    assert x.dtype == torch.int64 and y.dtype == torch.int64
    assert x.shape == y.shape and x.dim() == 2 and x.shape[1] == 3
    n = x.shape[0]
    npad = ((n + MUL_B - 1) // MUL_B) * MUL_B
    lstride = npad * 3
    limbs = ws if ws is not None else torch.empty((NSLOT * lstride,), dtype=torch.int64, device=x.device)
    out = torch.empty((npad, 2, 3), dtype=torch.int64, device=x.device)
    table = K.get_table(x.device)

    limbs[0:lstride].view(npad, 3)[:n].copy_(x)
    limbs[lstride:2 * lstride].view(npad, 3)[:n].copy_(y)

    gs = (npad // SIGMA_R + 1,)
    gf = (npad // FROB_R + 1,)
    gm = (npad // MUL_B,)

    def mul(sa, sb, dst):
        CM._mul_clmad_kernel[gm](limbs[sa * lstride:], limbs[sb * lstride:],
                                 limbs[dst * lstride:], BLOCK=MUL_B, num_warps=MUL_W)

    def sig(src, dst, k):
        _sigma_slot[gs](limbs, table, src, dst, k, n, lstride, R=SIGMA_R, num_warps=SIGMA_W)

    _frob_slot[gf](limbs, table, n, lstride, R=FROB_R, num_warps=FROB_W)

    v8._xor_limbs[gs](limbs, 0, 2, 4, n, lstride, R=SIGMA_R, num_warps=SIGMA_W)
    v8._copy_limbs[gs](limbs, 4, 5, n, lstride, R=SIGMA_R, num_warps=SIGMA_W)
    acc, tmp = 5, 7
    for k in (1, 2, 4, 8, 16, 32):
        sig(acc, 6, k)
        mul(6, acc, tmp)
        acc, tmp = tmp, acc
    sig(acc, 6, 1)
    mul(6, 4, tmp)
    acc, tmp = tmp, acc
    sig(acc, 6, 65)
    mul(6, acc, tmp)
    acc, tmp = tmp, acc
    sig(acc, 6, 1)                      # slot 6 = d^-1

    v8._xor_limbs[gs](limbs, 1, 3, 12, n, lstride, R=SIGMA_R, num_warps=SIGMA_W)
    mul(12, 6, 8)                       # lam
    sig(8, 9, 1)
    v8._xor3_limbs[gs](limbs, 9, 8, 4, 9, n, lstride, R=SIGMA_R, num_warps=SIGMA_W)
    v8._xor_limbs[gs](limbs, 0, 9, 12, n, lstride, R=SIGMA_R, num_warps=SIGMA_W)
    mul(8, 12, 13)
    v8._xor3_limbs[gs](limbs, 13, 9, 1, 13, n, lstride, R=SIGMA_R, num_warps=SIGMA_W)

    out[:, 0] = limbs[9 * lstride:9 * lstride + lstride].view(npad, 3)
    out[:, 1] = limbs[13 * lstride:13 * lstride + lstride].view(npad, 3)
    return out[:n]


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
    torch.manual_seed(113)
    n = 1 << 16
    x = wsp._random_elements(n).cuda()
    y = wsp._random_elements(n).cuda()
    rep = {"gpu": torch.cuda.get_device_name(), "rows": n}
    want = K.kernel_function(x, y)
    got = kernel_function(x, y)
    rep["exact"] = bool(torch.equal(got, want))

    npad = ((n + MUL_B - 1) // MUL_B) * MUL_B
    ws = torch.empty((NSLOT * npad * 3,), dtype=torch.int64, device=x.device)
    rep["dropins_ms"] = median_ms(lambda: kernel_function(x, y))
    rep["amortised_ms"] = median_ms(lambda: kernel_function(x, y, ws=ws))
    rep["dropin_updates_per_s"] = n / rep["dropins_ms"] * 1e3
    rep["amortised_updates_per_s"] = n / rep["amortised_ms"] * 1e3

    m = 40_001
    rep["exact_odd"] = bool(torch.equal(kernel_function(x[:m].contiguous(), y[:m].contiguous()),
                                        K.kernel_function(x[:m].contiguous(), y[:m].contiguous())))
    print(json.dumps(rep))
    (pathlib.Path("/results") if pathlib.Path("/results").is_dir() else HERE).joinpath("v11.json").write_text(
        json.dumps(rep, indent=2))
    return 0 if (rep["exact"] and rep["exact_odd"]) else 1


if __name__ == "__main__":
    raise SystemExit(main())
