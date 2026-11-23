"""Whole-step comparison on Blackwell: v5 convolution vs clmad product.

Same iteration, same scratch layout, same sigma; only the multiply changes
(one kernel vs pack+pack+clmad+unpack). Both are checked against each other
before any timing.
"""
import json
import pathlib
import statistics
import sys

import torch

HERE = pathlib.Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
import walk_step_problem as wsp  # noqa: E402
import kernel_v5 as K  # noqa: E402
import clmul_mul as CM  # noqa: E402

N = 1 << 16
R = 4
ITILE = 8
WARPS = 4


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


def make_step(scratch, table, npad, la, lb, lo):
    g = (npad,)
    gm = (npad // R,)
    S = K.STRIDE

    def mul(sa, sb, dst):
        CM._pack_bits_kernel[g](scratch, la, sa, npad, STRIDE=S, num_warps=WARPS)
        CM._pack_bits_kernel[g](scratch, lb, sb, npad, STRIDE=S, num_warps=WARPS)
        CM._mul_clmad_kernel[(npad // 128,)](la, lb, lo, BLOCK=128)
        CM._unpack_bits_kernel[g](lo, scratch, dst, npad, STRIDE=S, num_warps=WARPS)

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
    return step


def main():
    torch.manual_seed(17)
    dev = "cuda"
    rep = {"gpu": torch.cuda.get_device_name(), "rows": N}

    x = wsp._random_elements(N).cuda()
    y = wsp._random_elements(N).cuda()
    table = K.get_table(dev)

    # reference step
    want = K.kernel_function(x, y)

    # clmad step
    npad = N
    scratch = torch.empty((npad, K.STRIDE), dtype=torch.int8, device=dev)
    la = torch.empty((npad, 3), dtype=torch.int64, device=dev)
    lb = torch.empty((npad, 3), dtype=torch.int64, device=dev)
    lo = torch.empty((npad, 3), dtype=torch.int64, device=dev)
    K._unpack_kernel[(npad,)](x, y, scratch, N, STRIDE=K.STRIDE, num_warps=WARPS)
    step = make_step(scratch, table, npad, la, lb, lo)
    step()
    out = torch.empty((npad, 2, 3), dtype=torch.int64, device=dev)
    K._pack_kernel[(npad,)](scratch, out, STRIDE=K.STRIDE, num_warps=WARPS)
    rep["step_exact"] = bool(torch.equal(out, want))

    v5_ms = median_ms(lambda: K.kernel_function(x, y), reps=10)
    cl_ms = median_ms(lambda: (K._unpack_kernel[(npad,)](x, y, scratch, N, STRIDE=K.STRIDE, num_warps=WARPS), step()), reps=10)
    rep.update({"v5_step_ms": v5_ms, "clmad_step_ms": cl_ms,
                "speedup": v5_ms / cl_ms,
                "v5_updates_per_s": N / v5_ms * 1e3,
                "clmad_updates_per_s": N / cl_ms * 1e3})
    print(json.dumps(rep))
    (pathlib.Path("/results") if pathlib.Path("/results").is_dir() else HERE).joinpath("clmad_step.json").write_text(
        json.dumps(rep, indent=2))
    return 0 if rep["step_exact"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
