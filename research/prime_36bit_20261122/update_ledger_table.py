#!/usr/bin/env python3
"""Fold the word-arithmetic pair-table build into the ledger's 36-bit rung.

Reads the artifacts written after the change:
  a3_36bit_u64_table_after.txt   B probe (CPU + JSON per B)
  a3_36bit_u64_seeds_b3072.jsonl four seeds at the new optimum
  a3_36bit_seeds_b3072_timing.txt /usr/bin/time -p per seed
"""
import json
import pathlib

D = pathlib.Path(__file__).resolve().parent
LEDGER = D.parents[1] / "docs/ic/boundary_targets.json"


def cpu_by_block(path):
    out, cur = {}, None
    for line in path.read_text().splitlines():
        s = line.strip()
        if s.startswith("=="):
            cur = s.split()[1].split("=")[-1] if "seed=" in s else s.split()[1].split("=")[-1]
            out[cur] = 0.0
        elif s.startswith(("user ", "sys ")) and cur is not None:
            out[cur] += float(s.split()[1])
    return out


def rows(path):
    return [json.loads(l) for l in path.read_text().splitlines() if l.strip().startswith("{")]


def main():
    seeds = rows(D / "a3_36bit_u64_seeds_b3072.jsonl")
    cpu = cpu_by_block(D / "a3_36bit_seeds_b3072_timing.txt")

    probe = {}
    cur = None
    for line in (D / "a3_36bit_u64_table_after.txt").read_text().splitlines():
        s = line.strip()
        if s.startswith("=="):
            cur = int(s.split("B=")[1].split()[0])
            probe[cur] = 0.0
        elif s.startswith(("user ", "sys ")) and cur is not None:
            probe[cur] += float(s.split()[1])

    ledger = json.loads(LEDGER.read_text())
    cur_rec = ledger["regimes"]["prime"]["records"]["decomposition"]["current"]
    rung = cur_rec["metrics"]["mitm_36bit_rung"]

    rung["pair_table_word_build"] = {
        "change": "build_pair_sum_table computes the B(B+1)/2 sums in u64 via "
                  "add_point_u64 (one extended-Euclid inversion per pair) instead of "
                  "Point::add (one modular exponentiation per pair, plus BigUint "
                  "allocation); the u64 key conversion alone was not the arithmetic",
        "equivalence_gate": "examples/prime_pair_table_check -- identical abscissae, "
                            "index pairs and count at 16/32/36 bits (7260 / 80200 / "
                            "131328 pairs)",
        "b_probe_cpu_ms": {str(b): round(v * 1e3) for b, v in sorted(probe.items())},
        "optimal_B_after": 3072,
        "optimal_B_before": 2048,
        "four_seeds_B3072_cpu_ms": [round(cpu[str(r["target_seed"])] * 1e3) for r in seeds],
        "four_seeds_B3072_pair_table_ms": [
            round(r["ic_stages"]["pair_table_ms"], 1) for r in seeds
        ],
        "four_seeds_B3072_relations_ms": [
            round(r["ic_stages"]["relations_ms"], 1) for r in seeds
        ],
        "four_seeds_B3072_la_ms": [
            round(r["ic_stages"]["linear_algebra_ms"], 1) for r in seeds
        ],
        "all_ic_match_truth": all(r["ic_matches_truth"] for r in seeds),
        "speedup_vs_rung_record": "IC CPU 31.4-33.1 s -> 13.8-16.1 s (~2.2x); "
                                  "table stage ~13x; IC/tuned rho ~1.5e3 -> ~6-9e2",
        "claim_boundary": "same synthetic fixture; constant-factor stage fix only, "
                          "no regime change, no vs_rho claim",
    }

    cur_rec["next_target"] = {
        "summary": "The pair-table build is no longer the bottleneck: at B>=4096 the "
                   "wall is sparse F_n fill-in (nnz_peak 9.5M at B=3072, LA 3.6 s -> "
                   "9.8 s -> 21 s -> 53 s over B=4096/5120/6144). Cut LA fill-in "
                   "(better pivoting or a structured/block Wiedemann path) before "
                   "pushing the factor base past 3072, then take the 40-bit rung.",
        "acceptance_gates": [
            "same rank and same recovered scalar as the current sparse engine",
            "nnz_peak and LA wall reported per B on the 36-bit fixture",
            "B sweep re-run with LA charged in the same process",
        ],
    }
    if "research/prime_36bit_20261122/PAIR_TABLE_U64.md" not in cur_rec["evidence"]:
        cur_rec["evidence"].append("research/prime_36bit_20261122/PAIR_TABLE_U64.md")
    if "examples/prime_pair_table_check.rs" not in cur_rec["evidence"]:
        cur_rec["evidence"].append("examples/prime_pair_table_check.rs")
    ledger["updated"] = "2026-11-22"

    LEDGER.write_text(json.dumps(ledger, indent=2) + "\n")
    print("ledger updated:", LEDGER)


if __name__ == "__main__":
    main()
