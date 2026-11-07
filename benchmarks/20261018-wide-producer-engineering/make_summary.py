#!/usr/bin/env python3
"""Freeze summary.json (read by scripts/build_ic_leaderboard.py) from the raw
run_ab.py JSON files in this directory. Values are copied, not recomputed."""
import json, os
here = os.path.dirname(os.path.abspath(__file__))
L = lambda n: json.load(open(os.path.join(here, n)))
r = lambda s: {"median": round(s["median"], 3), "min": round(s["min"], 3), "max": round(s["max"], 3), "repeats": s["n"]}
scan = []
for k, f, arms in ((40, "scan_k40.json", ("control", "lazy_canon", "prefetch", "all_three")),
                   (300, "scan_k300.json", ("control", "lazy_canon", "prefetch", "all_three")),
                   (600, "scan_k600.json", ("control", "lazy_canon", "prefetch", "all_three"))):
    d = L(f)
    base = d["summary"]["control"]["median"]
    scan.append({"k": k, "states_cap": d["states_cap"], "source": "benchmarks/20261018-wide-producer-engineering/" + f,
                 "arms": {a: dict(r(d["summary"][a]), ratio_to_control=round(d["summary"][a]["median"] / base, 3)) for a in arms},
                 "hits": 0})
idx = []
for k, f in ((300, "index_k300.json"), (600, "index_k600.json")):
    d = L(f)
    base = d["summary"]["serial"]["median"]
    idx.append({"k": k, "source": "benchmarks/20261018-wide-producer-engineering/" + f,
                "arms": {a: dict(r(s), ratio_to_serial=round(s["median"] / base, 3)) for a, s in d["summary"].items()}})
doc = {
    "schema": "crypto.wide_producer_engineering/v1",
    "date": "2026-10-18",
    "class": "engineering",
    "claim": "no end-to-end S claim, no rho claim; wall time of two stages of the wide (64 < n <= 127) relation producer, same binary, interleaved A/B",
    "binary_source": "examples/koblitz_orbit_dlp_fast.rs",
    "n": 73,
    "scan_ns_per_probe": scan,
    "scan_control": "KIC_LAZY_CANON=0 KIC_PREFETCH=0",
    "scan_all_switches": "KIC_LAZY_CANON=1 KIC_PREFETCH=1 KIC_BLOCKED_BLOOM=1 (lazy + prefetch alone: 232.8 vs 226.7 at K=300, same set)",
    "index_build_seconds": idx,
    "index_equality": "KIC_VERIFY_INDEX identical at K=150 (1,642,500 states, 1,642,050 table entries) for 4 threads + batch, 3 threads, batch alone; unit test at n=73 and symmetric n=83",
    "e2e_identity": {"spec": "construct:69:0:60", "source": "benchmarks/20261018-wide-producer-engineering/e2e_n69_k60.txt",
                    "arms": 6, "all_identical_to_control": True, "rank_relations": 60, "targets_solved": 8},
    "host": "unisolated shared 4-vCPU Xeon 2.80 GHz KVM guest, no PMU, load 3 to 6",
    "caveats": ["hits are 0 in every timed scan; the hit path is covered by the n=69 end-to-end run and unit tests",
                "index rows: 3 repeats (K=300), 2 repeats (K=600); no A/A control",
                "scan covers 20,000 to 40,000 states from the start of the state order",
                "the 18,398 s of the landed n=73 K=600 run is the rank-stage wall; the serial index build is about 66 s"],
    "report": "docs/ic/perf/WIDE_PRODUCER_ENGINEERING_20261018.md",
    "commits": ["21dccf843", "0f831d70b"],
}
json.dump(doc, open(os.path.join(here, "summary.json"), "w"), indent=1)
print("ok")
