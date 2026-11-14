#!/usr/bin/env python3
"""Validation on the smallest cells (see PROTOCOL.md). Usage: validate.py [l ...]"""
import sys, json, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import sub

sub.CMS_OPTS.update({"autodisablegauss": "0", "sls": "0"})
out = {}
for l in [int(a) for a in sys.argv[1:]] or [3, 4]:
    cell = sub.Cell(l, 2026111400 + l)
    tg = [cell.random_target() for _ in range(24)] + [cell.planted_target() for _ in range(8)]
    E = cell.E
    r = {"targets": len(tg), "decomposable": 0, "models_lifted": 0, "leaf_checks": 0}
    for i, R in enumerate(tg):
        truth = cell.decomposable(R)
        if l <= 4:
            assert cell.brute(R) == truth
        assert i < 24 or truth
        r["decomposable"] += truth
        # per-x1 truth: x1 usable iff R -+ P1 is a sum of two signed base points
        pairs = set(E.add(a, b) for a in cell.signed for b in cell.signed)
        x1ok = set(P[0] for P in cell.signed if E.add(R, P) in pairs)
        for basis in (None, cell.alt_basis(cell.seed * 7 + 1)):
            S = sub.System(cell, R, basis)
            for enc in ("plain", "dual"):
                rd = S.render(enc)
                sat, model, _ = sub.CMS(rd).solve()
                assert sat == truth, (l, i, enc)
                if sat:
                    assert cell.lifts(S.x_of(model), R)
                    r["models_lifted"] += 1
            st = sub.eng_dfs(S, list(range(l)))
            assert st["sat"] == truth
            if truth:
                assert cell.lifts(S.x_of(st["model"]), R)
                r["models_lifted"] += 1
            rd = S.render("dual")
            for arm in (("abs", 0), ("abs", 10), ("prop", 1)):
                for keep in (True, False):
                    d = sub.cms_dfs(S, rd, list(range(l)), arm, keep)
                    assert d["sat"] == truth
                    if truth:
                        assert cell.lifts(S.x_of(d["model"]), R)
            inc = sub.CMS(rd)
            for x in range(1 << l):
                a = [S.c[k] if (x >> k) & 1 else -S.c[k] for k in range(l)]
                x1 = 0
                for k in range(l):
                    if (x >> k) & 1:
                        x1 ^= S.basis[k]
                sat, model, _ = inc.solve(a)
                assert sat == (x1 in x1ok), (l, i, x)
                e = sub.Eng(S)
                for k in range(l):
                    if not e.conflict:
                        e.assign(S.c[k], (x >> k) & 1)
                m = sub.eng_solve_leafdown(e, {"sub_nodes": 0, "xors": 0, "fix": 0})
                assert (m is not None) == (x1 in x1ok)
                r["leaf_checks"] += 2
    out[str(l)] = r
    print("validated l =", l, r, file=sys.stderr)
json.dump(out, open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "validation.json"), "w"), indent=1)
