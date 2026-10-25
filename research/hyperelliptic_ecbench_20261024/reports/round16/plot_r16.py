# Lays out recorded values from ../../round16/run1/cmd3.stdout and cmd4.stdout; computes nothing new.
import re, sys, matplotlib
matplotlib.use(sys.argv[4] if len(sys.argv) > 4 else "svg")
import matplotlib.pyplot as plt
ratio = []
for line in open(sys.argv[1]):
    m = re.match(r"\| icv1-hypg(\d)p(\d+)-n\d+-\w+ \| ([\d.]+) \| 8 \| ([\d.]+) \| \[([\d.]+), ([\d.]+)\]", line)
    if m:
        ratio.append((int(m[1]), float(m[3]), float(m[4]), float(m[5]), float(m[6])))
S = {}
for line in open(sys.argv[2]):
    m = re.match(r"\| \S+ \| (rho-neg|rho-plain) \| \S+ \| icv1-hypg(\d)p(\d+)-n\d+-\w+ \| ([\d.]+) \| 16/16 \| ([\d.]+)", line)
    if m:
        S.setdefault(m[1], []).append((int(m[2]), float(m[4]), float(m[5])))
fig, (ax, bx) = plt.subplots(1, 2, figsize=(10.5, 4.2))
mk = {2: "o", 3: "s", 4: "^"}
for g in (2, 3, 4):
    r = [x for x in ratio if x[0] == g]
    ax.errorbar([x[1] for x in r], [x[2] for x in r], yerr=[[x[2]-x[3] for x in r], [x[4]-x[2] for x in r]],
                fmt=mk[g], color="#2b6cb0", ms=5, capsize=2, label=f"genus {g}")
ax.axhline(0.7071, color="#555", ls="--", lw=1); ax.text(10.2, 0.67, "ideal 1/√2", color="#555", fontsize=8)
ax.axhline(1.0, color="#999", ls=":", lw=1); ax.axvline(16.61, color="#bbb", lw=0.8); ax.text(16.7, 1.75, "r ≥ 10⁵", color="#888", fontsize=7)
ax.set_xlabel("log2 r"); ax.set_ylabel("rho.negation / rho.plain (S, w = 0), 95% interval")
ax.set_title("Per-curve ratio, 8 targets × 2 rounds", fontsize=9); ax.legend(fontsize=7)
for arm, c in (("rho-plain", "#c05621"), ("rho-neg", "#2b6cb0")):
    for g in (2, 3, 4):
        r = [x for x in S.get(arm, []) if x[0] == g]
        bx.scatter([x[1] for x in r], [x[2] for x in r], marker=mk[g], color=c, s=22,
                   label=f"{arm}, g{g}" if g == 2 else None)
bx.axhline(1.2533, color="#c05621", ls="--", lw=1); bx.axhline(0.8862, color="#2b6cb0", ls="--", lw=1)
bx.set_xlabel("log2 r"); bx.set_ylabel("mean S (all charged group operations / √r)")
bx.set_title("ecbench S against the floors √(π/2), √(π/4)", fontsize=9); bx.legend(fontsize=7)
fig.suptitle("Round sixteen, CARD-HEIC-ROUND16-1: the thread's instances through ecbench (counted operations)", fontsize=10)
fig.tight_layout(); fig.savefig(sys.argv[3])
