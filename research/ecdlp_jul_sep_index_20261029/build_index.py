#!/usr/bin/env python3
"""Dated index of the July-September 2026 ECDLP decisions in the sibling
crypto-autoresearcher ledger. Lays out recorded values only: decision date,
verdict, goal, targets, evidence and the run directories of each experiment.
Nothing is inferred beyond the scope rule below.

Scope rule: a decision is ECDLP-related if its goal_id is in ECDLP_GOALS, or,
when it has no goal_id, if any target or evidence id carries an ECDLP family
token (FAMILIES)."""
import collections, csv, json, os, re, sys
import yaml

AR = os.environ.get("AUTORESEARCHER", os.path.join(os.path.dirname(__file__), "../../../crypto-autoresearcher"))
OUT = os.path.dirname(os.path.abspath(__file__))

ECDLP_GOALS = {
    "GOAL-ECDLP-001", "GOAL-ENDO-001", "GOAL-ECDLP2M-001", "GOAL-AUXIN-a93442",
    "GOAL-GFPN-380702", "GOAL-FIND-001", "GOAL-ECTD-001", "GOAL-ECDLP-bbc21f",
    "GOAL-SEMBIN-5078bc", "GOAL-SEMBIN-fcb7a2", "GOAL-SEMBIN-cbf422",
    "GOAL-MONO-001", "GOAL-SATIC-c49b77", "GOAL-DREG-001", "GOAL-PATH-001",
    "GOAL-ICPERF-e6b6a4", "GOAL-FROB-6333a9", "GOAL-RELN-001", "GOAL-SDEG-001",
}
FAMILIES = {
    "ECDLP", "SEMAEV", "MONO", "DREG", "RELN", "SDEG", "SEMBIN", "ICINV", "JMV",
    "SMTH", "PFDR", "SATIC", "FROB", "ICPERF", "ENDO", "ECDLP2M", "GFPN", "ECTD",
    "AUXIN", "FIND", "PATH", "SYC", "ICB", "ICP", "XEDN", "GHS", "WDSAT", "FFD",
    "LFD", "KUMMER", "PKM", "DESC", "DIEM",
}

def fam(i):
    p = str(i).split("-")
    return p[1] if len(p) > 2 else None

def listify(x):
    if x is None: return []
    if isinstance(x, (list, tuple)): return [str(v) for v in x]
    if isinstance(x, dict): return [str(v) for v in x.values()]
    return [str(x)]

def runs_of(exp):
    d = os.path.join(AR, "experiments", exp, "runs")
    if not os.path.isdir(d): return 0, None
    return len([e for e in os.listdir(d) if not e.startswith(".")]), f"crypto-autoresearcher:experiments/{exp}/runs/"

def main():
    rows, skipped = [], collections.Counter()
    ddir = os.path.join(AR, "ledger", "decisions")
    for f in sorted(os.listdir(ddir)):
        m = re.match(r"DEC-(2026(0[789]))(\d\d)", f)
        if not m: continue
        try: d = yaml.safe_load(open(os.path.join(ddir, f)))
        except Exception: skipped["parse_error"] += 1; continue
        if not isinstance(d, dict): skipped["not_mapping"] += 1; continue
        c = d.get("coordinator_decision", d.get("decision")) 
        if not isinstance(c, dict): skipped["no_decision_block"] += 1; continue
        goal = c.get("goal_id")
        targets = listify(c.get("target_ids"))
        ev = listify(c.get("evidence_ids")) + listify(c.get("evidence_refs"))
        ids = targets + ev
        if goal: in_scope = goal in ECDLP_GOALS
        else: in_scope = any(fam(i) in FAMILIES for i in ids)
        if not in_scope: continue
        exps = sorted({t for t in targets if t.startswith("EXP-")})
        run_paths, nruns = [], 0
        for e in exps:
            n, p = runs_of(e)
            if p: run_paths.append(p); nruns += n
        date = str(c.get("decided_at") or f"{m.group(1)[:4]}-{m.group(1)[4:]}-{m.group(3)}")[:10]
        ctx = " ".join(str(c.get("context") or "").split())
        rows.append({
            "date": date, "month": date[:7], "id": c.get("id") or f[:-5],
            "goal": goal or "", "verdict": str(c.get("decision") or ""),
            "families": sorted({fam(i) for i in ids if fam(i)}),
            "hypotheses": sorted({t for t in targets if t.startswith("H-")}),
            "experiments": exps, "evidence": sorted(set(ev)),
            "run_count": nruns, "run_paths": run_paths,
            "context": ctx[:400], "source": f"crypto-autoresearcher:ledger/decisions/{f}",
        })
    rows.sort(key=lambda r: (r["date"], r["id"]))
    json.dump({"schema": "ecdlp-jul-sep-index/v1", "scope_goals": sorted(ECDLP_GOALS),
               "scope_families": sorted(FAMILIES), "skipped": dict(skipped),
               "rows": rows}, open(os.path.join(OUT, "INDEX.json"), "w"), indent=1)
    with open(os.path.join(OUT, "INDEX.tsv"), "w", newline="") as fh:
        w = csv.writer(fh, delimiter="\t")
        w.writerow(["date", "id", "goal", "verdict", "families", "hypotheses", "experiments", "evidence", "run_count", "run_paths"])
        for r in rows:
            w.writerow([r["date"], r["id"], r["goal"], r["verdict"], ",".join(r["families"]), ",".join(r["hypotheses"]),
                        ",".join(r["experiments"]), ",".join(r["evidence"]), r["run_count"], ",".join(r["run_paths"])])
    # summaries
    by_month = collections.Counter(r["month"] for r in rows)
    verd = collections.Counter((r["month"], r["verdict"]) for r in rows)
    fams = collections.Counter((r["month"], f) for r in rows for f in r["families"] if f in FAMILIES)
    exps = {m: sorted({e for r in rows if r["month"] == m for e in r["experiments"]}) for m in by_month}
    summ = {"decisions_by_month": dict(sorted(by_month.items())),
            "distinct_experiments_by_month": {m: len(v) for m, v in sorted(exps.items())},
            "runs_by_month": {m: sum(runs_of(e)[0] for e in v) for m, v in sorted(exps.items())},
            "verdicts": {f"{m}|{v}": n for (m, v), n in sorted(verd.items())},
            "families": {f"{m}|{f}": n for (m, f), n in sorted(fams.items())},
            "skipped": dict(skipped)}
    json.dump(summ, open(os.path.join(OUT, "SUMMARY.json"), "w"), indent=1)
    print(json.dumps({k: summ[k] for k in ("decisions_by_month", "distinct_experiments_by_month", "runs_by_month", "skipped")}))

if __name__ == "__main__":
    main()
