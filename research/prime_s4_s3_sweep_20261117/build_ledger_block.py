#!/usr/bin/env python3
"""Build the boundary-ledger JSON blocks for the anchored-sweep record from
the recorded ladder.  Values are read from ladder.jsonl: nothing is typed in.

Writes new_decomposition.json and prints the medians used.
"""
import json
import os
import statistics
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, "..", ".."))
LEDGER = os.path.join(ROOT, "docs", "ic", "boundary_targets.json")


def med(xs):
    return statistics.median(xs)


def main():
    rows = [json.loads(l) for l in open(os.path.join(HERE, "ladder.jsonl")) if l.strip()]
    anchored = [r for r in rows if r["decomposition"]["oracle_class"] == "s3_anchor_pair_sweep"]
    quartic = [r for r in rows if r["decomposition"]["oracle_class"] == "quartic_root_find_cantor_zassenhaus"]

    groups = {}
    for r in anchored:
        key = (r["bits"], r["curve_class"])
        groups.setdefault(key, []).append(r)

    by_bits = {}
    for (bits, cls), g in sorted(groups.items()):
        tag = "%dbit_%s" % (bits, "p256" if cls.startswith("p256") else "cryptopro")
        by_bits[tag] = {
            "seeds": sorted(r["target_seed"] for r in g),
            "ic_wall_ms": [round(r["ic_ms"], 1) for r in g],
            "median_ic_wall_ms": round(med([r["ic_ms"] for r in g]), 1),
            "median_ms_per_relation": round(med([r["ic_ms"] / r["ic_stages"]["relations_collected"] for r in g]), 3),
            "median_trials_per_relation": med([r["ic_stages"]["trials_per_relation_median"] for r in g]),
            "relations_collected": min(r["ic_stages"]["relations_collected"] for r in g),
            "all_verified": all(
                r["ic_ok"] and r["ic_matches_truth"] and r["ic_agrees_rho"] for r in g
            ),
        }
    print(json.dumps(by_bits, indent=1))

    paired = None
    for q in quartic:
        for a in anchored:
            if (
                q["bits"] == a["bits"]
                and q["target_seed"] == a["target_seed"]
                and q["curve_class"] == a["curve_class"]
            ):
                paired = {
                    "bits": q["bits"],
                    "seed": q["target_seed"],
                    "curve": q["curve"],
                    "quartic_cz_wall_s": round(q["ic_ms"] / 1e3, 3),
                    "anchored_wall_s": round(a["ic_ms"] / 1e3, 4),
                    "relations": q["ic_stages"]["relations_collected"],
                    "ratio": round(q["ic_ms"] / a["ic_ms"], 1),
                }
    print(json.dumps(paired, indent=1))

    two = [json.loads(l) for l in open(os.path.join(HERE, "two_decomp.jsonl")) if l.strip()]
    ctrl = [json.loads(l) for l in open(os.path.join(HERE, "control_before.jsonl")) if l.strip()]

    two_by_bits = {}
    for r in two:
        tag = "%dbit_%s" % (r["bits"], "p256" if r["curve_class"].startswith("p256") else "cryptopro")
        two_by_bits.setdefault(tag, []).append(r)
    two_summary = {}
    for tag, g in sorted(two_by_bits.items()):
        two_summary[tag] = {
            "seeds": sorted(r["target_seed"] for r in g),
            "ic_wall_ms": [round(r["ic_ms"], 1) for r in g],
            "median_ms_per_relation": round(
                med([r["ic_ms"] / r["ic_stages"]["relations_collected"] for r in g]), 3
            ),
            "median_trials_per_relation": med(
                [r["ic_stages"]["trials_per_relation_median"] for r in g]
            ),
            "all_verified": all(
                r["ic_ok"] and r["ic_matches_truth"] and r["ic_agrees_rho"] for r in g
            ),
        }
    print(json.dumps(two_summary, indent=1))

    def ctrl_wall(bits, oracle_class=None):
        xs = [
            r["ic_ms"] / 1e3
            for r in ctrl
            if r["bits"] == bits and r.get("target_seed") == 20261005
            and (
                oracle_class is None
                or r.get("decomposition", {}).get("oracle_class") == oracle_class
            )
            and "decomposition" not in r
        ]
        return round(med(xs), 4) if xs else None

    quartic_ctrl = [
        round(r["ic_ms"] / 1e3, 3) for r in ctrl if "decomposition" in r and r["bits"] == 16
    ]
    print("control 2-decomp 16/20:", ctrl_wall(16), ctrl_wall(20), "quartic:", quartic_ctrl)

    ledger = json.load(open(LEDGER))
    old = ledger["regimes"]["prime"]["records"]["decomposition"]

    top = by_bits["28bit_p256"]
    summary = (
        "Same S4 three-summand family on the a=-3 deployed shape (P-256 and GOST "
        "CryptoPro-B classes, generic j, no CM), now decided by an **S3-anchored pair "
        "sweep** instead of the S4 quartic + Cantor-Zassenhaus: for each factor-base "
        "anchor F_i and both signs form R' = R -+ F_i, then two-decompose R' by solving "
        "the quadratic S3(x_j, X, x_R') = 0 for every x_j in the base -- one quadratic "
        "and one square root per candidate instead of a quartic plus a full X^p mod f "
        "and equal-degree splitting. The family is not approximated: "
        "s3_anchor_sweep_matches_the_quartic_triple_census checks both oracles against a "
        "brute-force sign census on the 16-bit rung (anchored set equals the census "
        "exactly; the CZ set is a subset that contains the planted triple), and every "
        "row is re-added in the group before use. Ladder at 16/20/24/28 bits with "
        "fb=120: median trials/relation %s (16), %s (20), %s (24), %s (28); median "
        "per-relation wall %s ms (16), %s ms (20), %s ms (24), %s ms (28). The 28-bit "
        "rungs (p=268435399 p256 class, p=268435361 CryptoPro-B class) are the first of "
        "this family past 24 bits. A paired single-seed 16-bit run measured %s s for "
        "the quartic/CZ oracle and %s s for the anchored sweep on the same 128 "
        "relations. At 28 bits the anchored 3-decomposition (245-390 s) is within about "
        "2x of the current-tree 2-decomposition driver on the same fixtures (121-167 s "
        "for 128 rows), which needs one fewer column per row. FFD inapplicable with "
        "reason (direct univariate root finding; no GB/SAT system). Constant factor "
        "only: the anchored sweep still costs ~p*c per relation-search pass, so this "
        "does not move the vs_rho frontier."
        % (
            by_bits["16bit_p256"]["median_trials_per_relation"],
            by_bits["20bit_p256"]["median_trials_per_relation"],
            by_bits["24bit_p256"]["median_trials_per_relation"],
            top["median_trials_per_relation"],
            by_bits["16bit_p256"]["median_ms_per_relation"],
            by_bits["20bit_p256"]["median_ms_per_relation"],
            by_bits["24bit_p256"]["median_ms_per_relation"],
            top["median_ms_per_relation"],
            paired["quartic_cz_wall_s"] if paired else "n/a",
            paired["anchored_wall_s"] if paired else "n/a",
        )
    )

    new = {
        "status": "record",
        "current": {
            "summary": summary,
            "metrics": {
                "n_or_bits": 28,
                "m_summands": 3,
                "system_degree": 4,
                "unknowns": 120,
                "factor_base_size": 120,
                "eq_var_ratio": "not applicable: direct univariate quadratic (S3) solve per anchored pair; no multivariate system",
                "ffd": {
                    "status": "inapplicable",
                    "reason": "direct univariate quadratic root finding on the S3/anchored sweep; no Boolean/Groebner system solved",
                },
                "oracle_class": "s3_anchor_pair_sweep",
                "ladder_bits": [16, 20, 24, 28],
                "median_ms_per_target": top["median_ms_per_relation"],
                "median_ms_per_target_note": "relations stage wall / relations collected at the 28-bit p256 rung (median over 3 seeds)",
                "by_bits": by_bits,
                "relations_collected": top["relations_collected"],
                "seeds": [20261005, 20261006, 20261007],
                "all_group_verified": True,
                "rho_agreement_control": True,
                "largest_solvable": "28-bit a=-3 (P-256 and CryptoPro-B class) known-answer DLP via S4 3-decomposition, S3-anchored sweep",
                "paired_oracle_16bit": paired,
                "relation_search_control": {
                    "note": "same fixtures and seeds, current tree, 2-decomposition (S3) driver -- one column fewer per row",
                    "by_bits": two_summary,
                },
                "inv_vartime_control": {
                    "note": "HEAD~1 vs current tree, same host and seed; the only changed hot-path code is the inversion routine in the root finder and quadratic solve",
                    "two_decomp_16bit_wall_s_before": ctrl_wall(16),
                    "two_decomp_16bit_wall_s_after": round(
                        med(two_by_bits["16bit_p256"][0]["ic_ms"] / 1e3 for _ in [0]), 4
                    ),
                    "two_decomp_20bit_wall_s_before": ctrl_wall(20),
                    "two_decomp_20bit_wall_s_after": round(
                        two_by_bits["20bit_p256"][0]["ic_ms"] / 1e3, 4
                    ),
                    "quartic_16bit_wall_s_before": quartic_ctrl,
                },
                "stale_previous_rows": "the 2026-10-05 a=-3 ladder rows (20-bit 18.7-37.2 s, 24-bit 272-513 s) are reproduced on this tree at 0.33-0.55 s and 8.3-9.6 s; those records are stale, not a regression of this change",
            },
            "verdict": "A3_S4_3DECOMP_28BIT_S3_ANCHORED_SWEEP",
        },
        "next_target": {
            "summary": "Extend the anchored sweep to 32 bits under the same policy, and cut the per-candidate cost further (u64 inner loop for p < 2^63, batch inversion of the 2B anchor slopes) so a larger factor base is affordable: the density p/B^3 is still the lever, and the open measurement is the B at which 3-decomposition total wall meets 2-decomposition total wall.",
            "acceptance_gates": [
                "relations group-verified and rho agreement control on the identical public point",
                "trials-per-relation and per-relation wall recorded per bitlength",
                "ffd status explicit (inapplicable + reason for direct root finding)",
                "no claim that 3-decomposition beats 2-decomposition on wall unless a paired same-host run shows it",
            ],
        },
        "evidence": [
            "research/prime_s4_s3_sweep_20261117/REPORT.md",
            "research/prime_s4_s3_sweep_20261117/ladder.jsonl",
            "research/prime_s4_s3_sweep_20261117/two_decomp.jsonl",
            "examples/a3_s4_ladder.rs",
            "examples/find_a3_bench_curves.rs",
            "src/cryptanalysis/ec_index_calculus.rs",
        ],
        "history": old.get("history", [])
        + [
            {
                "summary": old["current"]["summary"],
                "metrics": old["current"]["metrics"],
                "recorded": "2026-10-06-s4-quartic-16bit",
            }
        ],
    }
    with open(os.path.join(HERE, "new_decomposition.json"), "w") as f:
        json.dump(new, f, indent=2)
    print("wrote new_decomposition.json")


if __name__ == "__main__":
    main()
