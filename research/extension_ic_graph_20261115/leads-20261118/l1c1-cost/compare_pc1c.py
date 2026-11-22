"""PC1c and PC1b (positive controls added by l1c1-cost). Glue only: equality
checks against committed numbers (python3 -I). Prints JSON; exit 0 = pass.

PC1c  counted arms on Run50 (seed 389), FIXED_N 3415, counted build, HOOKS=1:
      the production solver's descent tally equals the complete-accounting /
      descent-walltime recount, the traced (C1) tally equals descent-walltime's,
      the learning-RREF bound equals its recorded value, the trace tally and the
      decomposing residuals are as recorded (descent-walltime/runs/counted-compare-seed389.json,
      complete-accounting/runs/recount-seed389-n3415.json, descent-finder README).
      The four K-stop counted arms on the same stream pass every collect gate
      (the counted L1 matcher reproduces the audit's rows on the Run50 stream).
PC1b  the instrument reproduces a recorded l1-paired fresh target exactly:
      prepare + audit FIXED_N 4096 on l1-paired's first measured public input with
      its recorded collect seed; every column of target.primary.tsv equals the
      committed ../l1-paired/results/targets.tsv row, except public_sha256 (the
      committed publics.jsonl line is the public.json with newlines removed).
Usage: python3 -I compare_pc1c.py PC1_DIR PC1B_DIR L1P_TARGETS_TSV"""
import json, sys

d, db, l1p = sys.argv[1], sys.argv[2], sys.argv[3]


def det(name):
    return json.load(open(f"{d}/{name}.det.json"))


def comp(x, k):
    return int(x["counted"]["components"].get(k, {}).get("bitops", "0"))


fb, fc = det("arm-base.fixed3415"), det("arm-c1.fixed3415")
full_idx = sorted({r["residual_index"] for r in fb["rows"] if r["kind"] == "full"})
checks = {
    "base_nagao_bitops_4358996976408": comp(fb, "nagao") == 4358996976408,
    "c1_nagao_bitops_2483262669096": comp(fc, "nagao") == 2483262669096,
    "c1_learning_bound_bitops_112775271840": int(fc["counted"]["learning_bound_bitops"]) == 112775271840,
    "c1_trace_tally": (fc["trace"]["learned"], fc["trace"]["accepted"], fc["trace"]["accepted_after_mismatch"], fc["trace"]["rejected"]) == (1, 3414, 33, 0),
    "base_no_trace": fb["trace"]["learned"] == 0 and comp(fb, "learning_bound") == 0,
    "decomposing_263_3414": full_idx == [263, 3414],
    "same_solutions_both_solvers": fb["chain_at_end"] == fc["chain_at_end"],
    "tau_base_3414": fb["tau"] == 3414 and fc["tau"] == 3414,
    "processed_3415": fb["processed"] == 3415 and fc["processed"] == 3415,
    "foreign_0": fb["foreign_hook_calls"] == 0 and fc["foreign_hook_calls"] == 0,
    "components_close": all(int(x["counted"]["window_bitops"]) == int(x["counted"]["components_sum_bitops"]) and int(x["counted"]["unattributed_bitops"]) == 0 for x in (fb, fc)),
}
line = open(f"{d}/line.primary.tsv").read().split("\n")
hdr, val = line[0].split("\t"), line[1].split("\t")
row = dict(zip(hdr, val))
checks["kstop_arms_all_gates"] = row.get("gates_failed") == "-"
checks["kstop_tau_base_3414"] = row.get("tau_base") == "3414" and row.get("tau_c1") == "3414"
checks["kstop_kill_flags_0"] = row.get("kill_l1") == "0" and row.get("kill_c1") == "0"
# PC1b
t = open(f"{db}/target.primary.tsv").read().split("\n")
mine = dict(zip(t[0].split("\t"), t[1].split("\t")))
ref = open(l1p).read().split("\n")
rh = ref[0].split("\t")
first = next(dict(zip(rh, x.split("\t"))) for x in ref[1:] if x.startswith("primary\t"))
diff = sorted(k for k in rh if k != "public_sha256" and mine.get(k) != first.get(k))
checks["pc1b_all_columns_equal"] = diff == [] and set(mine) == set(first)
ok = all(checks.values())
print(json.dumps({"schema": "l1c1-cost.pc1c/v1", "pcc": "unverified", "pass": ok, "checks": checks, "pc1b_differing_columns": diff}, sort_keys=True))
sys.exit(0 if ok else 1)
