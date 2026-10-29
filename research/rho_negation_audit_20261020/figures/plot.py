"""Lay out recorded values only: ecbench table (table.md, session
ECBS1h83b97eccdcda) and volcano-rho instance studies (volcano/instance-*).
No result is computed here beyond dividing a recorded mean by its stated floor."""
import json, math, re
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

H = Path(__file__).resolve().parent.parent
rows = []
for line in (H / "table.md").read_text().splitlines()[2:]:
    c = [x.strip() for x in line.strip("|").split("|")]
    lo, hi = map(float, re.findall(r"[\d.]+", c[7]))
    rows.append({"arm": c[1], "bits": float(c[4]), "S": float(c[6]), "lo": lo, "hi": hi, "theory": float(c[8])})

fig, ax = plt.subplots(1, 2, figsize=(10, 4))
for arm, col, lab in (("rho-neg", "#1f6feb", "rho.negation (A = 2)"), ("rho-plain", "#d1242f", "rho.plain (A = 1)")):
    r = [x for x in rows if x["arm"] == arm]
    ax[0].errorbar([x["bits"] for x in r], [x["S"] for x in r],
                   yerr=[[x["S"] - x["lo"] for x in r], [x["hi"] - x["S"] for x in r]],
                   fmt="o-", color=col, capsize=3, label=lab + ", mean S ± 95%")
    ax[0].axhline(r[0]["theory"], color=col, ls=":", lw=1)
ax[0].set_xlabel("log2 r"); ax[0].set_ylabel("S = GAE / sqrt(r)")
ax[0].set_title("ecbench, 64 targets per curve\nfloors sqrt(pi/4), sqrt(pi/2) dotted", fontsize=10)
ax[0].legend(fontsize=8); ax[0].set_ylim(0.6, 1.8)

on = json.loads((H / "volcano/instance-on/instance_study.json").read_text())
off = json.loads((H / "volcano/instance-off/instance_study.json").read_text())
n = on["curve"]["n"]
arms = [a["arm"] for a in on["arms"]]
x = range(len(arms))
fl_on, fl_off = math.sqrt(math.pi * n / 4), math.sqrt(math.pi * n / 2)
def bars(d, fl, dx, col, lab):
    m = [a["iters"]["mean"] / fl for a in d["arms"]]
    e = [a["iters"]["ci95_half"] / fl for a in d["arms"]]
    ax[1].bar([i + dx for i in x], m, 0.38, yerr=e, capsize=3, color=col, label=lab)
bars(on, fl_on, -0.2, "#1f6feb", "--negation on / sqrt(pi n/4)")
bars(off, fl_off, 0.2, "#d1242f", "--negation off / sqrt(pi n/2)")
ax[1].axhline(1, color="k", lw=0.8)
ax[1].set_xticks(list(x)); ax[1].set_xticklabels(arms, fontsize=8)
ax[1].set_ylabel("mean iterations / own floor (± 95%)")
ax[1].set_title(f"volcano-rho --instance-study, 36-bit n = {n}\n256 solves per arm", fontsize=10)
ax[1].legend(fontsize=8, loc="lower right"); ax[1].set_ylim(0, 1.4)
fig.suptitle("Pollard rho with and without the negation map (2026-10-20)", fontsize=11)
fig.tight_layout()
fig.savefig(H / "figures/negation_vs_plain.svg")
fig.savefig(H / "figures/negation_vs_plain.png", dpi=150)
