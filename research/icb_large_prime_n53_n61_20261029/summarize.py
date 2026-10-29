"""Summarise the large-prime ladder (lead LEAD-ICB-large-prime-variants), n = 53.

Reads sessions/*/records.jsonl and band/*.json (ecbench band output).  For every arm: runs,
verified, mean S.  For every large-prime arm: the paired geometric mean of S(arm) / S(control)
over (session, workload, round) against its equal-envelope plain control and against the best plain
table, with a 95% bootstrap interval over workloads.  Priced ratios (weights 0, 0.1, 1) come from the
band files: ratio of arm means per session and pooled.  Writes results.json.  Standard library only.
"""
import json
import math
import pathlib
import random
import statistics

HERE = pathlib.Path(__file__).parent
CONTROL = {"lp2-k20-c10": "plain-k220", "lp2-k80-c3": "plain-k320", "lp2-k40-c10": "plain-k440",
           "lp2-k20-c30": "plain-k660", "lp2-k40-c3": "plain-k220",
           "lp1-k20-c10": "plain-k220", "lp1-k40-c10": "plain-k440", "lp0-k440": "plain-k440",
           "plain-k440-aa": "plain-k440", "lp2-k60-c10": "plain-k660", "lp2-k40-c20": "plain-k880"}
REF = "rho-strong"
BOOT = 20000


GROUPS = {"n53": ("n53-s*", "n53-all"), "n61a": ("n61-s1", "n61-s1"), "n61b": ("n61-s2", "n61-s2")}


def load(group):
    recs = {}
    for path in sorted((HERE / "sessions").glob(f"{GROUPS[group][0]}/records.jsonl")):
        sess = path.parent.name
        for line in path.read_text().splitlines():
            if not line.strip():
                continue
            r = json.loads(line)
            if r["warmup"]:
                continue
            key = (sess, r["workload"]["target_index"], r["round"])
            recs.setdefault(r["arm"], {})[key] = (r["cost"]["s"], r["outcome"]["status"] == "verified")
    return recs


def geomean(xs):
    return math.exp(sum(math.log(x) for x in xs) / len(xs))


def paired(recs, a, b, rng):
    ka = recs.get(a, {})
    kb = recs.get(b, {})
    keys = [k for k in ka if k in kb and ka[k][1] and kb[k][1]]
    if not keys:
        return None
    ratios = {k: ka[k][0] / kb[k][0] for k in keys}
    blocks = {}
    for k, v in ratios.items():
        blocks.setdefault(k[:2], []).append(v)
    names = list(blocks)
    logs = {n: sum(math.log(v) for v in blocks[n]) / len(blocks[n]) for n in names}
    mean = math.exp(sum(math.log(v) for v in ratios.values()) / len(ratios))
    boots = []
    for _ in range(BOOT):
        s = [logs[rng.choice(names)] for _ in names]
        boots.append(math.exp(sum(s) / len(s)))
    boots.sort()
    return {"pairs": len(keys), "workloads": len(names), "ratio": mean,
            "ci95": [boots[int(0.025 * BOOT)], boots[int(0.975 * BOOT) - 1]]}


def analyse(group):
    recs = load(group)
    rng = random.Random(20261029)
    rows = {}
    for arm, d in recs.items():
        vals = [v[0] for v in d.values()]
        rows[arm] = {"runs": len(d), "verified": sum(1 for v in d.values() if v[1]),
                     "mean_s": statistics.mean(vals), "median_s": statistics.median(vals)}
    plains = {a: r for a, r in rows.items() if a.startswith("plain-k") and a != "plain-k440-aa"}
    best_plain = min(plains, key=lambda a: plains[a]["mean_s"]) if plains else None
    pairs = {}
    for arm, ctl in CONTROL.items():
        if arm in recs and ctl in recs:
            pairs[f"{arm}/vs/{ctl}"] = paired(recs, arm, ctl, rng)
    if best_plain:
        for arm in recs:
            if arm.startswith("lp") and arm != "lp0-k440":
                pairs[f"{arm}/vs/best-plain:{best_plain}"] = paired(recs, arm, best_plain, rng)
    if REF in recs:
        for arm in recs:
            if arm != REF:
                pairs[f"{arm}/vs/{REF}"] = paired(recs, arm, REF, rng)
    band = {}
    for path in sorted((HERE / "band").glob("*.json")):
        d = json.loads(path.read_text())
        band[path.stem] = {r["arm"]: r["mean_s"] for r in d["rows"]}
    priced = {}
    pooled = band.get(GROUPS[group][1], {})
    for arm in pooled:
        if arm == REF:
            continue
        priced[arm] = {"vs_rho": [pooled[arm][i] / pooled[REF][i] for i in range(3)]}
        ctl = CONTROL.get(arm)
        if ctl in pooled:
            priced[arm]["vs_control"] = [pooled[arm][i] / pooled[ctl][i] for i in range(3)]
        if best_plain in pooled:
            priced[arm]["vs_best_plain"] = [pooled[arm][i] / pooled[best_plain][i] for i in range(3)]
    sessions = sorted(k for k in band if k.startswith("n53-s")) if group == "n53" else []
    per_session = {}
    for arm in pooled:
        ctl = CONTROL.get(arm)
        if arm != REF and ctl in pooled:
            per_session[arm] = {s: [band[s][arm][i] / band[s][ctl][i] for i in range(3)]
                                for s in sessions if arm in band[s] and ctl in band[s]}
    headline = []
    for arm, r in rows.items():
        headline.append({"name": f"{group}/{arm}/mean_s", "claimed": r["mean_s"]})
    for name, p in pairs.items():
        if p:
            headline.append({"name": f"{group}/{name}/paired_ratio", "claimed": p["ratio"]})
    out = {"best_plain": best_plain, "rows": rows, "pairs": pairs, "priced": priced,
           "per_session_priced_vs_control": per_session, "headline": headline}
    print("==", group, "best plain:", best_plain)
    for arm in sorted(rows):
        r = rows[arm]
        extra = ""
        c = CONTROL.get(arm)
        p = pairs.get(f"{arm}/vs/{c}") if c else None
        if p:
            extra = f" vs {c}: {p['ratio']:.3f} [{p['ci95'][0]:.3f}, {p['ci95'][1]:.3f}]"
        print(f"{arm:14s} runs={r['runs']:3d} ver={r['verified']:3d} S={r['mean_s']:8.3f}{extra}")
    return out


def main():
    groups = {g: analyse(g) for g in GROUPS}
    headline = [h for g in groups.values() for h in g["headline"]]
    (HERE / "results.json").write_text(json.dumps({"groups": groups, "headline": headline}, indent=1, sort_keys=True) + "\n")


if __name__ == "__main__":
    main()
