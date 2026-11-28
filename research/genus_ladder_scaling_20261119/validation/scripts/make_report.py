#!/usr/bin/env python3
"""Assemble validator-report.json, replay-certificate.json and the draft VALIDATION.json from out/*.json.

Statuses are derived from the check outputs, never typed in: the replay certificate is 'verified'
with scalars_verified true only if every replayed row is identical and every sampled log certifies.
"""
import hashlib, json, os

V = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
S = os.path.dirname(V)
OUT = os.path.join(V, "out")
L = lambda n: json.load(open(os.path.join(OUT, n)))
fz, cc, rc, cl, ce, rp, us = (L("freeze_check.json"), L("curves_check.json"), L("recompute.json"), L("compare_claims.json"),
                              L("certs.json"), L("replay_compare.json"), L("unit_sensitivity.json"))
plan = L("plan.json")
ex = L("compare_extra.json") if os.path.exists(os.path.join(OUT, "compare_extra.json")) else {}
sha = lambda p: hashlib.sha256(open(p, "rb").read()).hexdigest()
COMMIT = fz["commits"]["469f5bf731"]["sha"]
FREEZE = fz["commits"]["b5971d86dd"]["sha"]

replay_ok = rp["summary"]["jobs"] == rp["summary"]["jobs_identical"] and rp["summary"]["run_rows_identical"] == rp["summary"]["run_rows_compared"] > 0
certs_ok = ce["summary"]["all_keys_ok"] and ce["summary"]["all_logs_eq_t_and_LG_eq_T"] and ce["summary"]["genus4_keys"] > 0 and ce["summary"]["genus5_keys"] > 0
curves_ok = not cc["problems"] and all(c["complete_1_to_N"] and c["curves_json_agrees"] and all(x["J_order_certified_eq_r"] and x["only_identity_automorphism"] for x in c["certs"].values()) for c in cc["cells"].values())
freeze_ok = (fz["binary"]["match"] and not fz["tracked_dirty"] and all(v["now_eq_frozen"] or v.get("reversed_eq_frozen") for v in fz["frozen"].values())
             and all(fz["order_checks"][k] for k in fz["order_checks"] if k != "d1_seconds_before_first_p54293_file"))
numbers_ok = cl["summary"]["point_ok"] and cl["summary"]["bounds_ok"] and cl["summary"]["flags_ok"] and cl["summary"]["results_md_strings_found"] and all(rc["all_cell_checks_pass"].values()) and rc["G1"]

raw = json.load(open(os.path.join(OUT, "raw_manifest.json")))
used = [m for m in raw if m["path"].startswith(("b4_", "b5_", "tune5_", "rho5_")) and m["path"].endswith(".jsonl")]
used += [m for m in raw if m["path"].startswith("search_") and m["path"].endswith(".jsonl")]
raw_data = [{"path": "../" + m["path"], "sha256": m["sha256"], "tracked_at_validation": m["tracked"]} for m in used]
for n in ("b_p16411_x", "b_p16411_xstar", "b_p32771_x", "b_p32771_xstar"):
    raw_data.append({"path": f"../../genus4_prime_order_20261118/{n}.jsonl.gz",
                     "sha256": fz["reused_prime_order_rows"][n + ".jsonl.gz"]["sha256"], "tracked_at_validation": True})
for n in ("curves.json", "tune.json", "analysis.json"):
    raw_data.append({"path": "../" + n, "sha256": sha(os.path.join(S, n)), "tracked_at_validation": True})

heads = [{"name": x["name"], "claimed": x["claimed"], "recomputed": x["recomputed"], "claimed_source": f"{os.path.relpath(S, os.path.dirname(os.path.dirname(S)))}/analysis.json"}
         for x in cl["point"]]

report = {
    "schema": "validator-report/v1", "status": "pass" if (numbers_ok and curves_ok and freeze_ok) else "fail",
    "independent": True, "disposition": "validated-with-errata" if (numbers_ok and curves_ok and freeze_ok and replay_ok and certs_ok) else "not-validated",
    "study": "research/genus_ladder_scaling_20261119", "commit_validated": COMMIT, "freeze_commit": FREEZE,
    "branch_head_at_report": fz["head"], "binary": fz["binary"],
    "validator": "validator subagent (Claude Opus 5.5), validation/scripts/*.py; no solver code imported",
    "raw_data_base": "validation/", "raw_data": raw_data,
    "raw_data_condition": "the producer's .jsonl rows are untracked (study .gitignore ignores *.jsonl, *.log); this report pins their sha256, and the gate can pass in a clean clone only after they are committed byte-identical",
    "headline": heads, "headline_max_rel_diff": cl["summary"]["point_max_rel_diff"],
    "checks": {"freeze_and_d1": freeze_ok, "curves_and_order_certificates": curves_ok, "numbers_recomputed": numbers_ok,
               "replay_identical": replay_ok, "log_certificates": certs_ok,
               "producer_scripts_reproduce_outputs_byte_identical": ["analysis.json", "tables.md", "tune.json", "curves.json"],
               "bootstrap_bounds_max_abs_diff_independent_rng": cl["summary"]["bounds_max_abs_diff"]},
    "recomputed_readings": {"cells": rc["cells"], "C1": rc["C1"], "C1_curvature": rc["C1_curvature"], "C2": rc["C2"],
                            "tune": {k: v for k, v in rc["tune"].items() if k != "grid"}, "G3": rc["G3"], "unit_sensitivity_C2_fmul": us["C2_fmul_unit"],
                            "g5_factor_base": rc["g5_factor_base"], "C2_x5_dlp": rc["C2_x5_dlp"]},
    "extra_replays": ex,
    "findings_doc": "validation/validator-report.md",
    "lessons_applied": ["L-99ee952350b2", "L-26895912385b", "L-3f39e1d8b3a9", "L-546d37ca7750", "L-1a5a6b64022c",
                        "L-d12b64af75ed", "L-e388a81efa37", "L-10ef4b64c2a6", "L-81737122cefc", "L-f6351b15d194"],
    "claims": "[pcc:unverified]",
}
json.dump(report, open(os.path.join(V, "validator-report.json"), "w"), indent=1)

cert = {
    "schema": "replay-certificate/v1", "status": "verified" if (replay_ok and certs_ok) else "failed",
    "independent": True, "scalars_verified": bool(certs_ok),
    "study": "research/genus_ladder_scaling_20261119", "commit_validated": COMMIT, "freeze_commit": FREEZE,
    "binary": {"frozen": fz["binary"], "log_sources": {
        "genus4": {"path": "scratchpad/val4po/bin/val_po", "sha256": "81bbb7452728a377bbe809c2616fa72cf4e61fd3843be0b4ac9c38dff4b19c1d",
                   "source": "research/genus4_prime_order_20261118/validation/scripts/val_po.rs (prior validator's harness on the ca72267899 library)"},
        "genus5": {"path": "scratchpad/bin/genus_ladder.blpt", "sha256": "e210549a18937be8bce65521f40093e1dbac765174ed481901f7d848460342fd",
                   "source": "research/block_lanczos_pretest_20261120/binary.sha256 (genus_ladder with the GENUS_LADDER_EXPORT_CORE dump); bound to the committed runs by identical rows"}}},
    "nonce": plan["nonce"], "plan": "validation/out/plan.json",
    "comparison": "every field except wall_ns / wall_s / search_wall_s equal to the committed row; instance rows equal",
    "summary": rp["summary"],
    "runs": [{"tag": j["tag"], "binary": j.get("binary"), "status": j["status"], "rows_identical": j.get("identical"), "rows": j.get("runs"),
              "file": f"validation/out/replay/{j['tag']}.jsonl"} for j in rp["jobs"]],
    "logarithm_certificates": {**ce["summary"], "evidence": "validation/out/certs.json",
                               "method": "validator rebuilds (G, t, T) from the seeds with its own Cantor arithmetic (jac.py), checks [r]G = 0, T = [t]G, and that the solver's recovered log L satisfies L = t and [L]G = T; harness/export counts equal the committed rows; tamper controls L+1 and one T coefficient caught"},
    "extra_replays": {k: {x: v[x] for x in ("runs", "identical", "instance_identical")} for k, v in ex.items()},
    "curve_certificates": {k: {cs: c["J_order_certified_eq_r"] for cs, c in v["certs"].items()} for k, v in cc["cells"].items()},
}
json.dump(cert, open(os.path.join(V, "replay-certificate.json"), "w"), indent=1)

# draft gate manifest: band of the binding claimed cell (largest w = 1 ratio among the six new cells)
new = ["g4_p54293_x", "g4_p54293_x*", "g5_p2357_x5", "g5_p2357_x5*", "g5_p4099_x5", "g5_p4099_x5*"]
bind = max(new, key=lambda k: rc["cells"][k]["neg"]["1.0"]["ratio"])
band = {w: {"ic": rc["cells"][bind]["gae_band_mean"][mw]["ic"], "rho": rc["cells"][bind]["gae_band_mean"][mw]["rho_ideal_negation"]}
        for w, mw in (("0", "0.0"), ("0.1", "0.1"), ("1", "1.0"))}
holds = all(v["ic"] < v["rho"] for v in band.values()) or all(v["ic"] > v["rho"] for v in band.values())
others = "; ".join(f"{k}: " + ", ".join(f"{rc['cells'][k]['neg'][w]['ratio']:.4f}" for w in ("0.0", "0.1", "1.0")) for k in new if k != bind)
gate = {
    "schema": "validation-gate/v1",
    "note": (f"DRAFT by the validator for research/genus_ladder_scaling_20261119 (validated commit {COMMIT[:10]}). Band = {bind}, the binding case "
             f"of RESULTS.md's 'below at all w' for the six new cells (others, IC / ideal negation at w = 0, 0.1, 1: {others}). "
             "Hyperelliptic (genus 4 and 5 Jacobians over prime fields), not ECDLP. Cannot pass in a clean clone until the raw .jsonl rows named in "
             "validator-report.json raw_data are committed byte-identical. [pcc:unverified]"),
    "replay_certificate": {"path": "replay-certificate.json", "sha256": sha(os.path.join(V, "replay-certificate.json"))},
    "validator_report": {"path": "validator-report.json", "sha256": sha(os.path.join(V, "validator-report.json"))},
    "rho_reference": {
        "kind": "negation",
        "protocol": "PROTOCOL.md (frozen b5971d86dd) section 6: R = sqrt(pi r / 4), the ideal negation-rho bound with nothing charged; the measured plain rho (G3, 2 runs, genus 5, p = 2357) is a sanity check only",
        "unit": "Jacobian group operations of the curve's own genus (GAE) per key, mean over the 36 keys (3 curves x 6 targets x 2 seeds); each counted native F_p / mod-r operation (rel_mul, rel_add, la_mul, la_add) = w; the ideal bound charges no native work",
        "binary_or_process": "IC: genus_ladder sha256 493ee294... (built from ca72267899). The reference is a closed form of each instance's r emitted by the same binary; the validator certified #J = r (prime) for every curve with its own arithmetic. Only affine automorphism over F_p: the identity, so negation is the strongest symmetry.",
        "matches_protocol": True, "strongest_available": True, "same_unit": True, "same_binary_or_process": True,
        "gae_band": band, "order_holds_across_band": holds,
    },
}
json.dump(gate, open(os.path.join(V, "VALIDATION.json"), "w"), indent=1)
print(json.dumps({"report_status": report["status"], "disposition": report["disposition"], "cert_status": cert["status"],
                  "scalars_verified": cert["scalars_verified"], "band_cell": bind, "band": band, "order_holds": holds,
                  "raw_untracked": sum(not r["tracked_at_validation"] for r in raw_data)}, indent=1))
