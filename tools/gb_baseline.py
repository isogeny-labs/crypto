#!/usr/bin/env python3
"""Baseline re-benchmark of the in-house Gröbner engines on the curve panel.

When the F4 / Gröbner code changes, every (curve, m) cell of
``benchmarks/gb-baseline/panel.json`` is re-run with the repository's own
harnesses, compared with the committed baseline, and the figures, REPORT.md
and REPORT.pdf are regenerated.  ``.github/workflows/gb-baseline-rebench.yml``
runs this automatically; the same commands work locally.

    python3 tools/gb_baseline.py run      --out /tmp/gbb            # measure every cell
    python3 tools/gb_baseline.py compare  --rows /tmp/gbb/rows.jsonl --out /tmp/gbb
    python3 tools/gb_baseline.py render   --rows /tmp/gbb/rows.jsonl --out benchmarks/gb-baseline
    python3 tools/gb_baseline.py promote  --rows /tmp/gbb/rows.jsonl    # new rows become the baseline
    python3 tools/gb_baseline.py all      --out /tmp/gbb [--promote]  # run + compare + render

Options for ``run``/``all``: ``--only N31,M31`` restricts curves, ``--m 2,3``
restricts m, ``--timeout S`` overrides the panel's per-cell timeout,
``--no-build`` skips ``cargo build``.

What is compared, per cell:

- **answer**: a digest of what the solver decided (the harness's verdict
  digest for the Koblitz stage; inconsistency / solving degree / basis length
  per target for the prime-field tower).  A change here is a change in
  *output*, not speed, and ``compare --strict`` exits non-zero on it.
- **counted cost**: the harness's own counted unit (64-bit word XORs for the
  GF(2) stage, multiply-adds for the F_p engine when it reports them, else
  the largest Macaulay matrix width).  This is the primary speed metric:
  hosted runners are shared, so wall time is descriptive only.
- **wall time**: reported, never used to flag a change.

A cell that does not finish within the timeout is *censored* (status
``timeout``); a cell the harness skips (inadmissible cofactor for that m,
missing factor base) is ``skipped``.  Neither is a measurement.

Scope: decomposition-stage / solver diagnostics only.  Nothing here is an
end-to-end ECDLP cost, a leaderboard row or a claim; rows are
``[pcc:unverified]`` until replayed under the validation gate.

Standard library plus matplotlib (figures and PDF).
"""

from __future__ import annotations

import argparse
import datetime as _dt
import hashlib
import json
import os
import platform
import shutil
import subprocess
import sys
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
PANEL = ROOT / "benchmarks" / "gb-baseline" / "panel.json"
BASELINE = ROOT / "benchmarks" / "gb-baseline" / "baseline.json"
REPORT_DIR = ROOT / "benchmarks" / "gb-baseline"

# Paths whose change should trigger a re-benchmark.  The workflow's `paths:`
# filter mirrors this list; `tools/gb_baseline.py paths` prints it.
ENGINE_PATHS = [
    "src/cryptanalysis/groebner_f4.rs",
    "src/cryptanalysis/groebner_cache.rs",
    "src/cryptanalysis/f4_*.rs",
    "src/cryptanalysis/inherited_f4.rs",
    "src/cryptanalysis/inherited_f4/**",
    "src/cryptanalysis/pq_f4_f2.rs",
    "src/cryptanalysis/pq_groebner_f2.rs",
    "src/cryptanalysis/matrix_f5_f2.rs",
    "src/cryptanalysis/sig_fp_tower*.rs",
    "src/cryptanalysis/koblitz_groebner.rs",
    "src/cryptanalysis/koblitz_groebner/**",
    "src/cryptanalysis/wide_groebner.rs",
    "src/cryptanalysis/gf2_elim.rs",
    "src/cryptanalysis/fp_backend*.rs",
    "src/cryptanalysis/semaev_*.rs",
    "src/cryptanalysis/binary_semaev*.rs",
    "examples/groebner_stage_bench.rs",
    "examples/pkm_tower_pilot.rs",
    "benchmarks/gb-baseline/panel.json",
    "tools/gb_baseline.py",
]

HARNESSES = ("groebner_stage_bench", "pkm_tower_pilot")

# Relative change in the counted cost below which a cell counts as unchanged.
COST_TOL = 0.01


# --------------------------------------------------------------------------- helpers


def _git(*args: str) -> str:
    try:
        return subprocess.check_output(["git", *args], cwd=ROOT, text=True, stderr=subprocess.DEVNULL).strip()
    except Exception:
        return ""


def _digest(obj) -> str:
    return hashlib.sha256(json.dumps(obj, sort_keys=True).encode()).hexdigest()[:16]


def load_panel(path: Path = PANEL) -> dict:
    panel = json.loads(path.read_text())
    if panel.get("schema") != "gb-baseline.panel/v1":
        sys.exit(f"{path}: schema must be gb-baseline.panel/v1")
    for c in panel["curves"]:
        if c.get("harness") not in HARNESSES:
            sys.exit(f"{path}: curve {c.get('id')}: harness must be one of {HARNESSES}")
    return panel


def read_rows(path: Path) -> list[dict]:
    return [json.loads(l) for l in Path(path).read_text().splitlines() if l.strip()]


def key(row: dict) -> tuple[str, int]:
    return (row["curve"], int(row["m"]))


def host_meta() -> dict:
    cpu = ""
    try:
        for line in Path("/proc/cpuinfo").read_text().splitlines():
            if line.startswith("model name"):
                cpu = line.split(":", 1)[1].strip()
                break
    except OSError:
        pass
    rustc = ""
    try:
        rustc = subprocess.check_output(["rustc", "--version"], text=True).strip()
    except Exception:
        pass
    return {
        "commit": os.environ.get("GITHUB_SHA") or _git("rev-parse", "HEAD"),
        "ref": os.environ.get("GITHUB_REF", _git("rev-parse", "--abbrev-ref", "HEAD")),
        "run_url": (
            f"{os.environ['GITHUB_SERVER_URL']}/{os.environ['GITHUB_REPOSITORY']}/actions/runs/{os.environ['GITHUB_RUN_ID']}"
            if os.environ.get("GITHUB_RUN_ID")
            else ""
        ),
        "host": platform.node(),
        "cpu": cpu,
        "cpus": os.cpu_count(),
        "rustc": rustc,
        "python": platform.python_version(),
        "utc": _dt.datetime.now(_dt.timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
    }


# --------------------------------------------------------------------------- run


def build(no_build: bool) -> None:
    if no_build:
        return
    cmd = ["cargo", "build", "--release"]
    for h in HARNESSES:
        cmd += ["--example", h]
    print("+", " ".join(cmd), flush=True)
    subprocess.run(cmd, cwd=ROOT, check=True)


def _bin(name: str) -> Path:
    target = Path(os.environ.get("CARGO_TARGET_DIR", ROOT / "target"))
    return target / "release" / "examples" / name


def cell_command(curve: dict, m: int, raw: Path, timeout: int) -> tuple[list[str], Path]:
    """The harness command for one cell and the file its rows land in."""
    tag = f"{curve['id']}-m{m}"
    if curve["harness"] == "groebner_stage_bench":
        out_dir = raw / tag
        out_dir.mkdir(parents=True, exist_ok=True)
        cell = f"{curve['a']}:{curve['n']}:{curve.get('factor_index', 0)}:{m}:{curve.get('targets', 4)}:{curve.get('first_target', 0)}"
        cmd = [str(_bin("groebner_stage_bench")), "--cells", cell, "--label", tag, "--out", str(out_dir)]
        return cmd, out_dir / "stage.json"
    # pkm_tower_pilot
    out = raw / f"{tag}.jsonl"
    t = str(curve.get("t", 2))
    cmd = [
        str(_bin("pkm_tower_pilot")),
        "--p", str(curve["p"]),
        "--kinds", curve.get("kind", "kummer"),
        "--engine", curve.get("engine_flag", "generic"),
        "--m", str(m),
        "--controls", "tower",
        "--t-min", t, "--max-t", t, "--max-t-m3", t, "--max-t-m4", t,
        "--planted", str(curve.get("planted", 2)),
        "--random", str(curve.get("random", 2)),
        "--ladder-t", "none",
        # the harness's own per-system budget; the outer timeout catches the rest
        "--budget", str(max(1, timeout - 5)),
        "--out", str(out),
    ]
    for extra in curve.get("extra_args", []):
        cmd.append(str(extra))
    return cmd, out


def normalise(curve: dict, m: int, out: Path) -> dict:
    """One panel row from a harness's output file."""
    base = {"curve": curve["id"], "m": m, "harness": curve["harness"], "engine": curve.get("engine", "")}
    if not out.exists():
        return {**base, "status": "error", "note": "harness wrote no output"}
    if curve["harness"] == "groebner_stage_bench":
        rows = json.loads(out.read_text()).get("rows", [])
        if not rows:
            return {**base, "status": "skipped", "note": "cell inadmissible for this m (curve, factor base or cofactor)"}
        r = rows[0]
        return {
            **base,
            "status": "ok",
            "answer_digest": str(r.get("verdict_digest", ""))[:16],
            "cost": r.get("word_ops"),
            "cost_unit": "word XORs (GF(2) Macaulay elimination)",
            "targets": r.get("targets"),
            "decomposed": r.get("decomposed"),
            "matrix_cols": r.get("matrix_cols"),
            "wall_s": (r.get("wall_ns") or 0) / 1e9,
            "detail": {k: r.get(k) for k in ("ell", "factor_base_points", "reductions", "splits", "exhausted", "build_ns", "reduce_ns")},
        }
    rows = [r for r in read_rows(out) if "summary" not in r]
    if not rows:
        return {**base, "status": "skipped", "note": "harness produced no rows for this m"}
    timed_out = any(r.get("timed_out") for r in rows)
    muladds = [r.get("muladds") for r in rows]
    if all(isinstance(x, (int, float)) for x in muladds):
        cost, unit = sum(muladds), "multiply-adds (F_p elimination)"
    else:
        cost, unit = max(int(r.get("max_cols") or 0) for r in rows), "max Macaulay columns"
    answers = [
        [r.get("target"), r.get("target_index"), r.get("inconsistent"), r.get("solving_degree_max"), r.get("basis_len"), r.get("planted_ok")]
        for r in rows
    ]
    return {
        **base,
        "status": "timeout" if timed_out else "ok",
        "answer_digest": _digest(answers),
        "cost": cost,
        "cost_unit": unit,
        "targets": len(rows),
        "solving_degree_max": max(int(r.get("solving_degree_max") or 0) for r in rows),
        "N": rows[0].get("N"),
        "wall_s": sum(float(r.get("wall_s") or 0) for r in rows),
    }


def cmd_run(a) -> Path:
    panel = load_panel(Path(a.panel))
    out = Path(a.out)
    raw = out / "raw"
    raw.mkdir(parents=True, exist_ok=True)
    build(a.no_build)
    only = set(a.only.split(",")) if a.only else None
    ms = [int(x) for x in a.m.split(",")] if a.m else panel["m_values"]
    timeout = a.timeout or panel.get("cell_timeout_s", 600)
    rows = []
    for curve in panel["curves"]:
        if only and curve["id"] not in only:
            continue
        for m in curve.get("m_values", ms):
            if a.m and m not in ms:
                continue
            cmd, result = cell_command(curve, m, raw, timeout)
            log = raw / f"{curve['id']}-m{m}.log"
            print(f"[{curve['id']} m={m}] {' '.join(cmd)}", flush=True)
            t0 = time.monotonic()
            try:
                with open(log, "w") as fh:
                    proc = subprocess.run(cmd, cwd=ROOT, stdout=fh, stderr=subprocess.STDOUT, timeout=timeout)
                row = normalise(curve, m, result)
                if proc.returncode != 0:
                    row.update(status="error", note=f"exit {proc.returncode}; see raw/{log.name}")
            except subprocess.TimeoutExpired:
                row = {"curve": curve["id"], "m": m, "harness": curve["harness"], "engine": curve.get("engine", ""),
                       "status": "timeout", "note": f"censored at {timeout}s"}
            row["elapsed_s"] = round(time.monotonic() - t0, 3)
            row["command"] = " ".join(Path(c).name if i == 0 else c for i, c in enumerate(cmd))
            print(f"    -> {row['status']} cost={row.get('cost')} wall={row.get('wall_s')}", flush=True)
            rows.append(row)
    (out / "rows.jsonl").write_text("".join(json.dumps(r, sort_keys=True) + "\n" for r in rows))
    (out / "meta.json").write_text(json.dumps({"panel": panel, "host": host_meta(), "timeout_s": timeout}, indent=2) + "\n")
    print(f"wrote {out/'rows.jsonl'} ({len(rows)} cells)")
    return out / "rows.jsonl"


# --------------------------------------------------------------------------- compare


def compare(rows: list[dict], baseline: list[dict], partial: bool = False) -> list[dict]:
    base = {key(r): r for r in baseline}
    out = []
    for r in rows:
        b = base.get(key(r))
        d = {"curve": r["curve"], "m": r["m"], "status": r["status"], "base_status": b["status"] if b else None}
        if not b:
            d["verdict"] = "new cell"
        elif r["status"] != "ok" or b["status"] != "ok":
            d["verdict"] = (f"still {r['status']}" if r["status"] == b["status"] else f"status {b['status']} -> {r['status']}")
        elif r.get("answer_digest") != b.get("answer_digest"):
            d["verdict"] = "ANSWER CHANGED"
        elif r.get("cost_unit") != b.get("cost_unit") or not b.get("cost"):
            d["verdict"] = "cost unit changed"
        else:
            ratio = r["cost"] / b["cost"]
            d["cost_ratio"] = ratio
            d["verdict"] = "faster" if ratio < 1 - COST_TOL else "slower" if ratio > 1 + COST_TOL else "unchanged"
        if b and r.get("wall_s") and b.get("wall_s"):
            d["wall_ratio"] = r["wall_s"] / b["wall_s"]
        out.append(d)
    seen = {key(r) for r in rows}
    for k, b in ({} if partial else base).items():
        if k not in seen:
            out.append({"curve": k[0], "m": k[1], "status": None, "base_status": b["status"], "verdict": "cell dropped"})
    return out


def load_baseline(path: Path) -> tuple[list[dict], dict]:
    if not path.exists():
        return [], {}
    doc = json.loads(path.read_text())
    return doc.get("rows", []), doc.get("host", {})


def cmd_compare(a) -> int:
    rows = read_rows(Path(a.rows))
    baseline, _ = load_baseline(Path(a.baseline))
    diff = compare(rows, baseline, partial=getattr(a, "partial", False))
    out = Path(a.out)
    out.mkdir(parents=True, exist_ok=True)
    (out / "compare.json").write_text(json.dumps(diff, indent=2) + "\n")
    md = summary_markdown(diff)
    (out / "compare.md").write_text(md)
    print(md)
    if os.environ.get("GITHUB_STEP_SUMMARY"):
        with open(os.environ["GITHUB_STEP_SUMMARY"], "a") as fh:
            fh.write(md + "\n")
    changed = [d for d in diff if d["verdict"] == "ANSWER CHANGED"]
    if a.strict and changed:
        print(f"::error::{len(changed)} cell(s) changed their answer against the baseline")
        return 1
    return 0


def summary_markdown(diff: list[dict]) -> str:
    counts: dict[str, int] = {}
    for d in diff:
        counts[d["verdict"]] = counts.get(d["verdict"], 0) + 1
    lines = ["## Gröbner baseline re-benchmark", ""]
    lines.append("Counts: " + ", ".join(f"{v} **{k}**" for k, v in sorted(counts.items())) if counts else "No cells.")
    lines += ["", "| curve | m | status | counted cost new/base | wall new/base | verdict |", "|:--|--:|:--|--:|--:|:--|"]
    for d in sorted(diff, key=lambda d: (d["curve"], d["m"])):
        cr = f"{d['cost_ratio']:.3f}" if "cost_ratio" in d else "—"
        wr = f"{d['wall_ratio']:.2f}" if "wall_ratio" in d else "—"
        lines.append(f"| {d['curve']} | {d['m']} | {d['status']} | {cr} | {wr} | {d['verdict']} |")
    lines += ["", "_Counted cost is the harness's own unit and decides faster/slower (tolerance "
              f"{COST_TOL:.0%}); wall time is descriptive (shared runner). Stage diagnostics only, "
              "`[pcc:unverified]`._", ""]
    return "\n".join(lines)


# --------------------------------------------------------------------------- render


def _fmt(x) -> str:
    if x is None:
        return "—"
    if isinstance(x, float):
        return f"{x:.3g}"
    if isinstance(x, int) and abs(x) >= 10_000:
        return f"{x:.3e}"
    return str(x)


def render(rows: list[dict], baseline: list[dict], meta: dict, out: Path) -> None:
    import matplotlib

    matplotlib.use("Agg")
    import matplotlib.pyplot as plt
    from matplotlib.backends.backend_pdf import PdfPages

    out.mkdir(parents=True, exist_ok=True)
    diff = compare(rows, baseline)
    curves = sorted({r["curve"] for r in rows})
    base = {key(r): r for r in baseline}
    figs: list[tuple[str, object]] = []

    # 1. counted cost vs m, current vs baseline, one panel per curve
    fig, axes = plt.subplots(1, len(curves), figsize=(5.2 * max(1, len(curves)), 4), squeeze=False)
    for ax, c in zip(axes[0], curves):
        cur = sorted((r for r in rows if r["curve"] == c), key=lambda r: r["m"])
        ok = [r for r in cur if r["status"] == "ok" and r.get("cost")]
        ax.plot([r["m"] for r in ok], [r["cost"] for r in ok], "o-", label="current", color="#2b6cb0")
        bok = [base[key(r)] for r in cur if key(r) in base and base[key(r)]["status"] == "ok" and base[key(r)].get("cost")]
        if bok:
            ax.plot([r["m"] for r in bok], [r["cost"] for r in bok], "s--", label="baseline", color="#a0aec0")
        for r in cur:
            if r["status"] in ("timeout", "error", "skipped"):
                ax.axvline(r["m"], color={"timeout": "#dd6b20", "error": "#c53030", "skipped": "#cbd5e0"}[r["status"]],
                           lw=2, ls=":", alpha=0.9)
        ax.set_yscale("log")
        ax.set_xlabel("m (summation-polynomial arity)")
        unit = ok[0]["cost_unit"] if ok else "counted cost"
        ax.set_ylabel(unit)
        ax.set_title(c)
        ax.grid(alpha=0.3)
        ax.legend(fontsize=8, title="dotted: orange timeout, red error, grey skipped", title_fontsize=7)
    fig.suptitle("Counted cost vs m on the baseline panel")
    fig.tight_layout()
    figs.append(("cost_vs_m", fig))

    # 2. ratio new/base
    with_ratio = [d for d in diff if "cost_ratio" in d]
    fig, ax = plt.subplots(figsize=(max(5, 0.5 * len(with_ratio) + 2), 3.6))
    if with_ratio:
        labels = [f"{d['curve']}\nm={d['m']}" for d in with_ratio]
        vals = [d["cost_ratio"] for d in with_ratio]
        colors = ["#2f855a" if v < 1 - COST_TOL else "#c53030" if v > 1 + COST_TOL else "#718096" for v in vals]
        ax.bar(range(len(vals)), vals, color=colors)
        ax.set_xticks(range(len(vals)), labels, fontsize=7)
        ax.axhline(1, color="k", lw=0.8)
    else:
        ax.text(0.5, 0.5, "no cell comparable with the baseline yet", ha="center", va="center", transform=ax.transAxes)
    ax.set_ylabel("counted cost, current / baseline")
    ax.set_title("Change against the committed baseline (green faster, red slower)")
    fig.tight_layout()
    figs.append(("delta_vs_baseline", fig))

    # 3. wall time (descriptive)
    fig, ax = plt.subplots(figsize=(6, 3.6))
    for c in curves:
        ok = sorted((r for r in rows if r["curve"] == c and r["status"] == "ok" and r.get("wall_s")), key=lambda r: r["m"])
        if ok:
            ax.plot([r["m"] for r in ok], [r["wall_s"] for r in ok], "o-", label=c)
    ax.set_yscale("log")
    ax.set_xlabel("m")
    ax.set_ylabel("wall s (descriptive)")
    ax.set_title("Wall time per cell (shared host, not a speed claim)")
    ax.grid(alpha=0.3)
    if curves:
        ax.legend(fontsize=8)
    fig.tight_layout()
    figs.append(("wall_vs_m", fig))

    for name, f in figs:
        f.savefig(out / f"{name}.png", dpi=150)
        f.savefig(out / f"{name}.svg")

    host = meta.get("host", {})
    table = ["| curve | m | status | counted cost | unit | answer digest | wall s | vs baseline |",
             "|:--|--:|:--|--:|:--|:--|--:|:--|"]
    dmap = {(d["curve"], d["m"]): d for d in diff}
    for r in sorted(rows, key=key):
        d = dmap.get(key(r), {})
        table.append(f"| {r['curve']} | {r['m']} | {r['status']} | {_fmt(r.get('cost'))} | {r.get('cost_unit', '—')} | "
                     f"`{r.get('answer_digest', '—')}` | {_fmt(r.get('wall_s'))} | {d.get('verdict', '—')} |")
    panel = meta.get("panel", {})
    curve_lines = [f"- **{c['id']}** — {c.get('label', '')}; harness `examples/{c['harness']}.rs`, engine {c.get('engine', '')}"
                   for c in panel.get("curves", [])]
    md = "\n".join([
        "# Gröbner engine baseline panel",
        "",
        "Regenerated automatically by `.github/workflows/gb-baseline-rebench.yml` "
        "(`tools/gb_baseline.py`) whenever the F4 / Gröbner engine code, the harnesses or "
        "`panel.json` change. Do not edit by hand.",
        "",
        f"- Commit: `{host.get('commit', '')}`  ",
        f"- Run: {host.get('run_url') or 'local'}  ",
        f"- Host: {host.get('cpu', '')} ({host.get('cpus', '')} CPUs), {host.get('rustc', '')}  ",
        f"- Generated: {host.get('utc', '')}; per-cell timeout {meta.get('timeout_s', '')} s",
        "",
        "## Panel",
        "",
        *curve_lines,
        f"- m values: {panel.get('m_values', '')}",
        "",
        "## Cells",
        "",
        *table,
        "",
        "![counted cost vs m](cost_vs_m.png)",
        "",
        "![change vs baseline](delta_vs_baseline.png)",
        "",
        "![wall time](wall_vs_m.png)",
        "",
        "## Reading this",
        "",
        "Counted cost is the harness's own unit and is what decides faster/slower; "
        "wall time is descriptive (shared host). `ANSWER CHANGED` means the solver decided "
        "a target differently from the baseline — a correctness question before any speed one. "
        "`timeout` cells are censored, `skipped` cells are inadmissible for that m. "
        "Stage diagnostics of the solver axis only: no end-to-end ECDLP cost, no `S`, no "
        "leaderboard row; every number is `[pcc:unverified]`.",
        "",
    ])
    (out / "REPORT.md").write_text(md)

    with PdfPages(out / "REPORT.pdf") as pdf:
        tp = plt.figure(figsize=(8.27, 11.69))
        tp.text(0.06, 0.96, "Gröbner engine baseline panel", fontsize=16, weight="bold", va="top")
        info = [f"commit {host.get('commit', '')[:12]}   generated {host.get('utc', '')}",
                f"host {host.get('cpu', '')} ({host.get('cpus', '')} CPUs)   {host.get('rustc', '')}",
                f"per-cell timeout {meta.get('timeout_s', '')} s   cost tolerance {COST_TOL:.0%}", ""]
        info += [f"{c['id']}: {c.get('label', '')}" for c in panel.get("curves", [])]
        info += ["", f"{'curve':6} {'m':>2}  {'status':8} {'counted cost':>13}  {'wall s':>8}  vs baseline"]
        for r in sorted(rows, key=key):
            d = dmap.get(key(r), {})
            info.append(f"{r['curve']:6} {r['m']:>2}  {r['status']:8} {_fmt(r.get('cost')):>13}  {_fmt(r.get('wall_s')):>8}  {d.get('verdict', '')}")
        info += ["", "Stage diagnostics only; not an ECDLP cost or claim. [pcc:unverified]"]
        tp.text(0.06, 0.92, "\n".join(info), fontsize=8, family="monospace", va="top")
        pdf.savefig(tp)
        plt.close(tp)
        for _, f in figs:
            pdf.savefig(f)
            plt.close(f)
    print(f"wrote {out}/REPORT.md, REPORT.pdf and {len(figs)} figures (png+svg)")


def cmd_render(a) -> None:
    rows = read_rows(Path(a.rows))
    baseline, _ = load_baseline(Path(a.baseline))
    meta_path = Path(a.rows).parent / "meta.json"
    meta = json.loads(meta_path.read_text()) if meta_path.exists() else {"host": host_meta(), "panel": load_panel()}
    render(rows, baseline, meta, Path(a.out))


def cmd_promote(a) -> None:
    rows = read_rows(Path(a.rows))
    meta_path = Path(a.rows).parent / "meta.json"
    meta = json.loads(meta_path.read_text()) if meta_path.exists() else {"host": host_meta()}
    # Merge: cells measured now replace their old rows; cells not in this run
    # (a partial `--only` / `--m` dispatch) keep their previous baseline.
    old, _ = load_baseline(Path(a.baseline))
    merged = {key(r): r for r in old}
    merged.update({key(r): r for r in rows})
    rows = [merged[k] for k in sorted(merged)]
    doc = {"schema": "gb-baseline.baseline/v1", "host": meta.get("host", {}), "timeout_s": meta.get("timeout_s"), "rows": rows}
    Path(a.baseline).write_text(json.dumps(doc, indent=2, sort_keys=True) + "\n")
    print(f"promoted {len(rows)} cells to {a.baseline}")


def cmd_all(a) -> int:
    rows_path = cmd_run(a)
    a.rows = str(rows_path)
    rc = cmd_compare(argparse.Namespace(rows=a.rows, baseline=a.baseline, out=a.out, strict=a.strict,
                                        partial=bool(a.only or a.m)))
    # Render into the report directory while the old baseline is still the reference.
    cmd_render(argparse.Namespace(rows=a.rows, baseline=a.baseline, out=a.report_dir))
    if a.promote:
        cmd_promote(argparse.Namespace(rows=a.rows, baseline=a.baseline))
    return rc


def main(argv=None) -> int:
    p = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    sub = p.add_subparsers(dest="cmd", required=True)

    def run_opts(sp):
        sp.add_argument("--panel", default=str(PANEL))
        sp.add_argument("--out", required=True)
        sp.add_argument("--only", help="comma-separated curve ids")
        sp.add_argument("--m", help="comma-separated m values (default: the panel's)")
        sp.add_argument("--timeout", type=int, help="per-cell timeout in seconds")
        sp.add_argument("--no-build", action="store_true")

    run_opts(sub.add_parser("run", help="measure every panel cell"))
    sp = sub.add_parser("compare", help="compare rows with the baseline")
    sp.add_argument("--rows", required=True)
    sp.add_argument("--baseline", default=str(BASELINE))
    sp.add_argument("--out", required=True)
    sp.add_argument("--strict", action="store_true", help="exit 1 if any cell's answer changed")
    sp.add_argument("--partial", action="store_true", help="rows cover part of the panel; do not report missing cells")
    sp = sub.add_parser("render", help="figures, REPORT.md, REPORT.pdf")
    sp.add_argument("--rows", required=True)
    sp.add_argument("--baseline", default=str(BASELINE))
    sp.add_argument("--out", default=str(REPORT_DIR))
    sp = sub.add_parser("promote", help="make these rows the committed baseline")
    sp.add_argument("--rows", required=True)
    sp.add_argument("--baseline", default=str(BASELINE))
    sp = sub.add_parser("all", help="run + compare + render [+ promote]")
    run_opts(sp)
    sp.add_argument("--baseline", default=str(BASELINE))
    sp.add_argument("--report-dir", default=str(REPORT_DIR))
    sp.add_argument("--strict", action="store_true")
    sp.add_argument("--promote", action="store_true")
    sub.add_parser("paths", help="print the engine paths that trigger a re-benchmark")

    a = p.parse_args(argv)
    if a.cmd == "run":
        cmd_run(a)
    elif a.cmd == "compare":
        return cmd_compare(a)
    elif a.cmd == "render":
        cmd_render(a)
    elif a.cmd == "promote":
        cmd_promote(a)
    elif a.cmd == "all":
        return cmd_all(a)
    elif a.cmd == "paths":
        print("\n".join(ENGINE_PATHS))
    return 0


if __name__ == "__main__":
    sys.exit(main())
