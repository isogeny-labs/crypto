#!/usr/bin/env python3
"""Summarise the pair-sum MITM ladder recordings.

Reads ladder.jsonl, ladder_tuned.jsonl, control_s3.jsonl and
equivalence.jsonl from this directory, writes summary.json, and prints the
tables used in REPORT.md.

    python3 summarise.py
"""

import json
import statistics
from pathlib import Path

HERE = Path(__file__).resolve().parent


def rows(name):
    path = HERE / name
    if not path.exists():
        return []
    out = []
    for line in path.read_text().splitlines():
        line = line.strip()
        if not line:
            continue
        record = json.loads(line)
        if "_meta" in record:
            continue
        out.append(record)
    return out


def cell(r):
    s = r["ic_stages"]
    return {
        "bits": r["bits"],
        "class": r["curve_class"],
        "fb": r["fb_size"],
        "seed": r["target_seed"],
        "total_s": round(r["ic_ms"] / 1000.0, 4),
        "table_s": round(s["pair_table_ms"] / 1000.0, 4),
        "relations_s": round(s["relations_ms"] / 1000.0, 4),
        "la_s": round(s["linear_algebra_ms"] / 1000.0, 4),
        "trials_per_relation": s["trials_per_relation_median"],
        "relations": s["relations_collected"],
        "rho_s": round(r["rho_ms"] / 1000.0, 4),
        "ok": bool(r["ic_ok"] and r["ic_agrees_rho"] and r["ic_matches_truth"]),
    }


def grouped(cells):
    out = {}
    for c in cells:
        out.setdefault((c["bits"], c["class"], c["fb"]), []).append(c)
    return out


def median(cells, key):
    return statistics.median(c[key] for c in cells)


def main():
    fixed = [cell(r) for r in rows("ladder.jsonl")]
    tuned = [cell(r) for r in rows("ladder_tuned.jsonl")]
    control = [cell(r) for r in rows("control_s3.jsonl")]
    equiv = rows("equivalence.jsonl")

    control_by_bits = {c["bits"]: c for c in control}
    paired = []
    for (bits, klass, fb), cells in sorted(grouped(fixed).items()):
        if bits in control_by_bits and fb == 120 and klass == "p256_shape":
            med = median(cells, "total_s")
            s3 = control_by_bits[bits]["total_s"]
            paired.append(
                {
                    "bits": bits,
                    "s3_total_s": s3,
                    "mitm_total_s": round(med, 4),
                    "speedup": round(s3 / med, 2),
                    "seeds": len(cells),
                }
            )

    tuned_summary = []
    for (bits, klass, fb), cells in sorted(grouped(tuned).items()):
        tuned_summary.append(
            {
                "bits": bits,
                "class": klass,
                "fb": fb,
                "seeds": len(cells),
                "median_total_s": round(median(cells, "total_s"), 4),
                "median_table_s": round(median(cells, "table_s"), 4),
                "median_relations_s": round(median(cells, "relations_s"), 4),
                "median_la_s": round(median(cells, "la_s"), 4),
                "all_verified": all(c["ok"] for c in cells),
            }
        )

    summary = {
        "claim_boundary": (
            "public synthetic known-answer fixtures; no deployed curve, no key "
            "recovery, no vs_rho claim, no asymptotic claim; exploratory wall times"
        ),
        "policy_fixed": {
            "fb_size": 120,
            "extra_relations": 8,
            "max_trials_per_relation": 1000000,
            "max_relation_attempts": 64,
            "oracle": "pair_sum_mitm",
        },
        "paired_fixed_fb_vs_s3": paired,
        "tuned": tuned_summary,
        "equivalence_16bit": [
            {
                "seed": e["seed"],
                "s3_ok": e["s3_ok"],
                "mitm_ok": e["mitm_ok"],
                "quartic_ok": e["quartic_ok"],
                "same_scalar": e["same_scalar"],
            }
            for e in equiv
        ],
        "all_rows_verified": all(c["ok"] for c in fixed + tuned),
    }
    (HERE / "summary.json").write_text(json.dumps(summary, indent=1) + "\n")

    print("## paired, fb=120, same host/seed")
    print("| bits | s3 | mitm | speed-up |")
    print("|--:|--:|--:|--:|")
    for p in paired:
        print(f"| {p['bits']} | {p['s3_total_s']:.4f} s | {p['mitm_total_s']:.4f} s | {p['speedup']}x |")
    print("\n## tuned")
    print("| bits | class | fb | seeds | total | table | relations | LA |")
    print("|--:|:--|--:|--:|--:|--:|--:|--:|")
    for t in tuned_summary:
        print(
            f"| {t['bits']} | {t['class']} | {t['fb']} | {t['seeds']} | "
            f"{t['median_total_s']:.3f} s | {t['median_table_s']*1000:.0f} ms | "
            f"{t['median_relations_s']:.3f} s | {t['median_la_s']*1000:.1f} ms |"
        )
    print(f"\nall rows verified: {summary['all_rows_verified']}")


if __name__ == "__main__":
    main()
