"""Sweep launch shapes for the whole step, not per kernel.

Per-kernel optima disagree: sigma likes 1 warp, the small xor/copy kernels are
worse at 1 warp because each then owns a 32-thread block. The step shares one
shape across sigma and the xors, so time the step directly over the combos.
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
import v9  # noqa: E402

N = 1 << 16


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
    torch.manual_seed(101)
    x = wsp._random_elements(N).cuda()
    y = wsp._random_elements(N).cuda()
    want = K.kernel_function(x, y)
    npad = ((N + v9.MUL_B - 1) // v9.MUL_B) * v9.MUL_B
    ws = v9._arena(npad, x.device)
    rep = {"gpu": torch.cuda.get_device_name(), "rows": N, "variants": []}

    combos = [(8, 1, 8, 1), (16, 2, 8, 2), (16, 1, 8, 1), (8, 1, 8, 2),
              (16, 2, 16, 1), (8, 2, 8, 1), (16, 1, 16, 1)]
    best = None
    for sr, sw, fr, fw in combos:
        v9.SIGMA_R, v9.SIGMA_W, v9.FROB_R, v9.FROB_W = sr, sw, fr, fw
        try:
            got = v9.kernel_function(x, y, ws=ws)
            ok = bool(torch.equal(got, want))
            ms = median_ms(lambda: v9.kernel_function(x, y, ws=ws))
            rec = {"sigma": [sr, sw], "frob": [fr, fw], "exact": ok,
                   "ms": ms, "updates_per_s": N / ms * 1e3}
        except Exception as exc:
            rec = {"sigma": [sr, sw], "frob": [fr, fw], "error": str(exc)[:100]}
        print(json.dumps(rec), flush=True)
        rep["variants"].append(rec)
        if rec.get("exact") and (best is None or rec["ms"] < best["ms"]):
            best = rec
    rep["best"] = best
    print(json.dumps({"best": best}))
    (pathlib.Path("/results") if pathlib.Path("/results").is_dir() else HERE).joinpath("step_sweep.json").write_text(
        json.dumps(rep, indent=2))
    return 0 if best else 1


if __name__ == "__main__":
    raise SystemExit(main())
