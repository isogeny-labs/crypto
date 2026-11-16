"""v7: limbs are produced by whoever writes the slot, so packs disappear.

The byte-per-bit slot layout is what makes sigma cheap (a 131-lane gather),
and packed limbs are what clmad wants. Until now every multiply paid two pack
kernels to rebuild limbs from bytes. But sigma/xor/copy/frob already hold the
produced bits in registers: three masked reductions give them the limbs for
nothing.

So: every kernel that writes a slot writes both forms; the multiply reads
limbs directly; only the multiply's output still needs an unpack (it is one
row per lane, which is the wrong shape to scatter bytes).

per multiply: pack + pack + clmad + unpack  ->  clmad + unpack
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
S = K.STRIDE


@triton.jit
def _emit(tile, offs, limbs, slot, rows, n_rows, LSTRIDE):
    """Store the 3 packed limbs of a (R, 256) bit tile."""
    v = tl.where(offs[None, :] < 131, tile, 0).to(tl.int64)
    sh = (offs % 64).to(tl.int64)
    w = v << sh[None, :]
    l0 = tl.sum(tl.where((offs < 64)[None, :], w, 0), axis=1)
    l1 = tl.sum(tl.where(((offs >= 64) & (offs < 128))[None, :], w, 0), axis=1)
    l2 = tl.sum(tl.where((offs >= 128)[None, :], w, 0), axis=1)
    base = limbs + slot * LSTRIDE + rows * 3
    rm = rows < n_rows
    tl.store(base + 0, l0, mask=rm)
    tl.store(base + 1, l1, mask=rm)
    tl.store(base + 2, l2, mask=rm)


@triton.jit
def _sigma_multi(scratch, table, limbs, src, dst, k, n_rows, LSTRIDE,
                 R: tl.constexpr, STRIDE: tl.constexpr):
    pid = tl.program_id(0)
    rows = pid * R + tl.arange(0, R)
    offs = tl.arange(0, 256)
    m = ((offs < 131)[None, :]) & (rows < n_rows)[:, None]
    idx = tl.load(table + k * 131 + offs, mask=offs < 131, other=0)
    v = tl.load(scratch + rows[:, None] * STRIDE + src * 264 + idx[None, :], mask=m, other=0)
    tl.store(scratch + rows[:, None] * STRIDE + dst * 264 + offs[None, :], v, mask=m)
    _emit(v, offs, limbs, dst, rows, n_rows, LSTRIDE)


@triton.jit
def _frob_multi(scratch, table, limbs, n_rows, LSTRIDE, R: tl.constexpr, STRIDE: tl.constexpr):
    pid = tl.program_id(0)
    rows = pid * R + tl.arange(0, R)
    offs = tl.arange(0, 256)
    m = ((offs < 131)[None, :]) & (rows < n_rows)[:, None]
    base = scratch + rows[:, None] * STRIDE
    xb = tl.load(base + 0 * 264 + offs[None, :], mask=m, other=0)
    hw = tl.sum(xb.to(tl.int32), axis=1)
    j = 3 + ((hw >> 1) & 7)
    idx = tl.load(table + j[:, None] * 131 + offs[None, :], mask=m, other=0)
    vx = tl.load(base + 0 * 264 + idx, mask=m, other=0)
    tl.store(base + 2 * 264 + offs[None, :], vx, mask=m)
    _emit(vx, offs, limbs, 2, rows, n_rows, LSTRIDE)
    vy = tl.load(base + 1 * 264 + idx, mask=m, other=0)
    tl.store(base + 3 * 264 + offs[None, :], vy, mask=m)
    _emit(vy, offs, limbs, 3, rows, n_rows, LSTRIDE)


@triton.jit
def _xor_multi(scratch, limbs, s1, s2, dst, n_rows, LSTRIDE, R: tl.constexpr, STRIDE: tl.constexpr):
    pid = tl.program_id(0)
    rows = pid * R + tl.arange(0, R)
    offs = tl.arange(0, 256)
    m = ((offs < 131)[None, :]) & (rows < n_rows)[:, None]
    base = scratch + rows[:, None] * STRIDE
    a = tl.load(base + s1 * 264 + offs[None, :], mask=m, other=0)
    b = tl.load(base + s2 * 264 + offs[None, :], mask=m, other=0)
    v = a ^ b
    tl.store(base + dst * 264 + offs[None, :], v, mask=m)
    _emit(v, offs, limbs, dst, rows, n_rows, LSTRIDE)


@triton.jit
def _xor3_multi(scratch, limbs, s1, s2, s3, dst, n_rows, LSTRIDE, R: tl.constexpr, STRIDE: tl.constexpr):
    pid = tl.program_id(0)
    rows = pid * R + tl.arange(0, R)
    offs = tl.arange(0, 256)
    m = ((offs < 131)[None, :]) & (rows < n_rows)[:, None]
    base = scratch + rows[:, None] * STRIDE
    a = tl.load(base + s1 * 264 + offs[None, :], mask=m, other=0)
    b = tl.load(base + s2 * 264 + offs[None, :], mask=m, other=0)
    c = tl.load(base + s3 * 264 + offs[None, :], mask=m, other=0)
    v = a ^ b ^ c
    tl.store(base + dst * 264 + offs[None, :], v, mask=m)
    _emit(v, offs, limbs, dst, rows, n_rows, LSTRIDE)


@triton.jit
def _copy_multi(scratch, limbs, src, dst, n_rows, LSTRIDE, R: tl.constexpr, STRIDE: tl.constexpr):
    pid = tl.program_id(0)
    rows = pid * R + tl.arange(0, R)
    offs = tl.arange(0, 256)
    m = ((offs < 131)[None, :]) & (rows < n_rows)[:, None]
    base = scratch + rows[:, None] * STRIDE
    v = tl.load(base + src * 264 + offs[None, :], mask=m, other=0)
    tl.store(base + dst * 264 + offs[None, :], v, mask=m)
    _emit(v, offs, limbs, dst, rows, n_rows, LSTRIDE)


@triton.jit
def _unpack_multi(limbs, scratch, dst, n_rows, LSTRIDE, R: tl.constexpr, STRIDE: tl.constexpr):
    pid = tl.program_id(0)
    rows = pid * R + tl.arange(0, R)
    offs = tl.arange(0, 256)
    rm = rows < n_rows
    base = limbs + dst * LSTRIDE + rows * 3
    l0 = tl.load(base + 0, mask=rm, other=0)
    l1 = tl.load(base + 1, mask=rm, other=0)
    l2 = tl.load(base + 2, mask=rm, other=0)
    li = offs // 64
    sh = (offs % 64).to(tl.int64)
    a = tl.where(li == 0, l0[:, None], tl.where(li == 1, l1[:, None], l2[:, None]))
    bits = ((a >> sh[None, :]) & 1).to(tl.int8)
    m = ((offs < 131)[None, :]) & rm[:, None]
    tl.store(scratch + rows[:, None] * STRIDE + dst * 264 + offs[None, :], bits, mask=m)


@triton.jit
def _pack_out_multi(limbs, out_ptr, slot, col, n_rows, LSTRIDE, R: tl.constexpr):
    pid = tl.program_id(0)
    rows = pid * R + tl.arange(0, R)
    rm = rows < n_rows
    base = limbs + slot * LSTRIDE + rows * 3
    tl.store(out_ptr + rows * 6 + col + 0, tl.load(base + 0, mask=rm, other=0), mask=rm)
    tl.store(out_ptr + rows * 6 + col + 1, tl.load(base + 1, mask=rm, other=0), mask=rm)
    tl.store(out_ptr + rows * 6 + col + 2, tl.load(base + 2, mask=rm, other=0), mask=rm)


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
    torch.manual_seed(41)
    dev = "cuda"
    rep = {"gpu": torch.cuda.get_device_name(), "rows": N, "R": R}
    x = wsp._random_elements(N).cuda()
    y = wsp._random_elements(N).cuda()
    table = K.get_table(dev)
    scratch = torch.empty((N, S), dtype=torch.int8, device=dev)
    NSLOT = 14
    LSTRIDE = N * 3
    limbs = torch.zeros((NSLOT * LSTRIDE,), dtype=torch.int64, device=dev)
    out = torch.empty((N, 2, 3), dtype=torch.int64, device=dev)
    gr = (N // R + 1,)
    g = (N,)

    def step():
        _frob_multi[gr](scratch, table, limbs, N, LSTRIDE, R=R, STRIDE=S, num_warps=WARPS)
        _xor_multi[gr](scratch, limbs, 0, 2, 4, N, LSTRIDE, R=R, STRIDE=S, num_warps=WARPS)
        _copy_multi[gr](scratch, limbs, 4, 5, N, LSTRIDE, R=R, STRIDE=S, num_warps=WARPS)
        acc, tmp = 5, 7
        for k in (1, 2, 4, 8, 16, 32):
            _sigma_multi[gr](scratch, table, limbs, acc, 6, k, N, LSTRIDE, R=R, STRIDE=S, num_warps=WARPS)
            CM._mul_clmad_kernel[(N // 128,)](limbs[6 * LSTRIDE:], limbs[acc * LSTRIDE:], limbs[tmp * LSTRIDE:], BLOCK=128)
            _unpack_multi[gr](limbs, scratch, tmp, N, LSTRIDE, R=R, STRIDE=S, num_warps=WARPS)
            acc, tmp = tmp, acc
        _sigma_multi[gr](scratch, table, limbs, acc, 6, 1, N, LSTRIDE, R=R, STRIDE=S, num_warps=WARPS)
        CM._mul_clmad_kernel[(N // 128,)](limbs[6 * LSTRIDE:], limbs[4 * LSTRIDE:], limbs[tmp * LSTRIDE:], BLOCK=128)
        _unpack_multi[gr](limbs, scratch, tmp, N, LSTRIDE, R=R, STRIDE=S, num_warps=WARPS)
        acc, tmp = tmp, acc
        _sigma_multi[gr](scratch, table, limbs, acc, 6, 65, N, LSTRIDE, R=R, STRIDE=S, num_warps=WARPS)
        CM._mul_clmad_kernel[(N // 128,)](limbs[6 * LSTRIDE:], limbs[acc * LSTRIDE:], limbs[tmp * LSTRIDE:], BLOCK=128)
        _unpack_multi[gr](limbs, scratch, tmp, N, LSTRIDE, R=R, STRIDE=S, num_warps=WARPS)
        acc, tmp = tmp, acc
        _sigma_multi[gr](scratch, table, limbs, acc, 6, 1, N, LSTRIDE, R=R, STRIDE=S, num_warps=WARPS)
        _xor_multi[gr](scratch, limbs, 1, 3, 12, N, LSTRIDE, R=R, STRIDE=S, num_warps=WARPS)
        CM._mul_clmad_kernel[(N // 128,)](limbs[12 * LSTRIDE:], limbs[6 * LSTRIDE:], limbs[8 * LSTRIDE:], BLOCK=128)
        _unpack_multi[gr](limbs, scratch, 8, N, LSTRIDE, R=R, STRIDE=S, num_warps=WARPS)
        _sigma_multi[gr](scratch, table, limbs, 8, 9, 1, N, LSTRIDE, R=R, STRIDE=S, num_warps=WARPS)
        _xor3_multi[gr](scratch, limbs, 9, 8, 4, 9, N, LSTRIDE, R=R, STRIDE=S, num_warps=WARPS)
        _xor_multi[gr](scratch, limbs, 0, 9, 12, N, LSTRIDE, R=R, STRIDE=S, num_warps=WARPS)
        CM._mul_clmad_kernel[(N // 128,)](limbs[8 * LSTRIDE:], limbs[12 * LSTRIDE:], limbs[13 * LSTRIDE:], BLOCK=128)
        _unpack_multi[gr](limbs, scratch, 13, N, LSTRIDE, R=R, STRIDE=S, num_warps=WARPS)
        _xor3_multi[gr](scratch, limbs, 13, 9, 1, 13, N, LSTRIDE, R=R, STRIDE=S, num_warps=WARPS)

    want = K.kernel_function(x, y)
    K._unpack_kernel[g](x, y, scratch, N, STRIDE=S, num_warps=WARPS)
    step()
    _pack_out_multi[gr](limbs, out, 9, 0, N, LSTRIDE, R=R)
    _pack_out_multi[gr](limbs, out, 13, 3, N, LSTRIDE, R=R)
    rep["step_exact"] = bool(torch.equal(out, want))

    ms = median_ms(lambda: (K._unpack_kernel[g](x, y, scratch, N, STRIDE=S, num_warps=WARPS), step()))
    rep["v7_step_ms"] = ms
    rep["v7_updates_per_s"] = N / ms * 1e3
    print(json.dumps(rep))
    (pathlib.Path("/results") if pathlib.Path("/results").is_dir() else HERE).joinpath("v7.json").write_text(
        json.dumps(rep, indent=2))
    return 0 if rep["step_exact"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
