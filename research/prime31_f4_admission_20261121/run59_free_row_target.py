#!/usr/bin/env python3
"""Prime31 run59 exact quotient-span target stopping, conditional sampler only.
Independent of Rust model implementation. NOT an S3 extractor or an ECDLP solver.
"""
import json, random, statistics
from pathlib import Path

HERE = Path(__file__).resolve().parent
F = json.loads((HERE / "run57_basis.json").read_text())
N = 2147446831
GRID = (0, 100, 200, 300, 400, 500, 600, 700, 800, 900,
        1000, 1200, 1500, 1800, 2300, 2800, 3500)
TARGETS = (1, 32, 128, 1024)
REPS = 256
SEED = 0x58ac16e3

class Graph:
    def __init__(self, b):
        self.parent = list(range(b))
        self.sign = [1]*b
        self.anchor = [False]*b

    def root(self, i):
        if self.parent[i] == i:
            return i, 1
        r, s = self.root(self.parent[i])
        self.parent[i] = r
        self.sign[i] *= s
        return r, self.sign[i]

    def add(self, i, a, j, b):
        u, s = self.root(i)
        v, t = self.root(j)
        if u == v:
            if a*s+b*t:
                self.anchor[u] = True
        else:
            self.parent[u] = v
            self.sign[u] = -a*s*b*t
            self.anchor[v] |= self.anchor[u]

    def graph_p(self):
        n = len(self.parent)
        counts = {}
        a = 0
        for i in range(n):
            r, _ = self.root(i)
            if self.anchor[r]:
                a += 1
            else:
                counts[r] = counts.get(r, 0)+1
        return (2*a*a+sum(s*(s-1) for s in counts.values()))/(2*n*n)

def free_span_p(d, free_rows):
    """Canonical normal-form of log coordinate modulo graph AND free-row span."""
    n = len(d.parent)
    roots = []
    signs = []
    for i in range(n):
        r, s = d.root(i)
        roots.append(r)
        signs.append(s)
    pivots = {}
    for raw in free_rows:
        row = {}
        for i, coefficient in zip(raw[::2], raw[1::2]):
            r = roots[i]
            if d.anchor[r]:
                continue
            v = (row.get(r, 0)+signs[i]*coefficient) % N
            if v:
                row[r] = v
            else:
                row.pop(r, None)
        while row:
            k = min(row)
            if k not in pivots:
                inv = pow(row[k], -1, N)
                pivots[k] = {j: x*inv%N for j, x in row.items()}
                break
            factor = row[k]
            for j, value in pivots[k].items():
                v = (row.get(j, 0)-factor*value)%N
                if v:
                    row[j] = v
                else:
                    row.pop(j, None)

    cache = {}
    counts = {}
    anchored = 0
    for r in roots:
        if d.anchor[r]:
            anchored += 1
            continue
        if r not in cache:
            expr = {r: 1}
            for k in sorted(pivots):
                factor = expr.get(k, 0)
                if not factor:
                    continue
                for j, value in pivots[k].items():
                    v = (expr.get(j, 0)-factor*value)%N
                    if v:
                        expr[j] = v
                    else:
                        expr.pop(j, None)
            pos = tuple(sorted(expr.items()))
            neg = tuple((j, (-x)%N) for j, x in pos)
            cache[r] = min(pos, neg)
        key = cache[r]
        if not key:
            anchored += 1
        else:
            counts[key] = counts.get(key, 0)+1
    good = 2*anchored*anchored + sum(s*(s-1) for s in counts.values())
    return good/(2*n*n), len(pivots)

def model(name):
    ix = ("dickson", "hybrid").index(name)
    b = F["base_points"][ix]
    rows = F["rows"][name]
    support = F["distinct_sums_including_identity"][ix]-1
    data = {m: [] for m in GRID}
    for trial in range(REPS):
        rng = random.Random(SEED+0x9e3779b1*trial+(0 if name == "dickson" else 104729))
        d = Graph(b)
        prev = 0
        for m in GRID:
            for _ in range(prev, m):
                i, j = rng.randrange(b), rng.randrange(b)
                a = 1 if rng.getrandbits(1) else -1
                sign_b = 1 if rng.getrandbits(1) else -1
                if i == j and a == -sign_b:
                    sign_b = a
                d.add(i, a, j, sign_b)
            old = d.graph_p()
            new, fre = free_span_p(d, rows)
            assert new >= old-1e-14, (name, trial, m)
            data[m].append((old, new, fre))
            prev = m

    grid = []
    for m in GRID:
        v = data[m]
        old = [x[0] for x in v]
        new = [x[1] for x in v]
        wait_old = statistics.mean(1/p for p in old) if all(old) else None
        wait_new = statistics.mean(1/p for p in new) if all(new) else None
        grid.append(dict(m=m, graph_p=statistics.mean(old), augmented_p=statistics.mean(new),
                         free_rank=statistics.mean(x[2] for x in v),
                         zero_old=sum(x==0 for x in old), zero_new=sum(x==0 for x in new),
                         graph_calls={str(t):(m+t*wait_old)*N/support if wait_old is not None else None for t in TARGETS},
                         augmented_calls={str(t):(m+t*wait_new)*N/support if wait_new is not None else None for t in TARGETS}))
    best = {}
    for t in TARGETS:
        old = min((r for r in grid if r["graph_calls"][str(t)] is not None),
                  key=lambda r:r["graph_calls"][str(t)])
        new = min((r for r in grid if r["augmented_calls"][str(t)] is not None),
                  key=lambda r:r["augmented_calls"][str(t)])
        q0, q1 = old["graph_calls"][str(t)], new["augmented_calls"][str(t)]
        best[str(t)] = dict(graph_m=old["m"], graph_calls=q0,
                            augmented_m=new["m"], augmented_calls=q1,
                            reduction_pct=100*(1-q1/q0))
    return dict(b=b, free_rows=len(rows), support=support, trials=REPS,
                best=best, grid=grid)

def main():
    assert (F["base_points"], list(map(len, (F["rows"]["dickson"], F["rows"]["hybrid"])))) == ([509,677],[13,88])
    out = dict(model="run59 exact quotient span, conditional uniform synthetic positive pairs",
               seed=SEED, trial_count=REPS, targets=TARGETS, models={k:model(k) for k in ("dickson","hybrid")})
    for name, arm in out["models"].items():
        print(name, "B", arm["b"], "free", arm["free_rows"])
        for t, v in arm["best"].items():
            print(" T",t,"graph",round(v["graph_calls"]),"augmented",round(v["augmented_calls"]),"reduction_pct",round(v["reduction_pct"],2))
    assert abs(out["models"]["hybrid"]["best"]["1"]["augmented_calls"]-957500) < 1.0
    assert abs(out["models"]["hybrid"]["best"]["1024"]["augmented_calls"]-4699949) < 1.0
    (HERE/"RUN59_MODEL.json").write_text(json.dumps(out, indent=2)+"\n")
    print("PASS: 256 paired trials per arm; exact free-row quotient spans; model only")

if __name__ == "__main__":
    main()
