#!/usr/bin/env python3
"""Performance history: one snapshot per commit, a trend chart, a PR check.

    perfhistory.py record --binary perfbench --ecbench ecbench --out docs/perf/history
    perfhistory.py record --ref REV --out DIR            # build REV (perfbench + ecbench) and record it
    perfhistory.py render --history DIR --md HISTORY.md --svg history.svg --json history.json
    perfhistory.py check  --binary perfbench --ecbench ecbench --history DIR [--threshold 0.10]

`perfindex.py` answers "is B faster than A?" for one pair of revisions.
This script answers "how has each kernel moved over the commits we
measured?": every snapshot records, for each `perfbench` kernel and for
each frozen `ecbench exec` child in `docs/perf/ecbench-children.json`,

  * the callgrind instruction count of the measured region (`Ir`):
    deterministic, host-independent up to the CPU features the code
    detects at run time, the number the chart is drawn from;
  * the wall time of one native run (median of a few samples): noisy on a
    shared CI host, recorded beside `Ir`, never charted as progress;
  * the kernel's fingerprint: when it changes, the kernel computed
    something else, the step is not a speedup, and the chained index
    takes no credit for it ("re-based").

The chained index of a kernel at snapshot t is the product over
consecutive snapshot pairs of Ir(previous) / Ir(current), with factor 1
where the fingerprint changed or the kernel was absent.  Areas are the
geometric mean of their kernels' indices, the overall index the weighted
geometric mean of areas (`docs/perf/weights.json`), exactly the formula of
`docs/perf/PERFORMANCE_INDEX.md` applied to Ir along the history instead
of across one pair.  Only the Python standard library is used.
"""

from __future__ import annotations

import argparse
import datetime as dt
import json
import math
import os
import shutil
import statistics
import subprocess
import sys
import tempfile
import time
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import perfindex  # noqa: E402  (same directory)

SCHEMA = "perf.history/v1"
ECBENCH_AREA = "ecbench"
MOVER = 0.05  # a single-step change below this is within build noise
PALETTE = {
    "index": "#111111",
    "gf2_la": "#4e79a7",
    "bool_gb": "#f28e2b",
    "fp_gb": "#e15759",
    "sat": "#76b7b2",
    "pdp": "#59a14f",
    "field_ec": "#edc948",
    "relation": "#b07aa1",
    "dlp": "#ff9da7",
    ECBENCH_AREA: "#9c755f",
    "ic_factor_base": "#6b6ecf",
}
# Factor-base symmetry classes (docs/ecbench/FACTOR_BASE_PROTOCOL.md): which
# automorphism the base is closed under and the oracle folds by.  Colours are
# fixed per class so every chart reads the same.
SYMMETRY_COLORS = {
    "none": "#9a9a9a",
    "negation": "#e15759",
    "signed Frobenius": "#4e79a7",
    "GLV (order 6)": "#59a14f",
}
# A kernel is flagged when its Ir rises more than REGRESSION_THRESHOLD above
# the highest value it showed over the previous REGRESSION_WINDOW snapshots
# with the same fingerprint (and symmetrically for improvements against the
# lowest).  Several kernels flip between two codegen levels from build to
# build (ct_scalar_mul_* by about 7 %); comparing against the recent range
# instead of the last value ignores that flapping.  Replayed over the 74
# steps recorded by 2026-10-17 this fires on 3 steps, against 38 for a plain
# 5 % step threshold.
REGRESSION_THRESHOLD = 0.08
REGRESSION_WINDOW = 5
SUPERSCRIPT = str.maketrans("-0123456789", "⁻⁰¹²³⁴⁵⁶⁷⁸⁹")


# ----------------------------------------------------------------------------
# git


def git(root: Path, *args: str) -> str:
    return subprocess.run(
        ["git", "-C", str(root), *args], capture_output=True, text=True, check=True
    ).stdout.strip()


def rev_facts(root: Path, rev: str) -> dict:
    sha = git(root, "rev-parse", rev)
    parents = git(root, "rev-list", "--parents", "-n", "1", sha).split()[1:]
    committed = git(root, "show", "-s", "--format=%cI", sha)
    subject = git(root, "show", "-s", "--format=%s", sha)
    return {
        "rev": sha,
        "rev_short": sha[:12],
        "parents": parents,
        "committed_at": committed,
        "subject": subject,
    }


# ----------------------------------------------------------------------------
# measuring


def wall_one(binary: str, kid: str, samples: int, max_seconds: float) -> dict:
    env = dict(os.environ, RAYON_NUM_THREADS="1")
    cmd = [
        binary, "run", "--filter", kid, "--exact",
        "--samples", str(samples), "--max-seconds", str(max_seconds),
    ]
    (row,) = perfindex.run_json(cmd, env)
    return row


def measure_perfbench(binary: str, filters: list[str], full: bool, wall_samples: int,
                      max_seconds: float, with_wall: bool, log) -> list[dict]:
    rows = []
    kernels = perfindex.list_kernels(binary, full, filters)
    for i, k in enumerate(kernels, 1):
        t0 = time.time()
        ir_row = perfindex.instr_one(binary, k["id"])
        row = {
            "id": k["id"],
            "area": k["area"],
            "tier": k.get("tier", "Quick"),
            "fingerprint": ir_row["fingerprint"],
            "ir": ir_row["ir"],
            "wall_ns": None,
        }
        if with_wall:
            w = wall_one(binary, k["id"], wall_samples, max_seconds)
            if w["fingerprint"] != row["fingerprint"]:
                raise RuntimeError(f"{k['id']}: fingerprint differs between callgrind and native runs")
            row["wall_ns"] = w["median_ns"]
        rows.append(row)
        log(f"[{i}/{len(kernels)}] {k['id']}: Ir={row['ir']} wall={_ms(row['wall_ns'])} ({time.time() - t0:.1f}s)")
    return rows


def ecbench_child_once(ecbench: str, child: dict, workdir: Path, callgrind: bool) -> tuple[dict, dict | None]:
    """Run one measured child; return (child_output, callgrind_ir or None)."""
    inp = json.dumps(child["input"])
    env = dict(os.environ)
    if callgrind:
        env["ECBENCH_CALLGRIND_SOLVE"] = "1"
        prefix = workdir / "cg.out"
        for f in workdir.glob("cg.out*"):
            f.unlink()
        cmd = ["valgrind", "--tool=callgrind", f"--callgrind-out-file={prefix}", ecbench, "exec"]
    else:
        cmd = [ecbench, "exec"]
    proc = subprocess.run(cmd, input=inp, capture_output=True, text=True, env=env)
    if proc.returncode != 0:
        raise RuntimeError(f"{child['id']}: ecbench exec exited {proc.returncode}:\n{proc.stderr[-3000:]}")
    out = json.loads(proc.stdout.strip().splitlines()[-1])
    if out.get("error"):
        raise RuntimeError(f"{child['id']}: child error: {out['error']}")
    ir = None
    if callgrind:
        p = subprocess.run(
            [ecbench, "callgrind-ir", "--prefix", str(prefix)], capture_output=True, text=True, check=True
        )
        ir = json.loads(p.stdout)
    return out, ir


def child_fingerprint(out: dict) -> str:
    """What the child computed: the recovered scalar and every counted phase."""
    rep = out["report"]
    phases = [
        {k: v for k, v in sorted(ph.items()) if not k.endswith("_ns") and k != "wall_ns"}
        for ph in rep.get("phases", [])
    ]
    blob = json.dumps({"recovered": rep.get("recovered"), "phases": phases}, sort_keys=True)
    import hashlib

    return hashlib.sha256(blob.encode()).hexdigest()[:16]


def child_facts(child: dict, report: dict) -> dict:
    """What a chart needs to say about a child beyond its Ir: the curve it
    ran on, the counted group operations, and for index calculus the factor
    base as the run actually built it (size, symmetry class, identity)."""
    facts: dict = {}
    if child.get("curve_label"):
        facts["curve_label"] = child["curve_label"]
    ops = 0
    for ph in report.get("phases", []):
        ops += int(ph.get("adds", 0) or 0) + int(ph.get("doubles", 0) or 0)
    facts["group_ops"] = ops
    declared = child.get("factor_base")
    if declared:
        fb = report.get("factor_base") or {}
        facts["factor_base"] = {
            "spec": declared["spec"],
            "symmetry": declared["symmetry"],
            "family": fb.get("family"),
            "columns": fb.get("columns"),
            "signed_points": fb.get("signed_points"),
            "abscissae": fb.get("abscissae"),
            "fb_id": fb.get("fb_id"),
        }
    return facts


def measure_ecbench(ecbench: str, children_path: Path, wall_samples: int, with_wall: bool, log) -> list[dict]:
    doc = json.loads(children_path.read_text())
    rows = []
    with tempfile.TemporaryDirectory() as td:
        work = Path(td)
        for child in doc["children"]:
            t0 = time.time()
            out, ir = ecbench_child_once(ecbench, child, work, callgrind=True)
            rec = out["report"].get("recovered")
            if rec != child["expected_recovered"]:
                raise RuntimeError(f"{child['id']}: recovered {rec}, expected {child['expected_recovered']}")
            row = {
                "id": child["id"],
                "area": child.get("area", ECBENCH_AREA),
                "tier": "Quick",
                "fingerprint": child_fingerprint(out),
                "ir": ir["solve_ir"],
                "prework_ir": ir["prework_ir"],
                "wall_ns": None,
            }
            row.update(child_facts(child, out["report"]))
            if with_wall:
                times = []
                for _ in range(max(1, wall_samples)):
                    t = time.perf_counter()
                    o2, _ = ecbench_child_once(ecbench, child, work, callgrind=False)
                    times.append(int((time.perf_counter() - t) * 1e9))
                    if child_fingerprint(o2) != row["fingerprint"]:
                        raise RuntimeError(f"{child['id']}: counted output differs between runs")
                row["wall_ns"] = int(statistics.median(times))
            rows.append(row)
            log(f"{child['id']}: solve Ir={row['ir']} wall={_ms(row['wall_ns'])} ({time.time() - t0:.1f}s)")
    return rows


def _ms(ns):
    return "n/a" if ns is None else f"{ns / 1e6:.3f}ms"


def valgrind_version() -> str:
    try:
        return subprocess.run(["valgrind", "--version"], capture_output=True, text=True).stdout.strip()
    except OSError:
        return ""


# ----------------------------------------------------------------------------
# record


def build_ref(root: Path, ref: str, keep_dir: Path, log) -> tuple[str, str, Path]:
    """Build perfbench (this tree's harness) and ecbench at REF in a sparse worktree."""
    crate = perfindex.crate_dir(root)
    src = keep_dir / "src"
    target = keep_dir / "target"
    subprocess.run(["git", "-C", str(root), "worktree", "add", "--no-checkout", "--detach", str(src), ref], check=True)
    dirs = perfindex.sparse_dirs(crate)
    if dirs:
        subprocess.run(["git", "-C", str(src), "sparse-checkout", "set", *dirs, "research/ecbench_calibration_20261002"], check=True)
    subprocess.run(["git", "-C", str(src), "checkout", "--detach", ref], check=True)
    harness_rel = Path(crate) / "examples" / perfindex.EXAMPLE
    dst = src / harness_rel
    if dst.exists():
        shutil.rmtree(dst)
    shutil.copytree(root / harness_rel, dst)
    env = dict(os.environ, CARGO_TARGET_DIR=str(target))
    t0 = time.time()
    subprocess.run(
        ["cargo", "build", "--release", "--example", perfindex.EXAMPLE, "--bin", "ecbench"],
        cwd=src / crate, env=env, check=True,
    )
    log(f"built {ref} in {time.time() - t0:.0f}s")
    return str(target / "release" / "examples" / perfindex.EXAMPLE), str(target / "release" / "ecbench"), src


def snapshot_name(facts: dict) -> str:
    t = dt.datetime.fromisoformat(facts["committed_at"]).astimezone(dt.timezone.utc)
    return f"{t.strftime('%Y%m%dT%H%M%SZ')}-{facts['rev_short']}.json"


def cmd_record(a: argparse.Namespace) -> None:
    root = perfindex.repo_root()
    log = lambda s: print(s, file=sys.stderr, flush=True)  # noqa: E731
    out_dir = Path(a.out)
    out_dir.mkdir(parents=True, exist_ok=True)
    tmp_build = None
    try:
        if a.ref:
            tmp_build = Path(tempfile.mkdtemp(prefix="perfhistory-build-"))
            perfbench, ecbench, src = build_ref(root, a.ref, tmp_build, log)
            facts = rev_facts(root, a.ref)
            harness = perfindex.sha256_tree(root / perfindex.crate_dir(root) / "examples" / perfindex.EXAMPLE)
        else:
            if not a.binary:
                raise SystemExit("--binary is required without --ref")
            perfbench, ecbench = a.binary, a.ecbench
            facts = rev_facts(root, a.rev or "HEAD")
            harness = perfindex.sha256_tree(root / perfindex.crate_dir(root) / "examples" / perfindex.EXAMPLE)
        host = perfindex.host_manifest()
        kernels = measure_perfbench(perfbench, a.filter, a.full, a.wall_samples, a.max_seconds, not a.no_wall, log)
        if ecbench and a.children and Path(a.children).exists():
            kernels += measure_ecbench(ecbench, Path(a.children), a.wall_samples, not a.no_wall, log)
        snap = {
            "schema": SCHEMA,
            **facts,
            "recorded_at": dt.datetime.now(dt.timezone.utc).isoformat(timespec="seconds"),
            "note": a.note or "",
            "harness_sha256": harness,
            "valgrind": valgrind_version(),
            "host": {k: v for k, v in host.items() if not k.startswith("loadavg")},
            "wall_samples": 0 if a.no_wall else a.wall_samples,
            "kernels": kernels,
        }
        path = out_dir / snapshot_name(facts)
        path.write_text(json.dumps(snap, indent=1) + "\n")
        log(f"wrote {path} ({len(kernels)} kernels)")
        print(path)
    finally:
        if tmp_build is not None and not a.keep:
            subprocess.run(["git", "-C", str(root), "worktree", "remove", "--force", str(tmp_build / "src")],
                           capture_output=True)
            shutil.rmtree(tmp_build, ignore_errors=True)


# ----------------------------------------------------------------------------
# the chained index


def load_history(history: Path) -> list[dict]:
    snaps = [json.loads(p.read_text()) for p in sorted(history.glob("*.json"))]
    snaps = [s for s in snaps if s.get("schema") == SCHEMA]
    snaps.sort(key=lambda s: (s["committed_at"], s["recorded_at"]))
    return snaps


def chained(snaps: list[dict], weights: dict) -> dict:
    """Per-kernel chained indices, area indices and the overall index at every snapshot."""
    ids: dict[str, str] = {}  # id -> area
    for s in snaps:
        for k in s["kernels"]:
            ids[k["id"]] = k["area"]
    per = {kid: [] for kid in ids}  # list of {ir, wall_ns, fingerprint, index, step, rebased}
    for t, s in enumerate(snaps):
        rows = {k["id"]: k for k in s["kernels"]}
        for kid in ids:
            prev = per[kid][-1] if per[kid] else None
            k = rows.get(kid)
            if k is None:
                per[kid].append({"present": False, "index": prev["index"] if prev else 1.0, "step": None,
                                 "rebased": False, "ir": None, "wall_ns": None, "fingerprint": None})
                continue
            step, rebased = None, False
            index = prev["index"] if prev else 1.0
            if prev and prev["present"]:
                if prev["fingerprint"] == k["fingerprint"] and prev["ir"] and k["ir"]:
                    step = prev["ir"] / k["ir"]
                    index *= step
                elif prev["fingerprint"] != k["fingerprint"]:
                    rebased = True
            per[kid].append({"present": True, "index": index, "step": step, "rebased": rebased,
                             "ir": k["ir"], "wall_ns": k.get("wall_ns"), "fingerprint": k["fingerprint"]})
    areas = sorted({a for a in ids.values()})
    area_index = {a: [] for a in areas}
    overall = []
    for t in range(len(snaps)):
        logs_by_area = {a: [] for a in areas}
        for kid, area in ids.items():
            logs_by_area[area].append(math.log(per[kid][t]["index"]))
        for a in areas:
            area_index[a].append(math.exp(statistics.fmean(logs_by_area[a])) if logs_by_area[a] else 1.0)
        present = [a for a in areas if logs_by_area[a]]
        w = {a: float(weights.get(a, 1.0)) for a in present}
        tot = sum(w.values()) or 1.0
        overall.append(math.exp(sum(w[a] / tot * math.log(area_index[a][t]) for a in present)))
    return {"ids": ids, "per": per, "areas": areas, "area_index": area_index, "overall": overall}


# ----------------------------------------------------------------------------
# render


def fmt_x(x: float) -> str:
    return f"{x:.3f}×"


def svg_chart(snaps: list[dict], ch: dict, title: str) -> str:
    n = len(snaps)
    W, H = 960, 420
    ml, mr, mt, mb = 70, 230, 40, 70
    pw, ph = W - ml - mr, H - mt - mb
    series = [("index", ch["overall"])] + [(a, ch["area_index"][a]) for a in ch["areas"]]
    vals = [math.log2(v) for _, ys in series for v in ys if v > 0]
    lo, hi = (min(vals), max(vals)) if vals else (0.0, 0.0)
    lo, hi = min(lo, 0.0), max(hi, 0.0)
    pad = max(0.25, (hi - lo) * 0.08)
    lo, hi = lo - pad, hi + pad

    def X(i):
        return ml + (pw * i / (n - 1) if n > 1 else pw / 2)

    def Y(v):
        return mt + ph * (1 - (math.log2(v) - lo) / (hi - lo))

    out = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" viewBox="0 0 {W} {H}" font-family="ui-sans-serif, system-ui, sans-serif" font-size="12">',
           f'<rect width="{W}" height="{H}" fill="#ffffff"/>',
           f'<text x="{ml}" y="22" font-size="15" font-weight="600" fill="#111">{title}</text>',
           f'<text x="{ml}" y="{mt - 6}" fill="#555">chained Ir index relative to the first snapshot (log scale; above 1 = fewer instructions). wall time is not charted.</text>']
    # gridlines at powers of two
    k0, k1 = math.floor(lo), math.ceil(hi)
    for k in range(k0, k1 + 1):
        y = mt + ph * (1 - (k - lo) / (hi - lo))
        if mt <= y <= mt + ph:
            out.append(f'<line x1="{ml}" x2="{ml + pw}" y1="{y:.1f}" y2="{y:.1f}" stroke="#e5e5e5"/>')
            label = f"{2 ** k:g}×" if k >= 0 else f"1/{2 ** (-k):g}×"
            out.append(f'<text x="{ml - 8}" y="{y + 4:.1f}" text-anchor="end" fill="#555">{label}</text>')
    out.append(f'<line x1="{ml}" x2="{ml + pw}" y1="{Y(1.0):.1f}" y2="{Y(1.0):.1f}" stroke="#999" stroke-dasharray="4 3"/>')
    # x labels
    step = max(1, math.ceil(n / 8))
    for i, s in enumerate(snaps):
        # the last snapshot is always labelled; a regular label too close to it is dropped
        if i == n - 1 or (i % step == 0 and n - 1 - i >= 0.8 * step):
            x = X(i)
            out.append(f'<line x1="{x:.1f}" x2="{x:.1f}" y1="{mt + ph}" y2="{mt + ph + 5}" stroke="#555"/>')
            date = s["committed_at"][:10]
            out.append(f'<text x="{x:.1f}" y="{mt + ph + 18}" text-anchor="middle" fill="#555">{date}</text>')
            out.append(f'<text x="{x:.1f}" y="{mt + ph + 32}" text-anchor="middle" fill="#888" font-size="10">{s["rev_short"][:8]}</text>')
    # series
    for name, ys in series:
        color = PALETTE.get(name, "#333")
        width = 3 if name == "index" else 1.6
        pts = " ".join(f"{X(i):.1f},{Y(v):.1f}" for i, v in enumerate(ys))
        if n == 1:
            out.append(f'<circle cx="{X(0):.1f}" cy="{Y(ys[0]):.1f}" r="4" fill="{color}"/>')
        else:
            out.append(f'<polyline points="{pts}" fill="none" stroke="{color}" stroke-width="{width}"/>')
            for i, v in enumerate(ys):
                out.append(f'<circle cx="{X(i):.1f}" cy="{Y(v):.1f}" r="2.5" fill="{color}"/>')
    # legend with latest values
    lx, ly = ml + pw + 16, mt
    for j, (name, ys) in enumerate(series):
        color = PALETTE.get(name, "#333")
        y = ly + j * 18
        out.append(f'<line x1="{lx}" x2="{lx + 18}" y1="{y + 5}" y2="{y + 5}" stroke="{color}" stroke-width="{3 if name == "index" else 2}"/>')
        label = "overall index" if name == "index" else name
        out.append(f'<text x="{lx + 24}" y="{y + 9}" fill="#111" font-weight="{600 if name == "index" else 400}">{label} {fmt_x(ys[-1])}</text>')
    out.append(f'<text x="{ml}" y="{H - 8}" fill="#888" font-size="10">{n} snapshot{"s" if n != 1 else ""}; generated by scripts/perf/perfhistory.py render</text>')
    out.append("</svg>")
    return "\n".join(out) + "\n"


def _esc(t: str) -> str:
    return str(t).replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;")


def _area_color(area: str) -> str:
    return PALETTE.get(area, "#333333")


def step_changes(snaps: list[dict], threshold: float, window: int = REGRESSION_WINDOW) -> dict:
    """The latest snapshot against the recent past: kernels whose Ir rose
    above the highest value of their last `window` same-fingerprint
    snapshots by more than `threshold` (regressions), fell below the lowest
    (improvements), or now compute something else (re-based).  `before` is
    that recent maximum (minimum for improvements), `previous` the last value."""
    out = {"base": None, "head": snaps[-1] if snaps else None, "regressions": [],
           "improvements": [], "rebased": [], "new": [], "threshold": threshold, "window": window}
    if len(snaps) < 2:
        return out
    cur = snaps[-1]
    out["base"] = snaps[-2]
    past = [{k["id"]: k for k in s_["kernels"]} for s_ in snaps[-1 - window:-1]][::-1]
    for k in cur["kernels"]:
        q = past[0].get(k["id"])
        if q is None:
            out["new"].append(k["id"])
            continue
        if q["fingerprint"] != k["fingerprint"]:
            out["rebased"].append(k["id"])
            continue
        hist = []
        for snap in past:
            h = snap.get(k["id"])
            if h is None or h["fingerprint"] != k["fingerprint"] or not h.get("ir"):
                break
            hist.append(h["ir"])
        if not hist or not k.get("ir"):
            continue
        hi, lo = max(hist), min(hist)
        if k["ir"] > (1 + threshold) * hi:
            out["regressions"].append({"id": k["id"], "area": k["area"], "before": hi, "previous": hist[0],
                                       "after": k["ir"], "ratio": k["ir"] / hi})
        elif k["ir"] * (1 + threshold) < lo:
            out["improvements"].append({"id": k["id"], "area": k["area"], "before": lo, "previous": hist[0],
                                        "after": k["ir"], "ratio": k["ir"] / lo})
    out["regressions"].sort(key=lambda r: -r["ratio"])
    out["improvements"].sort(key=lambda r: r["ratio"])
    return out


def regressions_md(sc: dict, history_url: str | None) -> str:
    head, base = sc["head"], sc["base"]
    if base is None:
        return "Only one snapshot: nothing to compare yet.\n"
    L = [f"### Performance history: `{base['rev_short'][:8]}` → `{head['rev_short'][:8]}`\n",
         f"Callgrind instruction counts (`Ir`) of each perfbench kernel and ecbench child against its range over "
         f"the previous {sc['window']} snapshots on `main` with the same fingerprint. Flagged when it leaves that "
         f"range by more than {sc['threshold'] * 100:.0f} % (kernels that flip between two build levels stay quiet). "
         "This is a signal to look, never a failing check.\n"]
    if sc["regressions"]:
        L.append(f"**{len(sc['regressions'])} kernel(s) got more expensive:**\n")
        L.append("| kernel | area | recent max Ir | this commit Ir | change |")
        L.append("|---|---|---:|---:|---:|")
        for r in sc["regressions"]:
            L.append(f"| `{r['id']}` | {r['area']} | {r['before']:,} | {r['after']:,} | **+{(r['ratio'] - 1) * 100:.1f} %** |")
        L.append("")
    else:
        L.append("No kernel got more expensive.\n")
    if sc["improvements"]:
        L.append(f"{len(sc['improvements'])} kernel(s) got cheaper: " + ", ".join(
            f"`{r['id']}` {(r['ratio'] - 1) * 100:+.1f} %" for r in sc["improvements"][:8])
            + (" …" if len(sc["improvements"]) > 8 else "") + "\n")
    if sc["rebased"]:
        L.append("Re-based (computes something else now, not a speed comparison): "
                 + ", ".join(f"`{k}`" for k in sc["rebased"]) + "\n")
    if history_url:
        L.append(f"Charts and every kernel: {history_url}\n")
    return "\n".join(L)


def cmd_regressions(a: argparse.Namespace) -> None:
    snaps = load_history(Path(a.history))
    if not snaps:
        raise SystemExit(f"no snapshots in {a.history}")
    sc = step_changes(snaps, a.threshold)
    md = regressions_md(sc, a.history_url)
    if a.md:
        Path(a.md).write_text(md)
    if a.json:
        Path(a.json).write_text(json.dumps({k: v for k, v in sc.items() if k not in ("head", "base")}
                                           | {"head": sc["head"]["rev"] if sc["head"] else None,
                                              "base": sc["base"]["rev"] if sc["base"] else None}, indent=1) + "\n")
    if a.annotate:
        for r in sc["regressions"]:
            print(f"::warning title=perf regression::{r['id']} +{(r['ratio'] - 1) * 100:.1f} % Ir "
                  f"({r['before']:,} -> {r['after']:,})")
    print(md)


def svg_panels(snaps: list[dict], ch: dict, threshold: float = REGRESSION_THRESHOLD) -> str:
    """Small multiples: one panel per area, one thin line per kernel's chained
    index, the area index drawn thick.  A kernel whose latest step got more
    expensive than the threshold is ringed in red, so regressions are visible
    without reading a table."""
    n = len(snaps)
    sc = step_changes(snaps, threshold)
    regressed = {r["id"] for r in sc["regressions"]}
    areas = ch["areas"]
    cols = 3
    pw, ph, gap = 300, 170, 24
    ml, mt = 16, 56
    rows = math.ceil(len(areas) / cols)
    W = ml * 2 + cols * pw + (cols - 1) * gap
    H = mt + rows * (ph + gap) + 20
    vals = [math.log2(h["index"]) for kid in ch["ids"] for h in ch["per"][kid] if h["index"] > 0]
    lo, hi = (min(vals + [0.0]), max(vals + [0.0])) if vals else (0.0, 0.0)
    pad = max(0.15, (hi - lo) * 0.06)
    lo, hi = lo - pad, hi + pad
    out = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" viewBox="0 0 {W} {H}" '
           'font-family="ui-sans-serif, system-ui, sans-serif" font-size="11">',
           f'<rect width="{W}" height="{H}" fill="#ffffff"/>',
           f'<text x="{ml}" y="22" font-size="15" font-weight="600" fill="#111">Every kernel, by area</text>',
           f'<text x="{ml}" y="40" fill="#555">chained Ir index over {n} snapshots (shared log scale, above the dashed '
           'line = fewer instructions). Thick line: the area index. Red ring: latest step more than '
           f'{threshold * 100:.0f} % more expensive.</text>']
    for i, area in enumerate(areas):
        r, c = divmod(i, cols)
        x0 = ml + c * (pw + gap)
        y0 = mt + r * (ph + gap)
        ix0, iy0, iw, ih = x0 + 34, y0 + 22, pw - 44, ph - 40

        def X(t):
            return ix0 + (iw * t / (n - 1) if n > 1 else iw / 2)

        def Y(v):
            return iy0 + ih * (1 - (math.log2(v) - lo) / (hi - lo))

        color = _area_color(area)
        kids = sorted(k for k, a in ch["ids"].items() if a == area)
        out.append(f'<rect x="{x0}" y="{y0}" width="{pw}" height="{ph}" fill="#fafafa" stroke="#e3e3e3"/>')
        out.append(f'<text x="{x0 + 8}" y="{y0 + 15}" font-weight="600" fill="#111">{_esc(area)}</text>')
        out.append(f'<text x="{x0 + pw - 8}" y="{y0 + 15}" text-anchor="end" fill="#333">'
                   f'{fmt_x(ch["area_index"][area][-1])} · {len(kids)} kernels</text>')
        for k in range(math.floor(lo), math.ceil(hi) + 1):
            y = iy0 + ih * (1 - (k - lo) / (hi - lo))
            if iy0 <= y <= iy0 + ih:
                out.append(f'<line x1="{ix0}" x2="{ix0 + iw}" y1="{y:.1f}" y2="{y:.1f}" stroke="#ececec"/>')
                lab = f"{2 ** k:g}×" if k >= 0 else f"1/{2 ** (-k):g}"
                out.append(f'<text x="{ix0 - 4}" y="{y + 3:.1f}" text-anchor="end" fill="#888" font-size="9">{lab}</text>')
        out.append(f'<line x1="{ix0}" x2="{ix0 + iw}" y1="{Y(1.0):.1f}" y2="{Y(1.0):.1f}" stroke="#aaa" stroke-dasharray="3 3"/>')
        for kid in kids:
            pts = [(X(t), Y(h["index"])) for t, h in enumerate(ch["per"][kid])]
            if n > 1:
                out.append('<polyline points="' + " ".join(f"{x:.1f},{y:.1f}" for x, y in pts)
                           + f'" fill="none" stroke="{color}" stroke-opacity="0.35" stroke-width="1"><title>{_esc(kid)}</title></polyline>')
            if kid in regressed:
                x, y = pts[-1]
                out.append(f'<circle cx="{x:.1f}" cy="{y:.1f}" r="4.5" fill="none" stroke="#d62728" stroke-width="2">'
                           f'<title>{_esc(kid)} regressed</title></circle>')
        apts = " ".join(f"{X(t):.1f},{Y(v):.1f}" for t, v in enumerate(ch["area_index"][area]))
        if n > 1:
            out.append(f'<polyline points="{apts}" fill="none" stroke="{color}" stroke-width="2.6"/>')
        out.append(f'<text x="{ix0}" y="{y0 + ph - 5}" fill="#888" font-size="9">{snaps[0]["committed_at"][:10]}</text>')
        out.append(f'<text x="{ix0 + iw}" y="{y0 + ph - 5}" text-anchor="end" fill="#888" font-size="9">{snaps[-1]["committed_at"][:10]}</text>')
    out.append("</svg>")
    return "\n".join(out) + "\n"


def factor_base_rows(snap: dict) -> list[dict]:
    """The latest snapshot's ecbench children that name a curve: index-calculus
    runs with their factor base, and the matched rho reference on that curve."""
    return [k for k in snap["kernels"] if k.get("curve_label")]


def svg_factor_bases(snaps: list[dict], ch: dict) -> str:
    """One row per index-calculus run, grouped by curve: a bar of its solve Ir
    (log scale) coloured by the factor base's symmetry class, labelled with the
    factor base and its size, next to the matched rho reference on the same
    curve (grey) and the run's index since it was first tracked."""
    rows = factor_base_rows(snaps[-1])
    if not rows:
        return ""
    curves: dict[str, list[dict]] = {}
    for k in rows:
        curves.setdefault(k["curve_label"], []).append(k)
    for v in curves.values():
        v.sort(key=lambda k: (k.get("factor_base") is not None, k.get("ir") or 0))
    irs = [k["ir"] for k in rows if k.get("ir")]
    lo, hi = math.log10(min(irs)) - 0.3, math.log10(max(irs)) + 0.15
    bar_h, gap, head = 22, 6, 30
    ml, label_w, chart_w, right_w = 16, 300, 420, 150
    total_rows = sum(len(v) for v in curves.values())
    H = 96 + total_rows * (bar_h + gap) + len(curves) * head + 40
    W = ml * 2 + label_w + chart_w + right_w
    x0 = ml + label_w

    def X(v):
        return x0 + chart_w * (math.log10(v) - lo) / (hi - lo)

    out = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" viewBox="0 0 {W} {H}" '
           'font-family="ui-sans-serif, system-ui, sans-serif" font-size="12">',
           f'<rect width="{W}" height="{H}" fill="#ffffff"/>',
           f'<text x="{ml}" y="22" font-size="15" font-weight="600" fill="#111">Factor bases: solve cost by size and symmetry</text>',
           f'<text x="{ml}" y="40" fill="#555">callgrind Ir of one frozen solve (log scale, shorter is cheaper), snapshot '
           f'{snaps[-1]["rev_short"][:8]} · grey = matched rho on the same curve · right: index since first tracked</text>']
    lx = ml
    for sym, colr in SYMMETRY_COLORS.items():
        out.append(f'<rect x="{lx}" y="52" width="12" height="12" fill="{colr}"/>')
        out.append(f'<text x="{lx + 17}" y="62" fill="#333">{_esc(sym)}</text>')
        lx += 30 + 7 * len(sym)
    out.append(f'<rect x="{lx}" y="52" width="12" height="12" fill="#d9d9d9" stroke="#999" stroke-dasharray="2 2"/>')
    out.append(f'<text x="{lx + 17}" y="62" fill="#333">rho reference</text>')
    y = 84
    for k in range(math.ceil(lo), math.floor(hi) + 1):
        x = X(10 ** k)
        out.append(f'<line x1="{x:.1f}" x2="{x:.1f}" y1="{y - 4}" y2="{H - 30}" stroke="#eeeeee"/>')
        out.append(f'<text x="{x:.1f}" y="{H - 14}" text-anchor="middle" fill="#888" font-size="10">10{str(k).translate(SUPERSCRIPT)}</text>')
    for curve, ks in curves.items():
        out.append(f'<text x="{ml}" y="{y + 16}" font-weight="600" fill="#111">{_esc(curve)}</text>')
        y += head
        for k in ks:
            fb = k.get("factor_base")
            if fb:
                colr = SYMMETRY_COLORS.get(fb["symmetry"], "#333")
                size = []
                if fb.get("columns") is not None:
                    size.append(f"{fb['columns']} columns")
                if fb.get("signed_points") is not None:
                    size.append(f"{fb['signed_points']} signed points")
                label = f"{fb['spec']}"
                sub = " · ".join(size)
                out.append(f'<rect x="{x0}" y="{y}" width="{max(2.0, X(k["ir"]) - x0):.1f}" height="{bar_h}" fill="{colr}">'
                           f'<title>{_esc(k["id"])}: {k["ir"]:,} Ir, {k.get("group_ops", 0):,} group ops</title></rect>')
            else:
                label, sub = k["id"].split("/")[-1], "rho reference"
                out.append(f'<rect x="{x0}" y="{y}" width="{max(2.0, X(k["ir"]) - x0):.1f}" height="{bar_h}" fill="#d9d9d9" '
                           f'stroke="#999" stroke-dasharray="3 2"><title>{_esc(k["id"])}: {k["ir"]:,} Ir</title></rect>')
            out.append(f'<text x="{x0 - 8}" y="{y + 10}" text-anchor="end" fill="#111">{_esc(label)}</text>')
            out.append(f'<text x="{x0 - 8}" y="{y + 21}" text-anchor="end" fill="#777" font-size="10">{_esc(sub)}</text>')
            out.append(f'<text x="{X(k["ir"]) + 6:.1f}" y="{y + 15}" fill="#333" font-size="11">{k["ir"]:,}</text>')
            idx = ch["per"].get(k["id"], [{}])[-1].get("index", 1.0)
            out.append(f'<text x="{W - ml}" y="{y + 15}" text-anchor="end" fill="#333">{fmt_x(idx)}</text>')
            y += bar_h + gap
        y += 6
    out.append("</svg>")
    return "\n".join(out) + "\n"


def factor_base_md(snaps: list[dict], ch: dict, svg_rel: str | None) -> list[str]:
    rows = factor_base_rows(snaps[-1])
    if not rows:
        return []
    L = ["## Factor bases\n",
         "Frozen index-calculus solves (`ecbench exec`, area `ic_factor_base`, weight 0 so they never move the "
         "overall index) with the factor base each one built, its size and the symmetry it is closed under, next "
         "to the matched rho reference on the same curve. Symmetry classes follow "
         "`docs/ecbench/FACTOR_BASE_PROTOCOL.md`: a base folded by signed Frobenius or by the GLV automorphism "
         "needs fewer columns for the same coverage.\n"]
    if svg_rel:
        L.append(f"![factor bases]({svg_rel})\n")
    L.append("| curve | run | factor base | symmetry | columns | signed points | solve Ir | group ops | × rho Ir | index |")
    L.append("|---|---|---|---|---:|---:|---:|---:|---:|---:|")
    by_curve: dict[str, list[dict]] = {}
    for k in rows:
        by_curve.setdefault(k["curve_label"], []).append(k)
    for curve, ks in by_curve.items():
        rho = next((k for k in ks if not k.get("factor_base")), None)
        for k in sorted(ks, key=lambda k: (k.get("factor_base") is not None, k["ir"])):
            fb = k.get("factor_base") or {}
            ratio = f"{k['ir'] / rho['ir']:.1f}×" if rho and fb else ("1×" if not fb else "")
            idx = ch["per"].get(k["id"], [{}])[-1].get("index", 1.0)
            L.append(f"| {curve} | `{k['id'].split('/')[-1]}` | {('`' + fb['spec'] + '`') if fb else 'rho'} | "
                     f"{fb.get('symmetry', '—')} | {fb.get('columns') if fb.get('columns') is not None else '—'} | "
                     f"{fb.get('signed_points') if fb.get('signed_points') is not None else '—'} | {k['ir']:,} | "
                     f"{k.get('group_ops', 0):,} | {ratio} | {fmt_x(idx)} |")
    L.append("")
    return L


def pct(a, b):
    """b relative to a, as a signed percentage of change in Ir (negative = fewer instructions)."""
    if not a or not b:
        return None
    return (b / a - 1.0) * 100.0


def render_md(snaps: list[dict], ch: dict, svg_rel: str | None, panels_rel: str | None = None,
              fb_rel: str | None = None) -> str:
    n = len(snaps)
    last = snaps[-1]
    L = []
    L.append("# Performance history\n")
    L.append("Generated by `scripts/perf/perfhistory.py render` from the snapshots in "
             "`docs/perf/history/`; do not edit by hand.  One snapshot per measured commit: "
             "callgrind instruction counts (`Ir`) of every quick `perfbench` kernel and of the frozen "
             "`ecbench` children in `docs/perf/ecbench-children.json`, with a wall-time reading beside "
             "each.  The index chains consecutive `Ir` ratios and takes no credit across a fingerprint "
             "change (the kernel computed something else: **re-based**).  Formula: "
             "`docs/perf/PERFORMANCE_INDEX.md`; what this does and does not mean: engineering class, "
             "`AGENTS.md` §3.  **Noise floor:** two builds of different sources move the `Ir` of "
             "untouched kernels by up to a few per cent (the release profile's 16 codegen units are "
             "partitioned afresh, and cross-module inlining changes with them), so a single step under "
             "about 5 % on one kernel is build noise, not a result; a trend over several snapshots, or "
             "an area index, is.\n")
    if svg_rel:
        L.append(f"![performance history]({svg_rel})\n")
    L.append(f"**Latest snapshot:** `{last['rev_short']}` ({last['committed_at'][:19]}Z) — {last['subject']}  ")
    L.append(f"**Snapshots:** {n}, from `{snaps[0]['rev_short']}` ({snaps[0]['committed_at'][:10]}).  ")
    L.append(f"**Overall chained Ir index since first snapshot:** **{fmt_x(ch['overall'][-1])}**  ")
    if last.get("note"):
        L.append(f"**Note on the latest snapshot:** {last['note']}  ")
    L.append("")
    sc = step_changes(snaps, REGRESSION_THRESHOLD)
    if sc["base"] is not None:
        if sc["regressions"]:
            L.append(f"> **{len(sc['regressions'])} kernel(s) got more than {REGRESSION_THRESHOLD * 100:.0f} % more expensive "
                     f"in the latest step:** " + ", ".join(f"`{r['id']}` +{(r['ratio'] - 1) * 100:.1f} %"
                                                       for r in sc["regressions"][:6])
                     + (" …" if len(sc["regressions"]) > 6 else "") + "\n")
        else:
            L.append(f"> No kernel got more than {REGRESSION_THRESHOLD * 100:.0f} % more expensive in the latest step.\n")
    if panels_rel:
        L.append(f"![every kernel by area]({panels_rel})\n")
    L += factor_base_md(snaps, ch, fb_rel)
    # areas
    L.append("## Areas\n")
    L.append("| area | kernels | index since first | step from previous |")
    L.append("|---|---:|---:|---:|")
    for a in ch["areas"]:
        ks = [kid for kid, ar in ch["ids"].items() if ar == a]
        cur = ch["area_index"][a][-1]
        prev = ch["area_index"][a][-2] if n > 1 else cur
        L.append(f"| {a} | {len(ks)} | {fmt_x(cur)} | {fmt_x(cur / prev) if prev else 'n/a'} |")
    L.append(f"| **overall** | {len(ch['ids'])} | **{fmt_x(ch['overall'][-1])}** | "
             f"{fmt_x(ch['overall'][-1] / ch['overall'][-2]) if n > 1 else 'n/a'} |\n")
    # snapshots
    L.append("## Snapshots\n")
    L.append("| # | committed | commit | overall | " + " | ".join(ch["areas"]) + " | kernels | host |")
    L.append("|---:|---|---|---:|" + "---:|" * len(ch["areas"]) + "---:|---|")
    for t, s in enumerate(snaps):
        cpu = s.get("host", {}).get("cpu", "")
        cpu = (cpu[:28] + "…") if len(cpu) > 29 else cpu
        L.append(f"| {t + 1} | {s['committed_at'][:10]} | `{s['rev_short'][:8]}` | {fmt_x(ch['overall'][t])} | "
                 + " | ".join(fmt_x(ch["area_index"][a][t]) for a in ch["areas"])
                 + f" | {len(s['kernels'])} | {cpu} |")
    L.append("")
    # movers
    if n > 1:
        moves = []
        for kid, hist in ch["per"].items():
            h = hist[-1]
            if h["step"]:
                moves.append((h["step"], kid, hist[-2]["ir"], h["ir"]))
        moves.sort(reverse=True)
        up = [m for m in moves if m[0] > 1 + MOVER][:12]
        down = sorted([m for m in moves if m[0] < 1 / (1 + MOVER)])[:12]
        L.append("## Movers in the latest step\n")
        L.append(f"Kernels whose `Ir` changed by more than {MOVER * 100:.0f} % against the previous snapshot with the "
                 "same fingerprint (the build-noise floor is a few per cent, see above).\n")
        if up:
            L.append("Fewer instructions:\n")
            L.append("| kernel | previous Ir | latest Ir | step |")
            L.append("|---|---:|---:|---:|")
            for s_, kid, a, b in up:
                L.append(f"| `{kid}` | {a:,} | {b:,} | {fmt_x(s_)} |")
            L.append("")
        if down:
            L.append("More instructions:\n")
            L.append("| kernel | previous Ir | latest Ir | step |")
            L.append("|---|---:|---:|---:|")
            for s_, kid, a, b in down:
                L.append(f"| `{kid}` | {a:,} | {b:,} | {fmt_x(s_)} |")
            L.append("")
        if not up and not down:
            L.append("None.\n")
    # per kernel
    L.append("## Kernels at the latest snapshot\n")
    L.append("`Δ prev` is the latest step's change in `Ir` (negative = fewer instructions); `index` is the chained "
             "index since the first snapshot that has the kernel; `wall` is one native median on the recording "
             "host and is informational only.\n")
    for a in ch["areas"]:
        L.append(f"### {a}\n")
        L.append("| kernel | Ir | wall | Δ prev | index | note |")
        L.append("|---|---:|---:|---:|---:|---|")
        for kid in sorted(k for k, ar in ch["ids"].items() if ar == a):
            hist = ch["per"][kid]
            h = hist[-1]
            if not h["present"]:
                L.append(f"| `{kid}` | — | — | — | {fmt_x(h['index'])} | absent in latest snapshot |")
                continue
            prev = hist[-2] if n > 1 else None
            d = pct(prev["ir"], h["ir"]) if prev and prev["present"] and not h["rebased"] else None
            note = "re-based (fingerprint changed)" if h["rebased"] else ("new" if n > 1 and prev and not prev["present"] else "")
            L.append(f"| `{kid}` | {h['ir']:,} | {_ms(h['wall_ns'])} | {'' if d is None else f'{d:+.1f} %'} | {fmt_x(h['index'])} | {note} |")
        L.append("")
    L.append("## How to add a point\n")
    L.append("```bash\ncargo build --release --example perfbench --bin ecbench\n"
             "python3 scripts/perf/perfhistory.py record --binary target/release/examples/perfbench \\\n"
             "    --ecbench target/release/ecbench --out docs/perf/history\n"
             "python3 scripts/perf/perfhistory.py render --history docs/perf/history \\\n"
             "    --md docs/perf/HISTORY.md --svg docs/perf/history.svg --json docs/perf/history.json\n```\n")
    L.append("`.github/workflows/perf-history.yml` does this for every push to `main` and checks each pull "
             "request against the latest snapshot.\n")
    return "\n".join(L)


def render_json(snaps: list[dict], ch: dict) -> dict:
    """A compact machine-readable view for agents: latest per-kernel numbers and the area series."""
    return {
        "schema": "perf.history_summary/v1",
        "snapshots": [{"rev": s["rev"], "committed_at": s["committed_at"], "subject": s["subject"],
                       "overall": ch["overall"][t],
                       "areas": {a: ch["area_index"][a][t] for a in ch["areas"]}}
                      for t, s in enumerate(snaps)],
        "kernels": {kid: {"area": ch["ids"][kid],
                          "latest_ir": ch["per"][kid][-1]["ir"],
                          "latest_wall_ns": ch["per"][kid][-1]["wall_ns"],
                          "factor_base": next((k.get("factor_base") for k in snaps[-1]["kernels"]
                                               if k["id"] == kid and k.get("factor_base")), None),
                          "index": ch["per"][kid][-1]["index"],
                          "series_ir": [h["ir"] for h in ch["per"][kid]],
                          "rebased_at": [snaps[t]["rev"] for t, h in enumerate(ch["per"][kid]) if h["rebased"]]}
                    for kid in sorted(ch["ids"])},
    }


def cmd_render(a: argparse.Namespace) -> None:
    snaps = load_history(Path(a.history))
    if not snaps:
        raise SystemExit(f"no snapshots in {a.history}")
    ch = chained(snaps, perfindex.load_weights(a.weights))
    if a.svg:
        Path(a.svg).write_text(svg_chart(snaps, ch, "crypto: perfbench + ecbench instruction-count history"))
    svg_rel = None
    if a.svg and a.md:
        svg_rel = os.path.relpath(a.svg, Path(a.md).parent)
    panels_rel = fb_rel = None
    if a.panels_svg:
        Path(a.panels_svg).write_text(svg_panels(snaps, ch))
        if a.md:
            panels_rel = os.path.relpath(a.panels_svg, Path(a.md).parent)
    if a.factor_base_svg:
        fb_svg = svg_factor_bases(snaps, ch)
        if fb_svg:
            Path(a.factor_base_svg).write_text(fb_svg)
            if a.md:
                fb_rel = os.path.relpath(a.factor_base_svg, Path(a.md).parent)
    if a.md:
        Path(a.md).write_text(render_md(snaps, ch, svg_rel, panels_rel, fb_rel))
    if a.json:
        Path(a.json).write_text(json.dumps(render_json(snaps, ch), indent=1) + "\n")
    print(f"{len(snaps)} snapshots; overall index {fmt_x(ch['overall'][-1])}; "
          + ", ".join(f"{a_}={fmt_x(ch['area_index'][a_][-1])}" for a_ in ch["areas"]))


# ----------------------------------------------------------------------------
# check


def cmd_check(a: argparse.Namespace) -> None:
    log = lambda s: print(s, file=sys.stderr, flush=True)  # noqa: E731
    snaps = load_history(Path(a.history))
    if not snaps:
        raise SystemExit(f"no snapshots in {a.history}")
    base = snaps[-1]
    rows = measure_perfbench(a.binary, a.filter, a.full, 1, 0.5, False, log)
    if a.ecbench and a.children and Path(a.children).exists():
        rows += measure_ecbench(a.ecbench, Path(a.children), 1, False, log)
    prev = {k["id"]: k for k in base["kernels"]}
    cur = {k["id"]: k for k in rows}
    regress, improve, rebased, new, gone, same = [], [], [], [], [], 0
    logs = []
    for kid in sorted(set(prev) | set(cur)):
        p, c = prev.get(kid), cur.get(kid)
        if p is None:
            new.append(kid)
        elif c is None:
            gone.append(kid)
        elif p["fingerprint"] != c["fingerprint"]:
            rebased.append(kid)
        else:
            r = c["ir"] / p["ir"]
            logs.append(math.log(r))
            if r > 1 + a.threshold:
                regress.append((r, kid, p["ir"], c["ir"]))
            elif r < 1 - a.threshold:
                improve.append((r, kid, p["ir"], c["ir"]))
            else:
                same += 1
    index = math.exp(-statistics.fmean(logs)) if logs else 1.0
    L = [f"## perf history check against `{base['rev_short']}` ({base['committed_at'][:10]})\n",
         f"Geometric-mean Ir index of this tree over the latest snapshot (same-fingerprint kernels): **{fmt_x(index)}** "
         f"(above 1 = fewer instructions). Threshold ±{a.threshold * 100:.0f} %.\n",
         f"- {len(improve)} kernels with fewer instructions, {len(regress)} with more, {same} within threshold, "
         f"{len(rebased)} re-based (fingerprint changed), {len(new)} new, {len(gone)} missing.\n"]
    for title, items in (("More instructions (regressions)", sorted(regress, reverse=True)),
                         ("Fewer instructions", sorted(improve))):
        if items:
            L.append(f"### {title}\n\n| kernel | snapshot Ir | this tree Ir | ratio |\n|---|---:|---:|---:|")
            for r, kid, pi, ci in items:
                L.append(f"| `{kid}` | {pi:,} | {ci:,} | {r:.3f} |")
            L.append("")
    if rebased:
        L.append("### Re-based (fingerprint changed; not a speed comparison)\n\n" + "\n".join(f"- `{k}`" for k in rebased) + "\n")
    if new:
        L.append("### New kernels\n\n" + "\n".join(f"- `{k}`" for k in new) + "\n")
    if gone:
        L.append("### Missing kernels\n\n" + "\n".join(f"- `{k}`" for k in gone) + "\n")
    md = "\n".join(L)
    if a.md:
        Path(a.md).write_text(md)
    print(md)
    if a.fail_on_regression and regress:
        sys.exit(1)


# ----------------------------------------------------------------------------


def main() -> None:
    p = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    sub = p.add_subparsers(dest="cmd", required=True)

    r = sub.add_parser("record", help="measure one revision and write a snapshot")
    r.add_argument("--binary", help="perfbench binary for the working tree")
    r.add_argument("--ecbench", help="ecbench binary for the working tree")
    r.add_argument("--ref", help="build and measure this revision instead (sparse worktree, this tree's harness)")
    r.add_argument("--rev", help="record the snapshot under this revision (default HEAD)")
    r.add_argument("--children", default="docs/perf/ecbench-children.json")
    r.add_argument("--out", required=True, help="snapshot directory")
    r.add_argument("--note", default="")
    r.add_argument("--filter", action="append", default=[])
    r.add_argument("--full", action="store_true", help="include Tier::Full kernels")
    r.add_argument("--wall-samples", type=int, default=5)
    r.add_argument("--max-seconds", type=float, default=1.0)
    r.add_argument("--no-wall", action="store_true", help="skip the native wall-time reading")
    r.add_argument("--keep", action="store_true", help="keep the --ref build directory")
    r.set_defaults(func=cmd_record)

    d = sub.add_parser("render", help="write HISTORY.md, history.svg and history.json from the snapshots")
    d.add_argument("--history", required=True)
    d.add_argument("--md")
    d.add_argument("--svg")
    d.add_argument("--json")
    d.add_argument("--weights")
    d.add_argument("--panels-svg", help="small multiples: one panel per area, one line per kernel")
    d.add_argument("--factor-base-svg", help="index-calculus solves by factor base, size and symmetry")
    d.set_defaults(func=cmd_render)

    g = sub.add_parser("regressions", help="surface Ir changes between the last two snapshots (never fails)")
    g.add_argument("--history", required=True)
    g.add_argument("--threshold", type=float, default=REGRESSION_THRESHOLD)
    g.add_argument("--md")
    g.add_argument("--json")
    g.add_argument("--history-url")
    g.add_argument("--annotate", action="store_true", help="print GitHub ::warning annotations")
    g.set_defaults(func=cmd_regressions)

    c = sub.add_parser("check", help="compare the working tree's Ir against the latest snapshot")
    c.add_argument("--binary", required=True)
    c.add_argument("--ecbench")
    c.add_argument("--children", default="docs/perf/ecbench-children.json")
    c.add_argument("--history", required=True)
    c.add_argument("--threshold", type=float, default=0.10)
    c.add_argument("--filter", action="append", default=[])
    c.add_argument("--full", action="store_true")
    c.add_argument("--md")
    c.add_argument("--fail-on-regression", action="store_true")
    c.set_defaults(func=cmd_check)

    a = p.parse_args()
    a.func(a)


if __name__ == "__main__":
    main()
