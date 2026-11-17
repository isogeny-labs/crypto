import json, os, sys
BASE = "/tmp/claude-0/-home-user/e260bf71-4677-5a1e-b0d3-1c3899bb5656/scratchpad/redteam/research/gaudry_k4_char2_crossover_20261015/sessions"
rows = []
sess_rows = []
for name in sorted(os.listdir(BASE)):
    d = os.path.join(BASE, name)
    if not os.path.isdir(d):
        continue
    sj = json.load(open(os.path.join(d, "session.json")))
    killed = "killed" in name
    sess_rows.append(dict(dir=name, status=sj.get("status"), counts=sj.get("status_counts"),
                          executions_planned=sj.get("executions_planned"), records_written=sj.get("records_written"),
                          started=sj.get("started_unix_ms"), finished=sj.get("finished_unix_ms"),
                          binary=sj.get("binary_sha256"), commit=sj.get("git_commit"), dirty=sj.get("git_dirty"),
                          quiet=(sj.get("preflight") or {}).get("quiet"), killed=killed,
                          session_id=sj.get("session_id")))
    rp = os.path.join(d, "records.jsonl")
    if not os.path.exists(rp):
        continue
    for line in open(rp):
        r = json.loads(line)
        ph = {p["name"]: p for p in r.get("phases", [])}
        nat_rel = ph.get("relations", {}).get("native", {})
        trials = nat_rel.get("trials")
        rels = nat_rel.get("relations") or r.get("counters", {}).get("relations")
        la = ph.get("linear_algebra", {}).get("native", {})
        rows.append(dict(
            dir=name, killed=killed, session_status=sj.get("status"),
            session_id=r["session_id"], m=r["workload"]["curve_spec"]["m"],
            curve_r=r["workload"]["curve"]["r"], cofactor=r["workload"]["curve"]["cofactor"],
            arm=r["arm"], method=r["method"]["id"], workload_id=r["workload"]["workload_id"],
            target_index=r["workload"]["target_index"], warmup=r["warmup"], round=r["round"],
            verified=r["outcome"]["status"] == "verified" and r["outcome"]["matches_planted"] is True,
            status=r["outcome"]["status"],
            S=r["cost"]["s"], total_gae=r["cost"]["total_gae"], lower_bound=r["cost"]["lower_bound"],
            trials=trials, relations=rels, la_row_ops=la.get("row_ops"), la_rank=la.get("rank"),
            steps=r.get("counters", {}).get("steps"), walks=r.get("counters", {}).get("walks"),
            wall_s=((r.get("online") or {}).get("wall_ns") or 0) / 1e9,
            isolation=(r.get("isolation") or {}).get("level"), binary=r["binary_sha256"][:12],
            expected_steps=(r.get("detail") or {}).get("expected_steps"),
            floor_s=(r.get("boundaries") or {}).get("floor_s"),
        ))
json.dump({"records": rows, "sessions": sess_rows}, open(os.path.join(os.path.dirname(__file__), "inventory.json"), "w"), indent=1, default=str)
for s in sess_rows:
    print("SESSION", s)
print()
hdr = ["dir","killed","m","arm","workload_id","t","verified","S","trials","relations","la_row_ops","total_gae","wall_s","isolation","warm"]
print("\t".join(hdr))
for r in rows:
    print("\t".join(str(x) for x in [r["dir"],r["killed"],r["m"],r["arm"],r["workload_id"],r["target_index"],r["verified"],
          f'{r["S"]:.4f}', r["trials"], r["relations"], r["la_row_ops"], f'{r["total_gae"]:.4g}', f'{r["wall_s"]:.2f}', r["isolation"], r["warmup"]]))
