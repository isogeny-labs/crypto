#!/usr/bin/env python3
"""Validation on the smallest cells.  Usage: validate.py [l ...] (default 2 3)."""
import sys, os, json, time
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import sub, dr4

sub.CMS_OPTS.update({"autodisablegauss": "0", "sls": "0"})
SEED0 = int(os.environ.get("DR4_SEED0", "2026111500"))
out = {}
for l in [int(a) for a in sys.argv[1:]] or [2, 3]:
    t0 = time.time()
    cell = dr4.Cell4(l, SEED0 + l)
    tg = [cell.random_target() for _ in range(16)] + [cell.planted_target() for _ in range(8)]
    r = {"n": cell.n, "B": cell.B, "base": len(cell.base), "targets": len(tg), "decomposable": 0,
         "brute_checked": 0, "models_lifted": 0, "leaf_checks": 0, "leaf_sat": 0,
         "dfs_runs": 0, "core_runs": 0}
    for i, R in enumerate(tg):
        truth = cell.decomposable(R)
        if l <= 2 or i % 2 == 0:
            assert cell.brute(R) == truth, (l, i)
            r["brute_checked"] += 1
        assert i < 16 or truth
        r["decomposable"] += truth
        S = dr4.System4(cell, R)
        for enc in ("plain", "dual"):
            rd = S.render(enc)
            sat, model, _ = dr4.CMS(rd).solve()
            assert sat == truth, (l, i, enc)
            if sat:
                assert cell.lifts(S.x_of(model), R)
                r["models_lifted"] += 1
        rd = S.render("dual")
        inc = dr4.CMS(rd)
        for x1 in range(1 << l):
            for x2 in range(1 << l):
                sat, model, _ = inc.solve(S.assump(x1, x2))
                want = x1 <= x2 and x1 != 0 and cell.pair_ok(R, x1, x2)
                assert sat == want, (l, i, x1, x2, sat, want)
                r["leaf_checks"] += 1
                if sat:
                    xs = S.x_of(model)
                    assert xs[:2] == (x1, x2) and cell.lifts(xs, R)
                    r["leaf_sat"] += 1
        for kind in ("il", "blk"):
            for bud in (0, 3, 10):
                for keep in (True, False):
                    st = dr4.cms_dfs4(S, rd, dr4.order_of(l, kind), bud, keep)
                    assert st["sat"] == truth
                    if st["sat"]:
                        assert cell.lifts(S.x_of(st["model"]), R)
                        r["models_lifted"] += 1
                    elif bud == 0:
                        assert st["leaves"] == (1 << l) * ((1 << l) - 1) // 2
                    r["dfs_runs"] += 1
        st = dr4.core_scan(S, rd)
        assert st["sat"] == truth
        if st["sat"]:
            assert cell.lifts(S.x_of(st["model"]), R)
            r["models_lifted"] += 1
        r["core_runs"] += 1
    r["seconds"] = round(time.time() - t0, 1)
    out[l] = r
    print(l, r, file=sys.stderr)
json.dump(out, open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "validation.json"), "w"), indent=1)
