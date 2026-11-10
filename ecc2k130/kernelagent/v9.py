"""v9: the v8 step with its launch shapes tuned, as a drop-in kernel_function.

Sigma and frob were 9x off their ALU bound because the tiles are small and
4-8 warps leave most of them idle. Sweeping the shapes (tune.py):

    sigma   R16, 2 warps   0.0386 ms   (was R8, 4 warps  0.0682)
    frob    R8,  2 warps   0.0725 ms   (was R8, 4 warps  0.1267)
    mul     B128, 4 warps  0.0163 ms

which takes the step from 0.829 ms to 0.435 ms: 150.5 M updates/s on one
RTX PRO 6000, bit-exact.

    python3 v9.py     # verify against the reference and time it
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
import v8  # noqa: E402

NSLOT = 14
SIGMA_R, SIGMA_W = 16, 2
FROB_R, FROB_W = 8, 2
MUL_B, MUL_W = 128, 4


def kernel_function(x, y):
    """One ECC2K-130 walk step for N points: (N,3) int64 in, (N,2,3) out."""
    assert x.is_cuda and y.is_cuda
    assert x.dtype == torch.int64 and y.dtype == torch.int64
    assert x.shape == y.shape and x.dim() == 2 and x.shape[1] == 3
    n = x.shape[0]
    npad = ((n + MUL_B - 1) // MUL_B) * MUL_B
    lstride = npad * 3
    limbs = torch.zeros((NSLOT * lstride,), dtype=torch.int64, device=x.device)
    limbs[0:lstride].view(npad, 3)[:n].copy_(x)
    limbs[lstride:2 * lstride].view(npad, 3)[:n].copy_(y)

    table = K.get_table(x.device)
    gs = (npad // SIGMA_R + 1,)
    gf = (npad // FROB_R + 1,)
    gm = (npad // MUL_B,)

    def mul(sa, sb, dst):
        CM._mul_clmad_kernel[gm](limbs[sa * lstride:], limbs[sb * lstride:],
                                 limbs[dst * lstride:], BLOCK=MUL_B, num_warps=MUL_W)

    v8._frob_limbs[gf](limbs, table, n, lstride, R=FROB_R, num_warps=FROB_W)
    v8._xor_limbs[gs](limbs, 0, 2, 4, n, lstride, R=SIGMA_R, num_warps=SIGMA_W)
    v8._copy_limbs[gs](limbs, 4, 5, n, lstride, R=SIGMA_R, num_warps=SIGMA_W)
    acc, tmp = 5, 7
    for k in (1, 2, 4, 8, 16, 32):
        v8._sigma_limbs[gs](limbs, table, acc, 6, k, n, lstride, R=SIGMA_R, num_warps=SIGMA_W)
        mul(6, acc, tmp)
        acc, tmp = tmp, acc
    v8._sigma_limbs[gs](limbs, table, acc, 6, 1, n, lstride, R=SIGMA_R, num_warps=SIGMA_W)
    mul(6, 4, tmp)
    acc, tmp = tmp, acc
    v8._sigma_limbs[gs](limbs, table, acc, 6, 65, n, lstride, R=SIGMA_R, num_warps=SIGMA_W)
    mul(6, acc, tmp)
    acc, tmp = tmp, acc
    v8._sigma_limbs[gs](limbs, table, acc, 6, 1, n, lstride, R=SIGMA_R, num_warps=SIGMA_W)
    v8._xor_limbs[gs](limbs, 1, 3, 12, n, lstride, R=SIGMA_R, num_warps=SIGMA_W)
    mul(12, 6, 8)
    v8._sigma_limbs[gs](limbs, table, 8, 9, 1, n, lstride, R=SIGMA_R, num_warps=SIGMA_W)
    v8._xor3_limbs[gs](limbs, 9, 8, 4, 9, n, lstride, R=SIGMA_R, num_warps=SIGMA_W)
    v8._xor_limbs[gs](limbs, 0, 9, 12, n, lstride, R=SIGMA_R, num_warps=SIGMA_W)
    mul(8, 12, 13)
    v8._xor3_limbs[gs](limbs, 13, 9, 1, 13, n, lstride, R=SIGMA_R, num_warps=SIGMA_W)

    out = torch.empty((npad, 2, 3), dtype=torch.int64, device=x.device)
    out[:, 0] = limbs[9 * lstride:9 * lstride + lstride].view(npad, 3)
    out[:, 1] = limbs[13 * lstride:13 * lstride + lstride].view(npad, 3)
    return out[:n]


def median_ms(fn, reps=15):
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
    torch.manual_seed(71)
    n = 1 << 16
    x = wsp._random_elements(n).cuda()
    y = wsp._random_elements(n).cuda()
    rep = {"gpu": torch.cuda.get_device_name(), "rows": n}

    want = K.kernel_function(x, y)
    got = kernel_function(x, y)
    rep["exact"] = bool(torch.equal(got, want))

    ms = median_ms(lambda: kernel_function(x, y), reps=20)
    rep["step_ms"] = ms
    rep["updates_per_s"] = n / ms * 1e3

    # odd batch size, to prove the padding path
    m = 40_001
    xs, ys = x[:m].contiguous(), y[:m].contiguous()
    rep["exact_odd_batch"] = bool(torch.equal(kernel_function(xs, ys), K.kernel_function(xs, ys)))

    print(json.dumps(rep))
    (pathlib.Path("/results") if pathlib.Path("/results").is_dir() else HERE).joinpath("v9.json").write_text(
        json.dumps(rep, indent=2))
    return 0 if (rep["exact"] and rep["exact_odd_batch"]) else 1


if __name__ == "__main__":
    raise SystemExit(main())
