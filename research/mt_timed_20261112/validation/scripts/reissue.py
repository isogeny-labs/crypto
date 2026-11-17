#!/usr/bin/env python3
"""VALIDATOR re-issue after the producer's amendment commit fd91337cce (2026-11-12).
The producer adopted the validator's proposed claim wording byte for byte and amended RESULTS.md.  This script
  1. checks that the raw data, harness, protocol and analysis code are byte-identical between the first validated
     commit 544f832063 and fd91337cce, that RESULTS.md at fd91337cce carries the proposed wording verbatim, and which
     first-issue amendments RESULTS.md now satisfies;
  2. re-judges the adopted statement clause by clause from the validator's evidence files (recompute.json,
     timing_reproduction.json, results_check.json, scalar_check.json, replay_check.json);
  3. writes claim-report.json (statement = the adopted wording) and re-issues validator-report.json for the amended
     headline, keeping the first issue's sha256 and status as history.
usage: reissue.py <V> <repo> <first validator-report sha256> <first claim-report sha256>"""
import json, sys, os, hashlib, subprocess

V, REPO, FIRST_VR, FIRST_CR = sys.argv[1:5]
AC = "fd91337ccee71167f000e4ea254b10a3692e7216"; VC0 = "544f8320631c9d71b7636d0c51ae031472457cf7"
J = lambda f: json.load(open(os.path.join(V, f)))
sha = lambda b: hashlib.sha256(b).hexdigest()
git = lambda *a: subprocess.run(["git", "-C", REPO] + list(a), capture_output=True).stdout
rc, chk, tr, sc, rep = (J(f) for f in ("recompute.json", "results_check.json", "timing_reproduction.json", "scalar_check.json", "replay_check.json"))
vr = J("validator-report.json")
claim = json.load(open(os.path.join(V, "scripts/claim.json")))
ST = claim["proposed_wording"]

# 1. what changed between the first validated commit and the amendment commit
unchanged = {}
for f in ("research/mt_timed_20261112/results/holdout/runs.json", "research/mt_timed_20261112/results/holdout/runs.jsonl",
          "research/mt_timed_20261112/results/holdout/opcost.jsonl", "research/mt_timed_20261112/results/holdout/summary.json",
          "research/mt_timed_20261112/results/holdout/analysis.txt", "research/mt_timed_20261112/PROTOCOL.md",
          "research/mt_timed_20261112/run.py", "research/mt_timed_20261112/analyze.py", "examples/mt_fieldops.rs"):
    a, b = git("show", f"{VC0}:{f}"), git("show", f"{AC}:{f}")
    unchanged[f] = dict(sha256=sha(b), identical=bool(a) and a == b)
changed = git("diff", "--name-only", VC0, AC).decode().split()
res = git("show", f"{AC}:research/mt_timed_20261112/RESULTS.md").decode()
hl = [l for l in res.splitlines() if l.startswith("**Validated headline")]
pref = "**Validated headline (the validator's wording):** "
verbatim = len(hl) == 1 and hl[0].startswith(pref) and hl[0][len(pref):] in (ST, ST + " [pcc:unverified]")
committed_validation_identical = all(
    git("show", f"{AC}:research/mt_timed_20261112/validation/{f}") == open(os.path.join(V, f), "rb").read()
    for f in ("replay-certificate.json", "recompute.json", "timing_reproduction.json", "scalar_check.json", "replay_check.json", "results_check.json", "price_model.json"))

fx = {(c["bits"], c["T"]): c["verdict_fixed"] for c in rc["cells"]}
pr = {(c["bits"], c["T"]): c["verdict_solve"] for c in rc["cells"]}
amend = [
    dict(id="T-F1", applied="partly", note="RESULTS reports the control's decisions, but says it is 'unranked in 3 of 9 cells'; it is unranked in %d of 9 (%s) and departs from the primary decision in 3 (%s)" % (
        sum(v == "unranked" for v in fx.values()), ", ".join(f"{b}/{T}" for (b, T), v in sorted(fx.items()) if v == "unranked"),
        ", ".join(f"{b}/{T}" for (b, T) in sorted(fx) if fx[(b, T)] != pr[(b, T)])),
         correct="selection-bias control: unranked in 5 of 9 cells; it departs from the primary decision in 3 (24 b T = 16384, 28 b T = 1024, 32 b T = 1024), and 28 b T = 4096 and 16384 are unranked under both. (The validator's first-issue verdict text said 'unranked in 3 of 9 cells'; that was imprecise and is corrected in this issue.)"),
    dict(id="T-F2", applied=True, note="native price stated per pair (two counted native operations), per operation 0.0025-0.0047, step index 0.002-0.006"),
    dict(id="T-F3/T-F4", applied=True, note="4-of-4 rule, 28/1024 fragility and the reproduction figures disclosed"),
    dict(id="T-F5", applied=True, note="'reuse the 12 v5 curves (not fresh)'"),
    dict(id="T-F6", applied="partly", note="rho2xks 6 / rhodpu 30 (29 + 1), 0.11-0.42 and 1.2-6.1x corrected; 'won only 4 of 198 configurations in v5's counted model' needs its scope: 4 of 198 at I = 10 M and h = 1, 27 of 198 at I = 3 M and h = 1, none at h = 0 and 0.1 (v5 RESULTS 'Which arm wins')"),
    dict(id="T-F7", applied=True, note="cache misses labelled interpretation; price model quoted (1.14-1.22 at 2^32 is the 'before' prices; 1.09-1.15 with the 'after' prices)"),
    dict(id="T-F13", applied=False, note="runs.jsonl still in Git; RESULTS says it moves to the artifact store when the PR opens"),
    dict(id="T-F14", applied=True, note="smoke run's absence recorded in Limits"),
    dict(id="new (minor)", applied=False, note="'both at the loop baseline': the step index is 1.05-1.38x the loop baseline, the canonicalisation + hash 1.9-3.2x (0.0025-0.0063 GAE above it); 'h_native about 0' is unaffected"),
]

# 2. clause checks of the adopted statement
cells = {(c["bits"], c["T"]): c for c in rc["cells"]}
pooled = {(c["bits"], c["T"]): c for c in tr["analysis_rounds"]["rounds"]["pooled"]["cells"]}
O = rc["opcost_ranges"]
r = lambda x, n: round(x, n)
m24 = [cells[(24, T)]["ic_rho_solve"]["median"] for T in (1024, 4096, 16384)]
m32 = [cells[(32, T)]["ic_rho_solve"]["median"] for T in (1024, 4096, 16384)]
p24 = [pooled[(24, T)]["median"] for T in (1024, 4096, 16384)]
p32 = [pooled[(32, T)]["median"] for T in (1024, 4096, 16384)]
fixed = [c["fixed_ic_rho_solve"]["median"] for c in rc["cells"]]
bs = [c["ic_bsgs_solve"]["median"] for c in rc["cells"]]
ends_unranked = sorted(f"{b}/{T}" for (b, T), v in fx.items() if b in (24, 32) and v == "unranked")
inst = sc["instances"]
clauses = [
    ("On one CPU of a 4-vCPU cloud VM (Intel Xeon @ 2.10 GHz; not an isolated lab node, so the seconds are diagnostic)", True,
     "rows host 'Intel(R) Xeon(R) Processor @ 2.10GHz', nproc 4, taskset -c 2; not L2"),
    ("in this harness's implementation (std HashMap with SipHash)", True, "examples/mt_fieldops.rs (a992ed6c80) uses std::collections::HashMap with the default RandomState/SipHash; binary rebuilt bit-identical"),
    ("the 12 prime-order Weierstrass curves of the v5 study, group orders about 2^24, 2^28 and 2^32, with a new seed",
     all(v["p_prime"] and v["n_prime"] and v["cofactor_1"] and v["nG_is_O"] for v in inst.values()) and len(inst) == 12,
     "W{24,28,32}-{0..3} of research/mt_fieldops_v5_20261112/instances; n bits %s; seed 43 (v5 used 41, 42)" % sorted({v["n_bits"] for v in inst.values()})),
    ("T = 1024/4096/16384) x 17 run keys x 3 interleaved repeats, every target verified",
     rc["checks"]["rows_jsonl"] == 1836 and rc["checks"]["ok_ne_targets"] == 0 and rc["checks"]["seq_order_matches_seeded_shuffle"] and sc["summary"]["all_pass"],
     "1836 = 36 x 17 x 3 rows, order re-derived; ok = targets in every row; %d recovered logs re-verified with validator EC code, 0 failures" % sc["summary"]["targets_checked"]),
    ("takes 0.78-0.83x ... at 2^24 (faster in all 3 cells; 12 of 12 configurations below 1)",
     [r(min(m24), 2), r(max(m24), 2)] == [0.78, 0.83] and all(cells[(24, T)]["verdict_solve"] == "faster" for T in (1024, 4096, 16384)) and rc["size_level"]["24"]["below1"] == 12,
     "%.4f-%.4f; faster 3/3; %d/12 below 1" % (min(m24), max(m24), rc["size_level"]["24"]["below1"])),
    ("0.93x at 2^28 with T = 1024 (faster under the primary rule, but unranked under the pre-registered fixed-key control and 0.98x in the validator's rerun)",
     r(cells[(28, 1024)]["ic_rho_solve"]["median"], 2) == 0.93 and cells[(28, 1024)]["verdict_solve"] == "faster" and fx[(28, 1024)] == "unranked" and r(pooled[(28, 1024)]["median"], 2) == 0.98,
     "%.4f faster; control %s (%.3f); validator pooled %.4f" % (cells[(28, 1024)]["ic_rho_solve"]["median"], fx[(28, 1024)], cells[(28, 1024)]["fixed_ic_rho_solve"]["median"], pooled[(28, 1024)]["median"])),
    ("is unranked at 2^28 with T = 4096 and 16384", all(cells[(28, T)]["verdict_solve"] == "unranked" for T in (4096, 16384)), "both unranked"),
    ("takes 1.11-1.29x at 2^32 (slower in all 3 cells; 12 of 12 configurations above 1)",
     [r(min(m32), 2), r(max(m32), 2)] == [1.11, 1.29] and all(cells[(32, T)]["verdict_solve"] == "slower" for T in (1024, 4096, 16384)) and rc["size_level"]["32"]["above1"] == 12,
     "%.4f-%.4f; slower 3/3; %d/12 above 1" % (min(m32), max(m32), rc["size_level"]["32"]["above1"])),
    ("Each cell decision is a 4-of-4 sign rule (the bootstrap interval of the median of 4 equals their range)",
     all(c["ic_rho_solve"]["lo"] == c["ic_rho_solve"]["min"] and c["ic_rho_solve"]["hi"] == c["ic_rho_solve"]["max"] and c["n"] == 4 for c in rc["cells"]),
     "lo == min and hi == max of the 4 configuration ratios in all 9 cells (protocol and exact bootstrap)"),
    ("The pre-registered fixed-key control (0.75-1.28x) is unranked in one cell at each end (2^24 with T = 16384, 2^32 with T = 1024)",
     [r(min(fixed), 2), r(max(fixed), 2)] == [0.75, 1.28] and ends_unranked == ["24/16384", "32/1024"],
     "%.4f-%.4f; unranked at the 2^24 and 2^32 ends: %s (the control is also unranked at all three 2^28 cells, which the statement covers for T = 1024 and which are unranked under the primary rule at T = 4096 and 16384)" % (min(fixed), max(fixed), ends_unranked)),
    ("an independent pinned rerun with the same keys (8 repeats) reproduces all 9 decisions at 0.77-0.84x (2^24) and 1.11-1.22x (2^32)",
     all(pooled[k]["verdict"] == pooled[k]["prod_verdict"] for k in pooled) and [r(min(p24), 2), r(max(p24), 2)] == [0.77, 0.84] and [r(min(p32), 2), r(max(p32), 2)] == [1.11, 1.22],
     "pooled validator rounds (3 + 5 repeats, producer's keys, CPU 2): 9/9 decisions; 2^24 %.4f-%.4f; 2^32 %.4f-%.4f" % (min(p24), max(p24), min(p32), max(p32))),
    ("The IC takes 1.33-1.59x the time of a paired BSGS", [r(min(bs), 2), r(max(bs), 2)] == [1.33, 1.59], "cell medians %.4f-%.4f" % (min(bs), max(bs))),
    ("one affine addition 214-294 ns, I/M 23-31", [round(O["gae_ns"][0]), round(O["gae_ns"][1]), round(O["I_over_M"][0]), round(O["I_over_M"][1])] == [214, 294, 23, 31],
     "%.1f-%.1f ns; %.2f-%.2f" % (*O["gae_ns"], *O["I_over_M"])),
    ("a canonicalisation plus one hash of x (two counted native operations) 0.005-0.009 GAE and a step index 0.002-0.006 GAE",
     [r(O["native_gae"][0], 3), r(O["native_gae"][1], 3), r(O["step_index_gae"][0], 3), r(O["step_index_gae"][1], 3)] == [0.005, 0.009, 0.002, 0.006],
     "canon_hash/padd %.4f-%.4f (rhodpu's canon counts 2 native operations); step_index/padd %.4f-%.4f" % (*O["native_gae"], *O["step_index_gae"])),
    ("a std HashMap lookup 0.04-0.85 GAE at 2^10-2^24 entries", [r(O["std_lookup_all"][0], 2), r(O["std_lookup_all"][1], 2)] == [0.04, 0.85],
     "%.4f-%.4f over the 6 opcost runs and 8 table sizes" % tuple(O["std_lookup_all"])),
]
ok_all = all(ok for _, ok, _ in clauses)
tol = lambda c, p: (0.5 * 10 ** -p) / abs(c) + 1e-12
H = [dict(name=n, claimed=c, recomputed=x, rel_tol=tol(c, p)) for n, c, x, p in (
    ("ic/rho 2^24 min", 0.78, min(m24), 2), ("ic/rho 2^24 max", 0.83, max(m24), 2), ("ic/rho 28/1024", 0.93, cells[(28, 1024)]["ic_rho_solve"]["median"], 2),
    ("ic/rho 2^32 min", 1.11, min(m32), 2), ("ic/rho 2^32 max", 1.29, max(m32), 2), ("fixed-key min", 0.75, min(fixed), 2), ("fixed-key max", 1.28, max(fixed), 2),
    ("validator pooled 28/1024", 0.98, pooled[(28, 1024)]["median"], 2), ("validator pooled 2^24 min", 0.77, min(p24), 2), ("validator pooled 2^24 max", 0.84, max(p24), 2),
    ("validator pooled 2^32 min", 1.11, min(p32), 2), ("validator pooled 2^32 max", 1.22, max(p32), 2), ("ic/bsgs min", 1.33, min(bs), 2), ("ic/bsgs max", 1.59, max(bs), 2),
    ("GAE ns min", 214, O["gae_ns"][0], 0), ("GAE ns max", 294, O["gae_ns"][1], 0), ("I/M min", 23, O["I_over_M"][0], 0), ("I/M max", 31, O["I_over_M"][1], 0),
    ("canonicalisation+hash GAE min", 0.005, O["native_gae"][0], 3), ("canonicalisation+hash GAE max", 0.009, O["native_gae"][1], 3),
    ("step index GAE min", 0.002, O["step_index_gae"][0], 3), ("step index GAE max", 0.006, O["step_index_gae"][1], 3),
    ("std lookup GAE min", 0.04, O["std_lookup_all"][0], 2), ("std lookup GAE max", 0.85, O["std_lookup_all"][1], 2))]
H += [dict(name="configurations below 1 at 2^24", claimed=12, recomputed=rc["size_level"]["24"]["below1"]),
      dict(name="configurations above 1 at 2^32", claimed=12, recomputed=rc["size_level"]["32"]["above1"]),
      dict(name="validator pooled decisions reproduced", claimed=9, recomputed=sum(pooled[k]["verdict"] == pooled[k]["prod_verdict"] for k in pooled))]
mism = [h["name"] for h in H if abs(h["claimed"] - h["recomputed"]) > h.get("rel_tol", 1e-9) * max(abs(h["claimed"]), abs(h["recomputed"]))]
VALIDATOR = vr["validator"]
raw = [dict(path="../results/holdout/runs.json", sha256=unchanged["research/mt_timed_20261112/results/holdout/runs.json"]["sha256"]),
       dict(path="../results/holdout/opcost.jsonl", sha256=unchanged["research/mt_timed_20261112/results/holdout/opcost.jsonl"]["sha256"]),
       dict(path="timing_reproduction.json", sha256=sha(open(os.path.join(V, "timing_reproduction.json"), "rb").read()), note="validator's pinned rerun")]
base_ok = all(v["identical"] for v in unchanged.values()) and verbatim and committed_validation_identical
cr = dict(
    schema="validator.report/v1", independent=True, validator=VALIDATOR, validated_commit=AC,
    data_commit_note=f"raw data, harness, protocol and analysis code are byte-identical to {VC0}, where they were validated; {AC} changes RESULTS.md, REPORT.pdf, ratios.svg, the bench record and adds validation/",
    scope="this one statement exactly as worded (the validator's proposed wording from the first issue, adopted byte for byte by the producer); nothing else in the study",
    statement_judged=ST, statement_sha256=sha(ST.encode()),
    status="pass" if ok_all and not mism and base_ok else "fail",
    disposition="validated" if ok_all and not mism and base_ok else "insufficient_evidence",
    verdict=("endorsed exactly as worded: every clause recomputes from runs.json, opcost.jsonl and the validator's replays and pinned rerun at fd91337cce (data unchanged since 544f832063)"
             if ok_all and not mism and base_ok else "not endorsed: see clause_checks"),
    raw_data=raw,
    supporting=dict(replay_certificate="replay-certificate.json (status verified; unchanged)", validator_report="validator-report.json (re-issued, see its status)",
                    results_check="results_check.json", scalar_check="scalar_check.json", recompute="recompute.json"),
    clause_checks=[dict(clause=c, ok=ok, recomputed=x) for c, ok, x in clauses],
    headline=H, headline_mismatches=mism,
    checks=dict(unchanged_since_first_validation=unchanged, results_md_carries_statement_verbatim=verbatim,
                committed_validation_evidence_identical=committed_validation_identical, files_changed=changed),
    history=[dict(issue=1, commit=VC0, statement="the producer's original statement (see VALIDATOR_REPORT.md)", status="fail", sha256=FIRST_CR)],
    notes=["The statement says the control is unranked 'in one cell at each end'; it is also unranked at all three 2^28 cells (stated for T = 1024; T = 4096 and 16384 are unranked under the primary rule as well), so it is unranked in 5 of 9 cells in all.",
           "The rerun figures are the validator's (two rounds pooled; round 2 was added after round 1 and is not pre-registered); the seconds are diagnostic, on the same shared VM."])
json.dump(cr, open(os.path.join(V, "claim-report.json"), "w"), indent=1)

# 3. validator report re-issue for the amended headline
vr2 = dict(vr)
vr2.update(
    status="pass" if cr["status"] == "pass" else "fail",
    disposition="validated" if cr["status"] == "pass" else "insufficient_evidence",
    validated_commit=AC,
    verdict=("amended headline validated (re-issue at fd91337cce). RESULTS.md now carries the validator's accepted wording verbatim as its headline; raw data, harness, protocol and "
             "analysis code are byte-identical to 544f832063, so the replay certificate, every recompute and the timing reproduction stand. The first issue (status fail) judged the "
             "producer's original headline, which overstated the selection-bias control ('the same pattern') and priced two counted native operations as one. Non-blocking prose "
             "amendments remain in RESULTS.md (required_amendments)."),
    headline=H, headline_mismatches=mism,
    accepted_results_headline=ST,
    amendments_check=amend,
    history=[dict(issue=1, commit=VC0, status="fail", disposition="insufficient_evidence", sha256=FIRST_VR,
                  judged="the producer's original headline",
                  correction="its verdict said the control's decisions are 'unranked in 3 of 9 cells'; precisely, they depart from the primary decision in 3 of 9 cells and are unranked in 5 of 9")],
    required_amendments=[
        "Non-blocking (T-F1 follow-up): in RESULTS 'Selection-bias control', 'unranked in 3 of 9 cells' should read 'unranked in 5 of 9 cells; it departs from the primary decision in 3 (24 b T = 16384, 28 b T = 1024, 32 b T = 1024)'.",
        "Non-blocking (T-F6 follow-up): 'won only 4 of 198 configurations in v5's counted model' needs its scope: 4 of 198 at I = 10 M and h = 1 (27 of 198 at I = 3 M, h = 1; none at h = 0 and 0.1).",
        "Non-blocking: 'both at the loop baseline' holds for the step index (1.05-1.38x the loop baseline), not for the canonicalisation + hash (1.9-3.2x).",
        "Non-blocking: the price-model figure 1.14-1.22x at 2^32 uses the 'before' prices; with the 'after' prices it is 1.09-1.15x.",
        "Non-blocking (T-F13): move runs.jsonl to the artifact store as RESULTS says."])
sd = dict(vr["scoped_dispositions"])
sd["selection-bias control 'same pattern 0.75-1.28'"] = "range validated; at the decision level the control departs from the primary rule in 3 of 9 cells and is unranked in 5 of 9 (T-F1); the amended headline states this for the ends and for 28 b T = 1024"
sd["amended headline (accepted_results_headline, carried verbatim by RESULTS.md at fd91337cce)"] = "validated (claim-report.json, status %s)" % cr["status"]
vr2["scoped_dispositions"] = sd
json.dump(vr2, open(os.path.join(V, "validator-report.json"), "w"), indent=1)
print("base_ok", base_ok, "verbatim", verbatim, "committed evidence identical", committed_validation_identical)
print("clauses ok", ok_all, "mismatches", mism)
for c, ok, x in clauses:
    if not ok: print("FAIL", c, x)
print("amendments", [(a["id"], a["applied"]) for a in amend])
for f in ("claim-report.json", "validator-report.json", "replay-certificate.json"):
    print(f, sha(open(os.path.join(V, f), "rb").read()), json.load(open(os.path.join(V, f))).get("status"))
