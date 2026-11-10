"""Chained rho-walk throughput for the v5 walk step.

    python3 walk.py            # in the GPU container

Each walker takes `steps` consecutive steps, so the input of step k+1 is the
output of step k (which is what a rho walker actually does; the step benchmark
feeds independent random inputs). Scratch is allocated once, outputs are
copied back into the input slots each step, and the whole chain is checked
against the PyTorch reference on a sample of walkers.
"""
import json
import statistics
import sys
import time
import pathlib

import torch

HERE = pathlib.Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
import walk_step_problem as wsp  # noqa: E402
import kernel_v5 as K  # noqa: E402

WALKERS = 1 << 16
STEPS = 64


def _step(scratch, table, g, gm, npad, r, itile, warps):
    kw = dict(num_warps=warps)
    S = K.STRIDE
    K._frob_kernel[g](scratch, table, STRIDE=S, **kw)
    K._xor_kernel[g](scratch, 0, 2, 4, STRIDE=S, **kw)
    K._copy_kernel[g](scratch, 4, 5, STRIDE=S, **kw)
    acc, tmp = 5, 7
    for k in (1, 2, 4, 8, 16, 32):
        K._sigma_kernel[g](scratch, table, acc, 6, k, STRIDE=S, **kw)
        K._mul_kernel[gm](scratch, 6, acc, tmp, R=r, ITILE=itile, STRIDE=S, **kw)
        acc, tmp = tmp, acc
    K._sigma_kernel[g](scratch, table, acc, 6, 1, STRIDE=S, **kw)
    K._mul_kernel[gm](scratch, 6, 4, tmp, R=r, ITILE=itile, STRIDE=S, **kw)
    acc, tmp = tmp, acc
    K._sigma_kernel[g](scratch, table, acc, 6, 65, STRIDE=S, **kw)
    K._mul_kernel[gm](scratch, 6, acc, tmp, R=r, ITILE=itile, STRIDE=S, **kw)
    acc, tmp = tmp, acc
    K._sigma_kernel[g](scratch, table, acc, 6, 1, STRIDE=S, **kw)
    K._xor_kernel[g](scratch, 1, 3, 12, STRIDE=S, **kw)
    K._mul_kernel[gm](scratch, 12, 6, 8, R=r, ITILE=itile, STRIDE=S, **kw)
    K._sigma_kernel[g](scratch, table, 8, 9, 1, STRIDE=S, **kw)
    K._xor3_kernel[g](scratch, 9, 8, 4, 9, STRIDE=S, **kw)
    K._xor_kernel[g](scratch, 0, 9, 12, STRIDE=S, **kw)
    K._mul_kernel[gm](scratch, 8, 12, 13, R=r, ITILE=itile, STRIDE=S, **kw)
    K._xor3_kernel[g](scratch, 13, 9, 1, 13, STRIDE=S, **kw)
    # carry the result into the next step's input slots
    K._copy_kernel[g](scratch, 9, 0, STRIDE=S, **kw)
    K._copy_kernel[g](scratch, 13, 1, STRIDE=S, **kw)


def walk(x, y, steps, r=4, itile=8, warps=4):
    N = x.shape[0]
    npad = ((N + r - 1) // r) * r
    scratch = torch.empty((npad, K.STRIDE), dtype=torch.int8, device=x.device)
    table = K.get_table(x.device)
    g, gm, S = (npad,), (npad // r,), K.STRIDE
    K._unpack_kernel[g](x, y, scratch, N, STRIDE=S, num_warps=warps)
    for _ in range(steps):
        _step(scratch, table, g, gm, npad, r, itile, warps)
    out = torch.empty((npad, 2, 3), dtype=torch.int64, device=x.device)
    K._pack_kernel[g](scratch, out, STRIDE=S, num_warps=warps)
    return out[:N]


def main():
    torch.manual_seed(20261110)
    x = wsp._random_elements(WALKERS).cuda()
    y = wsp._random_elements(WALKERS).cuda()

    got = walk(x, y, STEPS)

    # chained correctness: run the reference forward STEPS times on a sample
    ref = wsp.Model().cuda()
    idx = torch.arange(8)
    rx = x[idx].clone()
    ry = y[idx].clone()
    for _ in range(STEPS):
        o = ref(rx, ry)
        rx, ry = o[:, 0].contiguous(), o[:, 1].contiguous()
    want = torch.stack([rx, ry], dim=1) if False else torch.cat(
        [rx.unsqueeze(1), ry.unsqueeze(1)], dim=1)
    ok = torch.equal(got[idx], want)

    # timed chain, repeated
    times = []
    for _ in range(3):
        torch.cuda.synchronize()
        t0 = time.perf_counter()
        walk(x, y, STEPS)
        torch.cuda.synchronize()
        times.append(time.perf_counter() - t0)
    dt = statistics.median(times)
    steps = WALKERS * STEPS
    rec = {"gpu": torch.cuda.get_device_name(), "walkers": WALKERS, "steps_per_walker": STEPS,
           "chain_exact": bool(ok), "seconds": dt,
           "updates_per_s": steps / dt,
           "ns_per_update": dt / steps * 1e9}
    print(json.dumps(rec))
    (pathlib.Path("/results") if pathlib.Path("/results").is_dir() else HERE / "walk.json").write_text(
        json.dumps(rec, indent=2))
    return 0 if ok else 1


if __name__ == "__main__":
    raise SystemExit(main())
