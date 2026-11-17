#!/usr/bin/env python3
"""Nonce-sampled replay plan for jv_sieve_line_vs_rho_20261103 (validator, 2026-11-11).

Nonce = sha256 of the origin/main commit id at validation time, which the study's producer
(frozen 2026-11-03) could not know. One seed per (arm, prime) for the cover; one rho record
per prime. Prints one shell command per line."""
import hashlib, json, os, sys
NONCE_COMMIT = sys.argv[1]
h = hashlib.sha256(NONCE_COMMIT.encode()).digest()
B = os.environ.get("JV_BIN", "target/release/examples/jv_cover")
R = "research/jv_sieve_line_vs_rho_20261103/results"
O = "research/jv_sieve_line_vs_rho_20261103/review/validator/replay"
env = {"P": "", "H": "JV_FB_LINE=4096 JV_FB_LINE_MODE=horner", "U": "JV_FB_LINE=32768 JV_FB_LINE_MODE=table"}
i = 0; plan = []
for p in (1009, 1511, 1823):
    for arm in "PHU":
        s = 17 + h[i] % 8; i += 1
        plan.append(dict(kind="cover", arm=arm, p=p, seed=s, src=f"{R}/cover/{arm}_{p}.json",
            out=f"{O}/{arm}_{p}_s{s}.json",
            cmd=f"env -u JV_FB_LINE -u JV_FB_LINE_MODE JV_LIN=sparse {env[arm]} {B} --exp sieve --stop 64 --sizes {p} --first-seed {s} --seeds 1 --rho-runs 0 --json {O}/{arm}_{p}_s{s}.json"))
rho = sorted(f for f in os.listdir(f"{R}/rho") if f.endswith(".json"))
for p in (1009, 1511, 1823):
    cands = [f for f in rho if f.startswith(f"rhostrong_p{p}_")]
    f = cands[h[i] % len(cands)]; i += 1
    s, k = f.split("_s")[1].split("_run"); k = k.split(".")[0]
    plan.append(dict(kind="rho", p=p, seed=int(s), run=int(k), src=f"{R}/rho/{f}", out=f"{O}/{f}",
        cmd=f"timeout 3600 {B} --exp rhostrong --sizes {p} --first-seed {s} --seeds 1 --neg 1 --batch 16 --r 256 --dp-bits 16 --run {k} --json {O}/{f}"))
json.dump({"nonce_commit": NONCE_COMMIT, "nonce_sha256": h.hex(), "plan": plan}, open(f"{O}/PLAN.json", "w"), indent=1)
for x in plan: print(x["cmd"])
