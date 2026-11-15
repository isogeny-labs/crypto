#!/usr/bin/env python3
"""Summary and gates for slice S4 (PLAN.md §10).

    python3 summarise_s4.py OUT_DIR [--gates-only]

Reads OUT_DIR/count.txt (count-ops build, verification on) and
OUT_DIR/time.txt (timing build, verification off).
- G10: every sieve relation verified (relations_verified == relations) and
  every k correct, both arms, count build.
- G11: per (arm, row, seed), count and timing lines equal apart from the
  `*_ms`, `ops_*` and `relations_verified` fields.
Primary statistics, per row, as ratios of per-arm medians over seeds:
  P1 = walk/sieve field products per relation, relation phase;
  P2 = the same for Fp::inv;
  P3 = walk/sieve field products over relation phase + solve.
Bar rows: every genus-3 row, genus-4 p >= 127.  E13: P1 >= 5; E14: P2 >= 10;
E15: P3 >= 2 on bar rows with p <= 1021; E16: at (3, 4001) the sieve's
median solve mulmods exceed the walk's.  Writes summary.md and summary.json.
"""
import json
import statistics as st
import sys
from collections import defaultdict


def load(path):
    out = {}
    for line in open(path):
        if line.startswith("JSON "):
            j = json.loads(line[5:])
            out[(j["method"], j["genus"], j["p"], j["seed"])] = j
    return out


def strip(j):
    return {k: v for k, v in j.items()
            if not (k.endswith("_ms") or k.startswith("ops_") or k == "relations_verified")}


def main():
    d = sys.argv[1]
    gates_only = "--gates-only" in sys.argv
    C, T = load(f"{d}/count.txt"), load(f"{d}/time.txt")
    fails = []
    for key, j in C.items():
        if not j["correct"]:
            fails.append(f"G10 k wrong {key}")
        if j["method"] == "sieve" and j["relations_verified"] != j["relations"]:
            fails.append(f"G10 unverified relations {key}")
        if key not in T:
            fails.append(f"G11 missing timing line {key}")
        elif strip(j) != strip(T[key]):
            diff = [k for k in strip(j) if strip(j)[k] != strip(T[key]).get(k)]
            fails.append(f"G11 differs {key}: {diff}")
    for key in T:
        if key not in C:
            fails.append(f"G11 missing count line {key}")
        if not T[key]["correct"]:
            fails.append(f"G10 k wrong (timing) {key}")
    print(f"G10/G11: {len(C)} count lines, {len(T)} timing lines, {len(fails)} failures")
    for f in fails[:20]:
        print("  ", f)
    if gates_only:
        sys.exit(1 if fails else 0)

    rows = defaultdict(lambda: defaultdict(list))
    for (meth, g, p, _), j in C.items():
        rows[(g, p)][meth].append(j)
    timing = defaultdict(lambda: defaultdict(list))
    for (meth, g, p, _), j in T.items():
        timing[(g, p)][meth].append(j)
    med = lambda xs: st.median(xs)
    rng = lambda xs: (min(xs), max(xs))
    out, verdict = [], {}
    hdr = ("| g | p | m | walk prod/rel | sieve prod/rel | **P1** | walk inv/rel | sieve inv/rel | **P2** "
           "| walk prod rel+solve | sieve prod rel+solve | **P3** | solve mulmods walk / sieve "
           "| sieve rel/line (p/(g+1)!) | line adds/rel | walk trials/rel | walk prod/trial "
           "| verify prod/rel (excluded) | total ms walk / sieve (timing build) | k ok |")
    out.append(hdr)
    out.append("|" + "--:|" * 19 + ":-:|")
    for (g, p), dd in sorted(rows.items()):
        w, s = dd["walk"], dd["sieve"]
        per = lambda xs, f: [f(x) for x in xs]
        wp = per(w, lambda x: x["ops_rel_products"] / x["relations"])
        sp = per(s, lambda x: x["ops_rel_products"] / x["relations"])
        wi = per(w, lambda x: x["ops_rel_inv"] / x["relations"])
        si = per(s, lambda x: x["ops_rel_inv"] / x["relations"])
        wt = per(w, lambda x: x["ops_rel_products"] + x["ops_solve_products"])
        stt = per(s, lambda x: x["ops_rel_products"] + x["ops_solve_products"])
        P1, P2, P3 = med(wp) / med(sp), med(wi) / med(si), med(wt) / med(stt)
        wsm, ssm = med(per(w, lambda x: x["solve_mulmods"])), med(per(s, lambda x: x["solve_mulmods"]))
        bar = g == 3 or (g == 4 and p >= 127)
        verdict[f"{g}:{p}"] = dict(
            g=g, p=p, m=s[0]["m"], bar=bar, P1=P1, P2=P2, P3=P3,
            walk_prod_rel=med(wp), sieve_prod_rel=med(sp), walk_prod_rel_range=rng(wp), sieve_prod_rel_range=rng(sp),
            walk_inv_rel=med(wi), sieve_inv_rel=med(si),
            walk_total=med(wt), sieve_total=med(stt), walk_total_range=rng(wt), sieve_total_range=rng(stt),
            walk_solve_mulmods=wsm, sieve_solve_mulmods=ssm,
            E13=(P1 >= 5) if bar else None, E14=(P2 >= 10) if bar else None,
            E15=(P3 >= 2) if (bar and p <= 1021) else None,
            E16=(ssm > wsm) if (g, p) == (3, 4001) else None,
        )
        tw, ts = timing[(g, p)]["walk"], timing[(g, p)]["sieve"]
        out.append(
            f"| {g} | {p} | {s[0]['m']} | {med(wp):.0f} | {med(sp):.0f} | **{P1:.1f}** "
            f"| {med(wi):.2f} | {med(si):.3f} | **{P2:.0f}** "
            f"| {med(wt):.3g} | {med(stt):.3g} | **{P3:.2f}** | {wsm:.3g} / {ssm:.3g} "
            f"| {med(per(s, lambda x: x['rel_per_line'])):.2f} ({s[0]['expected_rel_per_line']:.2f}) "
            f"| {med(per(s, lambda x: x['line_adds_per_rel'])):.3f} "
            f"| {med(per(w, lambda x: x['trials'] / x['relations'])):.2f} "
            f"| {med(per(w, lambda x: x['ops_rel_products'] / x['trials'])):.0f} "
            f"| {med(per(s, lambda x: x['ops_verify_products'] / x['relations'])):.0f} "
            f"| {med([x['total_ms'] for x in tw]):.2f} / {med([x['total_ms'] for x in ts]):.2f} "
            f"| {'yes' if all(x['correct'] for x in w + s) else 'NO'} |"
        )
    out.append("")
    for e in ("E13", "E14", "E15", "E16"):
        rel = {k: v[e] for k, v in verdict.items() if v[e] is not None}
        out.append(f"- **{e}**: {'holds' if all(rel.values()) else 'FALSIFIED'} "
                   f"({sum(rel.values())}/{len(rel)} rows; misses: {[k for k, v in rel.items() if not v] or 'none'})")
    out.append(f"- **G10/G11**: {'hold' if not fails else 'FAIL'} ({len(C)} count lines, {len(T)} timing lines)")
    open(f"{d}/summary.md", "w").write("\n".join(out) + "\n")
    json.dump(dict(rows=verdict, gate_failures=fails), open(f"{d}/summary.json", "w"), indent=1)
    print("\n".join(out))


main()
