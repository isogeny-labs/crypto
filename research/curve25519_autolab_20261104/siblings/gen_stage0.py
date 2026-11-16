#!/usr/bin/env python3
"""Build STAGE0.md tables from raw/k*.txt (run from this directory)."""
import re
T = "[pcc:unverified]"
FLOOR = {1: 1.329, 2: 1.662}
agg = {}
for k in (20, 24, 28):
    for line in open(f"raw/k{k}.txt"):
        m = re.match(r"AGG C=(\d) (\S+(?: m=\d+)?)\s+E_points = ([\d.]+) \+- ([\d.]+)(?:\s+E_parents = ([\d.]+) \+- ([\d.]+))?\s+hit = ([\d.]+)", line)
        if m:
            c, name = int(m[1]), m[2].strip()
            agg[(c, k, name)] = dict(e=float(m[3]), se=float(m[4]),
                                     p=float(m[5]) if m[5] else None, pse=float(m[6]) if m[6] else None,
                                     hit=float(m[7]))
out = []
for c in (1, 2):
    out.append(f"### C = {c} (K = {c+1}), Poisson floor {FLOOR[c]} {T}\n")
    out.append("| k | m | E_points +- SE | E_parents +- SE | rank-K hit | E_points / D0 | E_points / floor | E_points / E_parents |")
    out.append("|--:|--:|--:|--:|--:|--:|--:|--:|")
    for k in (20, 24, 28):
        d0 = agg[(c, k, "D0")]; di = agg[(c, k, "D0i")]
        out.append(f"| {k} | D0 | {d0['e']:.4f} +- {d0['se']:.4f} | n/a | {d0['hit']:.4f} | 1.000 | {d0['e']/FLOOR[c]:.3f} | n/a |")
        out.append(f"| {k} | D0i | {di['e']:.4f} +- {di['se']:.4f} | n/a | {di['hit']:.4f} | {di['e']/d0['e']:.3f} | {di['e']/FLOOR[c]:.3f} | n/a |")
        for m in (1, 2, 4, 8, 16):
            a = agg.get((c, k, f"SB m={m}"))
            if not a:
                continue
            rt = a['e'] / d0['e']
            ce = ">= " if a['hit'] < 1 else ""
            out.append(f"| {k} | {m} | {ce}{a['e']:.4f} +- {a['se']:.4f} | {ce}{a['p']:.4f} +- {a['pse']:.4f} | {a['hit']:.4f} | {ce}{rt:.3f} | {ce}{a['e']/FLOOR[c]:.3f} | {a['e']/a['p']:.2f} |")
    out.append("")
tables = "\n".join(out)

# falsification
fal = []
for c in (1, 2):
    for m in (1, 2, 4, 8):
        over = [k for k in (20, 24, 28) if agg[(c, k, f"SB m={m}")]['e'] > 1.08 * agg[(c, k, "D0")]['e']]
        fal.append(f"| {c} | {m} | {', '.join(map(str, over)) or 'none'} | {len(over)} of 3 |")
falt = "\n".join(fal)
lo = [(c, k, m) for c in (1, 2) for k in (20, 24, 28) for m in (1, 2, 4, 8, 16)
      if (c, k, f"SB m={m}") in agg and agg[(c, k, f"SB m={m}")]['e'] < 0.95 * agg[(c, k, "D0")]['e']]
m16 = {c: agg[(c, 20, "SB m=16")]['e'] / agg[(c, 20, "D0")]['e'] for c in (1, 2)}
m8lo = min(agg[(c, k, "SB m=8")]['e'] / agg[(c, k, "D0")]['e'] for c in (1, 2) for k in (20, 24, 28))
m8hi = max(agg[(c, k, "SB m=8")]['e'] / agg[(c, k, "D0")]['e'] for c in (1, 2) for k in (20, 24, 28))

doc = open("STAGE0.template.md").read()
doc = (doc.replace("@@TABLES@@", tables).replace("@@FAL@@", falt)
          .replace("@@LOW@@", "none" if not lo else str(lo))
          .replace("@@M16C1@@", f"{m16[1]:.3f}").replace("@@M16C2@@", f"{m16[2]:.3f}")
          .replace("@@M8LO@@", f"{m8lo:.3f}").replace("@@M8HI@@", f"{m8hi:.3f}"))
open("STAGE0.md", "w").write(doc)
print(falt); print(lo, m16, m8lo, m8hi)
