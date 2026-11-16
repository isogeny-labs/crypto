#!/usr/bin/env python3
"""Measured runs (see PROTOCOL.md).  (Named cmeasure.py, not run.py: a first launch was killed by a
process-name kill from another job on the shared machine.)
Usage: cmeasure.py OUT.jsonl.gz DEADLINE_S L PHASES(csv of sweep,rand,planted,alpha,explore) [NTARGETS | a:b]"""
import sys, os, json, time, random, gzip
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import lib

lib.CMS_OPTS.update({"autodisablegauss": "0", "sls": "0"})
OUT, DEADLINE, L = sys.argv[1], float(sys.argv[2]), int(sys.argv[3])
PH = sys.argv[4].split(",")
NT = sys.argv[5] if len(sys.argv) > 5 else "24"   # number of targets, or a slice a:b of the 24 (continuation)
T0 = time.time()
CAP = 50000
fout = gzip.open(OUT, "at")


def emit(row):
    row.pop("model", None)
    fout.write(json.dumps(row, separators=(",", ":")) + "\n")
    fout.flush()


def late():
    return time.time() - T0 > DEADLINE


l = L
seed = int(os.environ.get("C_SEED_BASE", "2026111500")) + l
cell = lib.Cell(l, seed)
non, dec = cell.targets(24, 4)
_a, _b = ([0] + [int(x) for x in NT.split(":")])[-2:]
TG = list(enumerate(non))[_a:_b]
std = list(range(l))
ctx = {}


def get(idx, R):
    if idx not in ctx:
        S = lib.System(cell, R)
        ctx.clear()
        ctx[idx] = (S, S.render("dual"))
    return ctx[idx]


def base(idx, R, isdec, part):
    return {"l": l, "n": cell.n, "seed": seed, "B": cell.B, "idx": idx, "R": list(R),
            "decomposable": isdec, "part": part}


def dfs_row(idx, R, isdec, part, bud, rep, order, extra=None, cube_order=None):
    S, rd = get(idx, R)
    row = base(idx, R, isdec, part)
    row.update({"budget": bud, "rep": rep, "order": order})
    row.update(extra or {})
    if late():
        row["status"] = "NOT_RUN"
        return emit(row)
    if rep == 0:
        r2, cv = rd, S.c
    else:
        r2, perm = lib.shuffle_rendered(rd, seed * 31 + 7 * idx + rep)
        cv = [perm[v] for v in S.c]
    t = time.time()
    solver = lib.CMS(r2)
    st, nodes = lib.rec_dfs(S, solver, order, bud, cv)
    dt = time.time() - t
    u0 = cell.u0_decomposable(R)
    assert (st["sat"] or u0) == isdec, (idx, st["sat"], u0, isdec)
    if st["sat"]:
        m = st["model"]
        if rep:
            inv = [0] * (len(perm))
            m = [None] + [m[perm[v]] for v in range(1, len(perm))]
        assert cell.lifts(S.x_of(m), R)
    st.update(row)
    st.update({"status": "OK", "time": dt, "nodes": nodes})
    emit(st)


def leaf_assump(S, x):
    return [S.c[k] if (x >> k) & 1 else -S.c[k] for k in range(l)]


def alpha(idx, R):
    S, rd = get(idx, R)
    rng = random.Random(seed * 19 + idx)
    stop = False
    for j in range(0, min(6, l - 1) + 1):
        k = l - j
        both = 0
        for rep in range(2):
            vals = [rng.getrandbits(1) for _ in range(k)]
            wseed = rng.getrandbits(32)
            asm = [S.c[i] if v else -S.c[i] for i, v in enumerate(vals)]
            fixed = sum(v << i for i, v in enumerate(vals))
            for arm in ("cold", "warm32", "warmq"):
                row = base(idx, R, False, "alpha")
                row.update({"arm": arm, "j": j, "srep": rep, "vals": vals})
                if arm == "cold" and (j > 5 or stop):
                    row["status"] = "SKIPPED_AFTER_CENSOR" if stop and j <= 5 else "NOT_IN_PROTOCOL"
                    emit(row)
                    continue
                if late():
                    row["status"] = "NOT_RUN"
                    emit(row)
                    continue
                s = lib.CMS(rd)
                wc = 0
                if arm != "cold":
                    w = 32 if arm == "warm32" else 1 << (l - 2)
                    r2 = random.Random(wseed)
                    outside = [x for x in range(1 << l) if (x & ((1 << k) - 1)) != fixed]
                    for x in r2.sample(outside, w):
                        sat, _, cf = s.solve(leaf_assump(S, x))
                        assert sat is False
                        wc += cf
                t = time.time()
                sat, _, cf = s.solve(asm, CAP)
                assert sat is not True
                row.update({"status": "OK", "conflicts": cf, "censored": sat is None, "warm_conflicts": wc,
                            "time": time.time() - t})
                emit(row)
                if arm == "cold" and sat is None:
                    both += 1
        if both == 2:
            stop = True


def leafclass_row(idx, R):
    row = base(idx, R, False, "leafclass")
    row["cls"] = "".join(str(lib.leaf_class(cell, R, x)) for x in range(1 << l))
    row["status"] = "OK"
    emit(row)


BUDS = list(range(11)) if l <= 8 else [0, 1, 2, 3, 5, 10]
for ph in PH:
    if ph == "sweep":
        for idx, R in TG:
            leafclass_row(idx, R)
            for rep in (0, 1):
                for b in BUDS:
                    dfs_row(idx, R, False, "sweep", b, rep, std)
            for rep in (2, 3):
                for b in (0, 3):
                    dfs_row(idx, R, False, "sweep", b, rep, std)
    elif ph == "rand":
        for idx, R in TG:
            rnd = std[:]
            random.Random(seed * 13 + idx).shuffle(rnd)
            for b in (0, 3):
                dfs_row(idx, R, False, "rand", b, 0, rnd)
    elif ph == "planted":
        for i, R in enumerate(dec):
            for b in (0, 3, 10):
                dfs_row(24 + i, R, True, "planted", b, 0, std)
    elif ph == "alpha":
        for idx, R in [t for t in TG if t[0] < 12]:
            alpha(idx, R)
    elif ph == "explore":
        import explore
        explore.run(globals())
    print("l", l, "phase", ph, "done at", round(time.time() - T0, 1), file=sys.stderr)
