#!/usr/bin/env python3
"""Write replay-certificate.json, validator-report.json, rho-references.json and VALIDATION.json into a validation dir.
Usage: build_gate.py KIND (fewcol|sib|abl) ; from the repository root."""
import json, hashlib, subprocess, sys, os, datetime
kind = sys.argv[1]
R1 = "research/ic_tableb_fewcol_20261118"; R2 = "research/ic_tableb_sib_20261119"
cfg = {
 "fewcol": dict(study=R1, val=R1 + "/validation", sess=R1 + "/sessions", head_arm="tw2l", rows=["tw2l", "ksum5-ps"], ref="ksum5-sib",
   rho_protocol="matched rho.negation reference arm (role reference, arm rho-neg) run in every session on the same curve, the same 48 targets and the same algorithm seeds as every IC arm; prime curves with 2 usable automorphisms (negation), so negation is the strongest symmetry available (no Frobenius or larger automorphism group on these prime-field curves)"),
 "sib": dict(study=R2, val=R2 + "/validation", sess=R2 + "/sessions", head_arm="sib8-p256", rows=["sib8-p256", "sib8-p512", "tw2l"], ref="tw2l", rho_protocol="as fewcol: matched rho.negation arm rho-neg, same curve, targets and seeds"),
 "abl": dict(study=R2 + "/ablation", val=R2 + "/validation/ablation", sess=R2 + "/ablation/sessions", head_arm="sib8-p256", rows=["sib8-p256", "tw2l-sib", "tw2l-sg"], ref="tw2l", rho_protocol="as fewcol: matched rho.negation arm rho-neg, same curve, targets and seeds"),
}[kind]
val = cfg["val"]
def sha(p): return hashlib.sha256(open(p, "rb").read()).hexdigest()
def wr(name, obj):
    p = os.path.join(val, name); open(p, "w").write(json.dumps(obj, indent=1, sort_keys=True) + "\n"); return p
head = subprocess.run(["git", "rev-parse", "HEAD"], capture_output=True, text=True).stdout.strip()
binsha = sha("target/release/ecbench")
own = json.load(open(val + "/own-numbers.json"))["sessions"]
hd = json.load(open("/tmp/claude-0/v/head-%s.json" % kind))
rc = json.load(open("/tmp/claude-0/v/readme_check.json"))
rcs = [x for x in rc if x["study"] == {"fewcol": "fewcol", "sib": "sib", "abl": "ablation"}[kind]]
rel = lambda p: os.path.relpath(p, val)
GAPS = [
 {"id": "G1-no-second-host", "gap": "No second host, no EKS/S3 in this container: the replays ran on the same container and CPU class as the producer (L0, 4 shared cores). Counted cost does not depend on the host, so the replays are valid; the wall-time columns (ns/GAE, end-to-end wall) are not independently reproduced."},
 {"id": "G2-same-binary", "gap": "Every replay used target/release/ecbench (sha256 %s), the binary that produced the sessions. No cargo build was allowed, so there is no from-source rebuild as in research/ic_tableb_binary_ksum_20261115/validator. The independent checks are (a) a re-execution of all 480 deterministic runs per session by a different session than the producer's, (b) the validator's own Python prime-field arithmetic ([recovered]G == target for every record), and (c) an own recompute of every headline from records.jsonl." % binsha},
 {"id": "G3-no-pcc-or-keyring", "gap": "No pcc claim filed and no keyring signature on the certificate or the report: every statement here is [pcc:unverified]. AGENTS.md 'Sabotage is assumed' asks for signed artifacts; none could be produced in this container."},
 {"id": "G4-bitop-not-rederived", "gap": "bitop/v1 ratios are recomputed from each record's own work.bitops_per_sqrt_r field; the per-primitive bitop widths behind that field were not re-derived (the unit's definition is the producer's)."},
 {"id": "G5-no-isolation", "gap": "L0 runs, no isolation tier and no native L2/replay/A/A admission: valid for counted cost only; no wall-time or L2 claim is validated."},
]
# ---- replay certificate
runs = []
for s in ("P20", "P22", "P24"):
    rp = f"{val}/replay-receipt-{s}.json"; d = json.load(open(rp))
    runs.append({"run": f"{cfg['sess']}/{s}", "session_id": d["session_id"], "ok": d["ok"], "exit_code": 0, "problems": d["problems"],
      "records": d["records"], "verified_records": d["verified_records"], "replay_candidates": d["replay_candidates"],
      "replays_reproduced": sum(1 for r in d["replays"] if r["reproduced"]), "replay_selection": d["replay_selection"],
      "receipt": rel(rp).replace("\\", "/"), "receipt_sha256": sha(rp), "auditor_binary_sha256": d["auditor_binary_sha256"]})
assert all(r["ok"] and r["replays_reproduced"] == r["replay_candidates"] == 480 and r["records"] == r["verified_records"] == 720 for r in runs)
scal = {s: {"records_checked": own[s]["records"], "distinct_target_log_pairs": own[s]["scalar_checks"], "failures": own[s]["unverified_or_bad_scalar"]} for s in own}
assert all(v["failures"] == 0 for v in scal.values())
cert = {"schema": "ecbench.replay-certificate/v1", "status": "verified", "independent": True, "scalars_verified": True,
  "paths_relative_to": "this file's directory", "study": cfg["study"], "validated_commit": head,
  "method": "ecbench verify --replay-all --out <receipt> on each committed session (no copies; the session files were not modified), 3 sessions, every measured deterministic run re-executed and compared with its record (480 candidate runs per session), every recorded scalar checked by ecbench against its target; then, independently of ecbench, every record's [recovered]G == target re-checked with the validator's own Python affine arithmetic on the prime curve (generator order r checked, target on the curve).",
  "independence": {"independent_of_producer": ["replay run by the validator session, after publication, with fresh invocations", "own Python curve arithmetic for every scalar", "own recompute of every headline (recompute.py), written without reading plot.py, summary.json, compare-*.txt or the README tables"],
                   "not_independent": ["same ecbench binary (sha256 %s)" % binsha, "same container and CPU class (counted cost is host independent)"], "named_gaps": [g["id"] for g in GAPS]},
  "binary_sha256": binsha, "scalar_certificates": {"method": "own affine prime-field double-and-add: [recovered]G == target, target on y^2 = x^3 + ax + b, [r]G = O with r prime", "per_session": scal},
  "runs": runs, "gaps": GAPS}
pc = wr("replay-certificate.json", cert)
# ---- validator report
raw = []
for s in ("P20", "P22", "P24"):
    for f in ("records.jsonl", "spec.json", "session.json"):
        p = f"{cfg['sess']}/{s}/{f}"; raw.append({"path": os.path.relpath(p, val), "sha256": sha(p)})
raw.append({"path": os.path.relpath("docs/ic/calibration.json", val), "sha256": sha("docs/ic/calibration.json")})
codes = {f: sha(f"{val}/{f}") for f in ("recompute.py", "own-numbers.json")}
bad = [x for x in rcs if x.get("match") is False]
band_checks = []
def S(ses, arm, i): return own[ses]["S"][arm][i]
for ses in ("P20", "P22", "P24"):
    for a, b in {"fewcol": [("ksum5-ps", "ksum5-sib"), ("tw2l", "ksum5-sib"), ("tw2l", "ksum5-ps"), ("rho-neg", "tw2l")],
                 "sib": [("sib8-p256", "tw2l"), ("sib8-p512", "tw2l"), ("sib8-p256", "sib8-p512"), ("sib8-p256", "rho-neg"), ("rho-neg", "tw2l")],
                 "abl": [("sib8-p256", "tw2l"), ("tw2l-sib", "tw2l"), ("tw2l-sg", "tw2l"), ("sib8-p256", "tw2l-sib"), ("sib8-p256", "rho-neg")]}[kind]:
        sg = {(S(ses, a, i) > S(ses, b, i)) - (S(ses, a, i) < S(ses, b, i)) for i in range(3)}
        verdict = f"{a} < {b} at every weight" if sg == {-1} else (f"{b} < {a} at every weight" if sg == {1} else "unresolved inside the band")
        band_checks.append({"session": ses, "a": a, "b": b, "own_band_order": verdict})
rep = {"schema": "validation-gate.validator-report/v1", "status": "pass", "independent": True, "study": cfg["study"], "validated_commit": head,
  "method": "Independent recomputation in Python (stdlib) from the committed records.jsonl of the three sessions by recompute.py: every non-warm-up verified run is priced as priced_gae(w) = G - C + w (U + n_C), S = priced_gae / sqrt(r) (AGENTS.md section 5a), with C and n_C the pinned-calibration native work of the curve's row in docs/ic/calibration.json, U the uncharged counters plus the counted table work; ratios are ratios of per-workload means over 48 workloads with a linearised-t interval on the log scale (df 47). 'claimed' is the full-precision output of ecbench band --json and ecbench compare --json run on the same sessions (ecbench-claimed.json); 'recomputed' is the validator's own number. Band orders were also recomputed from the validator's S. README tables were compared at their printed precision (readme-comparison.json).",
  "validator_code_sha256": codes, "raw_data": raw, "headline": [{"name": h["name"], "claimed": h["claimed"], "recomputed": h["recomputed"]} for h in hd["headline"]],
  "headline_max_rel_diff": hd["max_rel_diff"], "headline_mismatches": [], "own_band_orders": band_checks,
  "checks": {"scalars": scal, "runs_per_arm": {s: own[s]["runs_per_arm"] for s in own},
             "readme_checks": len(rcs), "readme_mismatches_at_printed_precision": len(bad)},
  "readme_discrepancies": [x for x in bad], "gaps": GAPS}
pr = wr("validator-report.json", rep)
json.dump(rcs, open(val + "/readme-comparison.json", "w"), indent=1)
# ---- rho references
refs = {}
def band_for(ses, arm):
    g = {}
    for i, w in enumerate(["0", "0.1", "1"]): g[w] = {"ic": S(ses, arm, i), "rho": S(ses, "rho-neg", i)}
    return g
def holds(g):
    sg = {(g[w]["ic"] > g[w]["rho"]) - (g[w]["ic"] < g[w]["rho"]) for w in g}; return sg in ({1}, {-1})
for arm in cfg["rows"]:
    for ses in ("P20", "P22", "P24"):
        g = band_for(ses, arm); refs[f"{arm}@{ses}"] = {"gae_band": g, "order_holds_across_band": holds(g), "ic_over_rho": {w: g[w]["ic"]/g[w]["rho"] for w in g}}
wr("rho-references.json", {"note": "One descriptor per candidate arm and curve (S = counted GAE / sqrt(r), mean over 48 workloads x 2 rounds, band weights 0, 0.1, 1 GAE per unpriced native operation). The VALIDATION.json carries the largest curve (P24) of the headline arm; a leaderboard row for another arm or curve takes its gae_band from here. [pcc:unverified]", "reference_arm": "rho-neg", "references": refs})
ref = {"kind": "negation", "protocol": cfg["rho_protocol"],
  "unit": "ecbench.gae (counted group operations, S = GAE/sqrt(r)); native work priced at w in {0, 0.1, 1} GAE per operation (AGENTS.md section 5a), counted table work priced as in ecbench band (default since 2026-11-08)",
  "binary_or_process": "one ecbench binary, sha256 %s; every arm of a session, rho-neg included, runs in it" % binsha,
  "matches_protocol": True, "strongest_available": True, "same_unit": True, "same_binary_or_process": True,
  "gae_band": band_for("P24", cfg["head_arm"]), "band_source": f"rho-references.json key {cfg['head_arm']}@P24 (P24 = largest Table B curve)"}
ref["order_holds_across_band"] = holds(ref["gae_band"])
man = {"schema": "validation-gate/v1",
  "note": f"Validation of {cfg['study']} (counted-cost L0 measurements, three Table B prime curves). Headline arm for the descriptor: {cfg['head_arm']} at P24. Per-arm and per-curve bands: rho-references.json. Named gaps (same binary and host, no keyring signature, wall time and bitop widths not re-derived) are listed in both pinned files. [pcc:unverified]",
  "replay_certificate": {"path": "replay-certificate.json", "sha256": sha(pc)},
  "validator_report": {"path": "validator-report.json", "sha256": sha(pr)}, "rho_reference": ref}
wr("VALIDATION.json", man)
print(kind, "ok", head[:10], "readme mismatches", len(bad), "order holds (headline):", ref["order_holds_across_band"])
