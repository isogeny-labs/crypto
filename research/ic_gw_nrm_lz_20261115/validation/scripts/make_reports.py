#!/usr/bin/env python3
"""Writes replay-certificate.json and validator-report.json (research validation dir) and the benchmarks copy of
the report, from the validator's own outputs (recompute.json, replay-compare.json, plan2-compare.json) and the
producer's claims (analysis.json, REPORT.typ/README numbers).
usage: make_reports.py WORK_DIR VDIR BENCH_OUT_DIR REPO"""
import hashlib, json, os, sys
W, V, BOUT, REPO = sys.argv[1:5]
def sha(p):
    return hashlib.sha256(open(p, "rb").read()).hexdigest()
def J(p):
    return json.load(open(p))
STUDY = "research/ic_gw_nrm_lz_20261115"
rec = J(os.path.join(V, "recompute.json"))
rc = J(os.path.join(V, "replay-compare.json"))
p2 = J(os.path.join(V, "plan2-compare.json"))
sample = J(os.path.join(W, "sample.json"))
nonce = J(os.path.join(W, "nonce.json"))
an = J(os.path.join(REPO, STUDY, "analysis.json"))
vj = J(os.path.join(REPO, "research/ic_gw_bl_reuse_20261115/validation/VALIDATION.json"))
stress = open(os.path.join(W, "stress/stress200k.out")).read().strip()
stress20 = open(os.path.join(W, "stress/stress20k.out")).read().strip()
bins = {l.split()[1].rsplit("/", 1)[1]: l.split()[0] for l in open(os.path.join(W, "replay/binary.sha256"))}
muts = {l.split()[1].rsplit("/", 1)[1]: l.split()[0] for l in open(os.path.join(W, "replay2/mutants.sha256"))}
Q = [262111, 524287, 1048507, 2097091, 4194271]
K = {262111: 18, 524287: 19, 1048507: 20, 2097091: 21, 4194271: 22}
tar = os.path.join(V, "replay/replay-runs-nrmlz-20261115.tar.gz")
ot = rc["oracle_totals"]; st = p2["small_q"]["totals"]
ratios = rec["ratios"]

snapshot = {
    "repository": "/home/user/crypto (branch claude/f4-f5-algorithm-speed-bvp7qd)",
    "data_commit": "be6570bbcb46b646f3ea8a9c362ccb607581ada1 (2026-11-15T05:57:03Z; results, report, benchrec record)",
    "protocol_freeze_commit": "851b8da629837ffa6e27fdb318fddcd1cadb4f9d (2026-11-15T05:39:39Z: PROTOCOL.md, analyze.py, make-units.sh, run-local.sh, units.tsv, src/; producer run.log start 05:39:53Z)",
    "read_from": "git archive be6570bbcb (the working tree was briefly on another branch during this validation; nothing was read from it)",
    "freeze_integrity": "PROTOCOL.md, analyze.py, make-units.sh, run-local.sh, units.tsv and src/*.rs byte-identical between 851b8da629 and be6570bbcb; REF source research/ic_gw_bl_reuse_20261115/amendment-1/src unchanged since its freeze 98a4fc6639; shared includes research/quartic_n4_20261019/{common,ghs,ic,solver}.rs and src/cryptanalysis/plane_quartic_pic.rs unchanged since 9a63c9161b",
    "head_at_write": "e87d1ee9f7 (branch head when the reports were written; git diff be6570bbcb..HEAD is empty for research/ic_gw_nrm_lz_20261115, benchmarks/20261115-ic-gw-nrm-lz, the REF source, the shared includes, summary.json and the BLR1 VALIDATION.json)",
    "dirty_tree": "git status clean for every path read",
}
binaries = {
    "REF": {"sha256": bins["ref"], "recorded": "e236855d7fef58d1c6618b25c399e462c7099bcb1f634047a7258e5f5d074035",
            "matches_recorded": bins["ref"] == "e236855d7fef58d1c6618b25c399e462c7099bcb1f634047a7258e5f5d074035",
            "build": "cd research/ic_gw_bl_reuse_20261115/amendment-1/src && rustc --edition 2021 -O v3.rs --cfg combo (rustc 1.97.0, from git archive be6570bbcb)"},
    "CAND": {"sha256": bins["cand"], "recorded": "a7e4ad7a41e5a8a61466791af069af75914c2ed883c61b0d0f18d6ef67d23577",
             "matches_recorded": bins["cand"] == "a7e4ad7a41e5a8a61466791af069af75914c2ed883c61b0d0f18d6ef67d23577",
             "build": "cd research/ic_gw_nrm_lz_20261115/src && rustc --edition 2021 -O v3.rs --cfg combo"},
    "ORACLE": {"sha256": bins["oracle"], "build": "CAND src edited by scripts/make_oracle.py (uncounted u128 checks only), same rustc flags"},
    "MUT_LZWY": {"sha256": muts["mut_lzwy"], "build": "scripts/make_mutants.py: w_{i+1}^T y recurrence without the b1 term"},
    "MUT_LZVV": {"sha256": muts["mut_lzvv"], "build": "scripts/make_mutants.py: v^T v_prev taken as v^T v"},
    "MUT_NRM": {"sha256": muts["mut_nrm"], "build": "scripts/make_mutants.py: g3 also skips the product by b[1] (operand not 1)"},
    "producer_binary_sha256_file": "binary.sha256 inside runs-nrmlz-20261115.tar.gz lists the same two hashes",
}
identity = {
    "NRM": {
        "arithmetic": "Fp::mul(a, b) = a*b mod q, Fp::add reduces; every operand at the skipped sites is already reduced (< q): ga, gb from Fp::add, ha[0] and mb[0] from Fp::mul, u from Fp::mul. So mul(x, 1) = x and add(0, x) = x exactly.",
        "sites": {
            "g3": "skips mul(ga[0], b[0]) iff b[0] == 1 (operand tested directly)",
            "g1": "skips mul(gb[0], a[0]) iff a[0] == 1",
            "g2": "replaces add(0, mul(ha[0], mb[0])) by ha[0] iff b[0] == 1; mb[0] = local_data mo[0] = mul(b0, b0) = 1 then; also drops the add of the initial 0 (exact)",
            "disc": "4*g1 = (g1+g1)+(g1+g1) mod q: one mul -> two adds, always when NRM=1",
            "lin": "skips mul(u, a[0]) iff a[0] == 1",
            "normaliser": "pv = 1/(first nonzero coordinate); when p[0] != 0 that coordinate is p[0], so p[0]*pv = 1; p[0] == 0 takes the full three-multiplication path (counted, including 0*pv)",
        },
        "first_coordinate_zero": "points with first coordinate 0 never take a skip: a[0]/b[0] == 0 fails the == 1 test, p[0] == 0 fails the != 0 test. Exercised: 56 residual points with p[0] = 0 in the production-size oracle runs, 134 factor-base pairs with b[0] = 0 and 158 such residual points in the small-q sweep, all exact.",
        "moved_work": "none: the only new code are comparisons on values already computed, uncounted exactly as REF's own g3 == 0, disc == 0 and pv[k] != 1 tests; local_data, batch_inv, the fb normalisation (pnorm) and the kind-1 point formula are unchanged (fb.* identical in every unit; kind-1 still multiplies by a[0]).",
        "count_identity": "REF - CAND relations.mul = number of skips (6 sites); REF - CAND relations.add = #g2 skips - 2 #disc. Holds exactly in every oracle run.",
    },
    "LZ": {
        "setup": "A = R^T D R (matvec_b0 and matvec_pm1 compute the same field values: acc_coef replaces a*v by v or -v only when a = 1 or ell - 1), symmetric. w_0 = y = A b; at iteration i with s_i = w_i^T A w_i != 0: v_i = A w_i, a_i = v_i^T v_i / s_i, b_i = v_i^T v_{i-1} / s_{i-1} (i >= 1), w_{i+1} = v_i - a_i w_i - b_i w_{i-1} (no b term at i = 0). Arithmetic exact in F_ell.",
        "P": "P(i): w_i^T A w_{i-1} = 0. P(1) = v_0^T v_0 - a_0 s_0 = 0; P(i+1) = v_i^T v_i - a_i s_i - b_i w_{i-1}^T A w_i = 0 by P(i) and symmetry. a_i is always a direct dot, so P holds in both modes.",
        "identity_1": "v_i^T v_{i-1} = w_i^T A v_{i-1} = w_i^T A (w_i + a_{i-1} w_{i-1} + b_{i-1} w_{i-2}) = s_i + a_{i-1} P(i) + b_{i-1} Q(i), Q(i) = w_i^T A w_{i-2} = v_{i-1}^T v_{i-2} - a_{i-1} P(i-1) - b_{i-1} s_{i-2} = 0 whenever b_{i-1} was exact; at i = 1 there is no w_{-1} term. By induction from i = 1 identity 1 is exact at every iteration, including when it is itself used (LZ) to form every b.",
        "identity_2": "w_{i+1}^T y = (A w_i)^T w_0 - a_i w_i^T y - b_i w_{i-1}^T y. (A w_0)^T w_0 = s_0 gives the first-iteration form s0 - a_0 wy_0 (wy_0 = y^T y is a direct dot). For i >= 1, w_i^T A w_0 = 0 by full A-orthogonality: for k <= i-2, w_{i+1}^T A w_k = w_i^T A (w_{k+1} + a_k w_k + b_k w_{k-1}) - 0 - 0 = 0 (standard Lanczos induction, exact over a field; scalar Lanczos never narrows, so unlike the block case no column is ever dropped).",
        "breakdown": "s == 0 returns None before wy or vvp of that iteration is used, in both modes; s_prev is a previously checked nonzero s; the w == 0 break and the n + 5 iteration cap see identical iterates. So results, None cases and iteration counts are identical; the no-env path computes both dots exactly as REF (diff and A/A).",
        "count_identity": "per call with I iterations passing the s check, REF - LZ = (I - 1)(2n - 2) - 1 lmul and the same ladd (two n-term dots removed per iteration after the first; 2 lmul + 2 ladd added there and 1 + 1 at the first). Holds exactly for every call in every oracle run and in the stress harness.",
    },
}
cert_runs = []
for r in rc["runs"]:
    cert_runs.append({k: r[k] for k in ("unit", "idx", "role", "arm", "q", "seed", "c_D", "command", "env", "exit_code", "start_utc",
                                        "loadavg_before", "ic_records", "ic_arms_attempts", "k_found_equals_k_and_ok", "checks", "verified")
                      if k in r} | ({"reconciliation": r["reconciliation"], "oracle_clean": r["oracle_clean"]} if "oracle" in r else {})
                     | {"log_equal_ref_mod_wall": r.get("log_equal_ref_mod_wall"), "log_equal_committed_mod_wall": r.get("log_equal_committed_mod_wall")})
for c in p2["small_q"]["cells"]:
    cert_runs.append({"unit": f"small-q q={c['q']} seed={c['seed']} (REF, BOTH, ORACLE)", "role": "small", "q": c["q"], "seed": c["seed"], "c_D": 1.7,
                      "checks": [{"against": k, "fields_compared": c[k]["fields"], "n_mismatches": len(c[k]["mismatches"]), "allowed_fields_changed": c[k]["changed"]}
                                 for k in ("BOTH vs REF", "ORACLE vs BOTH")],
                      "oracle_attempts": c["oracle_attempts"], "verified": all(len(c[k]["mismatches"]) == 0 for k in ("BOTH vs REF", "ORACLE vs BOTH"))})
small_ok = st["mismatches"] == 0 and st["nrm_bad"] == 0 and st["full_value_bad"] == 0 and st["lz_bad"] == 0 and not st["reconcile_fail"] and st["rc_nonzero"] == 0 and not st["final_not_ok"] and not st["log_diff"]
neg_ok = all(n["detected"] for n in p2["negative_controls"])
stress_ok = "bad=0" in stress and "bad=0" in stress20
all_ok = rc["all_verified"] and small_ok and neg_ok and stress_ok and binaries["REF"]["matches_recorded"] and binaries["CAND"]["matches_recorded"]
cert = {
    "schema": "replay-certificate/v1",
    "study": STUDY,
    "claim_under_test": "CAND = BLR1 (research/ic_gw_bl_reuse_20261115/amendment-1/src, sha256 e236855d7fef...) plus NRM=1 (ic2.rs find_relations_2lp: skip the products by a first coordinate equal to 1 and by b0^2, 4*g1 as two additions) and LZ_REUSE=1 (la2.rs lanczos: v^T v_prev = s, w_i^T y by the three-term recurrence). With both: counted output equal to REF except relations.mul/add (all arms) and D, DL1 linear_algebra.lmul/ladd; every BOTH/REF ratio < 1 at all 30 (q, seed) cells and w_add 0, 0.5, 1 for DL (geomean 0.957-0.961 / 0.979 / 0.987), DL1 (0.835-0.850 / 0.915-0.921 / 0.939-0.944) and D (0.971-0.978); relations, graph, matrix and logs unchanged; k = 22 DL IC/rho 0.00213 / 0.00199 / 0.00168 at w = 0 / 0.1 / 1. [pcc:unverified], L0 local.",
    "status": "verified" if all_ok else "failed",
    "independent": True,
    "independence_note": "Validator subagent with its own context, nonce, sample, runner, comparators, oracle build, mutants, stress harness and recompute script; the producer's analyze.py, plot.py and gen-report.py were not read or imported (only hashed). Same Claude Code session family and container as the producer: independence of reasoning and code, not of host or organization; no keyring signature.",
    "scalars_verified": all_ok,
    "validated_snapshot": snapshot,
    "binaries": binaries,
    "nonce": {"value": nonce["value"], "derivation": f"sha256 of the UTF-8 string '{nonce['remote_main_head']} {nonce['utc_time']}' = (origin main from `git ls-remote origin refs/heads/main`) + ' ' + (`date -u +%Y-%m-%dT%H:%M:%S.%NZ`), drawn after the data commit be6570bbcb and after scripts/select_units.py was written, before any unit was selected or run",
              "remote_main_head": nonce["remote_main_head"], "utc_time": nonce["utc_time"]},
    "selection_rule": "scripts/select_units.py: universe q in {262111, 524287, 1048507, 2097091, 4194271} x seed 1..12, ranked by sha256('<nonce>:<q>:<seed>'); first 6, extended until >= 2 seeds in 7..12, >= 2 units with q >= 2097091 and >= 2 seeds in 1..6 (no extension needed). A/A: the first two sampled units with seed in 1..6. Plus: one unit at c_D = 1.55 (q 524287 seed 7), the oracle build on all 27 producer cells not sampled (all 30 audited), a small-q sweep (q 223..32719, seeds 1-5) and 6 negative-control mutant runs.",
    "sample": sample["sample"], "aa_units": sample["aa"],
    "commands": {"unit": "env -u NRM -u LZ_REUSE [NRM=1 LZ_REUSE=1] <BIN> ic <q> <seed> D 3.0 <c_D> (c_D 1.7; 1.55 for the extra unit)",
                 "runner": "scripts/replay.sh (xargs -P 3; counted work only), plans from scripts/make_plan.py and make_plan2.py"},
    "comparison_rule": "counted fields = every key of every ic record except those ending in wall or cpu, load1 and s.controls*; key lists equal in order; record (arm, attempt) lists equal. BOTH vs REF: only relations.mul/add (D, DL1, DL) and linear_algebra.lmul/ladd (D, DL1) may differ, and all of them must. A0 vs REF and ORACLE vs BOTH: every counted field equal. Replayed vs committed (seeds 1-6): every counted field equal. stderr equal after masking wall=…s and removing ORACLE lines.",
    "fields_compared": rc["fields_compared"] + st["fields"],
    "coverage": {"sampled_REF_BOTH_pairs": len(sample["sample"]), "pairs_with_seed_outside_1_6": sum(1 for u in sample["sample"] if u["seed"] > 6),
                 "pairs_with_q_ge_2097091": sum(1 for u in sample["sample"] if u["q"] >= 2097091), "aa_units": len(sample["aa"]),
                 "extra_c_D_1_55_units": 3, "producer_cells_audited_by_oracle": 30, "oracle_runs_production_sizes": ot["reconciled_units"],
                 "small_q_units": st["units"], "negative_control_runs": len(p2["negative_controls"]), "units_total": rc["n_runs"] + st["units"] + len(p2["negative_controls"])},
    "runs": cert_runs,
    "total_mismatches": rc["total_mismatches"] + st["mismatches"],
    "mismatches": [],
    "all_exit_codes_zero": rc["all_exit_codes_zero"] and st["rc_nonzero"] == 0,
    "all_ok_and_k_found_equals_k": rc["all_ok_and_k_found_equals_k"] and not st["final_not_ok"],
    "oracle_audit": {"production_sizes": ot, "small_q": {k: st[k] for k in ("attempts", "multi_attempt_units", "failed_attempt_records", "b0zero_pairs", "p0zero_resid", "nrm_bad", "full_value_bad", "lz_calls", "vvp_used", "wy_used", "lz_bad", "reconciled_records", "reconcile_fail")},
                     "nrm_site_order": ["g3", "g1", "g2", "disc", "lin", "normaliser"],
                     "meaning": "Every skipped multiplication had an operand equal to 1 (or was the exact 4*g1 = 2(g1+g1) rewrite) and every full value (g3, g1, g2, disc, residual points, normalised points) equals REF's; every reused v^T v_prev and w^T y equals the direct dot and every w_i^T A w_0 (i >= 1) is 0; the counted differences REF - CAND equal the oracle's skip/reuse counts exactly in every record (34 production-size units incl. all 30 producer cells, 207 small-q records incl. failed attempts)."},
    "identity_proofs": identity,
    "stress_test": {"what": "scripts/lzstress.rs drives la2::lanczos directly (CAND la2.rs without and with LZ_REUSE, REF la2.rs, oracle la2.rs) on random sparse matrices, ell in {2, 3, 5, 7, 11, 13, 101, 65537, 4501899029752451}, n 2..41, rows 1..2n+4, coefficients 1 / ell-1 / random, pm1 random, seed 20261115",
                    "summary_200k": stress, "summary_20k": stress20,
                    "oracle_200k": "1437027 reused v^T v_prev and 1437027 reused w^T y, 0 differing from the direct dot, 0 nonzero w_i^T A w_0; results: 49319 some, 44967 zero vector, 105714 None (45073 of them after >= 3 iterations, i.e. breakdown after the identities were in use)",
                    "meaning": "the no-env path equals REF in result and counts on every trial; LZ gives the identical result (incl. every None) on every trial and its count saving follows (I-1)(2n-2)-1 exactly, also at ell = 2, 3, 5 where breakdown is frequent"},
    "negative_controls": p2["negative_controls"],
    "negative_controls_note": "Each mutant makes one reuse/skip inexact: MUT_LZWY and MUT_LZVV make D and DL1 linear algebra fail (6 attempts instead of 1) and are flagged by the comparator; MUT_NRM produces residual points off the curve and panics in plane_quartic_pic.rs (exit 101, flagged by the exit-code and record checks). So the G1-style comparison has power against an inexact skip or reuse.",
    "ratios_record": rc["replayed_ratios"],
    "raw_replay_files": {"tarball": "replay/replay-runs-nrmlz-20261115.tar.gz", "sha256": sha(tar), "inner_sums": "replay/SHA256SUMS.inner",
                         "contents": "replay/ (plan 1: 50 units), replay2/ (plan 2: 120 small-q units + 6 mutant runs, mutants.sha256), stress/*.out, plan.tsv, plan2.tsv, sample.json, nonce.json"},
    "notes": ["Units ran up to 3 at a time on the 4-vCPU session container (L0); only counted fields are compared, wall/CPU are not.",
              "The extra c_D = 1.55 unit (q 524287 seed 7) has heavier relation rows; BOTH/REF there is DL 0.9649 / 0.9851 / 0.9912, DL1 0.8765 / 0.9468 / 0.9639, D 0.9846 / 0.9863 / 0.9874 at w_add 0 / 0.5 / 1 (outside the protocol's grid; descriptive)."],
}
open(os.path.join(V, "replay-certificate.json"), "w").write(json.dumps(cert, indent=1) + "\n")
cert_sha = sha(os.path.join(V, "replay-certificate.json"))

# ------------------------------------------------------------------ validator report
H = []
def h(name, claimed, recomputed, tol=1e-9, **kw):
    H.append({"name": name, "claimed": claimed, "recomputed": recomputed, "rel_tol": tol, **kw,
              "agrees": abs(claimed - recomputed) <= tol * max(abs(claimed), abs(recomputed), 1e-300)})
for arm in ("DL", "DL1", "D"):
    for q in Q:
        for wa in ("0.0", "0.5", "1.0"):
            key = f"BOTH/{arm}/{q}/{wa}/0.0"
            for f in ("geomean", "max"):
                h(f"{arm} cold M-eq BOTH/REF {f} over seeds 1-6, k={K[q]} (q={q}), w_add={wa}, w_h=0", an["ratios"][key][f], ratios[key][f])
for x in ("NRM", "LZ", "BOTH"):
    for arm in ("DL", "DL1", "D"):
        h(f"decision max ratio {x}/{arm} over 30 cells x w_add (w_h=0)", an["decision"][x][arm]["max_ratio"], rec["decision"][x][arm]["max_ratio"])
g = rec["gap_k22"]
for x in ("REF", "BOTH"):
    for arm in ("DL", "DL1"):
        for w in ("0", "0.1", "1"):
            h(f"k=22 {x} {arm} mean cold M-eq seeds 1-3, w_add=0, hash+cmp at w={w} GAE", an["gap_k22"]["ic_mean_meq"][f"{x}/{arm}/{w}"], g["ic_mean_meq"][f"{x}/{arm}/{w}"])
for arm in ("DL", "DL1"):
    for w in ("0", "0.1", "1"):
        h(f"k=22 rho/IC BOTH {arm}, w={w}", an["gap_k22"]["rho_over_ic"][f"BOTH/{arm}/{w}"], g["rho_over_ic"][f"BOTH/{arm}/{w}"])
rounded = {("DL", "0"): 0.00213, ("DL", "0.1"): 0.00199, ("DL", "1"): 0.00168, ("DL1", "0"): 0.00116, ("DL1", "0.1"): 0.00125, ("DL1", "1"): 0.00143}
for (arm, w), v in rounded.items():
    h(f"k=22 IC/rho BOTH {arm}, w={w} (as printed in README/REPORT/tables, 3 significant digits)", v, g["ic_over_rho"][f"BOTH/{arm}/{w}"], 5e-3)
for w in ("0", "0.1", "1"):
    h(f"REF DL k=22 mean reproduces the frozen band IC value (BLR1 VALIDATION.json gae_band[{w}].ic)", vj["rho_reference"]["gae_band"][w]["ic"], g["ic_mean_meq"][f"REF/DL/{w}"])
for q in Q:
    h(f"relations.mul ratio NRM/REF (DL record) geomean, k={K[q]}", an["relations_mul_ratio"]["NRM"][str(q)], rec["relations_mul_ratio_DL"]["NRM"][str(q)])
    h(f"linear_algebra.lmul ratio BOTH/REF DL1 geomean, k={K[q]}", an["lmul_ratio"]["BOTH/DL1"][str(q)], rec["la_lmul_ratio"]["BOTH/DL1"][str(q)])
assert all(x["agrees"] for x in H), [x for x in H if not x["agrees"]]

def raw_items(prefix):
    items = [
        ("../runs-nrmlz-20261115.tar.gz", f"{STUDY}/runs-nrmlz-20261115.tar.gz", "producer raw JSONL of the 150 units + binary.sha256, host.txt, run.log, SHA256SUMS (303/303 inner OK)"),
        ("../units.tsv", f"{STUDY}/units.tsv", "frozen unit list (851b8da629)"),
        ("../PROTOCOL.md", f"{STUDY}/PROTOCOL.md", "frozen protocol (851b8da629)"),
        ("../analysis.json", f"{STUDY}/analysis.json", "producer's claimed numbers (compared, 405 cells)"),
        ("../src/ic2.rs", f"{STUDY}/src/ic2.rs", "CAND source (NRM)"),
        ("../src/la2.rs", f"{STUDY}/src/la2.rs", "CAND source (LZ_REUSE)"),
        ("../../ic_gw_bl_reuse_20261115/amendment-1/src/ic2.rs", "research/ic_gw_bl_reuse_20261115/amendment-1/src/ic2.rs", "REF (BLR1) source"),
        ("../../ic_gw_bl_reuse_20261115/amendment-1/src/la2.rs", "research/ic_gw_bl_reuse_20261115/amendment-1/src/la2.rs", "REF (BLR1) source"),
        ("../../quartic_n4v2_20261020/results/summary.json", "research/quartic_n4v2_20261020/results/summary.json", "frozen r_inv, r_lmul, gae (kind ic_size, arm DL)"),
        ("../../ic_gw_bl_reuse_20261115/validation/VALIDATION.json", "research/ic_gw_bl_reuse_20261115/validation/VALIDATION.json", "frozen matched negation-map rho band (gae_band)"),
        ("replay/replay-runs-nrmlz-20261115.tar.gz", f"{STUDY}/validation/replay/replay-runs-nrmlz-20261115.tar.gz", "validator replay raw JSONL/stderr (176 units), plans, nonce, stress summaries"),
        ("replay/SHA256SUMS.inner", f"{STUDY}/validation/replay/SHA256SUMS.inner", "inner checksums of the replay tarball (365 files)"),
        ("recompute.json", f"{STUDY}/validation/recompute.json", "validator recompute from the producer tarball (gates, 405 cells, decision, gap)"),
        ("replay-compare.json", f"{STUDY}/validation/replay-compare.json", "validator replay comparison (50 units)"),
        ("plan2-compare.json", f"{STUDY}/validation/plan2-compare.json", "small-q sweep and negative controls"),
    ]
    out = []
    for p, rp, role in items:
        out.append({"path": prefix + p if prefix else p, "repo_path": rp, "sha256": sha(os.path.join(REPO, rp)) if os.path.exists(os.path.join(REPO, rp)) else sha(os.path.join(V, p)), "role": role})
    return out

worst = {arm: max(ratios[f"BOTH/{arm}/{q}/{wa}/0.0"]["max"] for q in Q for wa in ("0.0", "0.5", "1.0")) for arm in ("DL", "DL1", "D")}
report = {
    "schema": "validator-report/v1",
    "study": STUDY,
    "status": "pass" if all_ok else "fail",
    "independent": True,
    "independence_note": cert["independence_note"],
    "validated_snapshot": snapshot,
    "replay_certificate": {"path": "replay-certificate.json", "sha256": cert_sha},
    "raw_data_path_base": f"paths in raw_data[].path are relative to {STUDY}/validation/ (the directory of this report); repo_path is repository-relative",
    "raw_data": None,
    "headline": H,
    "headline_note": "claimed = producer's analysis.json (and the 3-digit IC/rho printed in README/REPORT.typ/tables.md, rel_tol 5e-3); recomputed = scripts/recompute.py from runs-nrmlz-20261115.tar.gz with the frozen constants. All 405 ratio cells (3 variants x 3 arms x 5 sizes x 3 w_add x 3 w_h, geomean/max/min/per-seed) agree with analysis.json with max relative difference 0.0.",
    "decisions": {
        "BOTH_improvement_recomputed": {a: rec["decision"]["BOTH"][a]["improvement"] for a in ("DL", "DL1", "D")},
        "BOTH_improvement_claimed": {a: an["decision"]["BOTH"][a]["improvement"] for a in ("DL", "DL1", "D")},
        "BOTH_band_below_1_all_w_h": {a: rec["decision"]["BOTH"][a]["band_below_1"] for a in ("DL", "DL1", "D")},
        "NRM_LZ_descriptive_recomputed_equal_claimed": all(rec["decision"][x][a][f] == an["decision"][x][a][f] for x in ("NRM", "LZ", "BOTH") for a in ("DL", "DL1", "D") for f in ("improvement", "all_cells_below_1", "band_below_1")),
        "LZ_DL_exactly_1": rec["decision"]["LZ"]["DL"]["max_ratio"] == 1.0 and min(ratios[f"LZ/DL/{q}/{wa}/{wh}"]["min"] for q in Q for wa in ("0.0", "0.5", "1.0") for wh in ("0.0", "0.1", "1.0")) == 1.0,
        "gates": {k: rec["gates"][k]["pass"] for k in ("G0", "G1", "G2", "G3")},
        "units_with_allowed_changes": rec["gates"]["units_with_allowed_changes"],
        "ref_reproduces_frozen_band": {w: v["rel_err"] for w, v in g["ref_reproduces_band"].items()},
        "worst_BOTH_ratio_w_h_0": worst,
        "ranges_BOTH_geomean": {arm: {wa: [min(ratios[f"BOTH/{arm}/{q}/{wa}/0.0"]["geomean"] for q in Q), max(ratios[f"BOTH/{arm}/{q}/{wa}/0.0"]["geomean"] for q in Q)] for wa in ("0.0", "0.5", "1.0")} for arm in ("DL", "DL1", "D")},
        "k22_gap": {"IC_over_rho_BOTH_DL": [g["ic_over_rho"][f"BOTH/DL/{w}"] for w in ("0", "0.1", "1")],
                    "IC_over_rho_BOTH_DL1": [g["ic_over_rho"][f"BOTH/DL1/{w}"] for w in ("0", "0.1", "1")],
                    "IC_over_rho_REF_DL": [g["ic_over_rho"][f"REF/DL/{w}"] for w in ("0", "0.1", "1")],
                    "IC_over_rho_REF_DL1": [g["ic_over_rho"][f"REF/DL1/{w}"] for w in ("0", "0.1", "1")],
                    "order_IC_below_rho_holds_across_band": g["order_holds_across_band_BOTH_DL_vs_rho"] and all(g["ic_over_rho"][f"BOTH/DL1/{w}"] < 1 for w in ("0", "0.1", "1"))},
    },
    "checks": {
        "artifacts": "runs tarball sha256 b59649528e51... (= benchrec raw record); 303/303 inner SHA256SUMS OK; 150 units (5 arms x 30 cells), 150 .jsonl + 150 .err + binary.sha256, host.txt, run.log; binary.sha256 inside equals REF e236855d7fef... and CAND a7e4ad7a41e5...",
        "build": "REF and CAND rebuilt from git archive be6570bbcb with rustc 1.97.0 to the recorded sha256 (bit-identical)",
        "freeze": snapshot["freeze_integrity"],
        "diff_review": "CAND vs REF: rho2.rs, solver2.rs, summ2.rs identical; v3.rs differs only in five #[path] prefixes; ic2.rs differs only in the NRM-gated lines of find_relations_2lp; la2.rs only in the LZ_REUSE-gated lines of lanczos. Env read once per call (std::env::var(...).is_ok()); the runner unsets both before setting.",
        "no_env_equals_ref": "by reading (identical op sequence: g2 = add(0, mul(ha0, mb0)) then t = 1..5 equals REF's t = 0..5 loop from 0; disc, lin, normaliser and both dots unchanged) and by measurement: G0 on all 30 producer cells, 2 replayed A/A units, 200,000 stress trials (result and counts equal to REF la2.rs)",
        "gates_recomputed": f"G0-G3 pass on the producer tarball ({rec['gates']['fields_compared']} counted fields); stderr equal after masking wall in all 150 units",
        "replay": f"{rc['n_runs']} units of plan 1, all verified, 0 mismatches, {rc['fields_compared']} fields; plan 2: {st['units']} small-q units, 0 mismatches, 6/6 mutants detected",
        "oracle": cert["oracle_audit"]["meaning"],
        "stress": cert["stress_test"]["meaning"],
        "metric_model": "recompute.py implements PROTOCOL.md's model from its text (not from analyze.py): per unit and arm, sum over that arm's ic records and the seven phases of mul + r_inv inv + r_lmul lmul + w_add (add + ladd) + w_h gae (hash + cmp), frozen per-q constants (kind ic_size, arm DL)",
        "benchrec": "benchmarks/records/20261115T0539Z-ic-gw-nrm-lz.json metrics 95.69 / 98.71 / 83.82 percent equal the recomputed k = 22 geomeans (DL w_add 0, 1; DL1 w_add 0); its raw entry has uploaded: false",
    },
    "discrepancies": [
        {"id": "N1", "severity": "none (disclosure)", "what": "NRM changes relations.add as well as relations.mul: the g2 skip also drops add(0, x), and 4*g1 becomes two additions, so CAND does #pairs-with-disc - #g2-skips more additions (net +1 per pair with g3 != 0). PROTOCOL H_nrm allows relations.add to change; the count identity holds exactly."},
        {"id": "N2", "severity": "none (scope note)", "what": "H_nrm says points are normalised with first coordinate 1; points with first coordinate 0 exist (56 residual points at production sizes; 134 fb pairs and 158 residual points in the small-q sweep). The code tests the operand at run time, so they take the counted path; verified exact."},
        {"id": "N3", "severity": "for the Coordinator (success criterion)", "what": "The NRM saving is the removal of products whose operand is the constant 1 (five sites) and of the product by the constant 4 (one site, replaced by two additions). These are real operations of the REF code path that CAND no longer executes, not a re-pricing; but they are baseline waste rather than a new algorithmic idea. At w_add = 1 the 4*g1 rewrite costs one M-eq more than it saves; the ratio stays < 1 because of the other five sites."},
        {"id": "N4", "severity": "for the Coordinator (headline)", "what": "LZ_REUSE does not touch DL (block Lanczos): LZ/REF for DL is exactly 1 at every cell, and the DL headline (BOTH) equals NRM alone. DL1 is cheaper than DL in this model at k = 22 already in REF (5.264e8 vs 8.440e8 M-eq at w = 0; 2.226e9 vs 2.544e9 at w = 1); the frozen band anchors only REF DL, so DL1's IC/rho (0.00116 / 0.00125 / 0.00143) uses the same rho but has no previously validated REF anchor; its records were replayed here (all sampled units) like DL's."},
        {"id": "N5", "severity": "none (disclosure)", "what": "PROTOCOL.md reports a main-session pilot (q 262111 and 1048507, seed 1) before the freeze, which informed the predictions; the decision rule does not use the predictions."},
    ],
    "null_object_control": "The claim is a deterministic counted-work ratio, not a statistical signal; there is no parameter whose increase should destroy it. Nulls of the same shape: (a) A0 (CAND, no env) = REF in every counted field on all 30 producer cells and 2 replayed units, ratio exactly 1; (b) LZ on the untouched DL arm gives exactly 1.000 at every cell, w_add and w_h (predicted negative case); (c) the oracle shows every skipped operand equals 1 and every reused dot equals the direct dot; (d) three mutants that make one skip or reuse inexact are all detected (6/6 runs). Mechanism check in place of a decay parameter: the LZ saving per call is (I-1)(2n-2)-1 lmul against matvec work growing with nnz, so its share should be larger after SGE (DL1: lmul ratio 0.784 -> 0.762 from k = 18 to 22) than on the full matrix (D: 0.981 -> 0.983) - observed.",
    "validation_ladder_checks": {
        "step1_assumptions_isolated": "NRM rests on operand == 1 at each skip; LZ on local and global A-orthogonality. Each measured separately per skip and per iteration by the oracle on all 30 producer cells, 6 sampled units, the c_D = 1.55 unit and 40 small-q cells, and stressed on 200,000 random matrices at ell down to 2 (failure mode hunted: an inexact reuse after a breakdown or a skip at a coordinate that is not 1). None found.",
        "step2_measured_ratio_on_full_pipeline": "the ratio is measured on the full pipeline against REF at every size claimed (k = 18..22, 30 cells), re-measured on new seeds (10, 11) and at c_D = 1.55; predicted negative cases (A0 = REF, LZ on DL = 1) checked. No extrapolation is claimed.",
        "step3_cheats": "none: no partial run; L0 host and 3-way concurrency affect only wall/CPU, which are not claimed.",
        "step4_reproducibility": "REF and CAND rebuilt from the committed sources to the recorded hashes; units re-run from the committed command line; validator artifacts listed under raw_data with sha256.",
    },
    "heuristic_validation_checks": "not applicable: no heuristic distribution is validated (deterministic counted operations).",
    "cost_model_checks": {
        "unit": "cold F_q-multiplication equivalents (M-eq): mul + r_inv inv + r_lmul lmul + w_add (add + ladd) + w_h gae (hash + cmp), frozen per-size r_inv, r_lmul, gae from research/quartic_n4v2_20261020/results/summary.json (kind ic_size, arm DL)",
        "optimistic_assumptions": "r_inv and r_lmul are frozen time ratios from an earlier grid, not re-measured; additions free in the primary cell and in the rho band (band at w_add = 0); the 4*g1 rewrite is a saving only when additions are cheaper than half a multiplication; hash/cmp priced at 0, 0.1, 1 GAE (section 5a)",
        "memory": "not reported by the study; NRM adds no storage; LZ adds three scalars (wy_cur, wy_prev, s0) per Lanczos call",
        "total_cost_bookkeeping": "per-unit totals include every record of the arm (all attempts; every producer unit succeeded on attempt 1), so no success-probability correction is needed; the k = 22 band uses the mean over seeds 1-3 as frozen",
        "rho_side": "frozen negation-map band of research/ic_gw_bl_reuse_20261115/validation/VALIDATION.json (not re-measured here); REF DL reproduces its IC values with relative error 0",
    },
    "limitations": [
        "L0: session container (4 vCPU), up to 3 units concurrent; only counted fields are claimed and compared. No EKS route, no native L2/replay/A/A admission; no wall-clock claim.",
        "Scope: GHS-weak family E0/F_{q^3}, driver ic <q> <seed> D 3.0 1.7, k = 18..22, seeds 1-6 (producer), seeds 10 and 11, one c_D = 1.55 unit and q 223..32719 (validator). Transfer beyond these instances rests on the identity proofs, not on measurement.",
        "The rho side is the frozen band; no rho was re-measured. The band prices additions at 0; with additions priced the IC side rises (w_add = 1 values are in tables.md) while the rho side was not re-priced.",
        "Counted-cost improvement of 1.3-4.3% (DL) and 5.6-16.5% (DL1) depending on w_add; NRM is a removal of trivial products (N3).",
        "Independence is of reasoning and code within the same session family and host; no keyring signature.",
    ],
    "method": "scripts/: select_units.py (nonce sample, written before the nonce), make_plan.py, make_plan2.py, replay.sh, make_oracle.py (oracle build), make_mutants.py (negative controls), lzstress.rs (stress harness), compare_replay.py, compare_plan2.py, recompute.py (headline from the producer tarball), make_reports.py (this file and the certificate).",
    "validation_report": {
        "id": "VAL-20261115-5c7af8",
        "task_id": "none named by the task card (main-session validation-gate request)",
        "run_ids": ["research/ic_gw_nrm_lz_20261115 runs-nrmlz-20261115 (150 units, data commit be6570bbcb)"],
        "artifact_checks": ["tarball sha256 + 303/303 inner checksums", "150/150 units rc 0", "binaries rebuilt bit-identically", "freeze unchanged since 851b8da629", "replay tarball 365/365 inner checksums"],
        "metric_recomputations": ["405 ratio cells = analysis.json (rel diff 0)", "decision (9 variant/arm pairs) equal", "k = 22 means and IC/rho equal", "REF reproduces the frozen band (rel err 0)"],
        "control_checks": ["A0 = REF (30 + 2)", "LZ on DL = 1 exactly", "oracle: 0 bad skips / reuses", "6/6 mutants detected", "200k stress trials exact"],
        "heuristic_validation_checks": [],
        "cost_model_checks": ["unit declared (cold M-eq)", "optimistic assumptions listed", "memory not reported by the study (no change by construction)"],
        "proof_architecture_checks": [],
        "verdict": "passed" if all_ok else "failed",
        "limitations": ["L0 counted only", "rho band frozen, not re-measured", "N3, N4 for the Coordinator"],
        "artifact_paths": [f"{STUDY}/validation/replay-certificate.json", f"{STUDY}/validation/validator-report.json", f"{STUDY}/validation/replay/replay-runs-nrmlz-20261115.tar.gz", "benchmarks/20261115-ic-gw-nrm-lz/replay-certificate.json", "benchmarks/20261115-ic-gw-nrm-lz/validator-report.json"],
    },
    "review_attestation": {
        "review_plan": "none attached to the task card; no joints assigned, so this report verdicts receipt admissibility",
        "joints_owned": [],
        "sources_read": ["crypto-autoresearcher/docs/agent-runtime-core.md", "crypto-autoresearcher/agents/validator.md", "crypto-autoresearcher/docs/inventor-protocol.md sections 3 and 6",
                         "crypto/docs/VALIDATION_GATE.md", "crypto/scripts/validation_gate.py (check functions)",
                         f"{STUDY}/PROTOCOL.md", f"{STUDY}/README.md", f"{STUDY}/tables.md", f"{STUDY}/REPORT.typ", f"{STUDY}/analysis.json", f"{STUDY}/run-local.sh", f"{STUDY}/make-units.sh", f"{STUDY}/units.tsv",
                         f"{STUDY}/src (diff vs REF; ic2.rs find_relations_2lp, local_data, pack2; la2.rs lanczos, matvec_pm1, dot; solver2.rs relation and linear-algebra dispatch; v3.rs arm map)",
                         "research/quartic_n4_20261019/common.rs (Fp, counters, Rng), research/quartic_n4_20261019/ic.rs (pnorm, line_points)",
                         f"{STUDY}/runs-nrmlz-20261115.tar.gz", "research/quartic_n4v2_20261020/results/summary.json (ic_size DL rows)",
                         "research/ic_gw_bl_reuse_20261115/validation/VALIDATION.json, replay-certificate.json and validator-report.json (format only)",
                         "benchmarks/records/20261115T0539Z-ic-gw-nrm-lz.json (raw, metrics)"],
        "not_read": [f"{STUDY}/analyze.py (hashed only)", f"{STUDY}/plot.py", f"{STUDY}/gen-report.py", "research/ic_gw_bl_reuse_20261115/analyze.py"],
        "read_sibling_reports": False,
        "verdict": "passed" if all_ok else "failed",
    },
}
report["raw_data"] = raw_items("")
open(os.path.join(V, "validator-report.json"), "w").write(json.dumps(report, indent=1) + "\n")
rep_sha = sha(os.path.join(V, "validator-report.json"))
# benchmarks copy
b = json.loads(json.dumps(report))
b["raw_data_path_base"] = "paths in raw_data[].path are relative to benchmarks/20261115-ic-gw-nrm-lz/ (the directory of this copy); repo_path is repository-relative"
b["raw_data"] = [dict(x, path=os.path.relpath(os.path.join(REPO, x["repo_path"]), os.path.join(REPO, "benchmarks/20261115-ic-gw-nrm-lz"))) for x in report["raw_data"]]
b["copy_of"] = {"path": f"{STUDY}/validation/validator-report.json", "sha256": rep_sha,
                "note": "same content; raw_data[].path rebased to benchmarks/20261115-ic-gw-nrm-lz/, replay_certificate points to the byte-identical copy in this directory"}
open(os.path.join(BOUT, "validator-report.json"), "w").write(json.dumps(b, indent=1) + "\n")
print("status", report["status"], "cert", cert_sha, "report", rep_sha, "headline items", len(H))
