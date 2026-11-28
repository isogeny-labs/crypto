"""v8: one representation. Sigma reads packed limbs, so nothing needs unpacking.

In v7 the byte-per-bit slots still existed only because sigma gathered from
them; every multiply therefore ended with an unpack kernel. But sigma is a
fixed permutation: output bit t comes from source bit p = table[k][t], i.e.
from limb p//64 at shift p%64. A 3-way select plus a shift gets it straight
out of the limbs, and the same three masked sums that used to emit limbs
produce the result. No bytes anywhere, no packs, no unpacks.
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

N = 1 << 16
R = 8
WARPS = 4


@triton.jit
def _store_limbs(rows, bit, t, out, n_rows):
    sh = (t % 64).to(tl.int64)
    w = bit << sh[None, :]
    o0 = tl.sum(tl.where((t < 64)[None, :], w, 0), axis=1)
    o1 = tl.sum(tl.where(((t >= 64) & (t < 128))[None, :], w, 0), axis=1)
    o2 = tl.sum(tl.where((t >= 128)[None, :], w, 0), axis=1)
    rm = rows < n_rows
    tl.store(out + 0, o0, mask=rm)
    tl.store(out + 1, o1, mask=rm)
    tl.store(out + 2, o2, mask=rm)


@triton.jit
def _sigma_limbs(limbs, table, src, dst, k, n_rows, LSTRIDE,
                 R: tl.constexpr):
    pid = tl.program_id(0)
    rows = pid * R + tl.arange(0, R)
    t = tl.arange(0, 256)
    rm = rows < n_rows
    code = tl.load(table + k * 131 + t, mask=t < 131, other=0)
    li = code >> 6
    sh = code & 63
    base = limbs + src * LSTRIDE + rows * 3
    l0 = tl.load(base + 0, mask=rm, other=0)
    l1 = tl.load(base + 1, mask=rm, other=0)
    l2 = tl.load(base + 2, mask=rm, other=0)
    a = tl.where(li[None, :] == 0, l0[:, None],
                 tl.where(li[None, :] == 1, l1[:, None], l2[:, None]))
    bit = (a >> sh[None, :].to(tl.int64)) & 1
    bit = tl.where((t < 131)[None, :], bit, 0)
    _store_limbs(rows, bit, t, limbs + dst * LSTRIDE + rows * 3, n_rows)


@triton.jit
def _frob_limbs(limbs, table, n_rows, LSTRIDE, R: tl.constexpr):
    pid = tl.program_id(0)
    rows = pid * R + tl.arange(0, R)
    t = tl.arange(0, 256)
    rm = rows < n_rows
    base = limbs + rows * 3
    x0 = tl.load(base + 0, mask=rm, other=0)
    x1 = tl.load(base + 1, mask=rm, other=0)
    x2 = tl.load(base + 2, mask=rm, other=0)
    hw = _popc(x0) + _popc(x1) + _popc(x2 & 7)
    j = 3 + ((hw >> 1) & 7)
    code = tl.load(table + j[:, None] * 131 + t[None, :],
                   mask=(t < 131)[None, :] & rm[:, None], other=0)
    li = code >> 6
    sh = (code & 63).to(tl.int64)
    a0 = tl.load(limbs + rows * 3 + 0, mask=rm, other=0)
    a1 = tl.load(limbs + rows * 3 + 1, mask=rm, other=0)
    a2 = tl.load(limbs + rows * 3 + 2, mask=rm, other=0)
    a = tl.where(li == 0, a0[:, None], tl.where(li == 1, a1[:, None], a2[:, None]))
    bit = (a >> sh) & 1
    bit = tl.where((t < 131)[None, :], bit, 0)
    _store_limbs(rows, bit, t, limbs + 2 * LSTRIDE + rows * 3, n_rows)
    b0 = tl.load(limbs + 1 * LSTRIDE + rows * 3 + 0, mask=rm, other=0)
    b1 = tl.load(limbs + 1 * LSTRIDE + rows * 3 + 1, mask=rm, other=0)
    b2 = tl.load(limbs + 1 * LSTRIDE + rows * 3 + 2, mask=rm, other=0)
    b = tl.where(li == 0, b0[:, None], tl.where(li == 1, b1[:, None], b2[:, None]))
    bitb = (b >> sh) & 1
    bitb = tl.where((t < 131)[None, :], bitb, 0)
    _store_limbs(rows, bitb, t, limbs + 3 * LSTRIDE + rows * 3, n_rows)


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
def _xor_limbs(limbs, s1, s2, dst, n_rows, LSTRIDE, R: tl.constexpr):
    pid = tl.program_id(0)
    rows = pid * R + tl.arange(0, R)
    rm = rows < n_rows
    a = limbs + s1 * LSTRIDE + rows * 3
    b = limbs + s2 * LSTRIDE + rows * 3
    o = limbs + dst * LSTRIDE + rows * 3
    for k in tl.static_range(0, 3):
        tl.store(o + k, tl.load(a + k, mask=rm, other=0) ^ tl.load(b + k, mask=rm, other=0), mask=rm)


@triton.jit
def _xor3_limbs(limbs, s1, s2, s3, dst, n_rows, LSTRIDE, R: tl.constexpr):
    pid = tl.program_id(0)
    rows = pid * R + tl.arange(0, R)
    rm = rows < n_rows
    a = limbs + s1 * LSTRIDE + rows * 3
    b = limbs + s2 * LSTRIDE + rows * 3
    c = limbs + s3 * LSTRIDE + rows * 3
    o = limbs + dst * LSTRIDE + rows * 3
    for k in tl.static_range(0, 3):
        tl.store(o + k, tl.load(a + k, mask=rm, other=0) ^ tl.load(b + k, mask=rm, other=0)
                 ^ tl.load(c + k, mask=rm, other=0), mask=rm)


@triton.jit
def _copy_limbs(limbs, src, dst, n_rows, LSTRIDE, R: tl.constexpr):
    pid = tl.program_id(0)
    rows = pid * R + tl.arange(0, R)
    rm = rows < n_rows
    a = limbs + src * LSTRIDE + rows * 3
    o = limbs + dst * LSTRIDE + rows * 3
    for k in tl.static_range(0, 3):
        tl.store(o + k, tl.load(a + k, mask=rm, other=0), mask=rm)


def median_ms(fn, reps=10):
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
    torch.manual_seed(53)
    dev = "cuda"
    rep = {"gpu": torch.cuda.get_device_name(), "rows": N, "R": R}
    x = wsp._random_elements(N).cuda()
    y = wsp._random_elements(N).cuda()
    table = K.get_table(dev)
    NSLOT = 14
    LSTRIDE = N * 3
    limbs = torch.zeros((NSLOT * LSTRIDE,), dtype=torch.int64, device=dev)
    out = torch.empty((N, 2, 3), dtype=torch.int64, device=dev)
    gr = (N // R + 1,)

    def load_xy():
        limbs[0 * LSTRIDE:1 * LSTRIDE].view(N, 3).copy_(x)
        limbs[1 * LSTRIDE:2 * LSTRIDE].view(N, 3).copy_(y)

    def mul(sa, sb, dst):
        CM._mul_clmad_kernel[(N // 128,)](limbs[sa * LSTRIDE:], limbs[sb * LSTRIDE:],
                                          limbs[dst * LSTRIDE:], BLOCK=128)

    def step():
        _frob_limbs[gr](limbs, table, N, LSTRIDE, R=R, num_warps=WARPS)
        _xor_limbs[gr](limbs, 0, 2, 4, N, LSTRIDE, R=R, num_warps=WARPS)
        _copy_limbs[gr](limbs, 4, 5, N, LSTRIDE, R=R, num_warps=WARPS)
        acc, tmp = 5, 7
        for k in (1, 2, 4, 8, 16, 32):
            _sigma_limbs[gr](limbs, table, acc, 6, k, N, LSTRIDE, R=R, num_warps=WARPS)
            mul(6, acc, tmp)
            acc, tmp = tmp, acc
        _sigma_limbs[gr](limbs, table, acc, 6, 1, N, LSTRIDE, R=R, num_warps=WARPS)
        mul(6, 4, tmp)
        acc, tmp = tmp, acc
        _sigma_limbs[gr](limbs, table, acc, 6, 65, N, LSTRIDE, R=R, num_warps=WARPS)
        mul(6, acc, tmp)
        acc, tmp = tmp, acc
        _sigma_limbs[gr](limbs, table, acc, 6, 1, N, LSTRIDE, R=R, num_warps=WARPS)
        _xor_limbs[gr](limbs, 1, 3, 12, N, LSTRIDE, R=R, num_warps=WARPS)
        mul(12, 6, 8)
        _sigma_limbs[gr](limbs, table, 8, 9, 1, N, LSTRIDE, R=R, num_warps=WARPS)
        _xor3_limbs[gr](limbs, 9, 8, 4, 9, N, LSTRIDE, R=R, num_warps=WARPS)
        _xor_limbs[gr](limbs, 0, 9, 12, N, LSTRIDE, R=R, num_warps=WARPS)
        mul(8, 12, 13)
        _xor3_limbs[gr](limbs, 13, 9, 1, 13, N, LSTRIDE, R=R, num_warps=WARPS)

    want = K.kernel_function(x, y)
    load_xy()
    step()
    out[:, 0] = limbs[9 * LSTRIDE:9 * LSTRIDE + LSTRIDE].view(N, 3)
    out[:, 1] = limbs[13 * LSTRIDE:13 * LSTRIDE + LSTRIDE].view(N, 3)
    rep["step_exact"] = bool(torch.equal(out, want))

    ms = median_ms(lambda: (load_xy(), step()))
    rep["v8_step_ms"] = ms
    rep["v8_updates_per_s"] = N / ms * 1e3
    print(json.dumps(rep))
    (pathlib.Path("/results") if pathlib.Path("/results").is_dir() else HERE).joinpath("v8.json").write_text(
        json.dumps(rep, indent=2))
    return 0 if rep["step_exact"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
