"""Lay out the R10 ledger figure from data/*.txt (written by rerate.sh).

Plotting only: every value is read from the derived files; nothing is
computed here beyond dividing a recorded cost by the pinned cost.
"""
import os
import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt

HERE = os.path.dirname(os.path.abspath(__file__))


def rows(name):
    with open(os.path.join(HERE, "data", name)) as f:
        return [line.rstrip("\n").split("\t") for line in f if line.strip()]


gm = {}
for r in rows("genus3_derived.txt"):
    if r[0] == "gm":
        gm[(r[1], int(r[2]))] = float(r[3])
    if r[0] == "rho":
        gm[("r1:rho", int(r[1]))] = float(r[2])

C4, C1, LP = "r4:c4:lpsgebw2o100c+C1C2", "r4:r2:lpsgebw2+C1", "r2a:main:lpsgewd"
pin = {b: gm[(LP, b)] for b in (10, 11)}
pin[12] = gm[(C1, 12)]
pin.update({b: gm[(C4, b)] for b in range(13, 23)})

arms = [
    ("r1:ic2lp", "round 1 IC-2LP (= item 7)", "#9e9e9e", "o"),
    ("r2a:main:lpsgewd", "round 2 lpsgewd, κ 2", "#4c72b0", "s"),
    ("r2a:tuned:lpsgewd", "round 2 TL (κ 1.41; exponent ref.)", "#55a868", "^"),
    ("r2b:sched:lpsgebw2", "round 2b sched + BW", "#8172b2", "v"),
    ("r3:r2:lpsgebw2+R1", "round 3 R2", "#c44e52", "D"),
    ("r4:r2:lpsgebw2+C1", "round 4 C1", "#dd8452", "P"),
    ("r4:w:lpsgebw2+C1C2", "round 4 C1+C2", "#937860", "X"),
    (C4, "round 4 C4 (pinned from 2^13)", "#000000", "*"),
    ("r1:rho", "E0 rho, negation (round 1)", "#1f77b4", "x"),
]

kob = {}
for r in rows("koblitz_derived.txt"):
    if r[0] == "kob":
        n, arm = r[1].split(" ")
        kob[(int(n), arm)] = float(r[3].split(" ")[1])
kpin = {41: kob[(41, "free@125")], 53: kob[(53, "free@280")], 61: kob[(61, "free@500")]}

fig, ax = plt.subplots(1, 3, figsize=(16, 6.2), gridspec_kw={"width_ratios": [2.2, 1.2, 1.2]})

a = ax[0]
for key, label, col, mk in arms:
    xs = [b for b in range(10, 23) if (key, b) in gm and b in pin]
    a.plot(xs, [gm[(key, b)] / pin[b] for b in xs], marker=mk, color=col, label=label,
           lw=1.6 if key == C4 else 1.0, ms=6)
a.axhline(1, color="black", lw=0.8, ls=":")
a.set_yscale("log")
a.set_xlabel("b  (q = next prime above 2^b)")
a.set_ylabel("cost / pinned cost (F_q-mult equivalents)")
a.set_title("(a) genus 3 after cubic GHS: each arm against g3ghs-v1\ncurves 0-2, geometric mean; below 1 at 2^12..2^20: rho (counted unit)")
a.legend(fontsize=7, loc="upper center", bbox_to_anchor=(0.5, -0.14), ncol=3)
a.set_yticks([0.5, 1, 2, 3, 4], ["0.5", "1", "2", "3", "4"])
a.grid(alpha=0.3, which="both")

b = ax[1]
ns = [41, 53, 61]
groups = [
    ("round-1 incumbent (repo K)", {41: "inc@255", 53: "inc@220", 61: "inc@500"}, "#9e9e9e"),
    ("round-1 candidate (free, repo K)", {41: "free@255", 53: "free@220", 61: "free@500"}, "#c44e52"),
    ("round-2 incumbent (tuned K)", {41: "inc@100", 53: "inc@240", 61: "inc@500"}, "#4c72b0"),
    ("pin: default (free, K 125/280/500)", {41: "free@125", 53: "free@280", 61: "free@500"}, "#000000"),
]
w = 0.2
for i, (label, m, col) in enumerate(groups):
    b.bar([j + (i - 1.5) * w for j in range(3)], [kob[(n, m[n])] / kpin[n] for n in ns], w, label=label, color=col)
b.axhline(1, color="black", lw=0.8, ls=":")
b.set_xticks(range(3), [f"k0n{n}" for n in ns])
b.set_ylabel("counted cost / pinned cost")
b.set_title("(b) compact-orbit Koblitz IC: kob-orbit-v1\nheld-out seeds, S3 root solves per solve")
b.legend(fontsize=7, loc="upper right")
b.grid(alpha=0.3, axis="y")

c = ax[2]
ic_tool = [7.657, 13.241, 20.97]       # rule v3, docs/ic/leaderboard.json
orbit = [9.49, 24.93, 48.40]           # kob-orbit-v1 rho_gap_cold_wall
c.bar([j - 0.18 for j in range(3)], ic_tool, 0.36, label="ic tool, rule v3 (in-process rho)", color="#55a868")
c.bar([j + 0.18 for j in range(3)], orbit, 0.36, label="compact orbit pin (item 09 rho, cross-run)", color="#000000")
c.set_xticks(range(3), [f"k0n{n}" for n in ns])
c.set_ylabel("cold IC / matched rho  (above 1: rho faster)")
c.set_title("(c) distance to rho, same ICV1 curves:\ncurve-level best is the ic tool pipeline")
c.legend(fontsize=7, loc="upper left")
c.grid(alpha=0.3, axis="y")

fig.suptitle("R10 pinned IC ledger (main cb2ec6873, 2026-10-19): rounds re-rated against the pin; derived by rerate.sh from committed records", fontsize=10)
fig.tight_layout(rect=(0, 0, 1, 0.96))
fig.savefig(os.path.join(HERE, "ledger.svg"))
fig.savefig(os.path.join(HERE, "ledger.png"), dpi=150)
