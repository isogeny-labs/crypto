#!/usr/bin/env python3
"""Measured runs (see PROTOCOL.md).
Usage: run.py OUT.jsonl DEADLINE_S L [TARGET_SLICE a:b of the 12 targets] [PHASES csv]"""
import sys, os, json, time, random
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import sub

sub.CMS_OPTS.update({"autodisablegauss": "0", "sls": "0"})
OUT, DEADLINE, L = sys.argv[1], float(sys.argv[2]), int(sys.argv[3])
SL = sys.argv[4] if len(sys.argv) > 4 else "0:12"
PH = sys.argv[5].split(",") if len(sys.argv) > 5 else ["Bkeep", "Ainc", "alpha", "Bdisc", "Bctl", "E", "Acold", "Aplain"]
T0 = time.time()
CAP = 20000
BUD = [("abs", 0), ("abs", 10), ("abs", 100), ("abs", 1000), ("prop", 0.25), ("prop", 0.5), ("prop", 1)]
CTL = [("abs", 0), ("abs", 10), ("abs", 100), ("prop", 1)]
DISC8 = [("abs", 0), ("abs", 100), ("prop", 1)]
fout = open(OUT, "a")


def emit(row):
    row.pop("model", None)
    fout.write(json.dumps(row) + "\n")
    fout.flush()


def late():
    return time.time() - T0 > DEADLINE


l = L
seed = 2026111400 + l
cell = sub.Cell(l, seed)
non, dec = cell.targets(8, 4)
tg = [(i, R, False) for i, R in enumerate(non)] + [(8 + i, R, True) for i, R in enumerate(dec)]
a, b = [int(x) for x in SL.split(":")]
tg = tg[a:b]
alt = cell.alt_basis(seed * 7 + 1)
std = list(range(l))
ctx = {}
for idx, R, isdec in tg:
    S = sub.System(cell, R)
    Sa = sub.System(cell, R, alt)
    rnd = std[:]
    random.Random(seed * 13 + idx).shuffle(rnd)
    ctx[idx] = dict(S=S, Sa=Sa, rnd=rnd, dual=S.render("dual"), plain=S.render("plain"),
                    duala=Sa.render("dual"))
    assert cell.decomposable(R) == isdec


def base(idx, R, isdec, part, arm):
    return {"l": l, "n": cell.n, "seed": seed, "B": cell.B, "poly": cell.F.poly, "idx": idx,
            "R": list(R), "decomposable": isdec, "part": part, "arm": arm}


def dfs_row(idx, R, isdec, arm, bud, keep, variant):
    c = ctx[idx]
    row = base(idx, R, isdec, "B", arm)
    row.update({"budget": list(bud), "keep": keep, "variant": variant})
    if late():
        row["status"] = "NOT_RUN"
        return emit(row)
    S, rd, order = {"std": (c["S"], c["dual"], std), "rand": (c["S"], c["dual"], c["rnd"]),
                    "alt": (c["Sa"], c["duala"], std)}[variant]
    t = time.time()
    st = sub.cms_dfs(S, rd, order, bud, keep)
    assert st["sat"] == isdec
    if st["sat"]:
        assert cell.lifts(S.x_of(st["model"]), R)
    st.update(row)
    st.update({"status": "OK", "time": time.time() - t, "order": order})
    emit(st)


def leaf_assump(S, x):
    return [S.c[k] if (x >> k) & 1 else -S.c[k] for k in range(l)]


def partA(idx, R, enc, cold, nleaf):
    c = ctx[idx]
    S = c["S"]
    row = base(idx, R, False, "A", "A-%s-%s" % (enc, "cold" if cold else "inc"))
    if late():
        row["status"] = "NOT_RUN"
        return emit(row)
    xs = list(range(1 << l))
    if nleaf < len(xs):
        random.Random(seed * 17 + idx).shuffle(xs)
        xs = xs[:nleaf]
    t = time.time()
    inc = None if cold else sub.CMS(c[enc])
    cf, cens, mx, zero = 0, 0, 0, 0
    for x in xs:
        s = sub.CMS(c[enc]) if cold else inc
        sat, _, k = s.solve(leaf_assump(S, x), CAP)
        assert sat is not True
        cens += sat is None
        cf += k
        mx = max(mx, k)
        zero += k == 0
    row.update({"status": "OK", "leaves": len(xs), "conflicts": cf, "censored": cens, "max": mx,
                "zero_conflict_leaves": zero, "time": time.time() - t,
                "matrices": (inc.matrices if inc else s.matrices)})
    emit(row)


def alpha(idx, R):
    c = ctx[idx]
    S = c["S"]
    rng = random.Random(seed * 19 + idx)
    stop = False
    for j in range(l + 1):
        k = l - j
        for rep in range(1 if j == l else 2):
            vals = [rng.getrandbits(1) for _ in range(k)]
            row = base(idx, R, False, "alpha", "alpha")
            row.update({"j": j, "rep": rep, "vals": vals})
            if late() or stop:
                row["status"] = "NOT_RUN" if not stop else "SKIPPED_AFTER_CENSOR"
                emit(row)
                continue
            t = time.time()
            sat, _, cf = sub.CMS(c["dual"]).solve([S.c[i] if v else -S.c[i] for i, v in enumerate(vals)], CAP)
            assert sat is not True
            row.update({"status": "OK", "conflicts": cf, "censored": sat is None, "time": time.time() - t})
            emit(row)
            row["_c"] = sat is None
            if rep == 0:
                first_c = sat is None
            if (j == l and sat is None) or (rep == 1 and first_c and sat is None):
                stop = True


def eng(idx, R, isdec, variant):
    c = ctx[idx]
    row = base(idx, R, isdec, "B", "E")
    row.update({"variant": variant})
    if late():
        row["status"] = "NOT_RUN"
        return emit(row)
    S, order = {"std": (c["S"], std), "rand": (c["S"], c["rnd"]), "alt": (c["Sa"], std)}[variant]
    t = time.time()
    st = sub.eng_dfs(S, order)
    assert st["sat"] == isdec
    if st["sat"]:
        assert cell.lifts(S.x_of(st["model"]), R)
    st.update(row)
    st.update({"status": "OK", "time": time.time() - t, "order": order})
    emit(st)


for ph in PH:
    for idx, R, isdec in tg:
        if ph == "Bkeep":
            for bud in BUD:
                dfs_row(idx, R, isdec, "keep", bud, True, "std")
        elif ph == "Ainc" and not isdec:
            partA(idx, R, "dual", False, 1 << l)
        elif ph == "alpha" and not isdec:
            alpha(idx, R)
        elif ph == "Bdisc" and not isdec:
            for bud in (DISC8 if l >= 8 else BUD):
                dfs_row(idx, R, isdec, "discard", bud, False, "std")
        elif ph == "Bctl" and not isdec:
            for v in ("rand", "alt"):
                for bud in CTL:
                    dfs_row(idx, R, isdec, "keep", bud, True, v)
        elif ph == "E":
            for v in (("std",) if isdec else ("std", "rand", "alt")):
                eng(idx, R, isdec, v)
        elif ph == "Acold" and not isdec:
            partA(idx, R, "dual", True, (1 << l) if l <= 6 else 64)
        elif ph == "Aplain" and not isdec:
            partA(idx, R, "plain", False, (1 << l) if l <= 5 else 32)
            partA(idx, R, "plain", True, (1 << l) if l <= 5 else 32)
    print("phase", ph, "done at", round(time.time() - T0, 1), file=sys.stderr)
