#!/usr/bin/env python3
"""Round-2 validator: write replay-certificate.json, validator-report.json, VALIDATOR_REPORT.md
and (only when both pass) VALIDATION.json into the benchmarks evidence directory.

Validator-written. Inputs are the outputs of this directory's scripts (scratch) and committed
repository files. Usage:
  write_outputs.py STUDY_DIR SCRATCH EVIDENCE_DIR SNAPSHOT_SHA VALIDATION_ID
"""
import gzip, hashlib, json, os, sys
import pandas as pd

study, scr, ev, snap, vid = sys.argv[1:6]
der = f"{scr}/derived"
REPO = os.path.abspath(os.path.join(study, "..", ".."))
REL = os.path.relpath(study, ev)  # ../../research/volcano_ecdlp_census_followup_20261023
BIN = "ac4bc5b26f858a4d8a374208297157840af25fb1dc25ea1c19dd41360735fb35"


def sha(p):
    h = hashlib.sha256()
    with open(p, "rb") as f:
        for b in iter(lambda: f.read(1 << 20), b""):
            h.update(b)
    return h.hexdigest()


cmp_ = json.load(open(f"{der}/compare.json"))
rec = json.load(open(f"{der}/recompute.json"))
red = json.load(open(f"{der}/rederive_summary.json"))
exc = json.load(open(f"{der}/extract_compare.json"))
rpc = json.load(open(f"{der}/report_check.json"))
pba = [json.loads(l) for l in open(f"{der}/phase_b_audits.jsonl")]
runs = [json.loads(l) for l in open(f"{scr}/replay.jsonl")]
sample = [l.strip() for l in open(f"{scr}/sample.txt") if l.strip()]
prog = {os.path.basename(d["out"]): d for d in (json.loads(l) for l in open(f"{study}/runs/progress.jsonl")) if d.get("status") == "done"}

# ------------------------------------------------------------------ replay certificate
rdir = os.path.join(ev, "replay-receipts", "round2")
cert_runs = []
for r in runs:
    s = r["session"]
    rp = os.path.join(rdir, f"{s}.{r['mode']}.json.xz")
    cert_runs.append({
        "run": s, "spec": prog[s]["spec"], "session_id": r["session_id"],
        "archive": f"{REL}/sessions/{s}.tar.xz", "archive_sha256": r["archive_sha256"],
        "archive_sha256_match": r["archive_sha256_match"], "binary_sha256_at_replay": r["binary_sha256_at_replay"],
        "mode": r["mode"], "fallback_reason": r["fallback_reason"], "verify_rc": r["verify_rc"], "verify_seconds": r["verify_s"],
        "audit_ok": r["audit_ok"], "session_status": r["session_status"], "problems": r["problems"], "checks": r["checks"],
        "records": r["records"], "verified_records": r["verified_records"],
        "scalars_verified": bool(r["verified_records"] == r["records"] and r["checks"].get("records") is True),
        "replays": r["replays"], "replays_identical": r["replays_reproduced"], "replay_details": r["replay_details"],
        "auditor_binary_sha256": r["auditor_binary_sha256"], "auditor_env_class_id": r["auditor_env_class_id"],
        "auditor_hardware_class_id": r["auditor_hardware_class_id"], "files_match_phase_b_audit": r["files_match_phase_b_audit"],
        "receipt_sha256": r["receipt_sha256"], "receipt": os.path.relpath(rp, ev),
        "receipt_xz_sha256": sha(rp) if os.path.exists(rp) else None})
tot = {"sessions_in_study": len(prog), "sessions_replayed": len(cert_runs),
       "records": sum(r["records"] for r in cert_runs), "records_scalar_verified": sum(r["verified_records"] for r in cert_runs),
       "measured_runs_replayed": sum(r["replays"] for r in cert_runs),
       "measured_runs_identical": sum(r["replays_identical"] for r in cert_runs),
       "replay_all_sessions": sum(1 for r in cert_runs if r["mode"] == "replay-all"),
       "fallback_sessions": [r["run"] for r in cert_runs if r["mode"] != "replay-all"]}
strata = sorted({"-".join(prog[r["run"]]["spec"].split("-")[:2]) for r in cert_runs})
all_strata = sorted({"-".join(d["spec"].split("-")[:2]) for d in prog.values()})
cert_ok = (len(cert_runs) == len(sample) and all(r["archive_sha256_match"] and r["audit_ok"] and r["verify_rc"] == 0 and
           r["scalars_verified"] and r["replays"] == r["replays_identical"] and r["files_match_phase_b_audit"] and
           r["binary_sha256_at_replay"] == BIN and not r["problems"] for r in cert_runs) and strata == all_strata)
rd_fail = {}
for s in red:
    for k, v in s["fails"].items():
        rd_fail[k] = rd_fail.get(k, 0) + v
cert = {
    "schema": "validation-gate/replay-certificate/v1",
    "status": "verified" if cert_ok else "failed",
    "independent": True, "scalars_verified": bool(cert_ok),
    "study": "research/volcano_ecdlp_census_followup_20261023", "validation_id": vid, "round": 2,
    "snapshot_commit": snap, "replayed_at_snapshot": snap,
    "replay_started_utc": runs[0].get("started_unix") and pd.Timestamp(runs[0]["started_unix"], unit="s").strftime("%Y-%m-%dT%H:%M:%SZ"),
    "binary": {"path": "target/release/ecbench (outside git)", "sha256_frozen": BIN,
               "sha256_checked_before_each_session": sorted({r["binary_sha256_at_replay"] for r in cert_runs}),
               "progress_header_sha256": BIN, "cargo_build_run": False},
    "sample": {"rule": "validation/round2/draw_sample.sh: the three V5 crater sessions with rho-cubic (V5-D/N/ctl-L0-c0); one session per (volcano, family) stratum by SHA-256('NONCE|V|FAMILY') mod n; extra V5 Tcr draws until >= 3 Tcr; one redone .r2 session by SHA-256('NONCE|r2'). Nonce = the snapshot commit, fixed before the draw; the draw differs from round 1's (nonce 12e5eb3f7d).",
               "nonce": snap, "sessions": sample, "strata_covered": strata, "strata_in_study": all_strata,
               "every_stratum_covered": strata == all_strata,
               "v5_crater_rho_cubic_sessions": [s for s in sample if s in ("V5-D-L0-c0", "V5-N-L0-c0", "V5-ctl-L0-c0")],
               "tcr_sessions": [s for s in sample if "-Tcr-" in s], "m_sessions": [s for s in sample if "-M-" in s],
               "redone_r2_sessions": [s for s in sample if s.endswith(".r2")],
               "busy_host_refused_then_rerun": [s for s in sample if s in ("V5-Tcr-L6-c45", "V5-Tcr-L6-c61")]},
    "what_was_replayed": "Each sampled session's committed archive sessions/<session>.tar.xz (SHA-256 checked against its runs/progress.jsonl line) was extracted into scratch, alone on disk, and audited with `ecbench verify --dir <extracted> --replay-all` (fallback `--replay 50` only past a 1800 s timeout; field `mode`). The audit re-derives the session from its own files (file hashes, spec and plan identity, seeds, execution order, record seals and count), re-checks every recovered logarithm [k]G = Q on every record (warm-up included), and re-executes every measured deterministic run, requiring identical answer, total, phases and counters. The receipt's file map and session id were compared with the committed Phase B audit receipt (sessions/<session>.audit.json). The extracted copy was deleted before the next session.",
    "independence": {"replayed_by": "round-2 validator process, separate from the Phase B driver, the Phase C analysis and the round-1 validation; started from committed archives only",
                     "same_vm_type_and_binary": True, "host": "shared 4-vCPU cloud VM, Intel Xeon @ 2.10 GHz, kernel 6.18.44-fc-v242",
                     "note": "operation counts do not depend on the host; this re-executes the counts with the same binary, it is not an independent solver implementation or independent hardware"},
    "totals": tot,
    "raw_record_rederivation": {
        "script": f"{REL}/validation/round2/rederive.py",
        "archives": len(red), "archive_sha256_equal_progress": sum(1 for s in red if s["archive_sha_ok"]),
        "spec_json_equal_committed_spec": sum(1 for s in red if s["spec_json_equal"]),
        "records": sum(s["records"] for s in red), "failures": rd_fail,
        "generators_order_r_checked": sum(s["generators"] for s in red),
        "generators_ok": all(s["generators_ok"] for s in red),
        "checks": "every record of all 147 archives streamed (nothing unpacked to disk): outcome verified and matches_planted; [recovered]G = Q and Q on the curve with validator-written affine arithmetic; [r]G = O per generator; S = total_gae / sqrt(r); binary sha256 = frozen; one session id; census vertex from j(a, b) (Tcr: source vertex, crater model, generator equal to census/V5/tcr/<id>.json g; M: model index from census/<V>/M.json); level from the vertex equals the spec level",
        "extract_comparison": exc},
    "phase_b_audits_all_sessions": {"sessions": len(pba), "ok_complete": sum(1 for a in pba if a["ok"] is True and a["st"] == "complete"),
                                    "records": sum(a["rec"] for a in pba), "verified": sum(a["ver"] for a in pba),
                                    "replays": sum(a["rep"] for a in pba), "replays_reproduced": sum(a["repok"] for a in pba),
                                    "auditor_binaries": sorted({a["bin"] for a in pba})},
    "runs": cert_runs,
}
json.dump(cert, open(os.path.join(ev, "replay-certificate.json"), "w"), indent=1)

# ------------------------------------------------------------------ GAE band (s5a)
df = pd.read_csv(f"{der}/rows.csv.gz")
m = df[df.warmup == 0]
ISO = {"V6": ["D", "Ua", "Ub", "S", "N"], "V5": ["D", "Ua", "Ub", "N"]}
band, byv = {}, {}
for V in ("V6", "V5"):
    d = m[(m.V == V) & (m.fam.isin(ISO[V])) & (m.arm.isin(["ic", "rho-neg"]))]
    if V == "V5":
        d = d[d.level >= 1]
    w = d.pivot_table(index=["slug", "workload_id", "target_index", "round"], columns="arm", values=["S", "total_gae", "unc"], aggfunc="first").dropna()
    for p, k in ((0.0, "0"), (0.1, "0.1"), (1.0, "1")):
        ic = (w["S"]["ic"] * (w["total_gae"]["ic"] + p * w["unc"]["ic"]) / w["total_gae"]["ic"]).mean()
        rh = (w["S"]["rho-neg"] * (w["total_gae"]["rho-neg"] + p * w["unc"]["rho-neg"]) / w["total_gae"]["rho-neg"]).mean()
        byv[f"{V}{'_levels_1-6' if V == 'V5' else ''}|{k}"] = {"ic": ic, "rho_neg": rh, "ratio": ic / rh, "n": int(len(w))}
        if V == "V6":
            band[k] = {"ic": ic, "rho": rh}
d = m[(m.V == "V5") & (m.vertex == 0) & (m.fam.isin(["D", "N"])) & (m.arm.isin(["ic", "rho-cubic", "rho-neg"]))]
w = d.pivot_table(index=["slug", "workload_id", "target_index", "round"], columns="arm", values=["S", "total_gae", "unc"], aggfunc="first").dropna()
for p, k in ((0.0, "0"), (0.1, "0.1"), (1.0, "1")):
    pr = {a: (w["S"][a] * (w["total_gae"][a] + p * w["unc"][a]) / w["total_gae"][a]).mean() for a in ("ic", "rho-cubic", "rho-neg")}
    byv[f"V5crater|{k}"] = {"ic": pr["ic"], "rho_cubic": pr["rho-cubic"], "rho_neg": pr["rho-neg"], "ic_over_cubic": pr["ic"] / pr["rho-cubic"], "n": int(len(w))}
order_ok = all(v["ratio"] > 1 for k, v in byv.items() if "ratio" in v) and all(v["ic_over_cubic"] > 1 for k, v in byv.items() if "ic_over_cubic" in v)
order_ok = order_ok and all(c["recomputed"] for c in cmp_["categorical"] if c["name"].endswith("ci95_lo>1"))

# ------------------------------------------------------------------ validator report
H = cmp_["headline"]; CAT = cmp_["categorical"]; S = cmp_["summary"]
hm = rec["H-M"]
round1 = [
    {"id": "F1", "severity": "material", "status": "resolved",
     "round1": "H-M verdict not reproducible: V6:98 ic Holm p 0.0511 (producer, 10,000 shuffles) vs 0.0448 (round-1 draw), 0.0498 +- 0.0004 (2,000,000 shuffles), 0.0489 (parametric F(7,504)); REPORT stated 'INCONCLUSIVE' as settled and interpreted beyond it ('absent on others', 'neither a general property nor noise').",
     "resolution": "Erratum (round 1, post hoc) in REPORT.md (top section, s1 item 5, s8, s12), report.typ/report.pdf (rebuilt) and AMENDMENTS.md (appended record): the frozen-seed count (5 of 12, inconclusive) is kept and the verdict is stated to sit on the Monte Carlo boundary with 'supported for IC', not robust either way; both phrases withdrawn; the IC model-dependence statement limited to 'significant on 5 or 6 of 12 curves, rho-neg and bsgs on none'.",
     "round2_evidence": f"Independent again: V6:98 ic F = {hm['curves']['V6:98']['ic']['F']:.10g} (equal). This validator's own 10,000-shuffle draw (seed 20261024, own draw order): p = {hm['curves']['V6:98']['ic']['p_perm']:.4f}, Holm {hm['curves']['V6:98']['ic']['p_holm_perm']:.4f}, ic count {hm['summary']['ic']['n_sig_perm_10000']} of 12 -> '{hm['verdict_perm_10000']}'; 1,000,000 shuffles: p = {hm['ic_precise_1e6']['V6:98']['p']:.6f} +- {hm['ic_precise_1e6']['V6:98']['mc_se']:.6f}, rank {hm['ic_precise_1e6']['V6:98']['rank']}, Holm multiplier {hm['ic_precise_1e6']['V6:98']['holm_multiplier']}, Holm {hm['ic_precise_1e6']['V6:98']['p_holm']:.4f} +- {hm['ic_precise_1e6']['V6:98']['p_holm_mc_se']:.4f} -> {hm['ic_precise_1e6_n_sig']} of 12; parametric Holm {hm['curves']['V6:98']['ic']['p_holm_parametric']:.4f} -> {hm['summary']['ic']['n_sig_parametric']} of 12 ('{hm['verdict_parametric']}'). Three independent 10,000-shuffle draws now give 5, 6 and 5; precise evaluations put Holm about one Monte Carlo SE below 0.05. The corrected report statement (boundary, not robust either way) is exactly what the recompute shows. rho-neg and bsgs: 0 of 12 in every evaluation."},
    {"id": "F2", "severity": "minor", "status": "resolved",
     "round1": "s1 item 2 heading 'no method depends on level' although V6 rho-neg L2 is inconclusive; two cells within about 2 MC SE of the TOST margin.",
     "resolution": "Heading now '19 of 20 ratios equivalent, 1 inconclusive, none different'; the old heading is withdrawn; both near-margin cells named in the erratum.",
     "round2_evidence": f"Own bootstrap: V6 rho-neg L2 90% CI upper {rec['level_effect']['V6|rho-neg|2']['ci90'][1]:.4f} > margin 1.0526 (inconclusive); V6 rho-plain L1 90% CI lower {rec['level_effect']['V6|rho-plain|1']['ci90'][0]:.4f} > 0.95 (equivalent): same sides as the producer. V6 counts {rec['level_effect_verdict_counts']['V6']}."},
    {"id": "F3", "severity": "minor", "status": "resolved",
     "round1": "s1 item 4 said H-T is 'not relative to the strongest rho at the source', contradicting s12.",
     "resolution": "s1 item 4 and the s12 H-T bullet now say rho-neg at the source is the strongest rho available on every (j != 0) source curve.",
     "round2_evidence": f"census/V5/vertices.jsonl: j recomputed from (a, b) for all 64 vertices; j = 0 only at vertex {rec['census_facts']['V5']['vertices_j0']}, j = 1728 nowhere; prime field (p = {rec['census_facts']['V5']['p']}), so Aut = {{+-1}} at all 63 sources."},
    {"id": "F4", "severity": "minor", "status": "resolved",
     "round1": "A/A ratio and L2 fractions computed on isogenous families although ANALYSIS_PLAN s8 lists ctl; row sets of 87% / 84% unstated.",
     "resolution": "Row sets stated in s1 item 7, s10 and s12; with-ctl values quoted; 'not admitted' unchanged.",
     "round2_evidence": f"Own recompute: A/A with ctl V5 {rec['wall']['V5|isogenous+ctl']['aa_ratio']:.4f} ({rec['wall']['V5|isogenous+ctl']['n_pairs']} pairs), V6 {rec['wall']['V6|isogenous+ctl']['aa_ratio']:.4f} ({rec['wall']['V6|isogenous+ctl']['n_pairs']} pairs); per-arm L2 fractions rise by 0.4-3.6 points with ctl and stay below 100%; all measured rows of the volcano at L2: V5 {rec['wall']['V5|all_rows_measured_frac_L2']:.4f}, V6 {rec['wall']['V6|all_rows_measured_frac_L2']:.4f}."},
    {"id": "F5", "severity": "minor", "status": "resolved",
     "round1": "AMENDMENTS A2.1 gave the busy-host refusal span as 05:41Z-05:43Z.",
     "resolution": "Appended AMENDMENTS record and REPORT erratum give 05:32:32Z-05:43:01Z and say the refusal message was not preserved; A2.1 text itself left unedited (append-only).",
     "round2_evidence": "runs/progress.jsonl: 9 'failed' lines, V5-Tcr-L5-c19 05:32:32Z ... V5-Tcr-L6-c62 05:43:01Z, all rc_run 2 / session_status 'missing'; no sessions/*.log or runs/*.log contains 'not quiet'; AMENDMENTS.md at 12e5eb3f7d is a byte prefix of the current file."},
    {"id": "F6", "severity": "minor", "status": "resolved",
     "round1": "'No curve is flagged' cannot be otherwise: smallest attainable Holm-adjusted per-curve p about 0.33 with 648 curves.",
     "resolution": "s5 erratum sentence: the per-curve flag cannot fire at this size; the claim rests on the aggregate F tests.",
     "round2_evidence": f"Own recompute: minimum attainable Holm p = 648 / (3 x 648) = 0.333; results.json minimum 0.3306 (vertex 545, z = 5.12). Aggregate F (own): rho-neg {rec['curve_effect_V6']['rho-neg']['F']:.4f} p {rec['curve_effect_V6']['rho-neg']['p_perm']:.3f}, rho-plain {rec['curve_effect_V6']['rho-plain']['F']:.4f} p {rec['curve_effect_V6']['rho-plain']['p_perm']:.3f}, ic {rec['curve_effect_V6']['ic']['F']:.4f} p {rec['curve_effect_V6']['ic']['p_perm']:.3f}."},
    {"id": "F7", "severity": "minor", "status": "resolved",
     "round1": "Constant V5 bsgs / bsgs-neg R(L) listed without comment (family-mix artifact).",
     "resolution": "Explanatory paragraph after the s4 table.",
     "round2_evidence": f"Own recompute on the crater: mean S(N)/S(D) = {rec['F7_family_mix']['bsgs']['mean_S_N_over_D_crater']:.5f} (bsgs), {rec['F7_family_mix']['bsgs-neg']['mean_S_N_over_D_crater']:.5f} (bsgs-neg); (3 + x)/(2(1 + x)) = {rec['F7_family_mix']['bsgs']['formula_(3+x)/(2(1+x))']:.6f} and {rec['F7_family_mix']['bsgs-neg']['formula_(3+x)/(2(1+x))']:.6f}, equal to R(L) at every level to 1e-14."},
    {"id": "N1", "severity": "note", "status": "carried", "round1": "max/min bootstrap CI of eight model means is biased upward; read IC against rho-neg.", "resolution": "Added to s8.", "round2_evidence": "Own recompute reproduces every max/min point value exactly."},
    {"id": "N2", "severity": "note", "status": "carried", "round1": "benchmarks directory name differs from ANALYSIS_PLAN s10.", "resolution": "Recorded in AMENDMENTS erratum.", "round2_evidence": "report.pdf and all 20 figure files in benchmarks/ equal the study copies (sha256)."},
    {"id": "N3", "severity": "note", "status": "carried", "round1": "row-pooled H-T 0.6000 / 0.5831 vs vertex-weighted 0.5946 / 0.5781.", "resolution": "Recorded in AMENDMENTS erratum.", "round2_evidence": f"Own row-pooled T: {rec['H-T']['overall']['0.0']['T_row_pooled']:.4f} (0 GAE), {rec['H-T']['overall']['1.0']['T_row_pooled']:.4f} (1 GAE); verdict unchanged."},
]
notes2 = [
    {"id": "R2-N1", "where": "REPORT.md s8 (sentence before the erratum F1 text)",
     "text": "'the V5 crater's own D/N model and its two ctl re-scalings give IC mean S 59.3, 59.2, 58.1' lists the values in the reverse order of the models: results.json crater_models_V5 has the D/N model at 58.10 and the two ctl re-scalings at 59.26 and 59.18. The sentence's point (no model gap on that curve) holds either way. Cosmetic."},
    {"id": "R2-N2", "where": "REPORT.md s1 item 2 (last sentence) and s8 ('the V6 curve-effect test (section 5) finds no curve differing from its re-scalings')",
     "text": "Both sentences predate the erratum and read slightly wider than the erratum allows: Q1 shows equivalence in 19 of 20 cells (one inconclusive), and the curve statement rests on the aggregate F tests only (erratum F6). The report's own rule ('where the original text below conflicts with this section, this section governs') covers both; no number is affected."},
    {"id": "R2-N3", "where": "REPORT.md s13 (Reproduce) and AMENDMENTS erratum",
     "text": "The erratum was applied to REPORT.md and report.typ by hand and the PDF rebuilt with pandoc/typst; analysis/build_report.py was not changed, so running the s13 reproduce chain would regenerate the pre-erratum text. AMENDMENTS says so; s13 does not. results.json and the figures are unaffected."},
    {"id": "R2-N4", "where": "ecbench record metadata (not the report)",
     "text": "The crater workload's metadata reports automorphisms_available = 2 although p = 1 (mod 3) gives the j = 0 curve an order-6 automorphism group over F_p, which rho-cubic uses (boundaries.automorphisms_used = 6). A metadata quirk of the frozen binary; no number depends on it."},
]
raw = [{"path": f"{REL}/runs/progress.jsonl", "sha256": sha(f"{study}/runs/progress.jsonl")},
       {"path": f"{REL}/runs/transfer_cost.jsonl", "sha256": sha(f"{study}/runs/transfer_cost.jsonl")},
       {"path": f"{REL}/census/V5/vertices.jsonl", "sha256": sha(f"{study}/census/V5/vertices.jsonl")},
       {"path": f"{REL}/census/V6/vertices.jsonl", "sha256": sha(f"{study}/census/V6/vertices.jsonl")},
       {"path": f"{REL}/census/V5/M.json", "sha256": sha(f"{study}/census/V5/M.json")},
       {"path": f"{REL}/census/V6/M.json", "sha256": sha(f"{study}/census/V6/M.json")}]
for s in sorted(prog):
    raw.append({"path": f"{REL}/sessions/{s}.tar.xz", "sha256": prog[s]["sha256_tar_xz"]})
claims = [{"path": f"{REL}/analysis/results.json", "sha256": sha(f"{study}/analysis/results.json")},
          {"path": f"{REL}/REPORT.md", "sha256": sha(f"{study}/REPORT.md")},
          {"path": f"{REL}/report.pdf", "sha256": sha(f"{study}/report.pdf")},
          {"path": f"{REL}/AMENDMENTS.md", "sha256": sha(f"{study}/AMENDMENTS.md")},
          {"path": f"{REL}/ANALYSIS_PLAN.md", "sha256": sha(f"{study}/ANALYSIS_PLAN.md")},
          {"path": f"{REL}/PLAN.md", "sha256": sha(f"{study}/PLAN.md")}]
status_ok = cert_ok and not S["disagreements"] and rpc["n_ok"] == rpc["n_checked"] and order_ok
rho_ref = {
    "kind": "negation",
    "protocol": "PLAN s2/s4 and ANALYSIS_PLAN s9 name rho-neg (rho.negation) as the matched rho, paired with ic on the same (workload, target, round), and rho-cubic (rho.cubic, the order-6 automorphism fold) on the V5 j = 0 crater (census A3 F1). rho-neg is the strongest available on all 648 V6 vertices and the 63 non-crater V5 vertices (j not in {0, 1728}, prime fields); on the V5 crater (p = 1 and r = 1 mod 3) the strongest is rho-cubic, and the report compares IC with it there (89.3x / 98.2x / 141.2x at 0 / 0.1 / 1 GAE). H-T's denominator is rho-neg at j != 0 sources, the strongest there.",
    "unit": "S (ecbench normalised group operations: total_gae / sqrt(r)), the unpriced native operations each arm names (cost.unpriced, from records.jsonl per A2.1) priced at 0 / 0.1 / 1 GAE on both arms",
    "binary_or_process": f"ecbench sha256 {BIN} (runs/progress.jsonl header and every record); every arm, references included, in the same sessions",
    "matches_protocol": True, "strongest_available": True, "same_unit": True, "same_binary_or_process": True,
    "order_holds_across_band": bool(order_ok),
    "note": "gae_band: V6 pooled mean priced S of ic and rho-neg over (ic, rho-neg) pairs of the isogenous families (the Q1 population). gae_band_by_volcano adds V5 levels 1-6 (rho-neg) and V5crater (ic against rho-cubic, D + N pairs; rho-neg shown for reference). Rho is ahead at every price, every level of both volcanoes and on the crater; the ratio is the distance remaining, not a pass/fail.",
    "gae_band": band, "gae_band_by_volcano": byv,
}
vr = {
    "schema": "validation-gate/validator-report/v1",
    "validation_report_id": vid, "round": 2,
    "status": "pass" if status_ok else "fail",
    "independent": True,
    "study": "research/volcano_ecdlp_census_followup_20261023", "pr": "isogeny-labs/crypto#3476",
    "branch": "claude/volcano-census-followup-20261023", "snapshot_commit": snap,
    "status_reason": ("every recomputed headline number agrees within its stated tolerance, every REPORT.md table cell checked equals results.json, the replay sample is verified, and the round-1 findings F1 (material) and F2-F7 are resolved by the round-1 erratum; four non-blocking notes" if status_ok else "see summary.disagreements"),
    "previous_validation": {"validation_id": "VAL-20261106-54796a", "snapshot_commit": "12e5eb3f7de9e764564bd7086e774a052887061a",
                            "validator_commit": "abd653551e", "status": "fail",
                            "reason": "1587 of 1588 recomputed numbers agreed; material F1 (H-M verdict on the Monte Carlo boundary), minor F2-F7, notes N1-N3",
                            "erratum_commit": "fb7f81aac38733d4b72043624d92d38347588c43"},
    "round1_findings_resolution": round1,
    "findings": [],
    "notes": notes2,
    "independence": {
        "validator": "round-2 independent validator; did not write the study, its sessions, its analysis, its report, the erratum or the round-1 validation",
        "producer_code_opened": False,
        "producer_code_not_opened": ["analysis/analyze.py", "analysis/extract_uncharged.py", "analysis/figures.py", "analysis/build_report.py", "analysis/build_pdf.sh"],
        "round1_validator_code_opened": False,
        "round1_validator_code_not_opened": "validation/*.py, validation/*.sh (directory listed only)",
        "producer_derived_not_used": ["runs/slugmap/ (vertex mapping recomputed from j-invariants)", "runs/extract and runs/uncharged used only as comparison targets"],
        "claims_read": ["REPORT.md", "analysis/results.json (by key)", "report.typ (grep for erratum strings only)", "report.pdf (pdftotext, erratum strings)", "benchmarks/20261023-volcano-census-followup/VALIDATOR_REPORT.md (round 1, to check its findings)"],
        "protocol_read": ["docs/VALIDATION_GATE.md", "scripts/validation_gate.py (check functions)", "PLAN.md", "EXECUTION.md", "ANALYSIS_PLAN.md", "AMENDMENTS.md (current and at 2f80fe625f, 141d319090, 12e5eb3f7d)", "DRIVE.md", "benchmarks/20261017-volcano-ecdlp-census/VALIDATOR_REPORT.md (head) and its VALIDATION.json / validator-report.json / replay-certificate.json (structure)", "research/volcano_ecdlp_census_20261017/validation/draw_sample.sh and replay_sample.sh (adapted)"],
        "code": f"validator-written Python and shell under {REL}/validation/round2/: draw_sample.sh, replay_sample.sh (both adapted from the census validation), rederive.py, compare_extracts.py, recompute.py, report_check.py, build_validation.py, write_outputs.py. Nothing imported or copied from analysis/*.py or the round-1 validator's scripts.",
        "seeds_and_draws": "bootstrap seeds 20261022 (s3, s6, s9), 20261023 (s5b, s4), 20261024 (s5c) with this code's own draw order; 1,000,000-shuffle H-M check with SeedSequence([20261106, vertex, volcano]); replay sample nonce = snapshot commit",
    },
    "snapshot_audit": {
        "head_equals_origin": True, "working_tree_clean": True,
        "erratum_commit_changes": ["REPORT.md", "report.typ", "report.pdf", "benchmarks/.../report.pdf", "AMENDMENTS.md (appended)"],
        "results_json_changed_since_round1": False,
        "results_json_inputs_pinned": 521, "results_json_inputs_equal_now": 520,
        "results_json_input_differs": "AMENDMENTS.md: pinned at its 12e5eb3f7d version (A1-A3), a byte prefix of the current file; only the erratum record was appended",
        "frozen_files_unchanged": "PLAN.md, ANALYSIS_PLAN.md (frozen at dbd0f9f274, 2026-10-23T00:42Z, before the progress header 01:09Z), specs/, census/ unchanged; sessions/, runs/progress.jsonl, runs/extract unchanged since the Phase B close df7a0e5f50",
        "amendments_append_only": "AMENDMENTS.md at 2f80fe625f (A1), 141d319090 (A1+A2), 12e5eb3f7d (A1-A3) are byte prefixes of the current file",
    },
    "amendment_audit": "A1 and A2 change no estimand, margin, family, test or threshold (A1.4a's H-T unit S -> priced ops_gae leaves the ratio unchanged: one r per volcano). A3 (post hoc, labelled) fills gaps without changing an estimand, margin, family, test, seed or threshold. The round-1 erratum changes no estimand, margin, family, test, seed, shuffle count, threshold or results.json value; it adds wording, states row sets and quotes the round-1 validator's numbers with attribution.",
    "verdict_vocabulary": "REPORT.md uses the plan's words: PASS (Q2a), equivalent / inconclusive / different / descriptive (s3), SUPPORTED / refuted / inconclusive (H-T), inconclusive / supported for IC (H-M, now stated as on the boundary between the two), not admitted / descriptive (wall), signal, not a speedup claim (s5), distance remaining (s9).",
    "matched_rho_audit": rho_ref["protocol"],
    "redone_sessions_audit": {"script": f"{REL}/validation/round2/orphans_check.py",
                              "result": json.load(open(f"{der}/orphans.json")),
                              "reading": "every record of each restart-orphaned partial session is identical (seq, round, warm-up, arm, workload, target, total_gae, recovered) to the same record of its redone .r2 session: the redo selected nothing"},
    "report_vs_results": {"cells_checked": rpc["n_checked"], "cells_equal": rpc["n_ok"], "mismatches": rpc["mismatches"],
                          "pdf": "report.pdf (study and benchmarks copies identical, sha256) contains the erratum section and corrected sentences (pdftotext)",
                          "figures": "20 figure files in benchmarks/ equal analysis/figures/ (sha256)"},
    "headline_summary": S,
    "headline": H,
    "categorical": CAT,
    "raw_data": raw,
    "claims_checked": claims,
    "recompute_output": f"{REL}/validation/round2/recompute.json",
    "proposed_validation_json_rho_reference": rho_ref,
    "limitations": [
        f"Replay (ecbench verify --replay-all) covers {tot['sessions_replayed']} of {tot['sessions_in_study']} sessions (seeded stratified sample: all 14 volcano x family strata, the 3 V5 crater rho-cubic sessions, 3 Tcr, 2 M, 2 redone .r2); every session also has its committed Phase B audit (1,764 replays identical) and every record of all 147 archives was re-derived by this validator ([k]G = Q, S, A2.1 uncharged counts, extracts row by row).",
        "The replay used the same shared cloud VM type and the same ecbench binary as the sessions: it re-executes operation counts; it is not an independent solver implementation or independent hardware.",
        "Bootstrap CIs and permutation p-values were recomputed with independent draws: they agree within Monte Carlo error, not bit for bit. Categorical outcomes within about 1-2 Monte Carlo SE of a threshold (H-M V6:98; V6 rho-neg L2 and rho-plain L1 TOST) can differ between draws; the report now says so for H-M.",
        "Not re-run: the s5 paired-source and transport-then-solve bootstraps and signal flags, per-vertex H-T CIs, per-family R(L), the s4 per-curve p-values (only the attainable minimum), the slug-cluster CI is compared informally; the s9 CIs were recomputed with a 2,000-replicate vertex-cluster bootstrap and compared only as 'lower end above 1'; the mixed model was re-run for ic only.",
        "Wall time is descriptive in the study; it was checked only as A/A ratios and L2 fractions.",
        "This validation makes the receipts admissible evidence for the benchmarks row; it does not support an ECDLP claim, a speedup, or promotion.",
    ],
}
json.dump(vr, open(os.path.join(ev, "validator-report.json"), "w"), indent=1, default=float)
json.dump({"status_ok": status_ok, "cert_ok": cert_ok, "order_ok": order_ok, "byv": byv}, open(f"{der}/write_status.json", "w"), indent=1, default=float)
print("cert", cert["status"], "report", vr["status"], "order_ok", order_ok, tot)
