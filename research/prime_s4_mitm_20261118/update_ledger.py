#!/usr/bin/env python3
"""Apply the pair-sum MITM measurements to the prime-regime ledger.

Reads the recordings in this directory (via summarise.py's summary.json plus
the raw jsonl), moves the S3-anchored record to history, and writes the new
`decomposition` current record, the `relation_yield` cost curve and the
`end_to_end_dlp` ladder block.

Idempotent: running twice produces the same file (the script refuses to move
`current` to history twice by checking `oracle_class`).

    python3 update_ledger.py [--check]
"""

import json
import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
REPO = HERE.parent.parent
LEDGER = REPO / "docs/ic/boundary_targets.json"

subprocess.run([sys.executable, str(HERE / "summarise.py")], check=True, capture_output=True)
SUMMARY = json.loads((HERE / "summary.json").read_text())


def read(name):
    out = []
    for line in (HERE / name).read_text().splitlines():
        line = line.strip()
        if not line:
            continue
        record = json.loads(line)
        if "_meta" not in record:
            out.append(record)
    return out


def by_bits(records):
    cells = {}
    for r in records:
        key = f"{r['bits']}bit_{'cryptopro' if 'cryptopro' in r['curve_class'] else 'p256'}"
        s = r["ic_stages"]
        cells.setdefault(key, []).append(
            {
                "seed": r["target_seed"],
                "fb": r["fb_size"],
                "total_ms": round(r["ic_ms"], 3),
                "relations_ms": round(s["relations_ms"], 3),
                "table_ms": round(s["pair_table_ms"], 3),
                "la_ms": round(s["linear_algebra_ms"], 3),
                "trials_per_relation": s["trials_per_relation_median"],
                "verified": bool(r["ic_ok"] and r["ic_agrees_rho"] and r["ic_matches_truth"]),
            }
        )
    return cells


def main():
    check = "--check" in sys.argv
    doc = json.loads(LEDGER.read_text())
    prime = doc["regimes"]["prime"]
    records = prime["records"]
    dec = records["decomposition"]

    if dec["current"]["metrics"].get("oracle_class") == "pair_sum_mitm":
        print("ledger already carries the MITM record; nothing to do")
        return 0

    fixed = by_bits(read("ladder.jsonl"))
    tuned = by_bits(read("ladder_tuned.jsonl"))

    old_current = json.loads(json.dumps(dec["current"]))
    old_current["recorded"] = "2026-11-17/18_pre_mitm"
    dec["history"] = [old_current] + dec.get("history", [])

    speedups = {str(p["bits"]): p["speedup"] for p in SUMMARY["paired_fixed_fb_vs_s3"]}
    tuned_med = {
        f"{t['bits']}_{t['class']}": t for t in SUMMARY["tuned"]
    }

    dec["current"] = {
        "summary": (
            "Same S4 three-summand family on the a=-3 deployed shape (P-256 and "
            "GOST CryptoPro-B classes, generic j, no CM), decided by a **pair-sum "
            "meet-in-the-middle** probe: build B(B+1)/2 pair sums once, keyed by "
            "abscissa, then for each anchor F_k and both signs probe R -+ F_k in "
            "the table. A draw costs 2B hash probes instead of the anchored sweep's "
            "2B^2 quadratic solves, and the shared in-group finaliser re-adds every "
            "candidate over the eight sign patterns, so the decided relation set is "
            "unchanged. Measured on one host: 5.9x / 45.6x / 91x / 89.7x faster than "
            "a paired S3-anchored run at 16/20/24/28 bits (fb=120, same seed). With "
            "B tuned near (2p)^(1/3) (256 at 24 bits, 640 at 28) the 28-bit p256 "
            "rung is 0.60-0.64 s against the recorded S3 255.2 s (approx 446x), and "
            "the relation-stage slope over the two tuned rungs is x6.8 per +4 bits "
            "(descriptive; consistent with the p^(2/3) projection, not a fitted "
            "exponent). This is a throughput change in the relation stage, not a "
            "subexponential decomposition: MITM still spends 2B probes per draw."
        ),
        "metrics": {
            "n_or_bits": 28,
            "m_summands": 3,
            "system_degree": 4,
            "unknowns": 120,
            "factor_base_size": "120 fixed policy; 256 at 24 bits and 640 at 28 bits tuned",
            "eq_var_ratio": "not applicable: hash probe into a prebuilt pair-sum table plus an in-group sign check",
            "ffd": {
                "status": "inapplicable",
                "reason": "direct univariate S3 quadratic per anchor; no Boolean/Groebner system solved",
            },
            "oracle_class": "pair_sum_mitm",
            "ladder_bits": [16, 20, 24, 28],
            "median_ms_per_target": 619.0,
            "median_ms_per_target_note": (
                "relations+table+LA total at the 28-bit p256 rung with fb=640 "
                "(median of 2 seeds); full IC wall 0.60-0.64 s"
            ),
            "policy_fixed_fb120": {
                "fb_size": 120,
                "extra_relations": 8,
                "max_trials_per_relation": 1000000,
                "max_relation_attempts": 64,
                "by_bits": fixed,
                "paired_s3_speedup_by_bits": speedups,
            },
            "policy_tuned": {
                "rule": "B near (2p)^(1/3), clamped below at 120",
                "by_bits": tuned,
            },
            "relations_collected": "128 fixed; 264 at fb=256; 648 at fb=640",
            "all_group_verified": all(
                c["verified"] for group in list(fixed.values()) + list(tuned.values()) for c in group
            ),
            "rho_agreement_control": True,
            "largest_solvable": (
                "28-bit a=-3 (P-256 and CryptoPro-B classes), IC wall 0.60-0.64 s, "
                "paired rho 0.10-0.61 s: still not a vs_rho win"
            ),
            "oracle_equivalence": (
                "examples/prime_mitm_check.rs: S3-anchored, MITM and quartic/CZ recover "
                "the same scalar on three frozen 16-bit targets; every relation "
                "group-verified by try_finalise_s4"
            ),
            "relation_stage_slope_note": (
                "fixed fb=120 relation stage x16 per +4 bits (p-shaped); tuned B gives "
                "x6.8 over the 24->28 rung (2 rungs only, descriptive)"
            ),
        },
        "next_target": {
            "summary": (
                "Extend the MITM ladder to 32 bits (B approx 1600): the blocker is "
                "fixture generation, not the oracle -- the in-tree order counter is "
                "O(p). Then a u64 key/probe path for p < 2^63, and a paired "
                "2-decomposition run under its own tuned B before any cross-family "
                "wall claim."
            ),
            "acceptance_gates": [
                "relations group-verified and rho agreement control on the identical public point",
                "table, relation and LA timers recorded separately per bitlength",
                "ffd status explicit (inapplicable + reason for direct S3 solving)",
                "no subexponential or vs_rho claim; MITM is still super-rho (p^(2/3) projection)",
            ],
        },
        "evidence": [
            "research/prime_s4_mitm_20261118/REPORT.md",
            "research/prime_s4_mitm_20261118/PROTOCOL.md",
            "research/prime_s4_mitm_20261118/summary.json",
            "src/cryptanalysis/ec_index_calculus.rs",
            "examples/prime_mitm_check.rs",
            "examples/a3_s4_ladder.rs",
        ],
    }

    # relation_yield: add the MITM cost curve beside the anchored one.
    ry = records["relation_yield"]["current"]["metrics"]
    ry["a3_s4_mitm_relation_stage_ms_fb120"] = {
        key: round(cells[0]["relations_ms"], 3)
        for key, cells in fixed.items()
        if "p256" in key
    }
    ry["a3_s4_mitm_tuned_relation_stage_ms"] = {
        f"{t['bits']}bit_{t['class']}": round(t["median_relations_s"] * 1000.0, 3)
        for t in SUMMARY["tuned"]
    }
    ry["a3_s4_mitm_policy"] = {
        "oracle": "pair_sum_mitm",
        "fb_fixed": 120,
        "fb_tuned_rule": "B near (2p)^(1/3)",
        "note": "per-relation wall includes the 2B probes; table build charged separately",
    }

    # end_to_end_dlp: record the tuned 28-bit ladder.
    e2e = records["end_to_end_dlp"]["current"]["metrics"]
    e2e["a3_s4_mitm_ladder"] = {
        "bits": [16, 20, 24, 28],
        "policy_fixed_fb": 120,
        "policy_tuned": "B=256 at 24 bits, B=640 at 28 bits",
        "mitm_wall_ms_fixed_fb120": {
            key: round(sum(c["total_ms"] for c in cells) / len(cells), 3)
            for key, cells in fixed.items()
            if "p256" in key
        },
        "mitm_wall_ms_tuned": {
            f"{t['bits']}bit_{t['class']}": round(t["median_total_s"] * 1000.0, 3)
            for t in SUMMARY["tuned"]
        },
        "paired_s3_wall_ms": {
            "16": 42.9,
            "20": 620.6,
            "24": 14637.4,
            "28": 276208.1,
        },
        "rho_wall_ms_range_28bit": [97.0, 605.0],
        "all_verified_and_agree_rho": SUMMARY["all_rows_verified"],
        "note": "not a vs_rho win; 28-bit IC is 1-6x the paired rho wall",
    }

    prime["implementation_caps"]["a3_s4_3decomp_mitm_ladder_bits"] = 28

    if check:
        print("check: ledger would be updated (no write)")
        return 0

    LEDGER.write_text(json.dumps(doc, indent=2) + "\n")
    print(f"updated {LEDGER}")
    print(f"decomposition history entries: {len(dec['history'])}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
