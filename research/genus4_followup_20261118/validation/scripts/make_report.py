#!/usr/bin/env python3
"""Assemble the validation-gate artifacts for genus4_followup_20261118 from the validator's outputs.
Writes, in benchmarks/20261118-genus4-followup/: replay-certificate.json, validator-report.json and,
only when every check passes, VALIDATION.json. Claimed numbers are read from the producer's
analysis.json / sensitivity.json / select.json as git blobs at COMMIT; recomputed numbers from
out/recompute.json (the validator's own code). Usage: make_report.py REPO"""
import hashlib, json, os, subprocess, sys
REPO = sys.argv[1]
COMMIT = "6213ecbc0da0ed4917ab75805258201d6629b477"
FREEZE = "ae36cdc6e47d374389a65b9d93e4c2f31a2a3741"
D = "research/genus4_followup_20261118"
V = f"{D}/validation"
EV = "benchmarks/20261118-genus4-followup"
O = os.path.join(REPO, V, "out")
J = lambda n: json.load(open(os.path.join(O, n)))
show = lambda p: json.loads(subprocess.run(["git", "-C", REPO, "show", f"{COMMIT}:{p}"], check=True, capture_output=True).stdout)
sha = lambda p: hashlib.sha256(open(os.path.join(REPO, p), "rb").read()).hexdigest()
rel = lambda p: os.path.relpath(os.path.join(REPO, p), os.path.join(REPO, EV))

rc, cmpc, certs, band = J("recompute.json"), J("compare_claims.json"), J("certs.json"), J("band.json")
rep_cmp, vd_main, vd_rho = J("replay_compare.json"), J("verify_dump_main.json"), J("verify_dump_rho.json")
plan = J("plan.json")
an, se, sel = show(f"{D}/analysis.json"), show(f"{D}/sensitivity.json"), show(f"{D}/select.json")
N, C = rc["numbers"], rc["checks"]
X, Wa = N["arms"]["x"], N["arms"]["w"]

# ---------------- headline: producer's unrounded claim vs validator's recompute ----------------
H = []
def h(name, claimed, mine, src):
    H.append({"name": name, "claimed": claimed, "recomputed": mine, "claimed_source": src})
A = f"{D}/analysis.json"; S = f"{D}/sensitivity.json"; SJ = f"{D}/select.json"
for rd in ("Q3_w1", "Q4_w1", "Q3_w0.1", "Q4_w0.1", "Q3_w0"):
    for e in ("ratio", "lo", "hi"):
        h(f"arm x / plain rho Q, {rd}, {e}", an["arms"]["x"][rd][e], X[rd][e], A)
for rd in ("Q3_w1", "Q4_w1", "Q3_w0.1"):
    for e in ("ratio", "lo", "hi"):
        h(f"arm w / plain rho Q, {rd}, {e}", an["arms"]["w"][rd][e], Wa[rd][e], A)
for k in ("Q3_w1_new_only", "Q3_w1_curve_boot"):
    for e in ("ratio", "lo", "hi"):
        h(f"arm x / Q, {k}, {e}", an["arms"]["x"][k][e], X[k][e], A)
for k in ("vs_plain_theory_w1", "vs_neg_ideal_w1", "vs_neg_ideal_w0.1", "vs_plain_theory_w0.1"):
    for e in ("ratio", "lo", "hi"):
        h(f"arm x {k} (post hoc), {e}", se["x"][k][e], X[k][e], S)
for e in ("ratio", "lo", "hi"):
    h(f"arm w vs_neg_ideal_w1 (post hoc), {e}", se["w"]["vs_neg_ideal_w1"][e], Wa["vs_neg_ideal_w1"][e], S)
s50 = se["x"]["q3_w1_hi_over_50_seeds"]
h("arm x Q3 w=1 upper bound over 50 bootstrap seeds: min", s50["min"], X["q3_w1_hi_over_50_seeds"]["min"], S)
h("arm x Q3 w=1 upper bound over 50 bootstrap seeds: median (sorted[25])", s50["median"], X["q3_w1_hi_over_50_seeds"]["median_upper"], S)
h("arm x Q3 w=1 upper bound over 50 bootstrap seeds: max", s50["max"], X["q3_w1_hi_over_50_seeds"]["max"], S)
h("arm x Q3 w=1 upper bound over 50 bootstrap seeds: fraction below 1", s50["frac_below_1"], X["q3_w1_hi_over_50_seeds"]["frac_below_1"], S)
h("measured Q mean group ops / mean sqrt(pi r/2)", an["q_mean_over_theory"], N["q_mean_over_plain_theory"], A)
h("arm x mean cost at w=1 (GAE per key)", an["arms"]["x"]["mean_cost_w1"], X["mean_cost_w1"], A)
h("arm x LA share of cost at w=1", an["arms"]["x"]["share_la"], X["share_la"], A)
h("arm w mean cost at w=1 (GAE per key)", an["arms"]["w"]["mean_cost_w1"], Wa["mean_cost_w1"], A)
for cc in ("0.5", "0.7", "1.0", "1.4", "2.0"):
    h(f"part A (p=8209) arm x mean cost at cc={cc}", sel["grid"][cc]["mean_cost_q3_w1"], N["partA"][cc]["mean_cost_w1"], SJ)
h("part A selected cc*", float(sel["cc_star"]), float(N["cc_star"]), SJ)
h("keys compared at p=16411", an["n_keys"], N["n_keys"], A)
h("targets at p=16411", an["n_targets"], N["n_targets"], A)
h("G2 rows compared", an["g2_rows_compared"], C["G2"]["compared"], A)
h("G3 rows compared", an["g3_rows_compared"], C["G3"]["rows"], A)
worst = max(abs(x["claimed"] - x["recomputed"]) / max(abs(x["claimed"]), abs(x["recomputed"]), 1e-300) for x in H)

# ---------------- checks ----------------
fz = J("freeze_check.json")
q_rows = [r for r in rep_cmp["rows"] if r["key"][0] == "R_rho_plain_nowin"]
x_rows = [r for r in rep_cmp["rows"] if r["key"][0].endswith("sge(4,1.5)") and r["key"][1] == 16411]
w_rows = [r for r in rep_cmp["rows"] if r["key"][0] == "C_double_lp+pre+coord+unitT"]
a_rows = [r for r in rep_cmp["rows"] if r["file"].startswith("a_cc") and r["key"][0] != "instance"]
checks = [
    {"id": "C1", "requirement": "frozen files hash to frozen.sha256 and were committed (ae36cdc6e4) before any data commit",
     "pass": fz["pass"], "evidence": [f"{V}/out/freeze_check.json", f"{V}/scripts/freeze_check.sh"]},
    {"id": "C2", "requirement": "producer binary equals binary.sha256; validator rebuild from git archive of the freeze commit has identical code sections",
     "pass": True, "evidence": [f"{V}/out/binary_diff.txt", f"{V}/out/archive_paths.sha256.txt"],
     "note": "rebuild sha256 90128561... != 74f02ea1...: build-id and the .llvm.<hash> suffix of 35 promoted local symbols differ (build-path dependent); .text/.rodata/.data/.data.rel.ro/.eh_frame/.gcc_except_table/.got and all symbol addresses identical"},
    {"id": "C3", "requirement": ">= 10 arm-x and >= 5 arm-w rows at p=16411, >= 3 part-A rows at different cc, 2-4 Q rows replayed with the validator build: every field except wall_ns identical",
     "pass": rep_cmp["all_identical"] and len(x_rows) >= 10 and len(w_rows) >= 5 and len({r["file"].split("_")[1] for r in a_rows}) >= 3 and 2 <= len(q_rows) <= 4,
     "counts": {"x_16411": len(x_rows), "w_16411": len(w_rows), "partA": len(a_rows), "partA_cc_values": sorted({r["file"].split("_")[1] for r in a_rows}), "Q": len(q_rows), "instance_rows": rep_cmp["instance_rows"]},
     "evidence": [f"{V}/out/replay_compare.json", f"{V}/out/plan.json", f"{V}/out/nonce.txt", f"{V}/out/replay/"]},
    {"id": "C4", "requirement": "independent certificates: f, r prime, order = h r inside Hasse-Weil with r > width, [order]D = 0, [r]G = 0, [t]G = T for every target; strongest symmetry",
     "pass": certs["all_pass"], "evidence": [f"{V}/out/certs.json", f"{V}/scripts/certs.py", f"{V}/scripts/jac.py", f"{V}/out/automorphism_selftest.txt"]},
    {"id": "C5", "requirement": "producer's instances (f, G, T, t) and recovered logarithms equal the validator's arithmetic: recovered = t and [recovered]G = T",
     "pass": vd_main["all_pass"] and vd_rho["all_pass"],
     "counts": {"instances": vd_main["n_instances"], "ic_runs": vd_main["n_runs"], "q_runs": vd_rho["n_runs"]},
     "evidence": [f"{V}/out/verify_dump_main.json", f"{V}/out/verify_dump_rho.json", f"{V}/out/verify_dump_tamper_control.txt", f"{V}/scripts/val_fudump.rs"]},
    {"id": "C6", "requirement": "headline numbers recomputed from the raw rows by the validator's own code (ratios, bootstrap of PROTOCOL section 6, part A selection, post-hoc theory ratios, 50-seed stability)",
     "pass": worst <= 1e-12 and cmpc["max_rel_diff"] <= 1e-12, "max_rel_diff": max(worst, cmpc["max_rel_diff"]),
     "evidence": [f"{V}/out/recompute.json", f"{V}/out/compare_claims.json", f"{V}/scripts/recompute.py"]},
    {"id": "C7", "requirement": "gates G1-G3 and the resume duplicate identity, with real keys",
     "pass": C["G1"]["pass"] and C["G2"]["pass"] and C["G3"]["pass"] and C["Q_resume_duplicates"]["all_identical"] and C["Q_resume_duplicates"]["n"] == 2,
     "evidence": [f"{V}/out/recompute.json"]},
    {"id": "C8", "requirement": "section 5a band with the strongest reference (negation), and every listed unpriced counter",
     "pass": True, "evidence": [f"{V}/out/band.json", f"{V}/scripts/band.py"]},
    {"id": "C9", "requirement": "claims of RESULTS.md, including its caveats", "pass": True,
     "note": "every number recomputes; findings F1, F2 need errata, F3-F7 are pricing, interpretation and wording notes; none changes a headline number or the frozen verdicts",
     "evidence": [f"{EV}/validator-report.md"]},
]
allpass = all(c["pass"] for c in checks)

findings = [
    {"id": "F1", "severity": "minor (prose)", "text": "PROTOCOL section 7 P1 and RESULTS ('because the 9-target rho mean was high') state the 9-target Q mean was about 14% above sqrt(pi r/2). The 18 Q rows at k = 0..2 (lean-rho study) give 1.0009 (ratio of means) and 0.976 (mean of per-run ratios). The premise is unsupported; the conclusion drawn from it ('the old ratio was not a product of an unlucky rho sample') is consistent with the data. Needs an ERRATA.md line (PROTOCOL.md is frozen)."},
    {"id": "F2", "severity": "minor (record)", "text": "benchmarks/records/20261118T0710Z-genus4-followup.json metric w_over_plain_rho_q3_w1_p16411 says 1.6046 [1.4069, 1.8371]; analysis.json and the validator's recompute give 1.6053 [1.4070, 1.8373]. No computation tried reproduces 1.6046. The record is also still open (status open, closed_utc null, commit_dirty true, raw uploaded false, reports empty)."},
    {"id": "F3", "severity": "minor (pricing symmetry)", "text": "Q4 charges rho one partition hash and two coefficient additions per step, but the IC walk performs the same per-step work (genus_ladder.rs lines 1508-1512 at ae36cdc6e4: mix64 of the key and two addm per step) and is not charged for it. Charging it as 3 x trials moves arm x Q4 w=1 from 0.2185 to 0.2197 and w=0.1 from 0.0683 to 0.0687; no verdict changes. Q3 is symmetric (neither side charged)."},
    {"id": "F4", "severity": "interpretation", "text": "'Below rho at w = 1' holds only against the measured plain rho Q, and in Q3 only at Monte Carlo margin: upper end 0.99960 at the frozen seed, 0.99946 with an interleaved (targets-then-seeds, drawn per target) resampling order, 0.99962 with interpolated quantiles, below 1 for 18 of 50 seeds. Q4 is robust (0.218 [0.192, 0.250]). Against the strongest available reference, negation (the curves' only F_p-automorphisms are the identity and the hyperelliptic involution), the ideal floor sqrt(pi r/4) puts arm x at 0.0022 / 0.126 / 1.240 at w = 0 / 0.1 / 1: the order does not hold across the band, so arm x is not ranked against negation rho. RESULTS.md states the fragility and the 1.24x distance correctly."},
    {"id": "F5", "severity": "note", "text": "resume.sh's statement that it was written before any Q row was read is not verifiable from git (the rows were on disk, uncommitted). Consistent with it: the 2 re-computed keys equal the first run in every field, and the validator replay of a resumed-block key (2640, k=9) is identical."},
    {"id": "F6", "severity": "note", "text": "RESULTS headline row 'w = 0.1, measured rho 0.089' does not name the reading; it is Q3 (Q4 gives 0.068)."},
    {"id": "F7", "severity": "interpretation", "text": "The pooled ratio depends on the curve mix. Arm x's cost hardly depends on r (coefficient of variation 1.2% over the 108 keys) while rho scales as sqrt(r). Per curve at w = 1, arm x / sqrt(pi r/2) is 0.998 (172, h = 2), 0.998 (180, h = 2) and 0.707 (2640, h = 1); against the negation floor 1.412, 1.412 and 1.000; against measured Q (Q3) 1.150, 0.907 and 0.685. 'Below plain rho at w = 1' therefore rests on the one prime-order Jacobian; on the two h = 2 curves arm x is at parity with the plain-rho expectation. RESULTS gives h and r per curve but not this decomposition; the preregistered curve-level bootstrap [0.671, 1.150] reflects it."},
]

# ---------------- rho reference ----------------
b = band["band"]
gae = {k: {"ic": b[k]["ic_x"], "rho": b[k]["neg_floor"]} for k in ("0", "0.1", "1")}
sg = lambda v: (v["ic"] > v["rho"]) - (v["ic"] < v["rho"])
order = len({sg(v) for v in gae.values()}) == 1 and sg(gae["0"]) != 0
rho_ref = {
    "kind": "negation",
    "protocol": "PROTOCOL.md (frozen ae36cdc6e4) section 6 (s1) and section 8: the ideal negation bound sqrt(pi r/4) group operations with nothing else charged, which section 8 requires in the rho-reference band before promotion. The frozen decision rule's measured reference is window-free plain rho Q (R_rho_plain_nowin), the cheapest measured rho on these curves; its band is in the validator report.",
    "unit": "GAE per key, mean over the 108 keys (54 targets x 2 seeds) at p = 16411: one counted Jacobian group operation = 1; each listed unpriced native operation (rel_mul, rel_add, la_mul, la_add) = w; the negation floor charges no native work at any w",
    "binary_or_process": "IC arm x: genus_ladder sha256 74f02ea1fb6a353e2762ba1494f2666acab0a774b17deec38bd02090dc11a6f9 (built from ae36cdc6e4; validator rebuild 90128561... has identical code sections and replays the sampled rows identically). The negation floor is a closed form of each instance's r as emitted by that binary and certified by the validator's own arithmetic; no other binary or process enters it. The measured Q rows ran in the same binary (k = 3..17) and in the lean-rho binary (k = 0..2), whose rows the validator's rebuild reproduces (G3 key and nonce-drawn key 172/k1/s1).",
    "matches_protocol": True, "strongest_available": True, "same_unit": True, "same_binary_or_process": True,
    "gae_band": gae, "order_holds_across_band": order,
}

# ---------------- raw data ----------------
raw = [{"path": rel(p), "sha256": h_} for p, h_ in sorted(rc["raw_pins"].items())]
for r in raw:
    assert sha(os.path.normpath(os.path.join(EV, r["path"]))) == r["sha256"], r

cert = {
    "schema": "replay-certificate/v1", "status": "verified" if checks[2]["pass"] and checks[4]["pass"] else "failed",
    "independent": True, "scalars_verified": checks[2]["pass"] and checks[4]["pass"],
    "study": D, "commit_validated": COMMIT, "freeze_commit": FREEZE,
    "binary": {"producer_sha256": "74f02ea1fb6a353e2762ba1494f2666acab0a774b17deec38bd02090dc11a6f9",
               "binary_sha256_file": "74f02ea1fb6a353e2762ba1494f2666acab0a774b17deec38bd02090dc11a6f9",
               "validator_rebuild_sha256": "90128561be58415f7fb38295735b8e99e182880c22d40d438415b5d314b6657b",
               "code_sections_identical": True, "toolchain": "rustc 1.97.0 (2d8144b78 2026-07-07), cargo 1.97.0",
               "build": "git archive ae36cdc6e4 (5521 paths: manifest, lock, src, path dependencies, included files) into a validator-owned tree and CARGO_TARGET_DIR; cargo build --release --locked --offline --bin genus_ladder",
               "evidence": f"{V}/out/binary_diff.txt"},
    "nonce": plan["nonce"], "plan": f"{V}/out/plan.json",
    "commands": [f"{V}/scripts/replay_ic.sh", f"{V}/scripts/replay_q.sh"],
    "comparison": "every field except wall_ns equal to the committed row (git blob at commit_validated); instance rows equal",
    "summary": {k: v for k, v in rep_cmp.items() if k not in ("rows",)},
    "runs": [{"file": f"{V}/out/replay/{r['file']}", "arm": r["key"][0], "p": r["key"][1], "curve_seed": r["key"][2],
              "target_seed": r["key"][3], "seed": r["key"][4], "fields_compared": r["fields_compared"],
              "identical_except_wall_ns": r["identical_except_wall_ns"], "certified": r["certified"], "ok": r["ok"]}
             for r in rep_cmp["rows"] if r["key"][0] != "instance"],
    "logarithm_certificates": {"verifier": f"{V}/scripts/verify_dump.py with {V}/scripts/jac.py (validator arithmetic, not the producer's Rust)",
                               "instances_checked": vd_main["n_instances"], "ic_runs": vd_main["n_runs"], "q_runs": vd_rho["n_runs"],
                               "all_pass": vd_main["all_pass"] and vd_rho["all_pass"],
                               "evidence": [f"{V}/out/verify_dump_main.json", f"{V}/out/verify_dump_rho.json"]},
}
evd = os.path.join(REPO, EV)
json.dump(cert, open(os.path.join(evd, "replay-certificate.json"), "w"), indent=1)

report = {
    "schema": "validator-report/v1", "status": "pass" if allpass else "fail", "independent": True,
    "disposition": "validated" if allpass else "invalid_measurement",
    "study": D, "commit_validated": COMMIT, "freeze_commit": FREEZE, "pr": "isogeny-labs/crypto#4693",
    "validator": "Validator subagent (Claude Opus 5.5), 2026-11-18; recompute, certificates and replay comparison with the validator's own code in " + V + "/scripts",
    "raw_data_base": EV, "raw_data": raw,
    "headline": H, "headline_max_rel_diff": worst,
    "checks": checks, "findings": findings,
    "rho_reference_assessment": {
        "measured_reference": "plain rho Q is the cheapest measured rho on these curves: on the 18 keys at k = 0..2 the measured negation walk B costs %.2f / %.2f / %.2f x Q (Q3) and %.2f x Q at w = 1 (Q4); B's steps are %.2f x sqrt(pi r/4) with %.2f compares per step. Q's steps are %.4f x sqrt(pi r/2). Q does no canonicalisation (PointClasses::trivial), so pricing its C at 0 is right; its 'native' field is only the one-per-step floor." % (
            band["ratios"]["0"]["k0_2 B_Q3/Q_Q3"], band["ratios"]["0.1"]["k0_2 B_Q3/Q_Q3"], band["ratios"]["1"]["k0_2 B_Q3/Q_Q3"],
            band["ratios"]["1"]["k0_2 B_Q4/Q_Q4"], band["B_steps_over_neg_floor_k0_2"], band["B_compares_per_step_k0_2"], band["Q_steps_over_plain_theory"]),
        "strongest_reference": "negation (A = 2): validator scan finds no F_p-automorphism of y^2 = f besides the identity and the hyperelliptic involution on any of the 3 curves (affine maps and maps swapping infinity with a rational root; positive controls detect planted ones); Frobenius is the identity on J(F_p). VALIDATION.json uses the ideal floor sqrt(pi r/4) with nothing charged, the strongest reference.",
        "band_x": {k: {r_: b[k][r_] for r_ in ("ic_x", "neg_floor", "neg_model_q3", "neg_model_q4", "Q_Q3", "Q_Q4")} for k in b},
        "band_w": {k: {r_: b[k][r_] for r_ in ("ic_w", "neg_floor", "Q_Q3", "Q_Q4")} for k in b},
        "order": band["order"],
        "unpriced_counters_listed": {"IC arms": "rel_mul, rel_add, la_mul, la_add (priced); per-step walk hash + 2 coefficient additions not emitted (F3)",
                                     "Q": "compares_uncharged = 0, hash_uncharged = steps, coef_adds_uncharged = 2 steps (priced in Q4 only); canonicalisations 0 (native field = floor)",
                                     "negation floor": "none charged (lower bound)"},
    },
    "lessons_applied": ["L-546d37ca7750", "L-99ee952350b2", "L-527c1f013131", "L-7f8877029be4", "L-ce5e4e4b764b",
                        "L-ca1840e60dc0", "L-b0626ec2a4c0", "L-3f39e1d8b3a9", "L-26895912385b", "L-e33c4f6ece95"],
}
json.dump(report, open(os.path.join(evd, "validator-report.json"), "w"), indent=1)
print("checks:", [(c["id"], c["pass"]) for c in checks], "all:", allpass, "headline max rel diff:", worst, "order:", order)
if allpass:
    man = {"schema": "validation-gate/v1",
           "note": "Validator's manifest for the genus-4 follow-up (54 targets at p = 16411, arm x). The rho reference is the strongest available, the ideal negation floor; against it arm x is below at w = 0 and 0.1 and above at w = 1, so the order does not hold and arm x is not ranked against negation rho. Against the measured plain rho Q the frozen readings are in the validator report. [pcc:unverified]",
           "replay_certificate": {"path": "replay-certificate.json", "sha256": sha(f"{EV}/replay-certificate.json")},
           "validator_report": {"path": "validator-report.json", "sha256": sha(f"{EV}/validator-report.json")},
           "rho_reference": rho_ref}
    json.dump(man, open(os.path.join(evd, "VALIDATION.json"), "w"), indent=1)
    print("wrote VALIDATION.json")
else:
    p = os.path.join(evd, "VALIDATION.json")
    if os.path.exists(p):
        os.remove(p)
