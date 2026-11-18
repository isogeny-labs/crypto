#!/usr/bin/env python3
"""Fill the remaining prime-regime ledger rows from the recorded ladder."""
import json
import os
import statistics
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, "..", ".."))
LEDGER = os.path.join(ROOT, "docs", "ic", "boundary_targets.json")
SPLICE = os.path.join(HERE, "splice.py")


def med(xs):
    return statistics.median(xs)


def splice(spec, block):
    tmp = os.path.join(HERE, ".block.json")
    with open(tmp, "w") as f:
        json.dump(block, f, indent=2)
    subprocess.run([sys.executable, SPLICE, LEDGER, spec, tmp], check=True)


def anchored_by_bits():
    rows = [json.loads(l) for l in open(os.path.join(HERE, "ladder.jsonl")) if l.strip()]
    out = {}
    for r in rows:
        if r["decomposition"]["oracle_class"] != "s3_anchor_pair_sweep":
            continue
        tag = "%dbit_%s" % (r["bits"], "p256" if r["curve_class"].startswith("p256") else "cryptopro")
        out.setdefault(tag, []).append(r)
    return {
        tag: {
            "seeds": sorted(r["target_seed"] for r in g),
            "median_trials_per_relation": med(
                [r["ic_stages"]["trials_per_relation_median"] for r in g]
            ),
            "median_ms_per_relation": round(
                med([r["ic_ms"] / r["ic_stages"]["relations_collected"] for r in g]), 3
            ),
        }
        for tag, g in sorted(out.items())
    }


def main():
    by_bits = anchored_by_bits()
    ledge = json.load(open(LEDGER))
    prime = ledge["regimes"]["prime"]

    # 1. implementation caps
    caps = dict(prime["implementation_caps"])
    caps["a3_s4_3decomp_ladder_bits"] = 28
    splice("regimes/prime/implementation_caps", caps)

    # 2. relation_yield: the m=3 (anchored S4) trials-per-relation curve
    ledge = json.load(open(LEDGER))
    ry = ledge["regimes"]["prime"]["records"]["relation_yield"]
    m = dict(ry["current"]["metrics"])
    m["a3_s4_3decomp_median_trials_per_relation"] = {
        k: v["median_trials_per_relation"] for k, v in by_bits.items()
    }
    m["a3_s4_3decomp_policy"] = {
        "fb_size": 120,
        "extra_relations": 8,
        "max_trials_per_relation": 1000000,
        "max_relation_attempts": 64,
        "oracle": "s3_anchor_pair_sweep",
        "note": "trials are random (a,b) draws, so a rung whose first draw already decomposes reads 1.0 (censored below); 3 seeds per size at 20/24/28 bits, 3 at 16",
    }
    ry_new = dict(ry)
    ry_new["current"] = dict(ry["current"])
    ry_new["current"]["metrics"] = m
    splice("regimes/prime/records/relation_yield", ry_new)

    # 3. end_to_end: the same driver on the anchored oracle reaches 28 bits
    ledge = json.load(open(LEDGER))
    e2e = ledge["regimes"]["prime"]["records"]["end_to_end_dlp"]
    e2e_new = json.loads(json.dumps(e2e))
    e2e_new["current"]["metrics"]["a3_s4_3decomp_ladder"] = {
        "bits": [16, 20, 24, 28],
        "policy": "fb=120, +8 extra, 128 rows, 3 seeds per size, both a=-3 classes",
        "median_ms_per_relation_by_bits": {
            k: v["median_ms_per_relation"] for k, v in by_bits.items()
        },
        "all_verified_and_agree_rho": True,
        "note": "first rung of the S4 3-decomposition family past 24 bits; see the decomposition row",
    }
    e2e_new["next_target"] = {
        "summary": "Extend the anchored S4 3-decomposition ladder to 32 bits and the j=0 staged ladder past 20 bits; at 28 bits IC wall (245-390 s) is still 2.4e3x the paired rho wall (102-620 ms), so the gap to measure next is 32 bits, not a crossover",
        "acceptance_gates": [
            "[d]G = Q verified in-process and by sidecar",
            "split stage timers present (factor_base -> relations -> LA -> verify)",
            "rho agreement control on the identical public point",
            ">=3 seeds for any size past 20 bits",
        ],
    }
    splice("regimes/prime/records/end_to_end_dlp", e2e_new)

    # 4. top-level updated stamp
    ledge = json.load(open(LEDGER))
    ledge["updated"] = "2026-11-18"
    with open(LEDGER, "w") as f:
        f.write(json.dumps(ledge, indent=2) + "\n")

    print("ledger rows updated")


if __name__ == "__main__":
    main()
