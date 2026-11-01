# independent re-computation of the black-box ("bb") s for a WDSat .anf file; no import of btn_census
import sys, collections
c = collections.Counter()
for l in open(sys.argv[1]):
    if l.startswith("x"):
        t = l.split()[1:-1]
        vs = {int(v) for v in t if v != "T" and not v.startswith(".")}
        for v in vs: c[v] += 1
print(sum(1 if d <= 2 else 2 * d - 3 for d in c.values()))
