"""Tune the v8 step: sweep sigma tile/warps and the clmad multiply block/warps.

Sigma is 83% of the step and runs ~9x off its ALU bound, which says it is
occupancy or reduction bound rather than arithmetic bound. Both of its knobs
and the multiply's are free to sweep, so sweep them, then rebuild the step
with the winners and re-time it.

    python3 tune.py
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

N = 1 << 16
LSTRIDE = N * 3
WARPS = 4


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
    torch.manual_seed(67)
    dev = "cuda"
    rep = {"gpu": torch.cuda.get_device_name(), "rows": N}
    table = K.get_table(dev)
    limbs = torch.zeros((14 * LSTRIDE,), dtype=torch.int64, device=dev)
    limbs.copy_(torch.randint(-(1 << 62), 1 << 62, (14 * LSTRIDE,), device=dev))
    out = torch.empty((N, 2, 3), dtype=torch.int64, device=dev)

    sig = {}
    for R in (4, 8, 16, 32):
        for w in (2, 4, 8):
            g = (N // R + 1,)
            try:
                ms = median_ms(lambda _r=R, _w=w, _g=g: v8._sigma_limbs[_g](
                    limbs, table, 5, 6, 1, N, LSTRIDE, R=_r, num_warps=_w))
            except Exception as exc:
                sig["R%d_w%d" % (R, w)] = "err: %s" % str(exc)[:40]
                continue
            sig["R%d_w%d" % (R, w)] = ms
    rep["sigma"] = sig

    fr = {}
    for R in (4, 8, 16):
        for w in (2, 4, 8):
            g = (N // R + 1,)
            try:
                ms = median_ms(lambda _r=R, _w=w, _g=g: v8._frob_limbs[_g](
                    limbs, table, N, LSTRIDE, R=_r, num_warps=_w))
            except Exception:
                continue
            fr["R%d_w%d" % (R, w)] = ms
    rep["frob"] = fr

    mul = {}
    for B in (64, 128, 256):
        for w in (2, 4, 8):
            g = (N // B,)
            try:
                ms = median_ms(lambda _b=B, _w=w, _g=g: CM._mul_clmad_kernel[_g](
                    limbs, limbs[LSTRIDE:], limbs[2 * LSTRIDE:], BLOCK=_b, num_warps=_w))
            except Exception:
                continue
            mul["B%d_w%d" % (B, w)] = ms
    rep["mul"] = mul

    def best(d):
        ok = {k: v for k, v in d.items() if isinstance(v, float)}
        return min(ok, key=ok.get) if ok else None

    bs, bf, bm = best(sig), best(fr), best(mul)
    rep["best_sigma"], rep["best_frob"], rep["best_mul"] = bs, bf, bm
    if not (bs and bf and bm):
        print(json.dumps(rep))
        return 1

    sR = int(bs.split("_")[0][1:])
    sw = int(bs.split("_")[1][1:])
    fR = int(bf.split("_")[0][1:])
    fw = int(bf.split("_")[1][1:])
    mB = int(bm.split("_")[0][1:])
    mw = int(bm.split("_")[1][1:])

    gs = (N // sR + 1,)
    gf = (N // fR + 1,)
    gm = (N // mB,)

    def load_xy():
        limbs[0 * LSTRIDE:1 * LSTRIDE].view(N, 3).copy_(x)
        limbs[1 * LSTRIDE:2 * LSTRIDE].view(N, 3).copy_(y)

    def mul(sa, sb, dst):
        CM._mul_clmad_kernel[gm](limbs[sa * LSTRIDE:], limbs[sb * LSTRIDE:],
                                 limbs[dst * LSTRIDE:], BLOCK=mB, num_warps=mw)

    def step():
        v8._frob_limbs[gf](limbs, table, N, LSTRIDE, R=fR, num_warps=fw)
        v8._xor_limbs[gs](limbs, 0, 2, 4, N, LSTRIDE, R=sR, num_warps=sw)
        v8._copy_limbs[gs](limbs, 4, 5, N, LSTRIDE, R=sR, num_warps=sw)
        acc, tmp = 5, 7
        for k in (1, 2, 4, 8, 16, 32):
            v8._sigma_limbs[gs](limbs, table, acc, 6, k, N, LSTRIDE, R=sR, num_warps=sw)
            mul(6, acc, tmp)
            acc, tmp = tmp, acc
        v8._sigma_limbs[gs](limbs, table, acc, 6, 1, N, LSTRIDE, R=sR, num_warps=sw)
        mul(6, 4, tmp)
        acc, tmp = tmp, acc
        v8._sigma_limbs[gs](limbs, table, acc, 6, 65, N, LSTRIDE, R=sR, num_warps=sw)
        mul(6, acc, tmp)
        acc, tmp = tmp, acc
        v8._sigma_limbs[gs](limbs, table, acc, 6, 1, N, LSTRIDE, R=sR, num_warps=sw)
        v8._xor_limbs[gs](limbs, 1, 3, 12, N, LSTRIDE, R=sR, num_warps=sw)
        mul(12, 6, 8)
        v8._sigma_limbs[gs](limbs, table, 8, 9, 1, N, LSTRIDE, R=sR, num_warps=sw)
        v8._xor3_limbs[gs](limbs, 9, 8, 4, 9, N, LSTRIDE, R=sR, num_warps=sw)
        v8._xor_limbs[gs](limbs, 0, 9, 12, N, LSTRIDE, R=sR, num_warps=sw)
        mul(8, 12, 13)
        v8._xor3_limbs[gs](limbs, 13, 9, 1, 13, N, LSTRIDE, R=sR, num_warps=sw)

    global x, y
    x = wsp._random_elements(N).cuda()
    y = wsp._random_elements(N).cuda()
    want = K.kernel_function(x, y)
    load_xy()
    step()
    out[:, 0] = limbs[9 * LSTRIDE:9 * LSTRIDE + LSTRIDE].view(N, 3)
    out[:, 1] = limbs[13 * LSTRIDE:13 * LSTRIDE + LSTRIDE].view(N, 3)
    rep["step_exact"] = bool(torch.equal(out, want))
    ms = median_ms(lambda: (load_xy(), step()), reps=10)
    rep["tuned_step_ms"] = ms
    rep["tuned_updates_per_s"] = N / ms * 1e3
    rep["gain_vs_v8"] = 0.829 / ms
    print(json.dumps(rep))
    (pathlib.Path("/results") if pathlib.Path("/results").is_dir() else HERE).joinpath("tune.json").write_text(
        json.dumps(rep, indent=2))
    return 0 if rep["step_exact"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
