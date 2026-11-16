"""Sweep kernel_v2 launch parameters on a GPU and compare against the v1 kernel.

    python3 sweep.py            # in the GPU container, cwd anywhere

Prints one JSON line per (batch, variant) with an exactness flag and the median
step time, then a summary. Nothing here is used by the harness; it is the
measurement that decides what to bake into kernel_v2's defaults.
"""
import importlib.util
import json
import pathlib
import statistics
import sys

import torch

HERE = pathlib.Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
import walk_step_problem as wsp  # noqa: E402


def load(path, name):
    spec = importlib.util.spec_from_file_location(name, path)
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


def time_ms(fn, x, y, reps):
    ev0, ev1 = torch.cuda.Event(True), torch.cuda.Event(True)
    ts = []
    for _ in range(reps):
        ev0.record()
        fn(x, y)
        ev1.record()
        torch.cuda.synchronize()
        ts.append(ev0.elapsed_time(ev1))
    return statistics.median(ts)


def main():
    v1 = load(HERE / "kernel_walk_step.generated.py", "k1")
    v2 = load(HERE / "kernel_v2.py", "k2")
    v3 = load(HERE / "kernel_v3.py", "k3")
    v4 = load(HERE / "kernel_v4.py", "k4")
    v5 = load(HERE / "kernel_v5.py", "k5")
    ref = wsp.Model().cuda()
    report = {"gpu": torch.cuda.get_device_name(), "torch": torch.__version__, "rows": []}

    for n in (1 << 16, 1 << 18):
        g = torch.Generator().manual_seed(20261109)
        x = wsp._random_elements(n, g).cuda()
        y = wsp._random_elements(n, g).cuda()
        want = ref(x, y)

        ref_ms = time_ms(ref, x, y, 5)
        row = {"batch": n, "reference_ms": ref_ms, "variants": []}
        print(json.dumps({"batch": n, "reference_ms": ref_ms}))

        variants = [("v1", v1.kernel_function, {}),
                    ("v3-it16-w4", lambda a, b: v3.kernel_function(a, b, itile=16, warps=4), {})]
        for rows, itile in ((4, 8), (4, 16), (8, 8)):
            for warps in (4, 8):
                variants.append(("v5-r%d-it%d-w%d" % (rows, itile, warps),
                                 lambda a, b, _r=rows, _i=itile, _w=warps:
                                 v5.kernel_function(a, b, r=_r, itile=_i, warps=_w),
                                 {}))
        for name, fn, _ in variants:
            try:
                got = fn(x, y)
                exact = bool(isinstance(got, torch.Tensor) and got.shape == want.shape
                             and torch.equal(got, want))
                for _ in range(3):
                    fn(x, y)
                ms = time_ms(fn, x, y, 20)
                rec = {"name": name, "exact": exact, "kernel_ms": ms,
                       "steps_per_s": n / ms * 1e3,
                       "speedup_vs_ref": ref_ms / ms}
            except Exception as exc:
                rec = {"name": name, "error": "%s: %s" % (type(exc).__name__, str(exc)[:200])}
            row["variants"].append(rec)
            print(json.dumps(rec), flush=True)
        report["rows"].append(row)

    best = {}
    for row in report["rows"]:
        ok = [v for v in row["variants"] if v.get("exact")]
        if ok:
            best[row["batch"]] = min(ok, key=lambda v: v["kernel_ms"])["name"]
    report["best"] = best
    print(json.dumps({"best": best}))
    (pathlib.Path("/results") if pathlib.Path("/results").is_dir() else HERE / "sweep.json").write_text(
        json.dumps(report, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
