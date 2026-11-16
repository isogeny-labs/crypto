#!/usr/bin/env python3
"""Combinatorial model of KSumTable::extend (src/cryptanalysis/ic_boundary.rs, commit 6d6aa93b8f), written by
the validator (VAL-20261115-276156) to check the pruning bound independently of group arithmetic.

The model enumerates exactly what the Rust recursion visits: at depth d it loops over abscissae ai in
[from, end), with end = len - (k - d - 1) under prune and len otherwise; depth 0 takes one sign per abscissa,
deeper levels both signs; every partial at depth >= 1 costs one add; depth k-1 inserts. (Identity partials,
which the Rust code skips together with their subtree, do not occur in any recorded build: every recorded
build matches the closed form exactly; see own-numbers.json ksum_build_audit.)

It checks, for every (a, k) of the unit test and of the study:
  1. the pruned enumeration inserts exactly the same packed index tuples, in the same order;
  2. its add count equals sum_{j=2..k} 2^(j-1) C(a-(k-j), j), the unpruned one sum 2^(j-1) C(a, j);
  3. every add the pruned build drops belongs to a partial whose subtree has no insert (dead end), and
     every partial the pruned build keeps has at least one insert below it;
  4. mutation: the bound shifted by -1 (more aggressive) loses inserts, shifted by +1 (more lenient) spends
     more adds than the unit test's upper bound allows, so the test pruned_build_is_the_same_table_with_fewer_adds
     distinguishes the exact bound from both neighbours.
usage: python3 -I prune_model.py <out.json>
"""
import json, math, sys


def enumerate_build(a, k, mode):
    inserts, adds = [], 0
    dead_adds = 0  # adds whose subtree holds no insert

    def end_of(depth):
        if mode is None:
            return a
        return max(0, a - (k - depth - 1) + mode)  # mode 0 = the commit's bound; -1 / +1 = mutations

    def rec(depth, start, idx):
        nonlocal adds, dead_adds
        produced = 0
        for ai in range(start, min(a, end_of(depth))):
            for sign in ((0,) if depth == 0 else (0, 1)):
                if depth > 0:
                    adds += 1
                cur = idx + [(ai, sign)]
                if depth == k - 1:
                    inserts.append(tuple(cur)); n = 1
                else:
                    n = rec(depth + 1, ai + 1, cur)
                if depth > 0 and n == 0:
                    dead_adds += 1
                produced += n
        return produced

    rec(0, 0, [])
    return inserts, adds, dead_adds


def formula(a, k, prune):
    return sum((1 << (j - 1)) * math.comb(a - (k - j) if prune else a, j) for j in range(2, k + 1))


def main():
    cases = {"unit_test": [(14, 3), (12, 4), (11, 5), (12, 6), (11, 7)],
             "study": [(7, 4), (7, 5), (10, 4), (9, 5), (12, 4), (10, 5), (14, 4), (11, 5), (16, 4), (12, 5)]}
    out = {"cases": {}, "all_ok": True}
    for group, lst in cases.items():
        for a, k in lst:
            full_i, full_a, full_dead = enumerate_build(a, k, None)
            pr_i, pr_a, pr_dead = enumerate_build(a, k, 0)
            agg_i, agg_a, _ = enumerate_build(a, k, -1)
            len_i, len_a, _ = enumerate_build(a, k, +1)
            r = {"a": a, "k": k, "inserts": len(full_i), "expected_inserts": (1 << (k - 1)) * math.comb(a, k),
                 "adds_unpruned": full_a, "formula_unpruned": formula(a, k, False),
                 "adds_pruned": pr_a, "formula_pruned": formula(a, k, True),
                 "dead_end_adds_unpruned": full_dead, "dead_end_adds_pruned": pr_dead,
                 "same_inserts_same_order": full_i == pr_i,
                 "saved_equals_dead_ends": full_a - pr_a == full_dead,
                 "mutation_minus1_loses_inserts": len(agg_i) < len(full_i),
                 "mutation_plus1_inserts_equal": len_i == full_i, "mutation_plus1_adds": len_a,
                 "mutation_plus1_exceeds_test_bound": len_a > formula(a, k, True)}
            r["ok"] = (r["inserts"] == r["expected_inserts"] and full_a == r["formula_unpruned"]
                       and pr_a == r["formula_pruned"] and pr_dead == 0 and r["same_inserts_same_order"]
                       and r["saved_equals_dead_ends"] and r["mutation_minus1_loses_inserts"]
                       and r["mutation_plus1_exceeds_test_bound"])
            out["all_ok"] &= r["ok"]
            out["cases"]["%s:a=%d,k=%d" % (group, a, k)] = r
    json.dump(out, open(sys.argv[1], "w"), indent=1, sort_keys=True)
    print("all_ok", out["all_ok"])


if __name__ == "__main__":
    main()
