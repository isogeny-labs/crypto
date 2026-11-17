#!/usr/bin/env python3
"""Summarize checked transfer, completed index calculus, and matched rho."""
import hashlib
import json
import re
from pathlib import Path

HERE = Path(__file__).resolve().parent


def load(name):
    path = HERE / name
    return json.loads(path.read_text()), hashlib.sha256(path.read_bytes()).hexdigest()


def field(pattern, s, cast=float):
    m = re.search(pattern, s)
    assert m, pattern
    return cast(m.group(1))


mapped, mh = load("subgroup_map_4222.json")
checked, ch = load("index_4222_q256_fb200_receipt.json")
timed, th = load("index_4222_q256_fb200_bench_receipt.json")
rho, rh = load("rho_magma_4222_receipt.json")
full_checked, fch = load("full_attack_4222_receipt.json")
full_timed, fth = load("full_attack_4222_bench_receipt.json")
assert all(mapped["checks"].values())
assert mapped["source_order"] == 4222 and mapped["subgroup_prime"] == 2111
assert all(rec["response"]["success"] and rec["response"]["exit_code"] == 0
           and rec["response"]["magma"]["version"] == "2.30-1"
           for rec in (checked, timed, rho, full_checked, full_timed))
cs, ts, rs = (rec["response"]["stdout"] for rec in (checked, timed, rho))
fcs = full_checked["response"]["stdout"]
fts = full_timed["response"]["stdout"]
assert "MAP_ORDER_CHECKS false true true" in fcs
assert "LIFT_ORDER_CHECKS false true true" in fcs
assert all("DLP mod 2111: 7" in s and "FULL_SOURCE_VERIFIED true" in s
           for s in (fcs, fts))
assert "BASE_ORDER_CHECKS false true true" in cs
assert "LIFT_ORDER_CHECKS false true true" in cs
assert all("DLP mod 2111: 7" in s and "SOURCE_VERIFIED true" in s
           for s in (cs, ts))
relations = field(r"Number of relations found: (\d+)", ts, int)
fb = field(r"Number of elements in factor base: (\d+)", ts, int)
setup = field(r"PRE_INDEX_CPU ([0-9.]+)", ts)
ic = field(r"INDEX_ANSWER 7 INDEX_CPU ([0-9.]+)", ts)
runs = field(r"RHO_SOLVED (\d+)", rs, int)
singular = field(r"SINGULAR (\d+)", rs, int)
median = field(r"RHO_STEP_CALLS_MEDIAN (\d+)", rs, int)
steps = field(r"RHO_STEP_CALLS_TOTAL (\d+)", rs, int)
rho_cpu = field(r"RHO_CPU_SECONDS ([0-9.]+)", rs)
dynamic_map = field(r"MAP_CPU ([0-9.]+)", fts)
dynamic_pre_index = field(r"PRE_INDEX_CPU ([0-9.]+)", fts)
dynamic_index = field(r"INDEX_ANSWER 7 INDEX_CPU ([0-9.]+)", fts)
dynamic_full = field(r"FULL_SOURCE_VERIFIED true FULL_CPU ([0-9.]+)", fts)
assert abs(dynamic_pre_index + dynamic_index - dynamic_full) < 0.011
assert relations == 260 and fb >= 200 and runs == 100 and singular == 0
data = {
    "schema": "ghs-order4222-cost-v1",
    "source_order": 4222, "subgroup_prime": 2111, "recovered_log": 7,
    "map_checks": mapped["checks"], "magma_version": "2.30-1",
    "checked_job_server_seconds": checked["response"]["magma"]["time_sec"],
    "timed_attack_server_seconds": timed["response"]["magma"]["time_sec"],
    "attack_core_setup_cpu_seconds": setup,
    "attack_core_index_cpu_seconds": ic,
    "attack_core_total_cpu_seconds": setup + ic,
    "relations": relations, "factor_base_points": fb,
    "rho_runs": runs, "rho_singular": singular,
    "rho_median_group_step_calls": median,
    "rho_total_group_step_calls": steps,
    "rho_total_cpu_seconds": rho_cpu,
    "rho_mean_cpu_seconds_per_solve": rho_cpu / runs,
    "rho_server_seconds": rho["response"]["magma"]["time_sec"],
    "attack_core_to_mean_rho_cpu_ratio": (setup + ic) / (rho_cpu / runs),
    "dynamic_checked_server_seconds": full_checked["response"]["magma"]["time_sec"],
    "dynamic_timed_server_seconds": full_timed["response"]["magma"]["time_sec"],
    "dynamic_source_map_cpu_seconds": dynamic_map,
    "dynamic_pre_index_cpu_seconds": dynamic_pre_index,
    "dynamic_index_cpu_seconds": dynamic_index,
    "dynamic_total_cpu_seconds": dynamic_full,
    "dynamic_full_to_mean_rho_cpu_ratio": dynamic_full / (rho_cpu / runs),
    "local_sage_jacobian_setup_seconds": mapped["jacobian_setup_seconds"],
    "local_sage_two_map_evaluations_seconds": mapped["subgroup_map_seconds"],
    "receipt_sha256": {"map": mh, "checked": ch, "timed": th, "rho": rh,
                       "dynamic_checked": fch, "dynamic_timed": fth},
}
(HERE / "cost_4222.json").write_text(json.dumps(data, indent=2) + "\n")
print(json.dumps(data, indent=2))
