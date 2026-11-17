#!/usr/bin/env python3
"""Specs of the two new families (setup only): Tcr on V5 (one spec per non-crater vertex) and M on V5 and V6
(one spec per chosen curve, 8 models). Inputs: census/V5/tcr.jsonl, census/V*/M.json, runs/manifest_std.lines
(N rounds per level), census/seeds.json. Output: specs/*.json and runs/manifest_new.lines."""
import json, os
STUDY = os.environ.get("STUDY", "/home/user/crypto/research/volcano_ecdlp_census_followup_20261023")
CENSUS = os.environ.get("CENSUS", "/home/user/crypto/research/volcano_ecdlp_census_20261017")
S = json.load(open(f"{STUDY}/census/seeds.json"))
ic = json.load(open(f"{CENSUS}/FROZEN.json"))["ic"]
icp = {"factor_base": ic["factor_base"], "oracle": ic["oracle"]}
std = [json.loads(l) for l in open(f"{STUDY}/runs/manifest_std.lines")]
nr = {(m["V"], m["level"]): m["rounds"] for m in std if m["family"] == "N"}
lines = []
D = [json.loads(l) for l in open(f"{STUDY}/census/V5/transport/D.jsonl")]
cr = D[0]
meas = lambda rounds: {"rounds": rounds, "warmup": 1, "order": "alternate", "seed": S["ecbench_measurement_seed"], "isolation_required": "L2", "timeout_seconds": 600}
desc = "Volcano ECDLP census follow-up Phase B (research/volcano_ecdlp_census_followup_20261023/PLAN.md). Known-answer synthetic curves, planted_shared targets; no security claim."
for t in (json.loads(l) for l in open(f"{STUDY}/census/V5/tcr.jsonl")):
    g = t["g"].split(",")
    rounds = nr[("V5", t["level"])]
    name = f"V5-Tcr-L{t['level']}-c{t['id']}"
    spec = {"schema": "ecbench.spec/v1", "label": f"volcano census follow-up V5 Tcr source vertex {t['id']} (level {t['level']})", "description": desc,
            "workloads": {"curves": [{"kind": "prime_explicit", "p": cr["p"], "a": 0, "b": cr["b"], "group_order": cr["order"], "r": cr["r"], "gx": int(g[0]), "gy": int(g[1])}],
                          "targets_per_curve": 8, "target_seed": S["target_seed_native"], "target_kind": "planted_shared"},
            "arms": [{"name": "rho-neg", "role": "reference", "method": {"id": "rho.negation"}},
                     {"name": "rho-cubic", "role": "control", "method": {"id": "rho.cubic"}}],
            "measurement": meas(rounds)}
    json.dump(spec, open(f"{STUDY}/specs/{name}.json", "w"), indent=1)
    lines.append({"name": name, "V": "V5", "family": "Tcr", "level": t["level"], "chunk": t["id"], "curves": 1, "targets": 8, "rounds": rounds, "warmup": 1,
                  "arms": 2, "cheap_arms": 2, "ic_arms": 0, "passes": 8 * (rounds + 1)})
for V in ("V5", "V6"):
    for c in json.load(open(f"{STUDY}/census/{V}/M.json"))["curves"]:
        name = f"{V}-M-L{c['level']}-c{c['id']}"
        spec = {"schema": "ecbench.spec/v1", "label": f"volcano census follow-up {V} M vertex {c['id']} (level {c['level']}), 8 isomorphic models", "description": desc,
                "workloads": {"curves": [{"kind": "prime_explicit", "p": c["p"], "a": m["a"], "b": m["b"], "group_order": c["order"], "r": c["r"], "gx": m["gx"], "gy": m["gy"]} for m in c["models"]],
                              "targets_per_curve": 16, "target_seed": S["target_seed_M"], "target_kind": "planted_shared"},
                "arms": [{"name": "rho-neg", "role": "reference", "method": {"id": "rho.negation"}},
                         {"name": "bsgs", "role": "candidate", "method": {"id": "bsgs.textbook"}},
                         {"name": "ic", "role": "candidate", "method": {"id": "ic.pipeline", "params": icp}}],
                "measurement": meas(4)}
        json.dump(spec, open(f"{STUDY}/specs/{name}.json", "w"), indent=1)
        lines.append({"name": name, "V": V, "family": "M", "level": c["level"], "chunk": c["id"], "curves": 8, "targets": 16, "rounds": 4, "warmup": 1,
                      "arms": 3, "cheap_arms": 2, "ic_arms": 1, "passes": 8 * 16 * 5})
with open(f"{STUDY}/runs/manifest_new.lines", "w") as f:
    for l in lines: f.write(json.dumps(l, separators=(",", ":")) + "\n")
print(len(lines), "new specs")
