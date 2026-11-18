#!/usr/bin/env python3
"""Validator check of L4 (F4 step trace): committed cells against the recorded exponent-study cells and against
this validator's own replays of the binaries, plus a reading of the per-target step logs against the frozen
predictions P1-P3. stdlib only.
Usage: l4_check.py <repo> <commit> <replay_dir> <out.json>
  replay_dir holds <name>.out files (last stdout line = the f4sub JSON) written by the validator's commands."""
import json, subprocess, sys, os

REPO, COMMIT, RDIR, OUT = sys.argv[1:5]
L4 = "research/curve25519_wave6_runs_20261115/L4_trace/cells"
REF = "research/curve25519_exponent_study_20261108/cells"
VOLATILE = {"ms"}


def git_json(path):
    data = subprocess.run(["git", "-C", REPO, "show", f"{COMMIT}:{path}"], check=True, capture_output=True).stdout
    with open(os.path.join(REPO, path), "rb") as fh:
        same = fh.read() == data
    return json.loads(data), same


def replay(name):
    with open(os.path.join(RDIR, name + ".out")) as fh:
        return json.loads(fh.read().strip().splitlines()[-1])


def targets_equal(a, b, fields=None):
    """Per-target equality over the fields both records carry (minus wall time)."""
    if [t["target_x"] for t in a["per_target"]] != [t["target_x"] for t in b["per_target"]]:
        return False, ["target order/set differs"]
    diffs = []
    for ta, tb in zip(a["per_target"], b["per_target"]):
        keys = (set(ta) & set(tb)) - VOLATILE if fields is None else set(fields)
        for k in sorted(keys):
            if ta.get(k) != tb.get(k):
                diffs.append(f"{ta['target_x']}:{k}:{ta.get(k)}!={tb.get(k)}")
    for k in ("seed", "p", "n_vars", "input_degree", "input_terms", "levels", "v_size", "mean_field_ops", "form", "m", "d_or_t"):
        if a.get(k) != b.get(k):
            diffs.append(f"cell:{k}:{a.get(k)}!={b.get(k)}")
    return not diffs, diffs[:12]


def read_log(cell):
    out = []
    for t in cell["per_target"]:
        L = t.get("step_log") or []
        deg = [s["degree"] for s in L]
        prod = [i for i, s in enumerate(L) if s["new_polys"] > 0]
        top = max(deg) if deg else None
        top_steps = [i for i, s in enumerate(L) if s["degree"] == top]
        out.append({
            "target_x": t["target_x"],
            "degrees": deg,
            "pairs": [s["pairs"] for s in L],
            "new_polys": [s["new_polys"] for s in L],
            "steps_reported": t["steps"], "steps_logged": len(L),
            "sum_step_ops": sum(s["ops"] for s in L), "field_ops": t["field_ops"],
            "solving_degree": t["solving_degree"], "solving_degree_max": t["solving_degree_max"],
            "max_step_degree": top,
            "max_degree_step_indices_1based": [i + 1 for i in top_steps],
            "max_degree_steps_pairs": [L[i]["pairs"] for i in top_steps],
            "max_degree_steps_new_polys": [L[i]["new_polys"] for i in top_steps],
            "last_logged_productive_step_degree": L[prod[-1]]["degree"] if prod else None,
            "highest_productive_degree_in_log": max(L[i]["degree"] for i in prod) if prod else None,
            "ops_share_at_max_degree": sum(L[i]["ops"] for i in top_steps) / max(1, sum(s["ops"] for s in L)),
            "steps_at_degree_ge_D_plus_2": [i + 1 for i, s in enumerate(L) if s["degree"] >= (cell["input_degree"] + 2)],
        })
    return out


res = {"commit": COMMIT}
cells = {}
for name in ("20_sub_m2_sym_d19", "20_sub_m2_sym-efirst_d19", "20_sub_m2_sym-efirst_d57"):
    cells[name], same = git_json(f"{L4}/{name}.json")
    res.setdefault("working_tree_equals_commit", {})[name] = same
refs = {}
for name in ("k20_m2_sym_d19_g0", "k20_m2_sym_d57_g0"):
    refs[name], _ = git_json(f"{REF}/{name}.json")

# 1. the committed d = 19 cell against the recorded cell (exact, every shared per-target field)
ok, d = targets_equal(cells["20_sub_m2_sym_d19"], refs["k20_m2_sym_d19_g0"])
res["committed_d19_vs_recorded"] = {"identical_shared_fields": ok, "diffs": d,
                                    "T": cells["20_sub_m2_sym_d19"]["mean_field_ops"], "T_recorded": refs["k20_m2_sym_d19_g0"]["mean_field_ops"],
                                    "SD": [t["solving_degree_max"] for t in cells["20_sub_m2_sym_d19"]["per_target"]],
                                    "SD_recorded": [t["solving_degree_max"] for t in refs["k20_m2_sym_d19_g0"]["per_target"]]}
# 2. validator replays
rep = {}
pairs = [("new_sym19_log", cells["20_sub_m2_sym_d19"]), ("new_sym19", refs["k20_m2_sym_d19_g0"]),
         ("old_sym19", refs["k20_m2_sym_d19_g0"]), ("old_sym57", refs["k20_m2_sym_d57_g0"]),
         ("new_sym57", refs["k20_m2_sym_d57_g0"]),
         ("new_ef19", cells["20_sub_m2_sym-efirst_d19"]), ("new_ef57", cells["20_sub_m2_sym-efirst_d57"]),
         ("new_ef19_log", cells["20_sub_m2_sym-efirst_d19"]), ("new_ef57_log", cells["20_sub_m2_sym-efirst_d57"]),
         ("new_sym57_log", refs["k20_m2_sym_d57_g0"])]
for name, want in pairs:
    if not os.path.exists(os.path.join(RDIR, name + ".out")):
        continue
    got = replay(name)
    ok, d = targets_equal(got, want)
    rep[name] = {"identical_shared_fields": ok, "diffs": d, "T": got["mean_field_ops"]}
res["replays"] = rep
# step-log replay equals committed step log exactly
got = replay("new_sym19_log")
res["replay_step_log_identical"] = all(a.get("step_log") == b.get("step_log") for a, b in zip(got["per_target"], cells["20_sub_m2_sym_d19"]["per_target"]))
# logging does not change counts: logged vs unlogged replay
a, b = replay("new_sym19_log"), replay("new_sym19")
res["logging_changes_no_count_d19"] = targets_equal(a, b, fields=["field_ops", "steps", "solving_degree_max", "solving_degree", "degree_reached", "max_rows", "max_cols"])[0]

# 3. step logs (committed d = 19 cell)
logs = read_log(cells["20_sub_m2_sym_d19"])
res["step_logs_d19"] = {"targets": len(logs), "all_targets_identical_logs": len({json.dumps({k: v for k, v in l.items() if k != 'target_x'}) for l in logs}) == 1,
                        "first_target": logs[0]}
# exploratory step logs from the validator's replays (not producer evidence)
expl = {}
for name in ("new_ef19_log", "new_ef57_log", "new_sym57_log"):
    if os.path.exists(os.path.join(RDIR, name + ".out")):
        c = replay(name)
        L = read_log(c)
        expl[name] = {"input_degree": c["input_degree"], "levels": c["levels"], "all_targets_identical_logs": len({json.dumps({k: v for k, v in l.items() if k != 'target_x'}) for l in L}) == 1, "first_target": L[0]}
res["exploratory_step_logs"] = expl

# 4. predictions
l0 = logs
P1_last_deg20 = all(l["last_logged_productive_step_degree"] == 20 for l in l0)
P1_pairs_le4 = all(all(p <= 4 for p in l["max_degree_steps_pairs"]) for l in l0)
res["P1"] = {"as_written": "last step with new_polys > 0 has degree 20, and that step has at most 4 pairs",
             "last_productive_step_is_degree_20": P1_last_deg20,
             "last_logged_productive_degree": sorted({l["last_logged_productive_step_degree"] for l in l0}),
             "reported_solving_degree_(step_where_1_appeared)": sorted({l["solving_degree"] for l in l0}),
             "degree_20_steps_pairs": l0[0]["max_degree_steps_pairs"], "degree_20_steps_new_polys": l0[0]["max_degree_steps_new_polys"],
             "all_degree_20_steps_have_le_4_pairs": P1_pairs_le4,
             "holds": P1_last_deg20 and P1_pairs_le4}
res["P2"] = {"steps_at_degree_ge_21": sorted({tuple(l["steps_at_degree_ge_D_plus_2"]) for l in l0}), "vacuous": all(not l["steps_at_degree_ge_D_plus_2"] for l in l0)}
e19, e57 = cells["20_sub_m2_sym-efirst_d19"], cells["20_sub_m2_sym-efirst_d57"]
r19, r57 = refs["k20_m2_sym_d19_g0"], refs["k20_m2_sym_d57_g0"]
res["P3"] = {
    "d19": {"SD_efirst": sorted({t["solving_degree_max"] for t in e19["per_target"]}), "SD_recorded": sorted({t["solving_degree_max"] for t in r19["per_target"]}),
            "T_efirst": e19["mean_field_ops"], "T_recorded": r19["mean_field_ops"], "ratio": e19["mean_field_ops"] / r19["mean_field_ops"],
            "same_targets": [t["target_x"] for t in e19["per_target"]] == [t["target_x"] for t in r19["per_target"]]},
    "d57": {"SD_efirst": sorted({t["solving_degree_max"] for t in e57["per_target"]}), "SD_recorded": sorted({t["solving_degree_max"] for t in r57["per_target"]}),
            "T_efirst": e57["mean_field_ops"], "T_recorded": r57["mean_field_ops"], "ratio": e57["mean_field_ops"] / r57["mean_field_ops"],
            "same_targets": [t["target_x"] for t in e57["per_target"]] == [t["target_x"] for t in r57["per_target"]]},
}
res["P3"]["holds"] = all(abs(res["P3"][k]["SD_efirst"][0] - res["P3"][k]["SD_recorded"][0]) <= 1 and len(res["P3"][k]["SD_efirst"]) == 1
                         and 0.5 < res["P3"][k]["ratio"] < 2 for k in ("d19", "d57"))
json.dump(res, open(OUT, "w"), indent=1)
print(json.dumps(res, indent=1))
