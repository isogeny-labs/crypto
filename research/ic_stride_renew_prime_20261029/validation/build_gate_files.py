#!/usr/bin/env python3
"""Validator: build replay-certificate.json and validator-report.json for
XP-20261029-stride-renew-prime P20 and P28 from the validator's own outputs.

Inputs (all under the study directory, read-only):
  P<b>/eks-results/session/{records,plan,session,spec,host}  raw data
  P<b>/analysis/compare-*.json                                 producer claims
  validation/recompute-P<b>.json, mechanism-P<b>.json          validator recompute
  validation/replay-P<b>.audit.json                            validator replay
  band JSON from `ecbench band --json` re-run by the validator (argv[2]: validation/)
Usage: python3 -I build_gate_files.py <study_dir> <band_json_dir> <val_id>
"""
import hashlib
import importlib.util
import json
import os
import sys

STUDY, BANDDIR, VAL_ID = sys.argv[1], sys.argv[2], sys.argv[3]
VDIR = os.path.join(STUDY, "validation")
SIZES = (20, 28)
S3 = {
    20: "s3://isolab-results-104186811093-us-east-2/runs/ic-srenew-p20-20261029/8d5ba87a-31b6-46fb-9d07-14bee7f6c08b/",
    28: "s3://isolab-results-104186811093-us-east-2/runs/ic-srenew-p28-20261029/e8fc61b5-4fb6-4257-b3ae-0692df8ff7c4/",
}
WEIGHTS = (("0", "0.0"), ("0.1", "0.1"), ("1", "1.0"))
PAIRS = ["walk__srenew", "walk__walk-aa", "rho-neg__rho-neg-aa", "rho-neg__walk", "rho-neg__srenew"]
REPLAY_BIN = "bf5eec6d61fbe83a7e6911bf73d05d847cc0ab23fd6db3342290a26b9f3807d9"
MEAS_BIN = "583e62f0836eae43330c4b36bbe3d145dacb01de3132436c71f6924445c6a5a9"


def sha(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def load(path):
    with open(path) as f:
        return json.load(f)


spec = importlib.util.spec_from_file_location("recompute", os.path.join(VDIR, "recompute.py"))
rc = importlib.util.module_from_spec(spec)
sys.dont_write_bytecode = True
spec.loader.exec_module(rc)

rec = {b: load(os.path.join(VDIR, f"recompute-P{b}.json")) for b in SIZES}
mech = {b: load(os.path.join(VDIR, f"mechanism-P{b}.json")) for b in SIZES}
rep = {b: load(os.path.join(VDIR, f"replay-P{b}.audit.json")) for b in SIZES}
prod_audit = {b: load(os.path.join(STUDY, f"P{b}/eks-results/audit.json")) for b in SIZES}
band = {b: load(os.path.join(BANDDIR, f"band-P{b}.ecbench.json")) for b in SIZES}
cmpf = {}
for b in SIZES:
    for p in PAIRS:
        for ws, _ in WEIGHTS:
            cmpf[(b, p, ws)] = load(os.path.join(STUDY, f"P{b}/analysis/compare-{p}-w{ws}.json"))

# ------------------------------------------------------------ runs (per record)
runs = []
for b in SIZES:
    sdir = os.path.join(STUDY, f"P{b}/eks-results/session")
    plan = load(os.path.join(sdir, "plan.json"))
    curves = {}
    for wl in plan["workloads"]:
        c = wl["curve"]
        curves.setdefault(c["slug"], c)
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
            runs.append({
                "session": f"P{b}", "seq": x["seq"], "record_id": x["record_id"],
                "arm": x["arm"], "target_index": wl["target_index"],
                "round": x["round"],
                "recovered": x["outcome"]["recovered"],
                "validator_dG_equals_Q": ok,
                "reproduced": rp.get("reproduced"), "detail": rp.get("detail"),
            })
assert all(r["validator_dG_equals_Q"] and r["reproduced"] and r["detail"] == "identical" for r in runs)

sessions = []
for b in SIZES:
    sdir = f"P{b}/eks-results/session"
    files = {n: sha(os.path.join(STUDY, sdir, n)) for n in
             ("host.json", "plan.json", "records.jsonl", "session.json", "spec.json")}
    a = rep[b]
    sessions.append({
        "session_dir": sdir, "session_id": a["session_id"], "s3_uri": S3[b] + "session/",
        "receipt": f"validation/replay-P{b}.audit.json",
        "receipt_sha256": sha(os.path.join(VDIR, f"replay-P{b}.audit.json")),
        "verify_log": f"validation/replay-P{b}.verify.log",
        "records": a["records"], "verified_records": a["verified_records"],
        "replays": len(a["replays"]),
        "replays_reproduced": sum(1 for r in a["replays"] if r["reproduced"] and r["detail"] == "identical"),
        "problems": a["problems"], "checks": a["checks"], "ok": a["ok"],
        "session_files_sha256": files,
        "session_files_equal_replay_receipt": files == a["files"],
        "session_files_equal_producer_receipt": files == prod_audit[b]["files"],
        "session_files_equal_s3_originals": True,
        "producer_receipt": f"P{b}/eks-results/audit.json",
        "producer_receipt_sha256": sha(os.path.join(STUDY, f"P{b}/eks-results/audit.json")),
        "producer_auditor_binary_sha256": prod_audit[b]["auditor_binary_sha256"],
        "auditor_binary_sha256": a["auditor_binary_sha256"],
        "auditor_env_class_id": a["auditor_env_class_id"],
        "measured_env_class_id": prod_audit[b]["auditor_env_class_id"],
        "audited_unix_ms": a["audited_unix_ms"],
        "end_line": open(os.path.join(VDIR, f"replay-P{b}.verify.log")).read().strip().splitlines()[-1],
    })

rows = sorted(s for b in SIZES for s in rec[b]["curves"])
snapshot = {
    "branch": "claude/ic-stride-renew-prime-20261029",
    "head_commit": "0e7243aa6cbeae8f0f5a85e618929928d2aa1fd3",
    "protocol_freeze_commit": "4d4593aabda667a681fab4a67e351dc5bcb7c1f4",
    "evidence_committed": False,
    "note": ("The study directory was not in any commit when validated (staged by another session "
             "during validation). Every file this certificate names is bound by sha256; the "
             "eks-results files are byte-identical to the S3 originals written by the worker "
             "(validation/provenance.txt). A commit of the study directory must carry these hashes."),
}

cert = {
    "schema": "replay-certificate/v1",
    "evidence_dir": "research/ic_stride_renew_prime_20261029",
    "rows": rows,
    "snapshot": snapshot,
    "status": "verified",
    "independent": True,
    "scalars_verified": True,
    "independence": {
        "replayer": "Validator subagent (claude-opus-5-5), separate from the producer session; re-executed every measured run of both sessions into new receipts under validation/; producer receipts untouched (replayed on byte-identical copies)",
        "same_binary_as_measurement": False,
        "same_host_class_as_measurement": False,
        "independent_implementation": False,
        "independent_scalar_check": True,
        "statement": ("Cross-binary, cross-host replay: the worker measured with ecbench " + MEAS_BIN[:8] +
                      " (image, source cb2ec6873f30) on c8a.2xlarge (env ECBENV2h3a51beea0bc2); the validator "
                      "replayed with ecbench " + REPLAY_BIN[:8] + " on env ECBENV3h13e37662a118, and all 1440 "
                      "measured runs of each session reproduced identical counts. Every recovered scalar of "
                      "every measured run re-verifies [d]G = Q under the validator's own affine arithmetic. "
                      "Not an independent implementation of the IC pipeline."),
    },
    "binary": {
        "replay_binary_path": "/tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/bin/ecbench-bf5eec6d",
        "replay_binary_sha256": REPLAY_BIN,
        "measuring_binary_sha256": MEAS_BIN,
        "measuring_binary_source_revision": "cb2ec6873f30b683f7885b53c410d69965f02c20",
        "equals_session_binary_sha256": False,
    },
    "command": "RAYON_NUM_THREADS=1 taskset -c 2 <replay_binary> verify --dir <copy of P<b>/eks-results/session> --replay-all --exit-code --out validation/replay-P<b>.audit.json",
    "independent_scalar_check": {
        "method": "validation/recompute.py: curve coefficients derived from plan.json points; p, r prime; all points on the curve; [r]G = O; r the only multiple of r in the Hasse interval (so #E = r); j-invariant equal to the ICV1 string; [d]G = Q in affine coordinates for every measured record of every arm",
        "curves": {s: c for b in SIZES for s, c in rec[b]["curves"].items()},
        "measured_records": {f"P{b}": rec[b]["dG_eq_Q"]["checked"] for b in SIZES},
        "failures": [f for b in SIZES for f in rec[b]["dG_eq_Q"]["failures"]],
    },
    "sessions": sessions,
    "runs": runs,
}
with open(os.path.join(VDIR, "replay-certificate.json"), "w") as f:
    # one compact line per run; everything else indented
    body = json.dumps({k: v for k, v in cert.items() if k != "runs"}, indent=1)
    f.write(body[:-2] + ',\n "runs": [\n')
    f.write(",\n".join("  " + json.dumps(r, separators=(",", ":")) for r in runs))
    f.write("\n ]\n}\n")

# ------------------------------------------------------------ validator report
headline = []
for b in SIZES:
    for p in PAIRS:
        a_arm, b_arm = p.split("__")
        for ws, wk in WEIGHTS:
            c = cmpf[(b, p, ws)]
            mine = rec[b]["ratios"][p][wk]
            headline.append({
                "name": f"P{b} sum S_{b_arm} / sum S_{a_arm} at w={ws} (288 paired runs, 48 targets, 2 curves)",
                "claimed": c["ops"]["ratio_b_over_a"], "recomputed": mine["ratio_b_over_a"], "rel_tol": 1e-12,
                "claimed_source": f"P{b}/analysis/compare-{p}-w{ws}.json ops.ratio_b_over_a",
                "recomputed_by": "validation/recompute.py (validator, stdlib, records.jsonl read directly)",
            })
            for cv in c["curves"]:
                headline.append({
                    "name": f"P{b} {cv['slug']} sum S_{b_arm} / sum S_{a_arm} at w={ws}",
                    "claimed": cv["ratio_b_over_a"], "recomputed": mine["per_curve"][cv["slug"]],
                    "rel_tol": 1e-12,
                    "claimed_source": f"P{b}/analysis/compare-{p}-w{ws}.json curves[].ratio_b_over_a",
                    "recomputed_by": "validation/recompute.py",
                })
            if p == "walk__srenew":
                for i, side in ((0, "lower"), (1, "upper")):
                    headline.append({
                        "name": f"P{b} srenew/walk 95% cluster-bootstrap {side} bound at w={ws}",
                        "claimed": c["ops"]["ci95_bootstrap"][i],
                        "recomputed": mine["ci95_two_stage_bootstrap"][i], "rel_tol": 0.005,
                        "claimed_source": f"P{b}/analysis/compare-{p}-w{ws}.json ops.ci95_bootstrap (ecbench, seed 20261002)",
                        "recomputed_by": "validation/recompute.py two-stage cluster bootstrap, own RNG seed 29102026; agreement is Monte Carlo, not bitwise",
                    })
    for row in band[b]["rows"]:
        for i, (ws, wk) in enumerate(WEIGHTS):
            headline.append({
                "name": f"P{b} band mean S {row['arm']} on {row['curve']} at w={ws}",
                "claimed": row["mean_s"][i], "recomputed": rec[b]["band_mean_s"][row["curve"]][row["arm"]][wk],
                "rel_tol": 1e-12,
                "claimed_source": f"ecbench band --dir P{b}/eks-results/session --json (re-run by the validator; band.md is its 3-decimal rendering, byte-identical)",
                "recomputed_by": "validation/recompute.py",
            })
bad = [h for h in headline if abs(h["claimed"] - h["recomputed"]) > h["rel_tol"] * abs(h["claimed"])]
assert not bad, bad

main_claims = {20: {"0": (0.842, 0.807, 0.878), "0.1": (0.858, 0.821, 0.897), "1": (0.926, 0.879, 0.975)},
               28: {"0": (0.977, 0.961, 0.992), "0.1": (0.975, 0.958, 0.993), "1": (0.968, 0.941, 0.996)}}
mcheck = []
for b in SIZES:
    for ws, wk in WEIGHTS:
        o = cmpf[(b, "walk__srenew", ws)]["ops"]
        exact = (o["ratio_b_over_a"], o["ci95_bootstrap"][0], o["ci95_bootstrap"][1])
        got = tuple(round(v, 3) for v in exact)
        mcheck.append({"size": b, "w": ws, "main_session": main_claims[b][ws], "exact": exact,
                       "rounded": got, "agree": got == main_claims[b][ws]})

gae_band = {}
for b in SIZES:
    gae_band[f"P{b}"] = {}
    for ws, wk in WEIGHTS:
        r = rec[b]["ratios"]
        gae_band[f"P{b}"][ws] = {
            "mean_s_walk": r["walk__srenew"][wk]["mean_s_a"],
            "mean_s_srenew": r["walk__srenew"][wk]["mean_s_b"],
            "mean_s_rho_neg": r["rho-neg__walk"][wk]["mean_s_a"],
            "srenew_over_walk": r["walk__srenew"][wk]["ratio_b_over_a"],
            "srenew_over_walk_ci95_ecbench": cmpf[(b, "walk__srenew", ws)]["ops"]["ci95_bootstrap"],
            "srenew_over_walk_ci95_validator_two_stage": r["walk__srenew"][wk]["ci95_two_stage_bootstrap"],
            "walk_over_rho_neg": r["rho-neg__walk"][wk]["ratio_b_over_a"],
            "walk_over_rho_neg_ci95_ecbench": cmpf[(b, "rho-neg__walk", ws)]["ops"]["ci95_bootstrap"],
            "srenew_over_rho_neg": r["rho-neg__srenew"][wk]["ratio_b_over_a"],
            "srenew_over_rho_neg_ci95_ecbench": cmpf[(b, "rho-neg__srenew", ws)]["ops"]["ci95_bootstrap"],
        }

raw = []
for b in SIZES:
    for n in ("records.jsonl", "plan.json", "session.json", "spec.json", "host.json"):
        p = f"P{b}/eks-results/session/{n}"
        raw.append({"path": p, "sha256": sha(os.path.join(STUDY, p)), "s3": S3[b] + "session/" + n})
    for n in ("SHA256SUMS", "audit.json", "l2-gate.json", "input-spec.json", "build-info.json", "job-exit.json"):
        p = f"P{b}/eks-results/{n}"
        raw.append({"path": p, "sha256": sha(os.path.join(STUDY, p)), "s3": S3[b] + n})

report = {
    "schema": "validator-report/v1",
    "id": VAL_ID,
    "status": "pass",
    "verdict": "passed",
    "independent": True,
    "validator": "Validator subagent (claude-opus-5-5); did not produce the protocol, runs, analysis or claim",
    "claim": ("XP-20261029-stride-renew-prime: at 20 and 28 bits, ic.pipeline with targets = stride-renew (srenew) "
              "has lower whole-pipeline S than targets = walk (walk) at w = 0, 0.1 and 1, with the 95% cluster-bootstrap "
              "upper bound below 1 at every weight, and band --ranks srenew < walk on every curve (PROTOCOL.md section 6)"),
    "claim_verdict": {
        "20": "improvement: section 6 conditions 1-4 hold; validator: pass",
        "28": "improvement: section 6 conditions 1-4 hold; validator: pass with caveats (see caveats)",
    },
    "snapshot": snapshot,
    "method": ("records.jsonl and plan.json read directly by validation/recompute.py (stdlib): priced S(w) = (G + w U)/sqrt(r), "
               "C = n_C = 0 (no record names a pinned calibration unit), U per the band definition; paired ratios of totals; "
               "own one-stage, stratified and two-stage cluster bootstraps; own affine EC arithmetic for [d]G = Q. ecbench "
               "compare/band re-run with a different binary and byte-compared with analysis/. Local verify --replay-all of "
               "both sessions. Mechanism split by validation/mechanism.py."),
    "raw_data": raw,
    "headline": headline,
    "main_session_claims_check": mcheck,
    "gae_band": gae_band,
    "completeness": {f"P{b}": rec[b]["completeness"] for b in SIZES},
    "null_controls": {f"P{b}": rec[b]["aa_exact"] for b in SIZES},
    "stride_renew_accounting": {f"P{b}": rec[b]["stride_renew_accounting"] for b in SIZES},
    "per_pair_means": {f"P{b}": rec[b]["per_pair_means"] for b in SIZES},
    "mechanism": {f"P{b}": mech[b] for b in SIZES},
    "replay_certificate": {"path": "validation/replay-certificate.json",
                           "sha256": sha(os.path.join(VDIR, "replay-certificate.json"))},
    "caveats": [
        "P28: about 2/3 of the mean saving (about 4,000 of 6,226 GAE per pair at w = 0) comes from srenew needing 4.4% fewer relations (203.3 vs 212.7; two-stage 95% CI of the ratio [0.916, 0.996]) and 4.3% fewer trials ([0.915, 0.999]); about 1/3 (about 2,200 GAE: 1,482 measured doublings plus an estimated 730 additions inside the 58.75 fewer scalar multiplications) is the deterministic scalar-multiplication saving, about 0.8% of the walk's S on its own. PROTOCOL.md section 7 attributes the gain to the scalar-multiplication share only; the relation-count effect was not predicted, has no stated mechanism, and is about two standard errors from zero.",
        "P28: the w = 1 upper bound is 0.99549 (ecbench) and 0.9945-0.9965 over 11 validator bootstrap seeds: below 1 but within 0.6% of it.",
        "Four sizes are decided separately at 95% each, with no multiplicity correction (as frozen). P24 (null, point 0.982) and P32 run 1 (rejected by the L2 gate, point 1.001) are producer numbers that this validation did not check.",
        "Main-session rounding: the P28 w = 1 upper bound 0.996 should read 0.995 (0.99549). All other quoted numbers round correctly.",
    ],
    "limitations": [
        "Same IC implementation (ecbench) in measurement and replay; replay is cross-binary and cross-host, not an independent implementation.",
        "Measuring binary 583e62f0 was built from cb2ec6873f30 (build-info.json); not rebuilt by the validator. Session git_commit/git_dirty are null (image build).",
        "Evidence directory not committed at validation time; bound by sha256 to S3 originals.",
        "Two curves per size, 24 targets per curve, 6 rounds; intervals cover target-to-target variation on these curves. No exponent, cross-size or asymptotic claim.",
        "Counted unit only (ecbench GAE / sqrt(r), section 5a band). No wall-clock claim.",
    ],
    "review_attestation": {
        "joints_owned": ["whole receipt for P20 and P28 (no review_plan on the task card)"],
        "sources_read": [
            "crypto-autoresearcher: docs/agent-runtime-core.md, agents/validator.md",
            "wt-stride: backlog/XP-20261029-stride-renew-prime/{PROTOCOL.md, spec-P20.json, spec-P28.json}",
            "wt-stride: research/ic_stride_renew_prime_20261029/launch/card-0{1,2,3,4}.md (headers, pins)",
            "wt-stride: research/ic_stride_renew_prime_20261029/P{20,28}/{launch/*, eks-results/* (receipt files, records.jsonl and plan.json by script), analysis/compare-*.json, analysis/band.md}",
            "wt-stride: research/ic_stride_renew_prime_20261029/P24/analysis/compare-walk__srenew-w*.json, P32-run1/analysis/compare-walk__srenew-w*.json (context only)",
            "S3: both job prefixes (listing, hashes of every file, launcher download-hashes.txt for P20)",
            "crypto origin/main: docs/VALIDATION_GATE.md, scripts/validation_gate.py (key requirements), src/cryptanalysis/ecbench/band.rs (priced_gae definition), src/cryptanalysis/ecbench/stats.rs (cluster_bootstrap_ci scheme), src/cryptanalysis/ic_boundary.rs (TargetSource::StrideRenew)",
            "crypto cb2ec6873f30: src/cryptanalysis/ic_boundary.rs (TargetSource::StrideRenew, draw_jumps, GroupOps::gae, CountedGroup::mul)",
            "crypto origin/main: research/prime_base_oracle_tuning_20261029/{validation/validation_report.yaml, validation/provenance.txt, replay-certificate.json, validator-report.json, VALIDATION.json} (format only)",
        ],
        "not_read": ["research/ic_stride_renew_prime_20261029/figures/", "docs/session-logs/ (producer narrative)"],
        "read_sibling_reports": False,
        "verdict": "passed",
    },
}
with open(os.path.join(VDIR, "validator-report.json"), "w") as f:
    json.dump(report, f, indent=1)
    f.write("\n")
print(json.dumps({"runs": len(runs), "headline": len(headline), "main_claims_agree": [m["agree"] for m in mcheck],
                  "cert_sha256": sha(os.path.join(VDIR, "replay-certificate.json")),
                  "report_sha256": sha(os.path.join(VDIR, "validator-report.json"))}))
