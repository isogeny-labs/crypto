"""Batch the bits<->limbs conversions, then re-time the whole step.

A pack currently runs one row per program: a 256-lane tile with three masked
reductions, most of the machine idle. Handling R rows per program turns the
reduction into tl.sum(axis=1) over an (R, 256) tile and amortises the launch.

Sweeps R, then times the full step with the best R against the current one.
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
WARPS = 4


@triton.jit
def _pack_multi(scratch, limbs, src, n_rows, R: tl.constexpr, STRIDE: tl.constexpr):
    pid = tl.program_id(0)
    rows = pid * R + tl.arange(0, R)
    offs = tl.arange(0, 256)
    rm = rows < n_rows
    base = scratch + rows[:, None] * STRIDE + src * 264
    m = (offs < 131)[None, :] & rm[:, None]
    b = tl.load(base + offs[None, :], mask=m, other=0).to(tl.int64)
    sh = (offs % 64).to(tl.int64)
    v = tl.where(m, b << sh, 0)
    l0 = tl.sum(tl.where((offs < 64)[None, :], v, 0), axis=1)
    l1 = tl.sum(tl.where(((offs >= 64) & (offs < 128))[None, :], v, 0), axis=1)
    l2 = tl.sum(tl.where((offs >= 128)[None, :], v, 0), axis=1)
    tl.store(limbs + rows * 3 + 0, l0, mask=rm)
    tl.store(limbs + rows * 3 + 1, l1, mask=rm)
    tl.store(limbs + rows * 3 + 2, l2, mask=rm)


@triton.jit
def _unpack_multi(limbs, scratch, dst, n_rows, R: tl.constexpr, STRIDE: tl.constexpr):
    pid = tl.program_id(0)
    rows = pid * R + tl.arange(0, R)
    offs = tl.arange(0, 256)
    rm = rows < n_rows
    l0 = tl.load(limbs + rows * 3 + 0, mask=rm, other=0)
    l1 = tl.load(limbs + rows * 3 + 1, mask=rm, other=0)
    l2 = tl.load(limbs + rows * 3 + 2, mask=rm, other=0)
    li = offs // 64
    sh = (offs % 64).to(tl.int64)
    a = tl.where(li == 0, l0[:, None], tl.where(li == 1, l1[:, None], l2[:, None]))
    bits = ((a >> sh[None, :]) & 1).to(tl.int8)
    m = (offs < 131)[None, :] & rm[:, None]
    tl.store(scratch + rows[:, None] * STRIDE + dst * 264 + offs[None, :], bits, mask=m)


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
    torch.manual_seed(23)
    dev = "cuda"
    rep = {"gpu": torch.cuda.get_device_name(), "rows": N}
    x = wsp._random_elements(N).cuda()
    y = wsp._random_elements(N).cuda()
    table = K.get_table(dev)
    S = K.STRIDE
    scratch = torch.empty((N, S), dtype=torch.int8, device=dev)
    la = torch.empty((N, 3), dtype=torch.int64, device=dev)
    lb = torch.empty((N, 3), dtype=torch.int64, device=dev)
    lo = torch.empty((N, 3), dtype=torch.int64, device=dev)
    K._unpack_kernel[(N,)](x, y, scratch, N, STRIDE=S, num_warps=WARPS)

    # baseline conversions (one row per program)
    base_pack = median_ms(lambda: CM._pack_bits_kernel[(N,)](scratch, la, 1, N, STRIDE=S, num_warps=WARPS))
    base_unpack = median_ms(lambda: CM._unpack_bits_kernel[(N,)](lo, scratch, 13, N, STRIDE=S, num_warps=WARPS))
    rep["base_pack_ms"] = base_pack
    rep["base_unpack_ms"] = base_unpack

    best = {}
    for R in (4, 8, 16):
        for warps in (4, 8):
            p = median_ms(lambda _r=R, _w=warps: _pack_multi[(N // _r + 1,)](scratch, la, 1, N, R=_r, STRIDE=S, num_warps=_w))
            u = median_ms(lambda _r=R, _w=warps: _unpack_multi[(N // _r + 1,)](lo, scratch, 13, N, R=_r, STRIDE=S, num_warps=_w))
            rep["pack_R%d_w%d" % (R, warps)] = p
            rep["unpack_R%d_w%d" % (R, warps)] = u

    # correctness of the batched pair against the scalar versions
    CM._pack_bits_kernel[(N,)](scratch, lb, 1, N, STRIDE=S, num_warps=WARPS)
    _pack_multi[(N // 8 + 1,)](scratch, la, 1, N, R=8, STRIDE=S, num_warps=4)
    rep["pack_matches"] = bool(torch.equal(la, lb))
    _unpack_multi[(N // 8 + 1,)](lb, scratch, 14, N, R=8, STRIDE=S, num_warps=4)
    CM._unpack_bits_kernel[(N,)](lb, scratch, 15, N, STRIDE=S, num_warps=4)
    rep["unpack_matches"] = bool(torch.equal(scratch[:, 14 * 264:14 * 264 + 131],
                                             scratch[:, 15 * 264:15 * 264 + 131]))

    # whole step with batched conversions
    R = 8

    def mul(sa, sb, dst):
        _pack_multi[(N // R + 1,)](scratch, la, sa, N, R=R, STRIDE=S, num_warps=4)
        _pack_multi[(N // R + 1,)](scratch, lb, sb, N, R=R, STRIDE=S, num_warps=4)
        CM._mul_clmad_kernel[(N // 128,)](la, lb, lo, BLOCK=128)
        _unpack_multi[(N // R + 1,)](lo, scratch, dst, N, R=R, STRIDE=S, num_warps=4)

    g = (N,)
    gm = (N // 4,)

    def step():
        K._frob_kernel[g](scratch, table, STRIDE=S, num_warps=WARPS)
        K._xor_kernel[g](scratch, 0, 2, 4, STRIDE=S, num_warps=WARPS)
        K._copy_kernel[g](scratch, 4, 5, STRIDE=S, num_warps=WARPS)
        acc, tmp = 5, 7
        for k in (1, 2, 4, 8, 16, 32):
            K._sigma_kernel[g](scratch, table, acc, 6, k, STRIDE=S, num_warps=WARPS)
            mul(6, acc, tmp)
            acc, tmp = tmp, acc
        K._sigma_kernel[g](scratch, table, acc, 6, 1, STRIDE=S, num_warps=WARPS)
        mul(6, 4, tmp)
        acc, tmp = tmp, acc
        K._sigma_kernel[g](scratch, table, acc, 6, 65, STRIDE=S, num_warps=WARPS)
        mul(6, acc, tmp)
        acc, tmp = tmp, acc
        K._sigma_kernel[g](scratch, table, acc, 6, 1, STRIDE=S, num_warps=WARPS)
        K._xor_kernel[g](scratch, 1, 3, 12, STRIDE=S, num_warps=WARPS)
        mul(12, 6, 8)
        K._sigma_kernel[g](scratch, table, 8, 9, 1, STRIDE=S, num_warps=WARPS)
        K._xor3_kernel[g](scratch, 9, 8, 4, 9, STRIDE=S, num_warps=WARPS)
        K._xor_kernel[g](scratch, 0, 9, 12, STRIDE=S, num_warps=WARPS)
        mul(8, 12, 13)
        K._xor3_kernel[g](scratch, 13, 9, 1, 13, STRIDE=S, num_warps=WARPS)

    want = K.kernel_function(x, y)

    # control: the same step with the scalar conversions that were verified in
    # clmad_step.py, run in this same script, to separate plumbing from kernels
    def mul_scalar(sa, sb, dst):
        CM._pack_bits_kernel[(N,)](scratch, la, sa, N, STRIDE=S, num_warps=WARPS)
        CM._pack_bits_kernel[(N,)](scratch, lb, sb, N, STRIDE=S, num_warps=WARPS)
        CM._mul_clmad_kernel[(N // 128,)](la, lb, lo, BLOCK=128)
        CM._unpack_bits_kernel[(N,)](lo, scratch, dst, N, STRIDE=S, num_warps=WARPS)

    def step_scalar():
        K._frob_kernel[g](scratch, table, STRIDE=S, num_warps=WARPS)
        K._xor_kernel[g](scratch, 0, 2, 4, STRIDE=S, num_warps=WARPS)
        K._copy_kernel[g](scratch, 4, 5, STRIDE=S, num_warps=WARPS)
        acc, tmp = 5, 7
        for k in (1, 2, 4, 8, 16, 32):
            K._sigma_kernel[g](scratch, table, acc, 6, k, STRIDE=S, num_warps=WARPS)
            mul_scalar(6, acc, tmp)
            acc, tmp = tmp, acc
        K._sigma_kernel[g](scratch, table, acc, 6, 1, STRIDE=S, num_warps=WARPS)
        mul_scalar(6, 4, tmp)
        acc, tmp = tmp, acc
        K._sigma_kernel[g](scratch, table, acc, 6, 65, STRIDE=S, num_warps=WARPS)
        mul_scalar(6, acc, tmp)
        acc, tmp = tmp, acc
        K._sigma_kernel[g](scratch, table, acc, 6, 1, STRIDE=S, num_warps=WARPS)
        K._xor_kernel[g](scratch, 1, 3, 12, STRIDE=S, num_warps=WARPS)
        mul_scalar(12, 6, 8)
        K._sigma_kernel[g](scratch, table, 8, 9, 1, STRIDE=S, num_warps=WARPS)
        K._xor3_kernel[g](scratch, 9, 8, 4, 9, STRIDE=S, num_warps=WARPS)
        K._xor_kernel[g](scratch, 0, 9, 12, STRIDE=S, num_warps=WARPS)
        mul_scalar(8, 12, 13)
        K._xor3_kernel[g](scratch, 13, 9, 1, 13, STRIDE=S, num_warps=WARPS)

    K._unpack_kernel[g](x, y, scratch, N, STRIDE=S, num_warps=WARPS)
    step_scalar()
    out = torch.empty((N, 2, 3), dtype=torch.int64, device=dev)
    K._pack_kernel[g](scratch, out, STRIDE=S, num_warps=WARPS)
    rep["control_scalar_step_exact"] = bool(torch.equal(out, want))

    K._unpack_kernel[g](x, y, scratch, N, STRIDE=S, num_warps=WARPS)
    step()
    out = torch.empty((N, 2, 3), dtype=torch.int64, device=dev)
    K._pack_kernel[g](scratch, out, STRIDE=S, num_warps=WARPS)
    rep["step_exact"] = bool(torch.equal(out, want))

    ms = median_ms(lambda: (K._unpack_kernel[g](x, y, scratch, N, STRIDE=S, num_warps=WARPS), step()), reps=10)
    rep["batched_step_ms"] = ms
    rep["batched_updates_per_s"] = N / ms * 1e3
    print(json.dumps(rep))
    (pathlib.Path("/results") if pathlib.Path("/results").is_dir() else HERE).joinpath("clmad_step2.json").write_text(
        json.dumps(rep, indent=2))
    return 0 if rep["step_exact"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
