#!/usr/bin/env python3
"""Exploratory arm (5), chosen after the l = 6, 7 census (see PROTOCOL.md addendum).  No rule.

The census shows the events are almost all dimension-2 nodes refuted by their own budgeted call, after
failed attempts at every ancestor.  Variants here make refutation attempts only near the leaves, so
that the ancestors' failed attempts (about 1.1 conflicts per leaf at budget 3) are not paid:
  j2only : budget 3 at nodes of dimension 2 only; no call at any other internal node
  jle2   : budget 3 at dimensions 1 and 2
  jle3   : budget 3 at dimensions 1, 2 and 3
Run through cmeasure.py (phase `explore`)."""
import time
import lib

VARIANTS = {"j2only": (2,), "jle2": (1, 2), "jle3": (1, 2, 3), "coreskip": ()}
# coreskip: the budget-0 leaf scan, except that a leaf lying inside the final-conflict cube of an earlier
# leaf refutation is not submitted to the solver at all (no leaf unit, no conflicts; bookkeeping only).


def sched_dfs(sysm, solver, order, budget, dims, cvars):
    """lib.rec_dfs with a call only at leaves and at internal nodes whose dimension is in dims."""
    l = sysm.l
    cv = cvars
    nodes = []
    pos = {cv[order[i]]: i for i in range(l)}
    stats = {"calls": 0, "leaves": 0, "conflicts": 0, "sat": False, "model": None, "skipped": 0}
    skip = len(dims) == 0
    cubes = []

    def rec(assump, k, prefix):
        j = l - k
        sat, model, cf = None, None, 0
        if j == 0 and skip:
            for cm, vm in cubes:
                if (prefix & cm) == vm:
                    stats["skipped"] += 1
                    nodes.append([k, prefix, 0, 0, cm, 0, 0])
                    return False, 0, 0
        if j == 0 or j in dims:
            stats["calls"] += 1
            sat, model, cf = solver.solve(assump, None if j == 0 else budget)
            stats["conflicts"] += cf
        lv = 0
        if j == 0:
            stats["leaves"] += 1
            lv = 1
        if sat is True:
            stats["sat"], stats["model"] = True, model
            nodes.append([k, prefix, 1, cf, -1, lv, cf])
            return True, lv, cf
        if sat is False:
            cl = 0
            for lit in solver.s.get_conflict():
                cl |= 1 << pos[abs(lit)]
            nodes.append([k, prefix, 0, cf, cl, lv, cf])
            if skip:
                cubes.append((cl, prefix & cl))
            return False, lv, cf
        v = cv[order[k]]
        tl, tc, found = 0, cf, False
        for b in (0, 1):
            f, a, c = rec(assump + [v if b else -v], k + 1, prefix | (b << k))
            tl += a
            tc += c
            if f:
                found = True
                break
        nodes.append([k, prefix, 2, cf, -1, tl, tc])
        return found, tl, tc

    rec([], 0, 0)
    return stats, nodes


def run(g):
    l, cell, seed, std = g["l"], g["cell"], g["seed"], g["std"]
    for idx, R in g["TG"]:
        S, rd = g["get"](idx, R)
        for rep in (0, 1):
            if rep == 0:
                r2, cv = rd, S.c
            else:
                r2, perm = lib.shuffle_rendered(rd, seed * 31 + 7 * idx + rep)
                cv = [perm[v] for v in S.c]
            for var, dims in VARIANTS.items():
                row = g["base"](idx, R, False, "explore")
                row.update({"budget": 0 if var == "coreskip" else 3, "rep": rep, "order": std, "variant": var})
                if g["late"]():
                    row["status"] = "NOT_RUN"
                    g["emit"](row)
                    continue
                t = time.time()
                st, nodes = sched_dfs(S, lib.CMS(r2), std, 3, dims, cv)
                assert st["sat"] is False
                st.update(row)
                st.update({"status": "OK", "time": time.time() - t, "nodes": nodes})
                g["emit"](st)
