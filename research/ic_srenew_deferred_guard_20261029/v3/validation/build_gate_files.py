#!/usr/bin/env python3
"""Validator: build replay-certificate.json, validator-report.json and
VALIDATION.draft.json for XP-20261029-srenew-deferred-guard-v3 (24 and 32
bits) from the validator's own outputs.

Inputs (read-only):
  <data>/P<b>/eks-results/...       the worker files (byte-identical to the S3
                                    originals and to the committed evidence)
  <data>/P<b>/analysis/compare-*.json, gate_v3.json   producer claims
  <vdir>/recompute-P<b>.json, mechanism-P<b>.json      validator recompute
  <vdir>/replay-P<b>.audit.json                        validator replay
  <vdir>/band-P<b>.ecbench.json                        ecbench band re-run by the validator
Paths written into the files are relative to the evidence directory
research/ic_srenew_deferred_guard_20261029 (validator files under
v3/validation/).

Usage: python3 -I build_gate_files.py <data_dir> <vdir> <val_id>
"""
import hashlib
import importlib.util
import json
import os
import sys

DATA, VDIR, VAL_ID = sys.argv[1], sys.argv[2], sys.argv[3]
SIZES = (24, 32)
EVID = "research/ic_srenew_deferred_guard_20261029"
VREL = "v3/validation"
POD = {24: "839dfb99-d7e2-4de7-aa8e-c067afa1191e", 28: "44c7bbd0-4d75-4625-9996-ba992ec3dd98",
       32: "bd38514c-48dc-49a0-82e0-9986502d8196"}
S3 = {b: f"s3://isolab-results-104186811093-us-east-2/runs/ic-dg3-p{b}-20261029/{POD[b]}/" for b in POD}
WEIGHTS = (("0", "0.0"), ("0.1", "0.1"), ("1", "1.0"))
PAIRS = ["walk__dg-small", "walk__walk-aa", "rho-neg__rho-neg-aa", "rho-neg__walk", "rho-neg__dg-small",
         "srenew__dg", "srenew__srenew-small", "srenew-small__dg-small", "walk__srenew"]
BIN = "88070375e77648aa12a561ff9ee59e34d469c55e1da56987234811b0956e412c"
SNAPSHOT = {
    "repository": "https://github.com/aburan28/crypto (reports itself as isogeny-labs/crypto)",
    "branch": "claude/ic-srenew-deferred-guard-run-20261029",
    "head_commit_at_validation": "727cd0436477976ce7bb64a2f39ec35544099075",
    "evidence_commit": "868fca5568aa806aeda7a2ae8bf1eb24907a8b43",
    "protocol_freeze_commit": "e277272384d683a4d4b9983a141b523bf28c9f4a",
    "p32_trigger_commit": "a767145ff66cfb5ffc341288db0a3917b681ba2e",
    "evidence_committed": True,
    "pushed": True,
    "note": ("Every committed eks-results and analysis file of v3/P24, v3/P28 and v3/P32 (blobs, and the sha256 "
             "of every .artifacts.json entry) is byte-identical to the worker's S3 originals, which the validator "
             "streamed through sha256sum (validation provenance.txt). The protocol freeze was pushed at "
             "07:56:20Z, before the first execution (P24 session start 08:02:08Z); the P32 trigger was pushed "
             "at 08:01:04Z, before the P32 session started (08:06:58Z)."),
}


def sha(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def load(path):
    with open(path) as f:
        return json.load(f)


sys.dont_write_bytecode = True
spec = importlib.util.spec_from_file_location("recompute", os.path.join(VDIR, "recompute.py"))
rc = importlib.util.module_from_spec(spec)
spec.loader.exec_module(rc)

rec = {b: load(os.path.join(VDIR, f"recompute-P{b}.json")) for b in (24, 28, 32)}
mech = {b: load(os.path.join(VDIR, f"mechanism-P{b}.json")) for b in SIZES}
rep = {b: load(os.path.join(VDIR, f"replay-P{b}.audit.json")) for b in SIZES}
band = {b: load(os.path.join(VDIR, f"band-P{b}.ecbench.json")) for b in SIZES}
prod_audit = {b: load(os.path.join(DATA, f"P{b}/eks-results/audit.json")) for b in SIZES}
gate3 = {b: load(os.path.join(DATA, f"P{b}/analysis/gate_v3.json")) for b in SIZES}
cmpf = {(b, p, ws): load(os.path.join(DATA, f"P{b}/analysis/compare-{p}-w{ws}.json"))
        for b in SIZES for p in PAIRS for ws, _ in WEIGHTS}

# ------------------------------------------------------------------ runs
runs = []
for b in SIZES:
    sdir = os.path.join(DATA, f"P{b}/eks-results/session")
    ec = {s: rc.Curve(v["p"], v["a"], v["b"]) for s, v in rec[b]["curves"].items()}
    replays = {r["record_id"]: r for r in rep[b]["replays"]}
    with open(os.path.join(sdir, "records.jsonl")) as f:
        for line in f:
            x = json.loads(line)
            if x["warmup"]:
                continue
            wl = x["workload"]
            E = ec[wl["curve"]["slug"]]
            ok = E.mul(int(x["outcome"]["recovered"]), rc.pt(wl["curve"]["generator"])) == rc.pt(wl["target"])
            rp = replays.get(x["record_id"], {})
            runs.append({"session": f"v3/P{b}", "seq": x["seq"], "record_id": x["record_id"], "arm": x["arm"],
                         "curve": wl["curve"]["slug"], "target_index": wl["target_index"], "round": x["round"],
                         "recovered": x["outcome"]["recovered"], "validator_dG_equals_Q": ok,
                         "reproduced": rp.get("reproduced"), "detail": rp.get("detail")})
assert len(runs) == 2 * 2304
assert all(r["validator_dG_equals_Q"] and r["reproduced"] is True and r["detail"] == "identical" for r in runs)

sessions = []
for b in SIZES:
    sd = os.path.join(DATA, f"P{b}/eks-results/session")
    files = {n: sha(os.path.join(sd, n)) for n in ("host.json", "plan.json", "records.jsonl", "session.json", "spec.json")}
    before = dict(reversed(l.split()) for l in open(os.path.join(VDIR, f"replay-P{b}.session-files.before.sha256")))
    after = dict(reversed(l.split()) for l in open(os.path.join(VDIR, f"replay-P{b}.session-files.after.sha256")))
    a = rep[b]
    sessions.append({
        "session_dir": f"v3/P{b}/eks-results/session", "session_id": a["session_id"],
        "job": f"ic-dg3-p{b}-20261029", "pod_uid": POD[b], "s3_uri": S3[b] + "session/",
        "receipt": f"{VREL}/replay-P{b}.audit.json",
        "receipt_sha256": sha(os.path.join(VDIR, f"replay-P{b}.audit.json")),
        "verify_log": f"{VREL}/replay-P{b}.verify.log",
        "records": a["records"], "verified_records": a["verified_records"], "replays": len(a["replays"]),
        "replays_reproduced_identical": sum(1 for r in a["replays"] if r["reproduced"] and r["detail"] == "identical"),
        "replays_not_identical": sum(1 for r in a["replays"] if not (r["reproduced"] and r["detail"] == "identical")),
        "problems": a["problems"], "checks": a["checks"], "ok": a["ok"], "regraded": a.get("regraded"),
        "session_files_sha256": files,
        "replay_copy_sha256_before_equals_after": before == after,
        "replay_copy_equals_evidence": before == files,
        "producer_receipt": f"v3/P{b}/eks-results/audit.json",
        "producer_receipt_sha256": sha(os.path.join(DATA, f"P{b}/eks-results/audit.json")),
        "producer_replays_identical": sum(1 for r in prod_audit[b]["replays"] if r["reproduced"] and r["detail"] == "identical"),
        "producer_auditor_binary_sha256": prod_audit[b]["auditor_binary_sha256"],
        "producer_auditor_env_class_id": prod_audit[b]["auditor_env_class_id"],
        "auditor_binary_sha256": a["auditor_binary_sha256"],
        "auditor_env_class_id": a["auditor_env_class_id"],
        "auditor_hardware_class_id": a.get("auditor_hardware_class_id"),
        "measured_env_class_id": rec[b]["session"]["env_class_id"],
        "audited_unix_ms": a["audited_unix_ms"],
        "end_line": open(os.path.join(VDIR, f"replay-P{b}.verify.log")).read().strip().splitlines()[-1],
        "stderr": open(os.path.join(VDIR, f"replay-P{b}.verify.stderr")).read().strip().splitlines(),
    })
    assert before == after == files and a["ok"] and a["problems"] == []

rows = sorted(s for b in SIZES for s in rec[b]["curves"])
cert = {
    "schema": "replay-certificate/v1",
    "evidence_dir": EVID,
    "rows": rows,
    "snapshot": SNAPSHOT,
    "status": "verified",
    "independent": True,
    "scalars_verified": True,
    "independence": {
        "replayer": ("Validator subagent (claude-opus-5-5), separate from the producer and analyst sessions; "
                     "re-executed every measured run of both sessions into new receipts on byte-identical copies "
                     "under /dev/shm/dg3val; producer receipts untouched"),
        "same_binary_as_measurement": True,
        "same_host_class_as_measurement": False,
        "independent_implementation": False,
        "independent_scalar_check": True,
        "why_same_binary": ("TargetSource::StrideRenewDeferredGuard first exists at be2969ce (PR #2831); every older "
                            "ecbench lacks the target source, so a cross-binary replay is impossible. The replay binary "
                            "is the measuring binary by sha256 (88070375), not a rebuild."),
        "statement": ("Same-binary, cross-host replay: the worker measured with ecbench 88070375 (image, source "
                      "be2969ce) on EKS c8a.2xlarge (env ECBENV3hc1c5e5126853); the validator replayed with the same "
                      "binary on a different host (env ECBENV3h13e37662a118, hardware ECBHW1he9e33daae9e7) and all "
                      "2304 measured runs of each session reproduced identical counts. Every recovered scalar of every "
                      "measured run re-verifies [d]G = Q under the validator's own affine arithmetic. Not an "
                      "independent implementation of the IC pipeline."),
    },
    "binary": {"replay_binary_path": "/dev/shm/isolab-build/ecbench", "replay_binary_sha256": BIN,
               "measuring_binary_sha256": BIN,
               "measuring_binary_source_revision": "be2969cebf30a0cfb79088fd10094324c3efb3bb",
               "equals_session_binary_sha256": True},
    "command": ("RAYON_NUM_THREADS=1 taskset -c <2|3> /dev/shm/isolab-build/ecbench verify --dir "
                "/dev/shm/dg3val/P<b>/session --replay-all --exit-code --out replay-P<b>.audit.json"),
    "independent_scalar_check": {
        "method": ("validation recompute.py: curve a, b derived from plan.json points; p and r prime; all points on the "
                   "curve; [r]G = O; r the only multiple of r in the Hasse interval (so #E = r); j-invariant equal to "
                   "the ICV1 string's; [d]G = Q in affine coordinates for every record of every arm, warm-ups included"),
        "curves": {s: c for b in SIZES for s, c in rec[b]["curves"].items()},
        "records_checked": {f"v3/P{b}": rec[b]["dG_eq_Q"]["checked"] for b in SIZES},
        "failures": [f for b in SIZES for f in rec[b]["dG_eq_Q"]["failures"]],
    },
    "sessions": sessions,
    "runs": runs,
}
with open(os.path.join(VDIR, "replay-certificate.json"), "w") as f:
    body = json.dumps({k: v for k, v in cert.items() if k != "runs"}, indent=1)
    f.write(body[:-2] + ',\n "runs": [\n')
    f.write(",\n".join("  " + json.dumps(r, separators=(",", ":")) for r in runs))
    f.write("\n ]\n}\n")

# ------------------------------------------------------------------ headline
headline = []
for b in SIZES:
    for p in PAIRS:
        a_arm, b_arm = p.split("__")
        for ws, wk in WEIGHTS:
            c = cmpf[(b, p, ws)]
            mine = rec[b]["ratios"][p][wk]
            src = f"v3/P{b}/analysis/compare-{p}-w{ws}.json"
            headline.append({"name": f"P{b} sum S_{b_arm} / sum S_{a_arm} at w={ws} (288 pairs, 48 targets, 2 curves)",
                             "claimed": c["ops"]["ratio_b_over_a"], "recomputed": mine["ratio_b_over_a"], "rel_tol": 1e-12,
                             "claimed_source": src + " ops.ratio_b_over_a",
                             "recomputed_by": "validation recompute.py (stdlib, records.jsonl read directly, ecbench summation order)"})
            for i, side in ((0, "lower"), (1, "upper")):
                headline.append({"name": f"P{b} {b_arm}/{a_arm} 95% cluster-bootstrap {side} bound at w={ws}",
                                 "claimed": c["ops"]["ci95_bootstrap"][i], "recomputed": mine["ci95_ecbench_replica"][i],
                                 "rel_tol": 1e-12, "claimed_source": src + " ops.ci95_bootstrap (ecbench, 4000, seed 20261002)",
                                 "recomputed_by": ("validation recompute.py boot_ecbench: the validator's stdlib re-implementation, "
                                                   "from the source of stats.rs/canonical.rs, of the two-stage splitmix64 bootstrap")})
            for cv in c["curves"]:
                mc = mine["per_curve"][cv["slug"]]
                headline.append({"name": f"P{b} {cv['slug']} sum S_{b_arm} / sum S_{a_arm} at w={ws}",
                                 "claimed": cv["ratio_b_over_a"], "recomputed": mc["ratio_b_over_a"], "rel_tol": 1e-12,
                                 "claimed_source": src + " curves[].ratio_b_over_a", "recomputed_by": "validation recompute.py"})
                for i, side in ((0, "lower"), (1, "upper")):
                    headline.append({"name": f"P{b} {cv['slug']} {b_arm}/{a_arm} cluster-bootstrap {side} bound at w={ws}",
                                     "claimed": cv["ci95_bootstrap"][i], "recomputed": mc["ci95_ecbench_replica"][i],
                                     "rel_tol": 1e-12, "claimed_source": src + " curves[].ci95_bootstrap",
                                     "recomputed_by": "validation recompute.py boot_ecbench"})
            if ws == "0":
                for side, arm in (("a", a_arm), ("b", b_arm)):
                    headline.append({"name": f"P{b} mean S (w = 0) of {arm} over the 288 pairs of {p}",
                                     "claimed": c[side]["mean_s"], "recomputed": mine[f"mean_s_{side}"], "rel_tol": 1e-12,
                                     "claimed_source": src + f" {side}.mean_s (unweighted; read by the leaderboard component)",
                                     "recomputed_by": "validation recompute.py"})
    for row in band[b]["rows"]:
        for i, (ws, wk) in enumerate(WEIGHTS):
            headline.append({"name": f"P{b} band mean S {row['arm']} on {row['curve']} at w={ws}",
                             "claimed": row["mean_s"][i], "recomputed": rec[b]["band_mean_s"][row["curve"]][row["arm"]][wk],
                             "rel_tol": 1e-12,
                             "claimed_source": (f"ecbench band --dir v3/P{b}/eks-results/session --ranks --json, re-run by the "
                                                f"validator; its Markdown is byte-identical to v3/P{b}/analysis/band.md"),
                             "recomputed_by": "validation recompute.py"})
    g = gate3[b]
    mine = rec[b]["gate_v3"]
    for name, claimed, got in (
            ("executions below L2 (l2-gate.json)", g["gate"]["below_l2_len"], mine["below_l2_from_records"]),
            ("rho executions waived under condition 5", g["records"]["rho_waived_count"],
             sum(v["count"] for v in mine["condition5_rho_waived_by_arm"].values())),
            ("IC executions below L2", g["records"]["ic_executions_below_l2"], len(mine["condition4_ic_executions_below_l2"])),
            ("measured replays identical in the worker audit", g["cond1_facts"]["replays_identical_and_reproduced"],
             next(s["replays_reproduced_identical"] for s in sessions if s["session_dir"] == f"v3/P{b}/eks-results/session"))):
        headline.append({"name": f"P{b} v3 section 6.1: {name}", "claimed": claimed, "recomputed": got, "rel_tol": 0.0,
                         "claimed_source": f"v3/P{b}/analysis/gate_v3.json",
                         "recomputed_by": "validation recompute.py from records.jsonl (and the validator's own replay for the last)"})
bad = [h for h in headline if abs(h["claimed"] - h["recomputed"]) > h["rel_tol"] * max(abs(h["claimed"]), abs(h["recomputed"]), 1e-300)]
assert not bad, bad

# README decision table (4 decimals) and rho distances (2 decimals)
readme = {24: {"0": (0.9105, 0.8824, 0.9402), "0.1": (0.8884, 0.8594, 0.9189), "1": (0.7848, 0.7519, 0.8197)},
          32: {"0": (0.9406, 0.9144, 0.9681), "0.1": (0.9187, 0.8918, 0.9468), "1": (0.8189, 0.7897, 0.8496)}}
readme_rho = {(24, "walk"): {"0": (12.48, 11.60, 13.41), "0.1": (12.49, 11.60, 13.44), "1": (12.56, 11.60, 13.58)},
              (24, "dg-small"): {"0": (11.36, 10.55, 12.26), "0.1": (11.10, 10.30, 11.99), "1": (9.86, 9.10, 10.68)},
              (32, "walk"): {"0": (32.07, 29.48, 34.87), "0.1": (32.00, 29.42, 34.80), "1": (31.71, 29.11, 34.50)},
              (32, "dg-small"): {"0": (30.16, 27.61, 32.93), "0.1": (29.40, 26.91, 32.11), "1": (25.96, 23.73, 28.37)}}
rcheck = []
for b in SIZES:
    for ws, wk in WEIGHTS:
        m = rec[b]["ratios"]["walk__dg-small"][wk]
        got = (round(m["ratio_b_over_a"], 4), round(m["ci95_ecbench_replica"][0], 4), round(m["ci95_ecbench_replica"][1], 4))
        rcheck.append({"size": b, "pair": "walk__dg-small", "w": ws, "readme": readme[b][ws], "recomputed_rounded": got,
                       "agree": got == readme[b][ws]})
        for arm in ("walk", "dg-small"):
            m = rec[b]["ratios"][f"rho-neg__{arm}"][wk]
            got = (round(m["ratio_b_over_a"], 2), round(m["ci95_ecbench_replica"][0], 2), round(m["ci95_ecbench_replica"][1], 2))
            rcheck.append({"size": b, "pair": f"rho-neg__{arm}", "w": ws, "readme": readme_rho[(b, arm)][ws],
                           "recomputed_rounded": got, "agree": got == readme_rho[(b, arm)][ws]})

# ------------------------------------------------------------------ band for the gate
gae_band = {}
for b in SIZES:
    r = rec[b]["ratios"]
    gae_band[f"P{b}"] = {}
    for ws, wk in WEIGHTS:
        gae_band[f"P{b}"][ws] = {
            "mean_s_walk": r["walk__dg-small"][wk]["mean_s_a"], "mean_s_dg_small": r["walk__dg-small"][wk]["mean_s_b"],
            "mean_s_rho_neg": r["rho-neg__walk"][wk]["mean_s_a"],
            "dg_small_over_walk": r["walk__dg-small"][wk]["ratio_b_over_a"],
            "dg_small_over_walk_ci95_ecbench": cmpf[(b, "walk__dg-small", ws)]["ops"]["ci95_bootstrap"],
            "dg_small_over_walk_ci95_validator_own_two_stage": r["walk__dg-small"][wk]["ci95_own_two_stage"],
            "walk_over_rho_neg": r["rho-neg__walk"][wk]["ratio_b_over_a"],
            "walk_over_rho_neg_ci95_ecbench": cmpf[(b, "rho-neg__walk", ws)]["ops"]["ci95_bootstrap"],
            "dg_small_over_rho_neg": r["rho-neg__dg-small"][wk]["ratio_b_over_a"],
            "dg_small_over_rho_neg_ci95_ecbench": cmpf[(b, "rho-neg__dg-small", ws)]["ops"]["ci95_bootstrap"],
        }

raw = []
for b in SIZES:
    for n in ("records.jsonl", "plan.json", "session.json", "spec.json", "host.json"):
        p = f"v3/P{b}/eks-results/session/{n}"
        raw.append({"path": p, "sha256": sha(os.path.join(DATA, f"P{b}/eks-results/session/{n}")),
                    "s3": S3[b] + "session/" + n,
                    **({"store": "tools/artifacts (hydrate with artifacts.py pull)"} if n in ("records.jsonl", "plan.json") else {})})
    for n in ("SHA256SUMS", "audit.json", "l2-gate.json", "input-spec.json", "build-info.json", "job-exit.json"):
        p = f"v3/P{b}/eks-results/{n}"
        raw.append({"path": p, "sha256": sha(os.path.join(DATA, f"P{b}/eks-results/{n}")), "s3": S3[b] + n,
                    **({"store": "tools/artifacts (hydrate with artifacts.py pull)"}
                       if n in ("audit.json",) or (n == "l2-gate.json" and b == 24) else {})})

r24, r32 = rec[24]["ratios"]["walk__dg-small"], rec[32]["ratios"]["walk__dg-small"]
report = {
    "schema": "validator-report/v1",
    "id": VAL_ID,
    "status": "pass",
    "verdict": "passed",
    "independent": True,
    "validator": "Validator subagent (claude-opus-5-5); did not produce the protocol, the runs, the analysis or the claim",
    "claim": ("XP-20261029-srenew-deferred-guard-v3: at 24 and 32 bits, ic.pipeline with targets = "
              "stride-renew-deferred-guard at N_b (dg-small; 130 / 741) has lower whole-pipeline S than the walk "
              "baseline (targets = walk, N 162 / 1024) under v3 section 6: instrument (v3 6.1), completeness, the 95% "
              "cluster-bootstrap upper bound of sum S_dg-small / sum S_walk below 1 at w = 0, 0.1 and 1, and "
              "band --ranks dg-small < walk on every curve"),
    "claim_verdict": {
        "24": "improvement: v3 6.1 and section 6.2-6.4 hold; validator: pass",
        "32": "improvement: v3 6.1 and section 6.2-6.4 hold; validator: pass with caveats (see caveats)",
        "28": ("not an improvement, as the producer reports: 6.1-6.3 hold (upper bounds 0.9756 / 0.9625 / 0.9016) but "
               "band --ranks is unresolved on icv1-fp28-tm7951-a51cdbf3 (dg-small 20.844 > srenew 20.646 at w = 0, "
               "lower at 0.1 and 1); the validator agrees and validates nothing else at 28 bits"),
    },
    "snapshot": SNAPSHOT,
    "method": ("records.jsonl, plan.json and the frozen spec read directly by validation recompute.py (stdlib, no "
               "ecbench code): priced S(w) = (G + w U)/sqrt(r), C = n_C = 0 (no record names a pinned calibration "
               "unit), U per band.rs; paired ratios of totals in ecbench's summation order; a from-source replica of "
               "ecbench's two-stage cluster bootstrap (bitwise equal to the analyst's intervals) beside the validator's "
               "own Mersenne-Twister two-stage and one-stage bootstraps; own affine EC arithmetic for [d]G = Q on every "
               "record; the v3 6.1 gate recounted from records.jsonl. ecbench band re-run (byte-identical to band.md). "
               "Local verify --replay-all of both sessions on another host. Mechanism split by mechanism.py."),
    "raw_data": raw,
    "headline": headline,
    "readme_claims_check": rcheck,
    "readme_presentation_slips": [
        "README predictions table row 1: dg/srenew at w = 0.1 is quoted as 0.957-0.959 across 24/28/32; measured 0.9573 / 0.9592 / 0.9563, so the range is 0.956-0.959.",
        "README predictions table row 2: srenew-small/srenew at 32 bits, w = 0, is quoted as 0.951; measured 0.95047, which is 0.950.",
    ],
    "gae_band": gae_band,
    "completeness": {f"P{b}": rec[b]["completeness"] for b in SIZES},
    "null_controls": {f"P{b}": rec[b]["aa_exact"] for b in SIZES},
    "instrument_v3_6_1": {f"P{b}": {
        "audit_worker": {"ok": prod_audit[b]["ok"], "problems": prod_audit[b]["problems"],
                         "replays_identical": next(s["producer_replays_identical"] for s in sessions if s["session_dir"].startswith(f"v3/P{b}/"))},
        "spec": rec[b]["spec"]["input_spec_equals_frozen"] and rec[b]["spec"]["session_spec_equals_frozen"],
        "gate": rec[b]["gate_v3"]} for b in SIZES},
    "param_differences": {f"P{b}": rec[b]["param_differences"] for b in SIZES},
    "deferred_guard_ledger": {f"P{b}": rec[b]["deferred_guard_ledger"] for b in SIZES},
    "gae_bookkeeping": {f"P{b}": rec[b]["gae_bookkeeping"] for b in SIZES},
    "mechanism": {f"P{b}": {"split": mech[b]["split"], "count_ratios": mech[b]["count_ratios"],
                            "trial_model": mech[b]["trial_model"], "telescoping": mech[b]["telescoping"],
                            "means": mech[b]["means"]} for b in SIZES},
    "p28_check": {"decision_pair": "srenew__dg-small",
                  "ratios": {wk: {k: rec[28]["ratios"]["srenew__dg-small"][wk][k] for k in ("ratio_b_over_a", "ci95_ecbench_replica")}
                             for _, wk in WEIGHTS},
                  "band": {s: {a: rec[28]["band_mean_s"][s][a] for a in ("dg-small", "srenew")} for s in rec[28]["curves"]},
                  "band_dg_small_lt_srenew_on_every_curve": rec[28]["band_dg-small_lt_baseline_on_every_curve"],
                  "gate_v3_conditions_4_5": [rec[28]["gate_v3"]["condition4_holds"], rec[28]["gate_v3"]["condition5_holds"]],
                  "reading": "not an improvement (section 6.4 fails on one curve); agrees with the producer"},
    "replay_certificate": {"path": f"{VREL}/replay-certificate.json", "sha256": sha(os.path.join(VDIR, "replay-certificate.json"))},
    "caveats": [
        ("P32: the gain is smaller than the section 7 model predicted (dg-small/srenew 0.950 / 0.929 / 0.831 against "
         "0.89 / 0.88 / 0.80). The w = 0 upper bound is 0.9681 (own seeds: 0.9663-0.9686). Prediction 3 holds in "
         "direction, not in magnitude."),
        ("The comparison is against walk, so the candidate changes two things: targets (walk -> stride-renew with the "
         "deferred guard) and N. The stride-renew stream itself contributes srenew/walk = 0.982 / 0.987 / 1.007 (24) "
         "and 0.990 / 0.989 / 0.985 (32), whose intervals include 1 at every weight; the resolved part is the smaller "
         "N at w = 0 and the deferred guard at w > 0."),
        ("Same binary for measurement and replay (88070375, no older binary has the target source); the replay is "
         "cross-host only. The [d]G = Q checks and the stdlib recompute are the implementation-independent checks."),
        ("v3 6.1 condition 5 waives the grading-v3 1 ms wall-time minimum on rho-reference executions only (P24: all "
         "672 rho executions, 84.7-996.3 us; P32: 14, 547.5-933.1 us). Their counts are replay-certified; their wall "
         "time is not used. No wall-clock figure is admitted."),
        ("Two fresh curves per size, 24 targets x 6 rounds; intervals cover target-to-target variation on these "
         "curves. 24 and 32 bits are each decided at 95% with no multiplicity correction (as frozen). No exponent "
         "claim."),
    ],
    "limitations": [
        "Same IC implementation (ecbench) in measurement and replay; not an independent implementation.",
        "The measuring binary 88070375 was not rebuilt by the validator from be2969ce; build-info.json and binary.sha256 name it.",
        "session.json git_commit and git_dirty are null (image build); the source binding is build-info.json.",
        "Counted unit only (ecbench GAE / sqrt(r), section 5a band).",
        ("HEAD 727cd043 already adds deferred_guard_component() to scripts/build_ic_leaderboard.py. It renders 24, 28 "
         "and 32 bits in one section and calls gate.check_evidence_dir(REPO / study, REPO) with no row slug, so one "
         "section-level 'validated (24 and 32 bits)' string covers the 28-bit line too. The 28-bit row is not among "
         "this draft's rows (the gate checker rejects it when asked by slug) and is reported as 'srenew kept', not as "
         "an improvement. Its 'dg-small becomes the 24/32-bit baseline' wording is a promotion reading for the "
         "Coordinator, not something this validation decides. Generated leaderboard files were not changed in that "
         "commit."),
    ],
    "review_attestation": {
        "joints_owned": ["whole receipt for v3 P24 and P32 (no review_plan on the task card); P28 only to confirm it is not an improvement"],
        "sources_read": [
            "crypto-autoresearcher: docs/agent-runtime-core.md, agents/validator.md",
            "wt-dg backlog/XP-20261029-srenew-deferred-guard-v3/{PROTOCOL.md, APPROVAL.md, P32-TRIGGER.md, spec-P24/P28/P32.json}",
            "wt-dg backlog/XP-20261029-srenew-deferred-guard/{PROTOCOL.md, AMENDMENT-01.md}",
            "/dev/shm/dg3data/P{24,28,32}/eks-results/* (receipts; records.jsonl, plan.json and l2-gate.json by script)",
            "/dev/shm/dg3data/P{24,28,32}/analysis/{compare-*.json, band.md, gate_v3.json, gate_v3-stdout.txt (summary lines), receipt.json (keys), small check files} after the validator's own recompute",
            "S3: the three job prefixes (listing; sha256 of every worker file; launcher request/result receipts)",
            "src-be2969ce: src/cryptanalysis/ic_boundary.rs (TargetSource, the relation loop and guard, PairTable probe, decompose_mitm), ic_framework/mod.rs (deferred-guard unit test), ecbench/{band.rs, stats.rs, compare.rs, canonical.rs splitmix64}, src/bin/ecbench.rs (compare and verify options)",
            "wt-pbo research/ic_stride_renew_prime_20261029/{VALIDATION.json, validation/*} (format), docs/VALIDATION_GATE.md, scripts/validation_gate.py (checks)",
            "wt-dg: git log/show of the freeze, trigger, evidence and leaderboard commits; research/ic_srenew_deferred_guard_20261029/README.md (v3 tables), the committed v3 evidence blobs and .artifacts.json manifests (by hash)",
        ],
        "not_read": ["figures/, README.pdf", "first-study records", "docs/session-logs (producer narrative)", "cards/ (except as hashed)"],
        "read_sibling_reports": False,
        "verdict": "passed",
    },
}
with open(os.path.join(VDIR, "validator-report.json"), "w") as f:
    json.dump(report, f, indent=1)
    f.write("\n")

# ------------------------------------------------------------------ VALIDATION.draft.json
def band_row(slug, b, arm):
    m = rec[b]["band_mean_s"][slug]
    return {ws: {"ic": m[arm][wk], "rho": m["rho-neg"][wk]} for ws, wk in WEIGHTS}


by_row = {s: band_row(s, b, "dg-small") for b in SIZES for s in rec[b]["curves"]}
base_by_row = {s: band_row(s, b, "walk") for b in SIZES for s in rec[b]["curves"]}
order = all(v[ws]["rho"] < v[ws]["ic"] for v in by_row.values() for ws, _ in WEIGHTS)
first = rows[0]
val = {
    "schema": "validation-gate/v1",
    "rows": rows,
    "replay_certificate": {"path": f"{VREL}/replay-certificate.json", "sha256": sha(os.path.join(VDIR, "replay-certificate.json"))},
    "validator_report": {"path": f"{VREL}/validator-report.json", "sha256": sha(os.path.join(VDIR, "validator-report.json"))},
    "rho_reference": {
        "kind": "negation",
        "protocol": ("first-study PROTOCOL.md section 2 arm rho-neg (carried into v3 section 1): rho.negation in the same "
                     "session per size (v3 P24 session of job ic-dg3-p24-20261029, P32 of ic-dg3-p32-20261029), same 48 "
                     "targets, rounds and algorithm seeds as dg-small and walk"),
        "unit": ("ecbench.gae / sqrt(r) (S), unpriced native operations priced at w GAE each (AGENTS.md section 5a); "
                 "rho canonicalisations priced the same way"),
        "binary_or_process": ("ecbench sha256 88070375e77648aa12a561ff9ee59e34d469c55e1da56987234811b0956e412c (isolab "
                              "image, source be2969ce), one session per size; replayed by the same binary on another host"),
        "matches_protocol": True,
        "strongest_available": True,
        "strongest_available_basis": ("All four curves are prime-order curves over prime fields with j not 0 or 1728 "
                                      "(validator check against the ICV1 strings), so Aut(E) = {+-1}: negation is the only "
                                      "automorphism fold and no Frobenius fold exists; rho records show automorphisms_used 2."),
        "same_unit": True,
        "same_binary_or_process": True,
        "gae_band_row": (f"{first}: ic = dg-small mean S over its 144 measured runs, rho = rho-neg mean S over the same "
                         "144 (target, round) pairs; every row in gae_band_by_row"),
        "gae_band": by_row[first],
        "gae_band_by_row": by_row,
        "gae_band_baseline_by_row": base_by_row,
        "order_holds_across_band": order,
    },
    "verdict": "passed",
    "caveats": [
        "P32 passes with caveats (" + VAL_ID + "): the gain (0.941 / 0.919 / 0.819) is smaller than the section 7 model "
        "predicted; the w = 0 upper bound is 0.9681.",
        "The candidate changes targets and N against walk; the stride-renew stream alone (srenew/walk) is not resolved at "
        "either size, the smaller N carries the w = 0 gain and the deferred guard the w > 0 gain.",
        "Measured and replayed with the same binary 88070375 on different hosts (no older binary has the target source); "
        "the validator's stdlib recompute and [d]G = Q checks on 5,376 records are the implementation-independent checks.",
        "Rho-reference executions below 1 ms are graded L1 under grading v3; v3 section 6.1 condition 5 admits their "
        "counts only. Counted unit only; no wall-clock figure.",
        "Two fresh curves per size, 24 targets x 6 rounds; 24 and 32 bits decided separately at 95%. 28 bits is not an "
        "improvement and is not covered by this file.",
    ],
    "promotion_blockers": [],
    "promotion_note": ("No evidence blocker known to the validator remains for the four rows. Promotion itself is the "
                       "Coordinator decision; this file records evidence admissibility, not authorization."),
    "draft_note": ("Draft written by the validator outside the repository. Place it as "
                   f"{EVID}/VALIDATION.json with the validator files under {EVID}/{VREL}/; the two pinned sha256 "
                   "values hold only if those files are copied byte for byte."),
}
with open(os.path.join(VDIR, "VALIDATION.draft.json"), "w") as f:
    json.dump(val, f, indent=1)
    f.write("\n")
print(json.dumps({"runs": len(runs), "headline": len(headline), "readme_agree": all(r["agree"] for r in rcheck),
                  "readme_disagree": [r for r in rcheck if not r["agree"]], "order_holds": order,
                  "cert_sha256": val["replay_certificate"]["sha256"], "report_sha256": val["validator_report"]["sha256"]}))
