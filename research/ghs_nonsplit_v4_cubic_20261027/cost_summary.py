#!/usr/bin/env python3
"""Check the completed q16 transfer, line-sieve, and matched rho receipts."""
import hashlib
import json
import re
from pathlib import Path

HERE = Path(__file__).resolve().parent


def load(name):
    path = HERE/name
    return json.loads(path.read_text()), hashlib.sha256(path.read_bytes()).hexdigest()


def read_number(pattern, stdout, cast=float):
    match = re.search(pattern, stdout)
    assert match, pattern
    return cast(match.group(1))


mapped, map_hash = load("subgroup_map_4054.json")
proof, proof_hash = load("index_4054_q256_fb200_receipt.json")
bench, bench_hash = load("index_4054_q256_fb200_bench_receipt.json")
rho, rho_hash = load("rho_magma_4054_receipt.json")
local_rho, local_rho_hash = load("rho_4054.json")
assert all(mapped["checks"].values())
assert all(x["response"]["success"] and x["response"]["exit_code"] == 0
           for x in (proof, bench, rho))
assert all(x["response"]["magma"]["version"] == "2.30-1"
           for x in (proof, bench, rho))
ps = proof["response"]["stdout"]
bs = bench["response"]["stdout"]
rs = rho["response"]["stdout"]
assert "BASE_ORDER_CHECKS false true true" in ps
assert "LIFT_ORDER_CHECKS false true true" in ps
assert "DLP mod 2027: 7" in ps and "SOURCE_VERIFIED true" in ps
assert "DLP mod 2027: 7" in bs and "SOURCE_VERIFIED true" in bs
relations = read_number(r"Number of relations found: (\d+)", bs, int)
factor_base = read_number(r"Number of elements in factor base: (\d+)", bs, int)
setup_cpu = read_number(r"PRE_INDEX_CPU ([0-9.]+)", bs)
index_cpu = read_number(r"INDEX_ANSWER 7 INDEX_CPU ([0-9.]+)", bs)
rho_runs = read_number(r"RHO_SOLVED (\d+)", rs, int)
rho_median = read_number(r"RHO_STEP_CALLS_MEDIAN (\d+)", rs, int)
rho_steps = read_number(r"RHO_STEP_CALLS_TOTAL (\d+)", rs, int)
rho_cpu = read_number(r"RHO_CPU_SECONDS ([0-9.]+)", rs)
assert (relations, factor_base, rho_runs, rho_median, rho_steps) == (
    260, 201, 100, 168, 18180)
assert local_rho["solved_runs"] == 100
assert mapped["source_order"] == 4054 and mapped["subgroup_prime"] == 2027
assert mapped["source_Q"] == local_rho["target"]
ratio = (setup_cpu+index_cpu)/(rho_cpu/rho_runs)
out = {"schema":"ghs-nonsplit-q16-full-cost-v1",
       "candidate_source_order":4054,"subgroup_prime":2027,
       "target_log":7,"map_checks":mapped["checks"],
       "magma_version":"2.30-1",
       "proof_job_seconds":proof["response"]["magma"]["time_sec"],
       "attack_core_setup_cpu_seconds":setup_cpu,
       "attack_core_index_cpu_seconds":index_cpu,
       "attack_core_server_seconds":bench["response"]["magma"]["time_sec"],
       "relations":relations,"factor_base_points":factor_base,
       "rho_runs":rho_runs,"rho_median_group_step_calls":rho_median,
       "rho_total_group_step_calls":rho_steps,
       "rho_total_cpu_seconds":rho_cpu,
       "rho_server_seconds":rho["response"]["magma"]["time_sec"],
       "rho_mean_cpu_seconds_per_solve":rho_cpu/rho_runs,
       "attack_core_to_mean_rho_cpu_ratio":ratio,
       "local_sage_jacobian_setup_seconds":mapped["jacobian_setup_seconds"],
       "local_sage_two_map_evaluations_seconds":mapped["subgroup_map_seconds"],
       "receipt_sha256":{"map":map_hash,"proof":proof_hash,
                         "attack_core":bench_hash,"magma_rho":rho_hash,
                         "local_rho":local_rho_hash}}
(HERE/"cost_summary.json").write_text(json.dumps(out,indent=2)+"\n")
print(json.dumps({key:out[key] for key in
                  ("attack_core_setup_cpu_seconds",
                   "attack_core_index_cpu_seconds",
                   "relations","rho_runs",
                   "rho_mean_cpu_seconds_per_solve",
                   "attack_core_to_mean_rho_cpu_ratio")},indent=2))
