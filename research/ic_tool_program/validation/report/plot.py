"""Lays out recorded values only: the committed VALIDATION.json files and raw outputs.

    python plot.py   (from this directory; writes validation.svg and validation.png)
"""
import json
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

HERE = Path(__file__).resolve().parent
REPO = HERE.parents[3]
ROWS = [
    ("R06", "benchmarks/20261016-ic-r06-scan-key"),
    ("R08", "benchmarks/20261016-ic-r08-scan-fold"),
    ("R10", "benchmarks/20261016-ic-r10-main-head"),
    ("R11", "benchmarks/20261016-ic-r11-build-streams"),
    ("R12", "benchmarks/20261017-ic-r12-admitted-prefetch"),
    ("rule v6", "benchmarks/20261016-ic-rule-v6"),
    ("rule v7", "benchmarks/20261016-ic-rule-v7"),
    ("rule v8", "research/ic_tool_program/rule/v8"),
    ("A14", "benchmarks/20261017-ic-a14-prefetch-size-gate"),
    ("A15", "benchmarks/20261017-ic-a15-filter-slots"),
    ("A16", "benchmarks/20261017-ic-a16-huge-pages"),
    ("A17", "benchmarks/20261017-ic-a17-filter-huge-pages"),
    ("A18", "benchmarks/20261017-ic-a18-wider-base"),
    ("A19", "benchmarks/20261017-ic-a19-filter-pages-build-parts"),
    ("stages v8", "benchmarks/20261017-ic-scan-stages-v8"),
]

labels, figures, certs, built, canon, sizes = [], [], [], [], [], []
for label, d in ROWS:
    v = json.loads((REPO / d / "VALIDATION.json").read_text())
    rep = json.loads((REPO / d / "validator-report.json").read_text())
    cert = json.loads((REPO / d / "replay-certificate.json").read_text())
    ref = v["rho_reference"]
    labels.append(label)
    figures.append(len(rep["headline"]))
    certs.append(cert["scalar_verification"]["certificates"])
    built.append(ref["ic_over_rho"])
    canon.append(ref["rho_canonical_step_model"]["ic_over_rho"])
    sizes.append(ref["band_basis"].split("2^")[1].split(")")[0])

fig, (a, b) = plt.subplots(1, 2, figsize=(12, 5.4), gridspec_kw={"width_ratios": [1, 1.25]})
y = range(len(labels))
h = 0.38
a.barh([i - h / 2 for i in y], figures, h, color="#4C78A8", label="published figures recomputed (all agree)")
a.barh([i + h / 2 for i in y], certs, h, color="#9ECAE9", label="certificates verified ([s]G = T, [s+1]G != T)")
a.set_yticks(list(y), labels)
a.invert_yaxis()
a.set_xscale("log")
a.set_xlabel("count (log scale)")
a.set_title("Recomputed figures and verified scalars, per evidence directory")
a.legend(loc="upper center", bbox_to_anchor=(0.5, -0.14), fontsize=8, ncol=1)
a.set_xlim(10, 5000)

w = 0.38
b.bar([i - w / 2 for i in y], built, w, color="#F58518", label="measured rho of the row's own process (the gate's reference)")
b.bar([i + w / 2 for i in y], canon, w, color="#B279A2", label="rho at the canonical step (a model, not a reference)")
b.set_xticks(list(y), [f"{l}\n2^{s}" for l, s in zip(labels, sizes)], fontsize=7, rotation=90)
b.set_ylabel("cold S, index calculus / rho  (above 1: rho faster)")
b.axhline(1, color="black", lw=0.8)
b.set_title("Distance to rho at each row's largest size")
b.legend(fontsize=8)
fig.suptitle("Validation gate for PR #1978's IC rows: seed in REPLAY_SEED.md, 64 of 64 native replays identical", fontsize=10)
fig.tight_layout()
fig.savefig(HERE / "validation.svg")
fig.savefig(HERE / "validation.png", dpi=150)
