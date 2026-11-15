#!/usr/bin/env python3
"""Measured runs (see PROTOCOL.md).  Usage: run.py OUT.jsonl DEADLINE_S L [a:b of the 12 targets]"""
import sys, os, json, time, random
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import sub, dr4

sub.CMS_OPTS.update({"autodisablegauss": "0", "sls": "0"})
OUT, DEADLINE, l = sys.argv[1], float(sys.argv[2]), int(sys.argv[3])
SL = sys.argv[4] if len(sys.argv) > 4 else "0:12"
SEED0 = int(os.environ.get("DR4_SEED0", "2026111500"))
T0 = time.time()
LEAFCAP, COLDCAP = 200000, 20000
half = (1 << (2 * l)) // 2
# Deviation (see README): at l = 6 the free cap is 20x instead of 100x and the dual2 cold sample is
# 4 leaves instead of 8, set after the first l = 6 process was killed externally (time budget).
FREECAP = (20 if (l >= 6 and os.environ.get("DR4_FULLCAP") != "1") else 100) * half
COLD2 = 4 if (l >= 6 and os.environ.get("DR4_FULLCAP") != "1") else 8
seed = SEED0 + l
cell = dr4.Cell4(l, seed)
non, dec = cell.targets(8, 4)
tg = [(i, R, False) for i, R in enumerate(non)] + [(8 + i, R, True) for i, R in enumerate(dec)]
a, b = [int(x) for x in SL.split(":")]
tg = tg[a:b]
DONE = set()
if os.path.exists(OUT):       # resume after an external kill: keep finished rows, redo the rest
    keepl = [x for x in open(OUT) if x.endswith("\n") and json.loads(x)["status"] == "OK"]
    DONE = set((json.loads(x)["idx"], json.loads(x)["arm"]) for x in keepl)
    open(OUT, "w").writelines(keepl)
fout = open(OUT, "a")
ctx = {}
for idx, R, isdec in tg:
    assert cell.decomposable(R) == isdec
    S = dr4.System4(cell, R)
    ctx[idx] = (S, {e: S.render(e) for e in ("dual", "dual2", "plain")})


def emit(row):
    row.pop("model", None)
    fout.write(json.dumps(row) + "\n")
    fout.flush()


def base(idx, R, isdec, arm):
    return {"l": l, "n": cell.n, "seed": seed, "B": cell.B, "poly": cell.F.poly, "idx": idx,
            "R": list(R), "decomposable": isdec, "arm": arm, "base_points": len(cell.base)}


def guard(row):
    if (row["idx"], row["arm"]) in DONE:
        return True
    if time.time() - T0 > DEADLINE:
        row["status"] = "NOT_RUN"
        emit(row)
        return True
    return False


def check(S, st, R, isdec):
    if st.get("censored"):
        return
    assert st["sat"] == isdec, (st["sat"], isdec)
    if st["sat"]:
        assert cell.lifts(S.x_of(st["model"]), R)


def dfs(idx, R, isdec, arm, enc, order, bud, keep):
    S, rd = ctx[idx]
    row = base(idx, R, isdec, arm)
    row.update({"enc": enc, "order": order, "budget": bud, "keep": keep})
    if guard(row):
        return
    t = time.time()
    st = dr4.cms_dfs4(S, rd[enc], dr4.order_of(l, order), bud, keep)
    check(S, st, R, isdec)
    st.update(row)
    st.update({"status": "OK", "time": time.time() - t})
    emit(st)


def core(idx, R, isdec):
    S, rd = ctx[idx]
    row = base(idx, R, isdec, "core")
    row.update({"enc": "dual"})
    if guard(row):
        return
    t = time.time()
    st = dr4.core_scan(S, rd["dual"])
    check(S, st, R, isdec)
    st.update(row)
    st.update({"status": "OK", "time": time.time() - t})
    emit(st)


def free(idx, R, isdec, arm, enc):
    S, rd = ctx[idx]
    row = base(idx, R, isdec, arm)
    row.update({"enc": enc, "cap": FREECAP})
    if guard(row):
        return
    t = time.time()
    sat, model, cf = dr4.CMS(rd[enc]).solve([], FREECAP)
    st = {"sat": sat, "model": model, "censored": sat is None, "conflicts": cf, "leaves": 0}
    check(S, st, R, isdec)
    st.update(row)
    st.update({"status": "OK", "time": time.time() - t})
    emit(st)


def cold(idx, R, enc, nleaf):
    S, rd = ctx[idx]
    row = base(idx, R, False, "cold-" + enc)
    row.update({"enc": enc})
    if guard(row):
        return
    pairs = [(x1, x2) for x1 in range(1, 1 << l) for x2 in range(x1, 1 << l)]
    random.Random(seed * 17 + idx).shuffle(pairs)
    pairs = pairs[:nleaf]
    t = time.time()
    cfs, cens = [], 0
    for x1, x2 in pairs:
        sat, _, cf = dr4.CMS(rd[enc]).solve(S.assump(x1, x2), COLDCAP)
        assert sat is not True
        cens += sat is None
        cfs.append(cf)
    row.update({"status": "OK", "leaves": len(pairs), "conflicts": sum(cfs), "per_leaf": cfs,
                "censored": cens, "time": time.time() - t})
    emit(row)


PH = ["B0", "B3", "B10", "core", "free", "cold", "B100", "var", "disc"]
for ph in PH:
    for idx, R, isdec in tg:
        if ph in ("B0", "B3", "B10", "B100"):
            dfs(idx, R, isdec, "keep-" + ph, "dual", "il", int(ph[1:]), True)
        elif ph == "core":
            core(idx, R, isdec)
        elif ph == "free":
            free(idx, R, isdec, "free", "dual")
        elif ph == "cold" and not isdec:
            cold(idx, R, "dual", 16)
            cold(idx, R, "dual2", COLD2)
        elif ph == "var" and not isdec:
            dfs(idx, R, isdec, "keep-B0-blk", "dual", "blk", 0, True)
            dfs(idx, R, isdec, "keep-B10-blk", "dual", "blk", 10, True)
            dfs(idx, R, isdec, "keep-B0-d2", "dual2", "il", 0, True)
            dfs(idx, R, isdec, "keep-B10-d2", "dual2", "il", 10, True)
            free(idx, R, isdec, "free-d2", "dual2")
            free(idx, R, isdec, "free-plain", "plain")
        elif ph == "disc" and not isdec and l <= 4:
            for bud in ((0, 3, 10, 100) if l <= 3 else (0, 10)):
                dfs(idx, R, isdec, "disc-B%d" % bud, "dual", "il", bud, False)
    print("l", l, SL, "phase", ph, "done at", round(time.time() - T0, 1), file=sys.stderr)
