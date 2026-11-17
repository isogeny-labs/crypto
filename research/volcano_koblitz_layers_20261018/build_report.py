#!/usr/bin/env python3
"""Lay out the recorded results of this study as figures, Markdown and PDF.

Layout only (AGENTS.md "Implementation language: no Python"; research-visuals skill): every
number is read from `tables.json` (written by `examples/volcano_layers_analyze.rs`) or from
the sessions' own files.  Nothing is computed here except axis limits and text formatting.
Needs matplotlib and typst (a scratch virtualenv outside the repository).
"""
import glob, json, math, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
T = json.load(open(f"{HERE}/tables.json"))["sizes"]
RUN = "run-20261018T1625Z"
OUT = f"{HERE}/reports/{RUN}"
os.makedirs(OUT, exist_ok=True)

import matplotlib
matplotlib.use("svg")
import matplotlib.pyplot as plt
plt.rcParams.update({"svg.fonttype": "none", "font.size": 9})

ns = [e["n"] for e in T]
def ratio(e, name):
    for r in e["ratios"]:
        if r["comparison"] == name:
            return r
# ---------------------------------------------------------------- figure
fig, ax = plt.subplots(1, 2, figsize=(9.2, 3.6))
for k, (arm, col, dx) in enumerate([("rho-neg", "#1f6fb2", -0.12), ("rho-plain", "#c4561a", 0.12)]):
    xs, ys, lo, hi = [], [], [], []
    for i, e in enumerate(T):
        r = ratio(e, f"level1/crater {arm}")
        xs.append(i + dx); ys.append(r["ratio"]); lo.append(r["ratio"] - r["lo"]); hi.append(r["hi"] - r["ratio"])
    ax[0].errorbar(xs, ys, yerr=[lo, hi], fmt="o", color=col, capsize=3, label=f"rho.{arm.split('-')[1]}")
ax[0].axhline(1.0, color="#555", lw=0.8, ls="--")
ax[0].set_xticks(range(len(ns)), [f"n={n}" for n in ns])
ax[0].set_ylabel("level-1 neighbours / crater\n(ratio of mean counted GAE)")
ax[0].set_title("(a) generic solvers, 95% hierarchical interval")
ax[0].legend(frameon=False, loc="upper right")
ax[0].set_ylim(0.5, 1.45)
xs, ys, lo, hi, ideal = [], [], [], [], []
for e in T:
    r = ratio(e, "crater frob-strong/neg")
    xs.append(e["n"]); ys.append(r["ratio"]); lo.append(r["ratio"] - r["lo"]); hi.append(r["hi"] - r["ratio"]); ideal.append(r["ideal"])
ax[1].errorbar(xs, ys, yerr=[lo, hi], fmt="s", color="#1f6fb2", capsize=3, label="measured, rho.signed_frobenius_strong / rho.negation")
ax[1].plot(xs, ideal, "k--", lw=0.9, label=r"ideal $1/\sqrt{n}$")
ax[1].axhline(1.0, color="#999", lw=0.6)
ax[1].set_yscale("log"); ax[1].set_xlabel("n (field degree)"); ax[1].set_ylabel("crater only: Frobenius solver / negation solver")
ax[1].set_title("(b) the Frobenius-aware solver, crater (paired targets)")
ax[1].legend(frameon=False, fontsize=7, loc="lower left")
fig.tight_layout()
fig.savefig(f"{OUT}/fig_layers.svg")
fig.savefig(f"{HERE}/fig_layers.svg")

# ---------------------------------------------------------------- facts from the sessions
def audit_line(name):
    p = f"{HERE}/sessions/{name}.verify.log"
    return open(p).read().strip().splitlines()[-1] if os.path.exists(p) else "(no receipt)"
rows = []
for d in sorted(glob.glob(f"{HERE}/sessions/n*-*/session.json"), key=lambda s: (int(re.search(r"/n(\d+)-", s).group(1)), s)):
    s = json.load(open(d)); name = d.split("/")[-2]
    rows.append((name, s["session_id"], s["spec_id"], s["executions_planned"], s["records_written"], s["status"],
                 s["started_unix_ms"], s["finished_unix_ms"], audit_line(name), s["git_commit"][:9], s["git_dirty"]))
import datetime
def utc(ms): return datetime.datetime.fromtimestamp(ms / 1000, datetime.timezone.utc).strftime("%H:%MZ")

def fmt(x, d=1): return f"{x:,.{d}f}"
def table_rows():
    out = []
    for e in T:
        for key, a in sorted(e["arms"].items()):
            if key.endswith("rho-neg-aa"): continue
            out.append((e["n"], key, a["curves"], a["targets"], fmt(a["mean_gae"]), fmt(a["sd_gae"]), f'{a["mean_s"]:.3f}'))
    return out
def ratio_rows():
    out = []
    for e in T:
        for r in e["ratios"]:
            out.append((e["n"], r["comparison"], f'{r["ratio"]:.3f}', f'[{r["lo"]:.3f}, {r["hi"]:.3f}]', f'{r["ideal"]:.3f}' if "ideal" in r else ""))
    return out

# ---------------------------------------------------------------- Markdown (descriptive run report)
def md_table(head, body):
    s = "| " + " | ".join(head) + " |\n|" + "|".join("---" for _ in head) + "|\n"
    return s + "".join("| " + " | ".join(str(c) for c in r) + " |\n" for r in body)

run_md = f"""# Run report {RUN}: crater against level-1 neighbours, Koblitz curves over F_(2^n)

Descriptive. Interpretation is in [`../../RESULT.md`](../../RESULT.md).

**What ran.** `crypto volcano ecbench-spec` wrote the specs in `specs/n*/`; `./run_sessions.sh n19 n23 n37 n41` and
`./run_sessions.sh n61` ran each with `ecbench run --wait --allow-busy --cpus none` and audited it with
`ecbench verify --replay 8`. Protocol: [`PROTOCOL.md`](../../PROTOCOL.md) (with its dated addendum). Base commit
`{rows[0][9]}`, working tree dirty (the native tools were uncommitted when the sessions ran).

**Host and isolation.** A busy host (a concurrent long computation held three cores), so every run is at level L0:
counted operations only, no wall-time figure is reported. `--cpus none`.

**Sessions.**

{md_table(["session dir", "session id", "spec id", "planned", "records", "status", "start", "end", "audit receipt"],
          [(r[0], r[1], r[2], r[3], r[4], r[5], utc(r[6]), utc(r[7]), r[8]) for r in rows])}
Every session finished with every planned execution recorded and every record verified; there were no failed or
timed-out executions.

**Arms.** `rho.negation` (reference on the level-1 curves), `rho.plain` (baseline), `rho.negation` repeated as control
(`rho-neg-aa`, bit-identical to `rho-neg` because ecbench's counts are deterministic), and on the crater only
`rho.signed_frobenius_strong` (reference there). Seeds: `target_seed` 20261018, measurement seed 18 (n = 61: 18).
Workloads: crater 32 targets (16 at n = 61); each level-1 neighbour 8 targets (4 at n = 61); 12 neighbours (6 at n = 61).

**Measured values** (unit `ecbench.gae`; replication unit a workload; computed by `examples/volcano_layers_analyze.rs`,
`TABLES.md`, `tables.json`):

{md_table(["n", "layer / arm", "curves", "targets", "mean GAE", "sd of per-target GAE", "mean S"], table_rows())}
**Ratios** (95% hierarchical bootstrap, curves then targets; 4000 resamples):

{md_table(["n", "comparison", "ratio of means", "95% interval", "ideal 1/sqrt(n)"], ratio_rows())}
![layers]({'fig_layers.svg'})

*Figure. (a) Ratio of mean counted GAE, level-1 neighbours over the crater, for the two generic solvers, with 95%
hierarchical-bootstrap intervals. (b) On the crater only: `rho.signed_frobenius_strong` over `rho.negation` on the same
targets, against `1/sqrt(n)`. Sessions in the table above; sample sizes in the measured-values table.*

**Missing or limited.** The neighbour curves are not in `docs/curves/registry.json` (`registered=false` in the plans).
The `rho-neg-aa` control carries no sampling information. At n = 61 only 6 neighbours and 4 targets each were run, so
its intervals are wide. No wall time. No isogeny map between a neighbour and the crater was computed.

**Raw records.** `sessions/n*-{{crater,neighbors}}/records.jsonl`; logs `sessions/*.run.log`, `sessions/*.verify.log`.
"""
open(f"{OUT}/report.md", "w").write(run_md)

# ---------------------------------------------------------------- Typst
def esc(t):
    for ch in "\\#$*_@<>[]`~":
        t = t.replace(ch, "\\" + ch)
    return t
def typ_table(head, body, cols=None):
    cols = cols or len(head)
    cells = "".join(f"[*{esc(h)}*]," for h in head) + "".join("".join(f"[{esc(str(c))}]," for c in r) for r in body)
    return f"#table(columns: {cols}, inset: 4pt, stroke: 0.4pt + gray, {cells})\n"
def typ_para(t): return esc(t) + "\n\n"

def build_typ(path, title, blocks, svg):
    s = '#set page(margin: 1.6cm)\n#set text(size: 9.5pt)\n#set par(justify: true)\n#show heading: set block(above: 1em, below: 0.6em)\n'
    s += f"= {esc(title)}\n\n"
    for kind, *a in blocks:
        if kind == "h": s += f"== {esc(a[0])}\n\n"
        elif kind == "p": s += typ_para(a[0])
        elif kind == "t": s += typ_table(a[0], a[1]) + "\n"
        elif kind == "fig": s += f'#figure(image("{svg}", width: 100%), caption: [{esc(a[0])}])\n\n'
    open(path, "w").write(s)

if "--no-pdf" not in sys.argv:
    import typst
    blocks = [("p", "Descriptive run report. Interpretation is in RESULT.pdf."),
              ("h", "Sessions"), ("t", ["session dir", "session id", "spec id", "planned", "records", "status", "start", "end"],
                                  [(r[0], r[1], r[2], r[3], r[4], r[5], utc(r[6]), utc(r[7])) for r in rows]),
              ("p", "All audits: " + "; ".join(sorted(set(r[8] for r in rows))) + ". Busy host, isolation level L0 (counted operations only). Base commit " + rows[0][9] + ", working tree dirty."),
              ("h", "Measured values (ecbench.gae)"), ("t", ["n", "layer / arm", "curves", "targets", "mean GAE", "sd", "mean S"], table_rows()),
              ("h", "Ratios (95% hierarchical bootstrap)"), ("t", ["n", "comparison", "ratio", "interval", "ideal"], ratio_rows()),
              ("fig", "Figure. (a) Level-1 neighbours over crater, mean counted GAE, generic solvers. (b) Crater only: Frobenius solver over negation solver on the same targets, against 1/sqrt(n). Sessions as listed; sample sizes in the table."),
              ("p", "Limits: neighbour curves unregistered; the rho-neg-aa control is bit-identical to rho-neg; n = 61 ran 6 neighbours with 4 targets each; no wall time; no isogeny map computed.")]
    build_typ(f"{OUT}/report.typ", f"Run report {RUN}: crater against level-1 neighbours", blocks, "fig_layers.svg")
    typst.compile(f"{OUT}/report.typ", output=f"{OUT}/report.pdf")
    print("wrote", f"{OUT}/report.pdf")
print("wrote", f"{OUT}/report.md", f"{OUT}/fig_layers.svg")
