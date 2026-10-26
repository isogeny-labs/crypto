#!/usr/bin/env python3
"""One run report per ecbench session (research-visuals, "Every experiment run"):
reports/run-<UTC start>-<session name>/{report.typ, report.pdf, per_curve.svg, per_curve.png, per_curve.json}.
Lays out recorded values only (cost.s, outcome.status, session/host identifiers).
Run: /tmp/report-venv/bin/python make_run_reports.py
"""
import json, os, statistics as st, datetime
from collections import defaultdict
import matplotlib; matplotlib.use("Agg"); import matplotlib.pyplot as plt
import typst
HERE = os.path.dirname(os.path.abspath(__file__)); SESS = os.path.join(HERE, "sessions"); REP = os.path.join(HERE, "reports")

def roles_of(spec): return [r.strip() for r in spec["description"].split("Curve roles, in order:")[1].rstrip(".").split(",")]

for name in sorted(os.listdir(SESS)):
    d = os.path.join(SESS, name)
    if not os.path.isfile(f"{d}/session.json"): continue
    ses = json.load(open(f"{d}/session.json")); spec = json.load(open(f"{d}/spec.json")); host = json.load(open(f"{d}/host.json"))
    if not os.path.getsize(f"{d}/records.jsonl"):
        print("no records yet, skipped", name); continue
    roles = roles_of(spec); key = lambda c: json.dumps(c, sort_keys=True)
    role_of = {key(c): roles[i] for i, c in enumerate(spec["workloads"]["curves"])}
    S = defaultdict(list); status = defaultdict(int); slugs = {}; levels = set(); first_ms = None
    for line in open(f"{d}/records.jsonl"):
        r = json.loads(line); status[r["outcome"]["status"]] += 1
        levels.add(r["isolation"]["level"])
        t0 = r["isolation"]["before"]["unix_ms"]; first_ms = t0 if first_ms is None else min(first_ms, t0)
        role = role_of[key(r["workload"]["curve_spec"])]; slugs[role] = r["workload"]["curve"]["slug"]
        if r["outcome"]["status"] == "verified" and not r["warmup"]:
            S[(r["arm"], role)].append(r["cost"]["s"])
    start = datetime.datetime.utcfromtimestamp((first_ms or 0) / 1000).strftime("%Y%m%dT%H%MZ")
    out = os.path.join(REP, f"run-{start}-{name}"); os.makedirs(out, exist_ok=True)
    rows = []
    for (arm, role), xs in sorted(S.items()):
        m = st.mean(xs); se = st.stdev(xs) / len(xs) ** 0.5 if len(xs) > 1 else 0.0
        rows.append({"arm": arm, "curve": role, "slug": slugs[role], "n": len(xs), "mean_s": m, "se": se, "min": min(xs), "max": max(xs)})
    json.dump({"session_id": ses["session_id"], "rows": rows, "status_counts": dict(status)}, open(f"{out}/per_curve.json", "w"), indent=1)
    # graph: mean S per curve, one marker per arm, +-1.96 SE
    arms = sorted({r["arm"] for r in rows}); fig, ax = plt.subplots(figsize=(8, max(3, 0.25 * len(roles) + 1.5)))
    for j, arm in enumerate(arms):
        rr = [r for r in rows if r["arm"] == arm]; ys = [roles.index(r["curve"]) + 0.15 * j for r in rr]
        ax.errorbar([r["mean_s"] for r in rr], ys, xerr=[1.96 * r["se"] for r in rr], fmt="o", ms=4, capsize=2, label=f"{arm} (n per curve {rr[0]['n']})")
    ax.set_yticks(range(len(roles))); ax.set_yticklabels(roles, fontsize=7); ax.invert_yaxis(); ax.legend(fontsize=7)
    ax.set_xlabel("mean S (charged group ops / sqrt r), +-1.96 SE over targets"); ax.set_title(f"{name}  session {ses['session_id']}", fontsize=9)
    fig.tight_layout(); fig.savefig(f"{out}/per_curve.svg"); fig.savefig(f"{out}/per_curve.png", dpi=150); plt.close(fig)
    typ = ['#set page(paper: "a4", margin: 1.5cm)\n#set text(size: 9pt)\n#show table: set text(size: 7pt)',
           f"= Run report: {name}", f"Session `{ses['session_id']}`, status `{ses['status']}`, {ses['records_written']} records; spec `{ses['spec_id']}` sha256 `{ses['spec_sha256'][:16]}…`; binary sha256 `{ses['binary_sha256'][:16]}…`, git commit `{ses.get('git_commit','?')}`; env class `{ses['env_class_id']}`; host env class sha256 `{host['env_class_sha256'][:16]}…` (Intel Xeon 6975P-C, 32 logical CPUs, per `ecbench host`). Isolation levels earned: {sorted(levels)} (shared host, no CPU reservation). Spec label: {spec['label']}.",
           f"Arms: {', '.join(a['name'] + ' = ' + a['method']['id'] + ' (' + a['role'] + ')' for a in spec['arms'])}. Workloads: {len(spec['workloads']['curves'])} curves × {spec['workloads']['targets_per_curve']} `{spec['workloads']['target_kind']}` targets, target seed {spec['workloads']['target_seed']}, measurement seed {spec['measurement']['seed']}, rounds {spec['measurement']['rounds']}, warm-up {spec['measurement']['warmup']}.",
           f"Record outcomes: {dict(status)}. Unit: S = charged group operations / sqrt(r) (`cost.s`, lower bound: canonicalisations uncharged). Wall time not read. Raw records: `sessions/{name}/records.jsonl`.",
           '#figure(image("per_curve.svg", width: 95%), caption: [Mean S per curve and arm with ±1.96 SE over targets (recorded values).])',
           "#table(columns: 7, [arm], [curve], [slug], [n], [mean S], [SE], [min / max],"]
    for r in rows: typ.append(f"[{r['arm']}], [{r['curve']}], [{r['slug']}], [{r['n']}], [{r['mean_s']:.4f}], [{r['se']:.4f}], [{r['min']:.3f} / {r['max']:.3f}],")
    typ.append(")")
    open(f"{out}/report.typ", "w").write("\n".join(typ)); typst.compile(f"{out}/report.typ", output=f"{out}/report.pdf", root=out)
    print("wrote", out)
