"""v6: clmad product + every per-row kernel batched over R rows per program.

The pack/unpack batching gave 2x on the step; sigma, xor, copy and frob have
the same one-row-per-program shape (a 256-lane gather with three quarters of
the lanes masked off), so they get the same treatment: an (R, 256) tile, with
the per-row index built from a broadcast table row.

frob is the one that cannot share a table row across the tile: j is a per-row
scalar (Hamming weight of that row's x), so its gather index is (R, 256).
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
import clmad_step2 as S2  # noqa: E402  (batched pack/unpack live there)

N = 1 << 16
R = 8
WARPS = 4
S = K.STRIDE


@triton.jit
def _row_mask(rows, n_rows, offs):
    return ((offs < 131)[None, :]) & (rows < n_rows)[:, None]


@triton.jit
def _sigma_multi(scratch, table, src, dst, k, n_rows, R: tl.constexpr, STRIDE: tl.constexpr):
    pid = tl.program_id(0)
    rows = pid * R + tl.arange(0, R)
    offs = tl.arange(0, 256)
    m = _row_mask(rows, n_rows, offs)
    idx = tl.load(table + k * 131 + offs, mask=offs < 131, other=0)
    v = tl.load(scratch + rows[:, None] * STRIDE + src * 264 + idx[None, :], mask=m, other=0)
    tl.store(scratch + rows[:, None] * STRIDE + dst * 264 + offs[None, :], v, mask=m)


@triton.jit
def _frob_multi(scratch, table, n_rows, R: tl.constexpr, STRIDE: tl.constexpr):
    pid = tl.program_id(0)
    rows = pid * R + tl.arange(0, R)
    offs = tl.arange(0, 256)
    m = _row_mask(rows, n_rows, offs)
    base = scratch + rows[:, None] * STRIDE
    xb = tl.load(base + 0 * 264 + offs[None, :], mask=m, other=0)
    hw = tl.sum(xb.to(tl.int32), axis=1)
    j = 3 + ((hw >> 1) & 7)
    idx = tl.load(table + j[:, None] * 131 + offs[None, :], mask=m, other=0)
    vx = tl.load(base + 0 * 264 + idx, mask=m, other=0)
    tl.store(base + 2 * 264 + offs[None, :], vx, mask=m)
    vy = tl.load(base + 1 * 264 + idx, mask=m, other=0)
    tl.store(base + 3 * 264 + offs[None, :], vy, mask=m)


@triton.jit
def _xor_multi(scratch, s1, s2, dst, n_rows, R: tl.constexpr, STRIDE: tl.constexpr):
    pid = tl.program_id(0)
    rows = pid * R + tl.arange(0, R)
    offs = tl.arange(0, 256)
    m = _row_mask(rows, n_rows, offs)
    base = scratch + rows[:, None] * STRIDE
    a = tl.load(base + s1 * 264 + offs[None, :], mask=m, other=0)
    b = tl.load(base + s2 * 264 + offs[None, :], mask=m, other=0)
    tl.store(base + dst * 264 + offs[None, :], a ^ b, mask=m)


@triton.jit
def _xor3_multi(scratch, s1, s2, s3, dst, n_rows, R: tl.constexpr, STRIDE: tl.constexpr):
    pid = tl.program_id(0)
    rows = pid * R + tl.arange(0, R)
    offs = tl.arange(0, 256)
    m = _row_mask(rows, n_rows, offs)
    base = scratch + rows[:, None] * STRIDE
    a = tl.load(base + s1 * 264 + offs[None, :], mask=m, other=0)
    b = tl.load(base + s2 * 264 + offs[None, :], mask=m, other=0)
    c = tl.load(base + s3 * 264 + offs[None, :], mask=m, other=0)
    tl.store(base + dst * 264 + offs[None, :], a ^ b ^ c, mask=m)


@triton.jit
def _copy_multi(scratch, src, dst, n_rows, R: tl.constexpr, STRIDE: tl.constexpr):
    pid = tl.program_id(0)
    rows = pid * R + tl.arange(0, R)
    offs = tl.arange(0, 256)
    m = _row_mask(rows, n_rows, offs)
    base = scratch + rows[:, None] * STRIDE
    v = tl.load(base + src * 264 + offs[None, :], mask=m, other=0)
    tl.store(base + dst * 264 + offs[None, :], v, mask=m)


@triton.jit
def _pack_slot(scratch, out_ptr, slot, col, rows, offs, m, n_rows, STRIDE: tl.constexpr):
    base = scratch + rows[:, None] * STRIDE
    sh = (offs % 64).to(tl.int64)
    b = tl.load(base + slot * 264 + offs[None, :], mask=m, other=0).to(tl.int64)
    v = tl.where(m, b << sh[None, :], 0)
    l0 = tl.sum(tl.where((offs < 64)[None, :], v, 0), axis=1)
    l1 = tl.sum(tl.where(((offs >= 64) & (offs < 128))[None, :], v, 0), axis=1)
    l2 = tl.sum(tl.where((offs >= 128)[None, :], v, 0), axis=1)
    tl.store(out_ptr + rows * 6 + col + 0, l0, mask=rows < n_rows)
    tl.store(out_ptr + rows * 6 + col + 1, l1, mask=rows < n_rows)
    tl.store(out_ptr + rows * 6 + col + 2, l2, mask=rows < n_rows)


@triton.jit
def _pack_out_multi(scratch, out_ptr, n_rows, R: tl.constexpr, STRIDE: tl.constexpr):
    pid = tl.program_id(0)
    rows = pid * R + tl.arange(0, R)
    offs = tl.arange(0, 256)
    m = _row_mask(rows, n_rows, offs)
    _pack_slot(scratch, out_ptr, 9, 0, rows, offs, m, n_rows, STRIDE)
    _pack_slot(scratch, out_ptr, 13, 3, rows, offs, m, n_rows, STRIDE)


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
    torch.manual_seed(31)
    dev = "cuda"
    rep = {"gpu": torch.cuda.get_device_name(), "rows": N, "R": R}
    x = wsp._random_elements(N).cuda()
    y = wsp._random_elements(N).cuda()
    table = K.get_table(dev)
    scratch = torch.empty((N, S), dtype=torch.int8, device=dev)
    la = torch.empty((N, 3), dtype=torch.int64, device=dev)
    lb = torch.empty((N, 3), dtype=torch.int64, device=dev)
    lo = torch.empty((N, 3), dtype=torch.int64, device=dev)
    gr = (N // R + 1,)
    g = (N,)

    def mul(sa, sb, dst):
        S2._pack_multi[(N // R + 1,)](scratch, la, sa, N, R=R, STRIDE=S, num_warps=WARPS)
        S2._pack_multi[(N // R + 1,)](scratch, lb, sb, N, R=R, STRIDE=S, num_warps=WARPS)
        CM._mul_clmad_kernel[(N // 128,)](la, lb, lo, BLOCK=128)
        S2._unpack_multi[(N // R + 1,)](lo, scratch, dst, N, R=R, STRIDE=S, num_warps=WARPS)

    def step():
        _frob_multi[gr](scratch, table, N, R=R, STRIDE=S, num_warps=WARPS)
        _xor_multi[gr](scratch, 0, 2, 4, N, R=R, STRIDE=S, num_warps=WARPS)
        _copy_multi[gr](scratch, 4, 5, N, R=R, STRIDE=S, num_warps=WARPS)
        acc, tmp = 5, 7
        for k in (1, 2, 4, 8, 16, 32):
            _sigma_multi[gr](scratch, table, acc, 6, k, N, R=R, STRIDE=S, num_warps=WARPS)
            mul(6, acc, tmp)
            acc, tmp = tmp, acc
        _sigma_multi[gr](scratch, table, acc, 6, 1, N, R=R, STRIDE=S, num_warps=WARPS)
        mul(6, 4, tmp)
        acc, tmp = tmp, acc
        _sigma_multi[gr](scratch, table, acc, 6, 65, N, R=R, STRIDE=S, num_warps=WARPS)
        mul(6, acc, tmp)
        acc, tmp = tmp, acc
        _sigma_multi[gr](scratch, table, acc, 6, 1, N, R=R, STRIDE=S, num_warps=WARPS)
        _xor_multi[gr](scratch, 1, 3, 12, N, R=R, STRIDE=S, num_warps=WARPS)
        mul(12, 6, 8)
        _sigma_multi[gr](scratch, table, 8, 9, 1, N, R=R, STRIDE=S, num_warps=WARPS)
        _xor3_multi[gr](scratch, 9, 8, 4, 9, N, R=R, STRIDE=S, num_warps=WARPS)
        _xor_multi[gr](scratch, 0, 9, 12, N, R=R, STRIDE=S, num_warps=WARPS)
        mul(8, 12, 13)
        _xor3_multi[gr](scratch, 13, 9, 1, 13, N, R=R, STRIDE=S, num_warps=WARPS)

    want = K.kernel_function(x, y)
    K._unpack_kernel[g](x, y, scratch, N, STRIDE=S, num_warps=WARPS)
    step()
    out = torch.empty((N, 2, 3), dtype=torch.int64, device=dev)
    _pack_out_multi[gr](scratch, out, N, R=R, STRIDE=S, num_warps=WARPS)
    rep["step_exact"] = bool(torch.equal(out, want))

    ms = median_ms(lambda: (K._unpack_kernel[g](x, y, scratch, N, STRIDE=S, num_warps=WARPS), step()))
    rep["v6_step_ms"] = ms
    rep["v6_updates_per_s"] = N / ms * 1e3
    print(json.dumps(rep))
    (pathlib.Path("/results") if pathlib.Path("/results").is_dir() else HERE).joinpath("v6.json").write_text(
        json.dumps(rep, indent=2))
    return 0 if rep["step_exact"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
