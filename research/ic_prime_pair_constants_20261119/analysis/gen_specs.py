#!/usr/bin/env python3
"""Generate specs/p24.json and specs/p28.json (same curves as
research/ic_fb_size_model_20261119/specs). Usage: gen_specs.py [targets] [rounds] [suffix]"""
import json, sys, copy, pathlib
here = pathlib.Path(__file__).resolve().parent.parent
src = here.parent / "ic_fb_size_model_20261119" / "specs"
T = int(sys.argv[1]) if len(sys.argv) > 1 else 8
R = int(sys.argv[2]) if len(sys.argv) > 2 else 3
suf = sys.argv[3] if len(sys.argv) > 3 else ""
FB = {"p24": (128, (24, 32, 48)), "p28": (256, (48, 64, 96))}
# name: (oracle, targets, linalg, size selector 'F' or 'S'(small), lever note)
ARMS = [
 ("inc",      "mitm-prime-blocked:m=2", "walk", "incremental-gauss", "F"),
 ("inc_aa",   "mitm-prime-blocked:m=2", "walk", "incremental-gauss", "F"),
 ("o_mitm_nf","mitm:negation_folded=1", "walk", "incremental-gauss", "F"),
 ("o_mitm_unf","mitm:m=2",              "walk", "incremental-gauss", "F"),
 ("o_batch",  "mitm-prime-batch:m=2",   "walk", "incremental-gauss", "F"),
 ("o_rfsub",  "rootfind-subtract:m=2",  "walk", "incremental-gauss", "F"),
 ("o_rfmitm", "rootfind-mitm-prime-batch:m=2", "walk", "incremental-gauss", "F"),
 ("o_sub",    "subtract:m=2",           "walk", "incremental-gauss", "F"),
 ("t_dg",     "mitm-prime-blocked:m=2", "stride-renew-deferred-guard", "incremental-gauss", "F"),
 ("t_stride", "mitm-prime-blocked:m=2", "stride", "incremental-gauss", "F"),
 ("t_srenew", "mitm-prime-blocked:m=2", "stride-renew", "incremental-gauss", "F"),
 ("t_pfast",  "mitm-prime-blocked:m=2", "walk-prime-fast", "incremental-gauss", "F"),
 ("t_pb16",   "mitm-prime-blocked:m=2", "walk-prime-batch-16", "incremental-gauss", "F"),
 ("t_random", "mitm-prime-blocked:m=2", "random", "incremental-gauss", "F"),
 ("dg_leader","mitm:negation_folded=1", "stride-renew-deferred-guard", "incremental-gauss", "F"),
 ("l_echelon","mitm-prime-blocked:m=2", "walk", "incremental-gauss-echelon", "F"),
 ("l_fullrank","mitm-prime-blocked:m=2","walk", "incremental-gauss-full-rank", "F"),
 ("k3_a",     "ksum:m=3",               "walk", "incremental-gauss", 0),
 ("k3_b",     "ksum:m=3",               "walk", "incremental-gauss", 1),
 ("k3_c",     "ksum:m=3",               "walk", "incremental-gauss", 2),
]
for c in ("p24","p28"):
    s = json.load(open(src/f"{c}.json"))
    s["label"] = f"prime pair-table pipeline constants, variants at the optimal |F|, curve {c}"
    s["description"] = "research/ic_prime_pair_constants_20261119/PROTOCOL.md"
    s["workloads"]["targets_per_curve"] = T
    s["measurement"]["rounds"] = R
    s["measurement"]["seed"] = 20261120
    s["workloads"]["target_seed"] = 20261120
    rho = s["arms"][0]
    arms = [rho]
    for name, orc, tg, la, sz in ARMS:
        n = FB[c][0] if sz == "F" else FB[c][1][sz]
        arms.append({"name": name, "role": "baseline" if name == "inc" else "candidate",
          "method": {"id": "ic.pipeline", "params": {
            "factor_base": f"prime-abscissa:size={n}", "linalg": la, "oracle": orc, "targets": tg}}})
    s["arms"] = arms
    json.dump(s, open(here/"specs"/f"{c}{suf}.json","w"), indent=1)
