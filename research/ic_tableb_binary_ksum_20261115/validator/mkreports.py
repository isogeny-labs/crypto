#!/usr/bin/env python3
"""Assemble the validator's replay certificate and validator report (gate format).

Usage: mkreports.py CONTEXT_JSON
CONTEXT_JSON names: repo, study_rel, validator_rel, bench_rel, own_numbers, replay_dir,
fresh_dirs, nonce_file, build (dict), sample (list), recompute_script.
"""
import gzip, hashlib, json, os, sys

ctx = json.load(open(sys.argv[1]))
REPO = ctx["repo"]
STUDY = ctx["study_rel"]
VAL = ctx["validator_rel"]
BENCH = ctx["bench_rel"]


def sha(p):
    with open(p, "rb") as f:
        return hashlib.sha256(f.read()).hexdigest()


def rel_from(base_rel, target_rel):
    return os.path.relpath(os.path.join(REPO, target_rel), os.path.join(REPO, base_rel))


own = json.load(open(ctx["own_numbers"]))
res = json.load(open(os.path.join(REPO, STUDY, "results.json")))
prod = {str(r["bits"]): r for r in res["rows"]}

# ---------------------------------------------------------------- replay certificate
runs = []
all_ok = True
for s in sorted(own["sessions"]):
    if not s.startswith("B"):
        continue
    receipt_rel = f"{VAL}/replay/{s}.verify.json.gz"
    v = json.load(gzip.open(os.path.join(REPO, receipt_rel)))
    rc = int(open(os.path.join(ctx["replay_dir"], f"{s}.rc")).read().strip())
    reproduced = sum(1 for x in v["replays"] if x.get("reproduced"))
    ok = bool(v["ok"] and rc == 0 and not v["problems"] and reproduced == len(v["replays"]) == v["replay_candidates"]
              and v["verified_records"] == v["records"])
    all_ok &= ok
    sm = own["sessions"][s]
    runs.append({
        "run": f"{STUDY}/sessions/{s}",
        "session_id": v["session_id"],
        "ok": ok,
        "exit_code": rc,
        "receipt": receipt_rel,
        "receipt_sha256": sha(os.path.join(REPO, receipt_rel)),
        "records": v["records"],
        "verified_records": v["verified_records"],
        "replays": len(v["replays"]),
        "reproduced": reproduced,
        "replay_selection": v["replay_selection"],
        "problems": v["problems"],
        "checks_all_true": all(v["checks"].values()),
        "auditor_binary_sha256": v["auditor_binary_sha256"],
        "auditor_hardware_class_id": v.get("auditor_hardware_class_id"),
        "nonce_sampled": s in ctx["sample"],
        "scalars_independently_checked": sm["records"],
        "recorded_by": f"ecbench {sm['session_binary_sha256'][:12]} at git {sm['git_commit'][:10]} (producer, this container)",
    })
nonce = dict(l.strip().split("=", 1) for l in open(ctx["nonce_file"]) if "=" in l)
cert = {
    "schema": "ecbench.replay-certificate/v1",
    "status": "verified" if all_ok and own["certificates"].get("failures", 0) == 0 else "failed",
    "independent": True,
    "scalars_verified": own["certificates"].get("failures", 0) == 0 and own["certificates"]["records_checked"] > 0,
    "paths_relative_to": "repository root",
    "study": STUDY,
    "method": ("ecbench verify --replay-all --exit-code on copies of every one of the 15 committed sessions "
               "(records.jsonl.gz gunzipped into a scratch directory; the committed files were not touched), "
               "with an ecbench the validator built itself from the recorded source (cargo build --release --locked "
               "--bin ecbench, separate CARGO_TARGET_DIR); every measured deterministic run re-executed and compared "
               "with its record, every recorded scalar checked by ecbench against its target. Independently of "
               "ecbench, every recovered log of every record (warm-ups included) was re-checked with the validator's "
               "own Python GF(2^n) curve arithmetic: [recovered]G == target, target on the curve, generator of prime "
               "order r. A nonce drawn after publication chose the designated sample (all 15 were replayed anyway), "
               "and the seeds of two fresh validator sessions."),
    "nonce": {"git_ls_remote_origin_main": nonce.get("ls_remote"), "utc": nonce.get("utc"),
              "sha256": nonce.get("nonce"),
              "rule": "sha256(ls-remote line + newline + UTC + newline); B27 replicate = int(n[0:8],16) mod 3 + 1; "
                      "second session = int(n[8:16],16) mod 12 over B15..B24 r1..r3; third = int(n[16:24],16) mod 11 "
                      "over the rest; fresh curve = [B24,B27][int(n[24:32],16) mod 2]",
              "designated_sample": ctx["sample"]},
    "build": ctx["build"],
    "independence": {
        "independent_of_producer": [
            "validator context (did not read analysis/*.mjs, summary.json or the producer's interval code)",
            "ecbench rebuilt from source by the validator",
            "replay nonce and fresh-session seeds drawn after publication",
            "scalar certificates re-checked by the validator's own Python GF(2^n) arithmetic",
        ],
        "not_independent": [
            "same host/container class as the producer's run (not a second host class, unlike the prime twin)",
            "the validator is a subagent launched from the producing session (same Claude-Session id as the "
            "study commits); independence is of context, code, build and seeds, not of operator",
            "ecbench's own replay shares the producer's source code (it is a re-execution, not a re-implementation)",
        ],
    },
    "scalar_certificates": own["certificates"],
    "fresh_sessions": ctx.get("fresh_summary", {}),
    "runs": runs,
}
cert_bytes = (json.dumps(cert, indent=1) + "\n").encode()
for d in (VAL, BENCH):
    with open(os.path.join(REPO, d, "replay-certificate.json"), "wb") as f:
        f.write(cert_bytes)

# ---------------------------------------------------------------- headline
heads = []


def H(name, claimed, recomputed, tol):
    heads.append({"name": name, "claimed": claimed, "recomputed": recomputed, "rel_tol": tol})


for b in ["21", "24", "27", "15", "18"]:
    ro, pr = own["rows"][b], prod[b]
    for cand in ["ksum5", "ksum4"]:
        for w in ["0", "0.1", "1"]:
            H(f"B{b}/{cand}_over_strictest_incumbent/w{w}", pr["weights"][w][cand]["over_incumbent"]["ratio"],
              ro["candidates"][cand]["by_weight"][w]["ratio"], 1e-9)
            for i, p in enumerate(pr["weights"][w][cand]["per_replicate"]):
                H(f"B{b}/{cand}_over_own_strictest/r{i + 1}/w{w}", p["ratio"],
                  ro["candidates"][cand]["by_weight"][w]["per_replicate"][i]["ratio"], 1e-9)
            H(f"B{b}/{cand}_gap_to_rho_neg/w{w}", pr["weights"][w][cand]["gap_to_rho"],
              ro["candidates"][cand]["by_weight"][w]["gap_to_rho_neg"], 1e-9)
        H(f"B{b}/{cand}_max_upper_pooled (interval implementations differ)", pr[cand]["max_upper_pooled"],
          ro["candidates"][cand]["max_upper_pooled_protocol_reading"], 0.02)
        H(f"B{b}/{cand}_max_upper_any_replicate (interval implementations differ)", pr[cand]["max_upper_replicate"],
          ro["candidates"][cand]["max_upper_any_replicate_protocol_reading"], 0.02)
        H(f"B{b}/{cand}_L1_holds (1 = true)", 1.0 if pr[cand]["holds"] else 0.0,
          1.0 if ro["candidates"][cand]["L1_holds_protocol_reading"] else 0.0, 0.0)
    H(f"B{b}/delta_replicate_noise", pr["delta"], ro["delta"], 1e-9)
    for w in ["0", "0.1", "1"]:
        for arm in ["ksum5", "rho-neg"]:
            H(f"B{b}/mean_S/{arm}/w{w}", pr["weights"][w]["S"][arm], ro["mean_S_pooled"][arm][w], 1e-9)

bad = [h for h in heads if abs(h["claimed"] - h["recomputed"]) > h["rel_tol"] * max(abs(h["claimed"]), abs(h["recomputed"]), 1e-300)]

raw_rel = [f"{STUDY}/sessions/{s}/records.jsonl.gz" for s in sorted(own["sessions"]) if s.startswith("B")]
code = {os.path.basename(p): sha(p) for p in ctx["validator_code"]}


def report(base_rel, copy_of=None):
    rep = {
        "schema": "validation-gate.validator-report/v1",
        "status": "pass" if (not bad and cert["status"] == "verified" and ctx["integrity_ok"]) else "fail",
        "independent": True,
        "study": STUDY,
        "validated_commit": ctx["validated_commit"],
        "method": ctx["method"],
        "validator_code_sha256": code,
        "raw_data": [{"path": rel_from(base_rel, p), "sha256": sha(os.path.join(REPO, p))} for p in raw_rel],
        "headline": heads,
        "headline_mismatches": bad,
        "checks": ctx["checks"],
        "validation_report": ctx["validation_report"],
    }
    if copy_of:
        rep["copy_of"] = copy_of
    return rep


val_rep = report(VAL)
vb = (json.dumps(val_rep, indent=1) + "\n").encode()
with open(os.path.join(REPO, VAL, "validator-report.json"), "wb") as f:
    f.write(vb)
bench_rep = report(BENCH, copy_of={"path": rel_from(BENCH, f"{VAL}/validator-report.json"),
                                   "sha256": hashlib.sha256(vb).hexdigest(),
                                   "note": "identical except raw_data paths rebased to this directory and this copy_of block"})
with open(os.path.join(REPO, BENCH, "validator-report.json"), "w") as f:
    f.write(json.dumps(bench_rep, indent=1) + "\n")
print("cert status", cert["status"], "report status", val_rep["status"], "headline", len(heads), "mismatches", len(bad))
