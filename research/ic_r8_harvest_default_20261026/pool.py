import json, random, math, sys, collections
random.seed(1)
rows = []
for f in sys.argv[1:]:
    for l in open(f):
        r = json.loads(l)
        rows.append(r)
cells = collections.defaultdict(list)
for r in rows:
    if not r.get("same_decomposition", True) and r["mode"] == "prod":
        continue
    cells[(r["mode"], r["cell"])].append(r)


def rom(rs, a, b, key):
    return sum(x[a + key] for x in rs) / sum(x[b + key] for x in rs)


def boot(rs, a, b, key, n=4000):
    v = []
    for _ in range(n):
        s = [random.choice(rs) for _ in rs]
        v.append(rom(s, a, b, key))
    v.sort()
    return v[int(.025 * n)], v[int(.975 * n)]


out = []
for mode in ("prod", "bench"):
    print("==", mode, "ratio of means off/default (>1: default cheaper); AA = off/off2")
    losing = {"cpu": 0, "word_ops": 0}
    lg = {"cpu": [], "word_ops": []}
    for (m, c), rs in sorted(cells.items()):
        if m != mode:
            continue
        line = c.ljust(22) + f"n={len(rs):2d} "
        for key in ("cpu", "word_ops"):
            k = "_" + key
            x = rom(rs, "i", "d", k)
            lo, hi = boot(rs, "i", "d", k)
            lose = hi < 1
            losing[key] += lose
            lg[key].append(math.log(x))
            line += f"{key}: {x:.3f} [{lo:.3f},{hi:.3f}]{'  LOSES' if lose else '       '}  "
        aa = rom(rs, "i", "i2", "_cpu")
        line += f"AA {aa:.3f}"
        print(line)
        out.append((m, c, len(rs)))
    # bootstrap of the geometric mean of cell ratios (resample targets within each cell)
    cs = [rs for (m, c), rs in sorted(cells.items()) if m == mode]
    for key in ("cpu", "word_ops"):
        g = []
        for _ in range(2000):
            lr = 0
            for rs in cs:
                s = [random.choice(rs) for _ in rs]
                lr += math.log(rom(s, "i", "d", "_" + key))
            g.append(math.exp(lr / len(cs)))
        g.sort()
        print(f"POOLED {mode} {key}: geo-mean of {len(cs)} cell ratios, 95% CI [{g[50]:.3f}, {g[1949]:.3f}]")
    for key in ("cpu", "word_ops"):
        print(f"  {key}: cells {len(lg[key])}, losing (CI hi<1) {losing[key]}, "
              f"point<1 {sum(1 for v in lg[key] if v<0)}, geo-mean of cell ratios {math.exp(sum(lg[key])/len(lg[key])):.3f}")
