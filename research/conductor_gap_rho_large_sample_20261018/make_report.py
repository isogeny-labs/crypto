#!/usr/bin/env python3
"""Render report.typ (and report.pdf via the typst module) from results/summary.json,
audits/*.audit.json and figures/*.svg. Layout of recorded values only.
Run: /tmp/report-venv/bin/python make_report.py
"""
import json, os, glob, datetime
HERE = os.path.dirname(os.path.abspath(__file__))
S = json.load(open(f"{HERE}/results/summary.json"))

def f(x): return f"{x:.3f}"
def row(k, v): return f"[{k}], [{v['n_base']}/{v['n_other']}], [{f(v['mean_base'])}], [{f(v['mean_other'])}], [{f(v['ratio'])}], [{f(v['ci95'][0])}, {f(v['ci95'][1])}], [{f(v['paired_geomean_ratio'])}], [{f(v['paired_ci95'][0])}, {f(v['paired_ci95'][1])}],"

audits = []
for p in sorted(glob.glob(f"{HERE}/audits/*.audit.json")):
    import hashlib
    a = json.load(open(p)); name = os.path.basename(p).replace(".audit.json", "")
    rep = a["replays"]; nrep = len(rep) if isinstance(rep, list) else rep
    good = sum(1 for x in rep if x.get("reproduced")) if isinstance(rep, list) else nrep
    audits.append((name, f"{a['ok']} ({len(a['problems']) if isinstance(a['problems'], list) else a['problems']} problems, {a['verified_records']} verified)", f"{good}/{nrep} identical", hashlib.sha256(open(p, "rb").read()).hexdigest()))

typ = []
typ.append('#set page(paper: "a4", margin: 1.6cm)\n#set text(size: 9pt)\n#show table: set text(size: 7.5pt)\n')
typ.append('= Does Pollard rho see a large prime conductor gap? A large-sample test\n')
typ.append(f'Study `research/conductor_gap_rho_large_sample_20261018`, rendered {datetime.datetime.utcnow().strftime("%Y-%m-%d %H:%M UTC")}. Unit: S = charged group operations / sqrt(r) as recorded by ecbench (`cost.s`); wall time is not read. Intervals: 95% bootstrap over targets (4000 draws) for the ratio of means; normal 95% for the paired per-target log-ratio. The host was shared (load 24–37 of 32), every run is L0.\n')
typ.append('== Pooled floor / crater, arm `rho.negation`\n')
typ.append('#table(columns: 8, [tier / family], [pairs], [n floor], [n crater], [ratio of means], [95% bootstrap], [paired geo-mean], [paired 95%],')
for k, v in sorted(S["pooled"].items()):
    typ.append(f"[{k}], [{v['pairs']}], [{v['n_floor']}], [{v['n_crater']}], [{f(v['ratio'])}], [{f(v['ci95'][0])}, {f(v['ci95'][1])}], [{f(v['paired_geomean_ratio'])}], [{f(v['paired_ci95'][0])}, {f(v['paired_ci95'][1])}],")
typ.append(')\n')
typ.append('#figure(image("figures/pooled_ratios.svg", width: 85%), caption: [Pooled floor/crater by family and tier, arm rho.negation. s: ℓ = 8209 (s1a–d) and 131101 (s2); m: ℓ ≈ 2^20.2 at 45 bits; l: ℓ ≈ 2^25 at 53 bits (l1–l4 frozen, l5r–l8r replication).])\n')
typ.append('#figure(image("figures/s_distributions.svg", width: 100%), caption: [Recorded S per solve, crater vs floor vs isomorphic twin.])\n')
typ.append('== Controls\n')
typ.append('#figure(image("figures/controls.svg", width: 85%), caption: [Positive control rho.plain / rho.negation (expected 1.414) and A/A arm (identical seeds, so exactly 1 per target).])\n')
typ.append('== Every comparison, arm `rho.negation`\n')
typ.append('#figure(image("figures/ratios_rho_neg.svg", width: 100%, height: 25cm, fit: "contain"), caption: [Every floor/crater (blue) and twin/original (grey) ratio with its 95% bootstrap interval.])\n')
for name, ses in sorted(S["sessions"].items()):
    typ.append(f'=== {name}\n{ses["label"]}. Record outcomes: {ses["status_counts"]}.\n')
    for arm in ("rho-neg", "rho-plain"):
        rows = ses["arms"].get(arm)
        if not rows: continue
        typ.append(f'Arm `{arm}`:\n#table(columns: 8, [comparison], [n], [mean S base], [mean S other], [ratio], [95% bootstrap], [paired geo-mean], [paired 95%],')
        for k, v in rows.items(): typ.append(row(k, v))
        typ.append(')\n')
typ.append('== Audits\n#table(columns: 4, [session], [ok], [replays], [receipt sha256],')
for a in audits: typ.append(f"[{a[0]}], [{a[1]}], [{a[2]}], [{a[3][:16]}…],")
typ.append(')\n')
typ.append('== Formal companion\nSee `lean/RepresentationHashInvariance.lean` and `lean/README.md`: transport of an r-adding walk along a group isomorphism preserves the ρ-length and is a bijection of the walk space (axioms: propext, Quot.sound).\n')
open(f"{HERE}/report.typ", "w").write("\n".join(typ))
import typst
typst.compile(f"{HERE}/report.typ", output=f"{HERE}/report.pdf", root=HERE)
print("report.pdf written")
