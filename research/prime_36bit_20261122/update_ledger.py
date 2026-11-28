#!/usr/bin/env python3
"""Fold the 36-bit a=-3 MITM rung (and the 32-bit rho correction) into
docs/ic/boundary_targets.json.

Data sources (all written by run_rung.sh):
  a3_36bit_sweep.log          B probe at seed 20261121
  a3_36bit_u64_seeds.jsonl    four paired seeds at the chosen B
  a3_36bit_rho_paired.jsonl   tuned `cryptanalysis` rho on the same targets
  a3_32bit_rho_paired.jsonl   same, on the four archived 32-bit targets

The file is round-trip exact under json.dumps(indent=2) + trailing newline,
so this edit shows as a minimal diff.
"""
import json
import pathlib
import sys

ROOT = pathlib.Path(__file__).resolve().parents[2]
D36 = ROOT / "research/prime_36bit_20261122"
D32 = ROOT / "research/prime_32bit_20261122"
LEDGER = ROOT / "docs/ic/boundary_targets.json"


def rows(path):
    out = []
    for line in path.read_text().splitlines():
        line = line.strip()
        if line.startswith("{"):
            out.append(json.loads(line))
    return out


def main():
    seeds36 = rows(D36 / "a3_36bit_u64_seeds.jsonl")
    rho36 = {r["target_seed"]: r for r in rows(D36 / "a3_36bit_rho_paired.jsonl")}
    rho32 = {r["target_seed"]: r for r in rows(D36 / "a3_32bit_rho_paired.jsonl")}
    if len(seeds36) != 4:
        sys.exit(f"expected 4 seed rows, found {len(seeds36)}")

    rung = {
        "curve": "a3-bench-36bit-p256class",
        "p": 68719476731,
        "a": 68719476728,
        "b": 33,
        "n": 68719022539,
        "generator": [19, 72892532],
        "nG_is_infinity": True,
        "B": 2048,
        "B_policy": "measured probe over {1600,2048,2560,3072} at seed 20261121; "
                    "the (2p)^(1/3) heuristic would say ~5158 and is 2x worse here "
                    "because the pair-table build dominates",
        "seeds": [r["target_seed"] for r in seeds36],
        "ic_wall_ms": [round(r["ic_stages"]["total_ms"], 1) for r in seeds36],
        "pair_table_ms": [round(r["ic_stages"]["pair_table_ms"], 1) for r in seeds36],
        "relations_ms": [round(r["ic_stages"]["relations_ms"], 1) for r in seeds36],
        "linear_algebra_ms": [round(r["ic_stages"]["linear_algebra_ms"], 1) for r in seeds36],
        "trials_total": [r["ic_stages"]["trials_total"] for r in seeds36],
        "all_ic_match_truth": all(r["ic_matches_truth"] for r in seeds36),
        "rho_arm": "cryptanalysis `ca solve --alg rho` (negation map, distinguished "
                   "points), single thread, on the identical public target",
        "rho_wall_ms": [round(1000 * rho36[r["target_seed"]]["seconds"], 2) for r in seeds36],
        "rho_ops": [rho36[r["target_seed"]]["ops"] for r in seeds36],
        "rho_ops_over_sqrt_n": [
            round(rho36[r["target_seed"]]["ops"] / (68719022539 ** 0.5), 3) for r in seeds36
        ],
        "rho_agrees": all(rho36[r["target_seed"]].get("rho_agrees") for r in seeds36),
        "ic_over_tuned_rho": [
            round((r["ic_stages"]["total_ms"] / 1000) / rho36[r["target_seed"]]["seconds"], 1)
            for r in seeds36
        ],
        "ladder_builtin_rho_note": "the ladder's own rho_ms field is a single unoptimised "
                                   "walk and exhausted its 5M-step budget on every 36-bit "
                                   "seed; it is a fixture sanity check, not a baseline",
        "claim_boundary": "single host, shared load; known-answer synthetic fixture only; "
                          "no vs_rho claim, no deployed-curve claim, no asymptotic claim",
    }

    ledger = json.loads(LEDGER.read_text())
    prime = ledger["regimes"]["prime"]
    caps = prime["implementation_caps"]
    caps["a3_s4_3decomp_mitm_ladder_bits"] = 36

    cur = prime["records"]["decomposition"]["current"]
    cur["metrics"]["mitm_36bit_rung"] = rung

    old32 = cur["metrics"]["mitm_32bit_rung"]
    order = [r["target_seed"] for r in rows(D32 / "a3_32bit_u64_seeds.jsonl")]
    old32["rho_arm_correction"] = {
        "arm": "cryptanalysis `ca solve --alg rho`, single thread, identical targets",
        "seeds": order,
        "ic_ms": [round(r["ic_stages"]["total_ms"], 1)
                  for r in rows(D32 / "a3_32bit_u64_seeds.jsonl")],
        "tuned_rho_ms": [round(1000 * rho32[s]["seconds"], 2) for s in order],
        "tuned_rho_ops": [rho32[s]["ops"] for s in order],
        "ic_over_tuned_rho": [
            round((r["ic_stages"]["total_ms"] / 1000) / rho32[r["target_seed"]]["seconds"], 1)
            for r in rows(D32 / "a3_32bit_u64_seeds.jsonl")
        ],
        "all_scalars_agree": all(rho32[s].get("rho_agrees") for s in order),
        "withdrawn": "the ic_over_rho 0.19-1.16 figure above compares IC with the ladder's "
                     "unoptimised single walk; against the tuned arm the gap is 295x-1285x",
    }

    cur["verdict"] = "A3_S4_3DECOMP_36BIT_MITM"
    if "research/prime_36bit_20261122/README.md" not in cur["evidence"]:
        cur["evidence"].append("research/prime_36bit_20261122/README.md")
    cur["next_target"] = {
        "summary": "Cut the pair-table build, which is now the dominant stage (B^2/2 pair "
                   "sums, each a full modular inversion through Point::add at "
                   "ec_index_calculus.rs:1371; the 'u64 fast path' only converts the key). "
                   "Batch the inversions (Montgomery's trick) over each pair row before any "
                   "larger rung.",
        "acceptance_gates": [
            "same relation set and same recovered scalar as the inversion-per-pair build",
            "table stage timed separately and charged in the same process",
            "36-bit four-seed row reproduced within 10%",
        ],
    }
    ledger["updated"] = "2026-11-22"
    LEDGER.write_text(json.dumps(ledger, indent=2) + "\n")
    print("ledger updated:", LEDGER)


if __name__ == "__main__":
    main()
