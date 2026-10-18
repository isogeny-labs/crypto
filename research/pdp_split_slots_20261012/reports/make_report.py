"""Lay out an ecbench run report (Typst) from a native ANALYSIS.json.

Layout only: every number comes from the analysis file that
`pdp_split_analyze` wrote from the session records; nothing is computed
here beyond formatting.

    python make_report.py ANALYSIS.json SESSION_DIR... --title T --out report.typ [--fig fig.svg]
"""

import json
import sys
from pathlib import Path


def esc(s):
    return str(s).replace("\\", "\\\\").replace("#", "\\#").replace("_", "\\_").replace("*", "\\*")


def fmt(x, nd=3):
    if x is None:
        return "—"
    if isinstance(x, float) and (abs(x) >= 1e5 or (abs(x) < 1e-2 and x != 0)):
        return f"{x:.3e}"
    return f"{x:.{nd}f}" if isinstance(x, float) else str(x)


def main():
    args = sys.argv[1:]
    analysis = json.loads(Path(args[0]).read_text())
    sessions = [a for a in args[1:] if not a.startswith("--") and Path(a).is_dir()]
    title = args[args.index("--title") + 1]
    out = Path(args[args.index("--out") + 1])
    figs = [args[i + 1] for i, a in enumerate(args) if a == "--fig"]
    rows = analysis["ecbench"]["rows"]
    lines = [
        '#set page(paper: "a4", margin: 16mm)', "#set text(size: 9pt)", "",
        f"= {esc(title)}", "",
        "*Descriptive run report* (research-visuals, \"Every experiment run\"): what ran and what was recorded; "
        "interpretation is in the study README.", "",
        "== Sessions", "",
    ]
    for s in sessions:
        sj = json.loads((Path(s) / "session.json").read_text())
        spec = json.loads((Path(s) / "spec.json").read_text())
        lines += [
            f"- `{esc(Path(s).name)}`: session `{sj.get('session_id')}`, spec `{sj.get('spec_id')}` "
            f"({esc(spec.get('label', ''))}); status `{sj.get('status')}`, "
            f"{sj.get('executions_planned')} executions planned, status counts `{json.dumps(sj.get('status_counts'))}`; "
            f"commit `{sj.get('git_commit')}` (dirty: {sj.get('git_dirty')}), binary SHA-256 `{str(sj.get('binary_sha256'))[:16]}…`; "
            f"targets: {spec['workloads'].get('targets_per_curve')} per curve, seed {spec['workloads'].get('target_seed')}, "
            f"rounds {spec['measurement'].get('rounds')}, isolation required {spec['measurement'].get('isolation_required')}.",
        ]
    lines += ["", "== Recorded values (medians over verified runs; per-workload values in ANALYSIS.json)", "",
              "#table(columns: 8, align: (left, left, right, right, right, right, right, right),",
              "[curve], [arm], [verified], [median S (recorded)], [median F4 XORs], [median systems], "
              "[median priced S], [priced ÷ strong rho (min–max)],"]
    for r in rows:
        vs = r.get("priced_over_strong_rho_per_workload") or []
        rng = f"{min(vs):.3g}–{max(vs):.3g}" if vs else "—"
        lines.append(
            f"[{esc(r['curve_slug'])}], [{esc(r['arm'])}], [{r['verified']}/{r['runs']}], [{fmt(r['median_s_recorded'])}], "
            f"[{fmt(r['median_f4_word_xors'])}], [{fmt(r['median_systems'], 1)}], [{fmt(r['median_priced_s'])}], "
            f"[{fmt(r['median_priced_over_strong_rho'])} ({rng})],"
        )
    lines += [")", "",
              "`S` = charged group-addition equivalents / √r. F4 arms: solver word XORs counted, not charged, so recorded S is a lower bound. "
              "Priced S adds XORs × the curve's pinned `ns_per_word_xor` (`docs/ic/calibration.json`, calibrated on another host) / √r: a sensitivity, "
              "and one that understates F4's per-call work. Table arms (`mitm-frobenius`) are fully charged. Wall time is descriptive only (L0).", ""]
    for f in figs:
        lines += [f'#figure(image("{Path(f).name}", width: 100%), caption: [Recorded values from ANALYSIS.json; layout `reports/plot_ecbench.py`.])', ""]
    lines += ["== Raw records", "", ", ".join(f"`{esc(s)}/records.jsonl`" for s in sessions) + "; audits under `audits/`."]
    out.write_text("\n".join(lines) + "\n")


if __name__ == "__main__":
    main()
