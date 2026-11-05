#!/usr/bin/env python3
"""How much of an arm's band-priced S is a byte-valued `*_uncharged` counter.

Read-only audit.  For each hydrated study (sessions/*/records.jsonl plus the
committed analysis/band-*.json) it mirrors the unpriced-count lookup of
src/cryptanalysis/ecbench/band.rs (`unpriced_counts`: record counters first,
then each phase's native map as `X_uncharged` or `X`), keeps the unpriced
counters whose name contains "bytes", and reports

  * the byte counter's share of the published mean S at each weight,
  * S and the ratio to the matched rho with the counter excluded,
  * the same under a per-byte weight (variant C below),
  * for each residual arm vs `plan`, the order across the band as published
    and with the counter excluded.

Published mean S is read from the band JSON, so only the byte counter's own
contribution (w * bytes / sqrt(r), averaged over verified non-warmup runs) is
recomputed.  Ratios are ratios of means, not the paired bootstrap of
`ecbench compare`.

Variant C: bytes priced at 1/8 operation per byte (one word store per eight
bytes, the rule full_price.rs applies to `factor_base_vec_bytes_floor`), the
operation then charged at the weight w like every other unpriced operation.

Usage: byte_share.py STUDY_DIR [STUDY_DIR ...] --out OUT_DIR
"""
import argparse, json, math, os
from collections import defaultdict

PER_BYTE_OPS = 1 / 8  # variant C


def unpriced_counts(r):
    out = {}
    for name in r["cost"]["unpriced"]:
        base = name[: -len("_uncharged")] if name.endswith("_uncharged") else name
        if name in r["counters"]:
            out[name] = r["counters"][name]
            continue
        tot, found = 0, False
        for p in r["phases"]:
            nat = p.get("native", {})
            v = nat.get(name, nat.get(base))
            if v is not None:
                tot += v
                found = True
        if not found and base.startswith("solver_") and r.get("solver"):
            out[name] = r["solver"]["ops"]
            continue
        out[name] = tot if found else None
    return out


def order(sa, sb):
    o = [x < y for x, y in zip(sa, sb)]
    if all(o):
        return "arm cheaper at every w"
    if not any(o):
        return "plan cheaper at every w"
    return "unresolved: " + "".join("a" if v else "p" for v in o)


def study_rows(study):
    rows = []
    for sess in sorted(os.listdir(os.path.join(study, "sessions"))):
        rec = os.path.join(study, "sessions", sess, "records.jsonl")
        bj = os.path.join(study, "analysis", f"band-{sess}.json")
        if not (os.path.isfile(rec) and os.path.isfile(bj)):
            continue
        band = json.load(open(bj))
        ws = band["weights"]
        pub = {(x["curve"], x["arm"]): x for x in band["rows"]}
        acc = defaultdict(lambda: {"n": 0, "B": 0.0, "names": set()})
        for line in open(rec):
            r = json.loads(line)
            if r.get("warmup") or r["outcome"]["status"] != "verified":
                continue
            sq = math.sqrt(r["workload"]["curve"]["r"])
            a = acc[(r["workload"]["curve"]["slug"], r["arm"])]
            a["n"] += 1
            for k, v in unpriced_counts(r).items():
                if v and "bytes" in k:
                    a["B"] += v / sq
                    a["names"].add(k)
        for (curve, arm), p in pub.items():
            a = acc.get((curve, arm))
            if a is None or a["n"] == 0 or None in p["mean_s"]:
                continue
            rows.append(dict(study=os.path.basename(study.rstrip("/")), session=sess,
                             curve=curve, arm=arm, n=a["n"], weights=ws,
                             mean_s=p["mean_s"], bytes_s=a["B"] / a["n"],
                             byte_counters=sorted(a["names"])))
    return rows


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("studies", nargs="+")
    ap.add_argument("--out", required=True)
    args = ap.parse_args()
    os.makedirs(args.out, exist_ok=True)
    allrows = []
    for st in args.studies:
        allrows += study_rows(st)
    by = {(x["study"], x["session"], x["curve"], x["arm"]): x for x in allrows}
    lines = []
    flips = defaultdict(lambda: [0, 0, 0])  # [total, unresolved as published, unresolved after]
    out = []
    for x in allrows:
        ws, ms, B = x["weights"], x["mean_s"], x["bytes_s"]
        ex = [m - w * B for m, w in zip(ms, ws)]
        cc = [m - w * B + w * B * PER_BYTE_OPS for m, w in zip(ms, ws)]
        i01, i1 = ws.index(0.1), ws.index(1.0)
        rho = by.get((x["study"], x["session"], x["curve"], "rho"))
        rec = dict(x, share_w01=(0.1 * B / ms[i01]) if B else 0.0,
                   share_w1=(B / ms[i1]) if B else 0.0, s_excluded=ex, s_variant_c=cc)
        if rho and x["arm"] != "rho":
            r = rho["mean_s"]
            rec["ratio_to_rho"] = {
                "published": [m / q for m, q in zip(ms, r)],
                "excluded": [m / q for m, q in zip(ex, r)],
                "variant_c": [m / q for m, q in zip(cc, r)],
            }
        plan = by.get((x["study"], x["session"], x["curve"], "plan"))
        if B > 0 and plan and x["arm"].startswith("res") and x["arm"] != "resN0":
            rec["vs_plan"] = {"published": order(ms, plan["mean_s"]),
                              "excluded": order(ex, plan["mean_s"]),
                              "variant_c": order(cc, plan["mean_s"])}
        out.append(rec)
        if B > 0:
            rr = rec.get("ratio_to_rho", {})
            f = lambda k: (f"{rr[k][i1]:6.2f}x/{rr[k][i01]:6.2f}x" if rr else "")
            lines.append(
                f"{x['session']:7s} {x['curve'][:22]:22s} {x['arm']:7s} n={x['n']:3d} "
                f"share w=.1 {rec['share_w01']*100:5.1f}% w=1 {rec['share_w1']*100:5.1f}% | "
                f"S(w=1) pub {ms[i1]:9.2f} excl {ex[i1]:8.2f} C {cc[i1]:8.2f} | "
                f"x rho at w=1/w=.1: pub {f('published')} excl {f('excluded')} C {f('variant_c')} "
                f"| {','.join(x['byte_counters'])}")
    json.dump(out, open(os.path.join(args.out, "byte_share.json"), "w"), indent=1)
    cnt = defaultdict(int)
    for r in out:
        if "vs_plan" in r:
            cnt["compared"] += 1
            for k in ("published", "excluded", "variant_c"):
                if r["vs_plan"][k].startswith("unresolved"):
                    cnt[f"unresolved_{k}"] += 1
    lines.append("")
    lines.append(f"residual arm vs plan comparisons (mean S, byte-bearing arms, resN0 excluded): {dict(cnt)}")
    open(os.path.join(args.out, "byte_share.txt"), "w").write("\n".join(lines) + "\n")
    print("\n".join(lines))


if __name__ == "__main__":
    main()
