"""Assemble validator-report.json and replay-certificate.json from the
validator's own outputs in code/out/ (TASK-20261029-654f73).

Claimed values are transcribed from cards/VERDICT-P32.md (sha256 415a89d7...9ea3)
and the handoff's claim_under_review, with the precision they are printed at;
each headline's rel_tol is half a unit in the last printed digit divided by
the claimed value (1e-9 where the claim is printed to 15 digits or exact).

raw_data paths are relative to a depth-2 evidence directory (benchmarks/<dir>/
or research/ic_packed_filter_20261029/), where VALIDATION.json is expected;
repo_path gives the repository-root path of each.

usage: build_report.py VALIDATION_DIR
"""
import json
import math
import os
import sys

import vcommon as v

OUT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "out")


def J(name):
    with open(os.path.join(OUT, name)) as f:
        return json.load(f)


def tol(printed):
    """rel_tol for a value printed as the string `printed`."""
    s = printed.lstrip("-")
    d = len(s.split(".")[1]) if "." in s else 0
    val = abs(float(printed))
    if d >= 12:
        return 1e-9
    return (0.5 * 10 ** (-d)) / val if val else 1e-9


heads = []


def H(name, printed, recomputed, source, exact=False):
    claimed = float(printed)
    exact = exact or printed in ("1.0", "0.0")  # an exact tie or identity is compared exactly
    t = 1e-12 if exact else tol(printed)
    heads.append({"name": name, "claimed": claimed, "claimed_as_printed": printed,
                  "recomputed": recomputed, "rel_tol": t, "source": source,
                  "reproduced": abs(claimed - recomputed) <= t * max(abs(claimed), abs(recomputed), 1e-300)})


def main():
    vdir = sys.argv[1]
    j1 = {b: J(f"j1-P{b}.json") for b in (32, 24)}
    j2, j3, j4 = J("j2.json"), J("j3.json"), J("j4.json")
    sc, xc, st = J("scalars.json"), J("j1-inpod-crosscheck.json"), J("selftest.json")
    s3, s3r, c2 = J("s3check.json"), J("s3requests.json"), J("c2_identity.json")
    V = "cards/VERDICT-P32.md"
    d = j1[32]["decision"]; r, a, c = d["dgb->dgbf"], d["aa dgb->dgb-aa"], d["corrected"]
    H("P32 ns_per_gae dgb", "84.5526", r["ns_per_gae_a"], V + " Facts used, decision block")
    H("P32 ns_per_gae dgbf", "81.0446", r["ns_per_gae_b"], V + " Facts used, decision block")
    H("P32 ratio_b_over_a dgbf/dgb", "0.958512", r["ratio"], V + " Facts used")
    H("P32 ratio ci95 lower", "0.956084", r["ci95_log"][0], V + " Facts used")
    H("P32 ratio ci95 upper", "0.960945", r["ci95_log"][1], V + " Facts used")
    H("P32 aa_ratio dgb-aa/dgb", "0.998337", a["ratio"], V + " Facts used")
    H("P32 aa ci95 lower", "0.996146", a["ci95_log"][0], V + " Facts used")
    H("P32 aa ci95 upper", "1.000534", a["ci95_log"][1], V + " Facts used")
    H("P32 corrected_ratio dgbf/dgb", "0.960107806", c["ratio"], V + " (A); claim_under_review")
    H("P32 corrected ci95 lower", "0.956898", c["ci95_log"][0], V + " (A); claim_under_review")
    H("P32 corrected ci95 upper", "0.963328", c["ci95_log"][1], V + " (A); claim_under_review")
    H("P32 corrected_dof", "562.6", c["dof"], V + " Facts used")
    H("P32 condition 9: ratio dgbf/dgb-aa (w=0 file)", "0.960107806240821", j1[32]["cond9"]["dgb-aa->dgbf"]["ratio"], V + " Facts used, condition 9")
    H("P32 condition 9: relative difference to corrected_ratio", "0.0", j1[32]["cond9"]["rel_diff_vs_corrected"], V + " Facts used, condition 9", exact=True)
    pc = j1[32]["dgb->dgbf-edge"]
    H("P32 position control dgbf-edge/dgb", "0.959282", pc["ratio"], V + " (A) position control")
    H("P32 position control ci95 lower", "0.956531", pc["ci95_log"][0], V + " (A)")
    H("P32 position control ci95 upper", "0.962040", pc["ci95_log"][1], V + " (A)")
    fe = j1[32]["dgbf-edge->dgbf"]
    H("P32 dgbf/dgbf-edge", "0.999197", fe["ratio"], V + " Facts used, A/A controls")
    H("P32 dgbf/dgbf-edge ci95 lower", "0.997648", fe["ci95_log"][0], V + " Facts used")
    H("P32 dgbf/dgbf-edge ci95 upper", "1.000749", fe["ci95_log"][1], V + " Facts used")
    H("P32 wall per charged GAE reduction, percent", "3.99", 100 * (1 - c["ratio"]), V + " (A)")
    H("P32 reduction interval low, percent", "3.67", 100 * (1 - c["ci95_log"][1]), V + " (A)")
    H("P32 reduction interval high, percent", "4.31", 100 * (1 - c["ci95_log"][0]), V + " (A)")
    b8 = j3["P32-wall-cond8"]["bitops"]
    H("P32 bitop/v1 dgbf/dgb (measured records)", "1.0030127", b8["measured_ratio_dgbf_over_dgb"], V + " Facts used")
    H("P32 bitop/v1 rise, percent", "0.301", 100 * (b8["measured_ratio_dgbf_over_dgb"] - 1), V + " (A)")
    H("P32 bitop difference dgbf - dgb (measured)", "115931990016", b8["measured_difference"], V + " Facts used", exact=True)
    tv = j3["P32-wall-cond8"]["tfbs"]["values_per_curve"]
    H("P32 table_filter_bit_sets icv1-fp32-t89141-95879fe3", "1048276", tv["icv1-fp32-t89141-95879fe3"][0], V + " Facts used", exact=True)
    H("P32 table_filter_bit_sets icv1-fp32-tm68303-f902d2f0", "1048295", tv["icv1-fp32-tm68303-f902d2f0"][0], V + " Facts used", exact=True)
    ph = j1[32]["phase_sums_ns"]
    ms = lambda arm, k: ph[arm][k] / 1e6
    H("P32 phase sum solve_wall dgb-aa, ms", "48192.2", ms("dgb-aa", "solve_wall_ns"), V + " Facts used, phase sums")
    H("P32 phase sum solve_wall dgbf, ms", "46269.7", ms("dgbf", "solve_wall_ns"), V + " Facts used")
    H("P32 phase sum target_PDP dgb-aa, ms", "14047.8", ms("dgb-aa", "online.target_PDP"), V + " Facts used")
    H("P32 phase sum target_PDP dgbf, ms", "10413.1", ms("dgbf", "online.target_PDP"), V + " Facts used")
    H("P32 phase sum oracle_setup dgb-aa, ms", "6396.3", ms("dgb-aa", "oracle_setup"), V + " Facts used")
    H("P32 phase sum oracle_setup dgbf, ms", "8167.2", ms("dgbf", "oracle_setup"), V + " Facts used")
    H("P32 phase sum target_query dgb-aa, ms", "26776.7", ms("dgb-aa", "online.target_query"), V + " Facts used")
    H("P32 phase sum target_query dgbf, ms", "26765.8", ms("dgbf", "online.target_query"), V + " Facts used")
    H("P32 solve_wall change dgbf vs dgb-aa, percent", "-3.99", 100 * (ms("dgbf", "solve_wall_ns") / ms("dgb-aa", "solve_wall_ns") - 1), V + " Facts used")
    H("P32 target_PDP change, percent", "-25.9", 100 * (ms("dgbf", "online.target_PDP") / ms("dgb-aa", "online.target_PDP") - 1), V + " (A), (D) 5")
    H("P32 oracle_setup change, percent", "27.7", 100 * (ms("dgbf", "oracle_setup") / ms("dgb-aa", "oracle_setup") - 1), V + " (D) 5")
    H("P32 oracle_setup rise, ms", "1770.9", ms("dgbf", "oracle_setup") - ms("dgb-aa", "oracle_setup"), V + " (A)")
    H("P32 oracle_setup rise as share of dgb-aa solve wall, percent", "3.67", 100 * (ms("dgbf", "oracle_setup") - ms("dgb-aa", "oracle_setup")) / ms("dgb-aa", "solve_wall_ns"), V + " (A)")
    H("P32 target_PDP share of solve wall, percent", "29", 100 * ms("dgb-aa", "online.target_PDP") / ms("dgb-aa", "solve_wall_ns"), V + " (A) prediction miss")
    H("P32 target_query share of solve wall, percent", "56", 100 * ms("dgb-aa", "online.target_query") / ms("dgb-aa", "solve_wall_ns"), V + " (A) prediction miss")
    sw, sc_ = j4["P32-wall"]["dgbf/dgb"], j4["P32-counted"]
    H("P32 S' dgbf/dgb w=0 (wall)", "1.0", sw["0.0"]["ratio_b_over_a"], V + " Facts used; claim_under_review")
    H("P32 S' dgbf/dgb w=0.1 (wall)", "1.050488", sw["0.1"]["ratio_b_over_a"], V + " Facts used; claim_under_review")
    H("P32 S' dgbf/dgb w=1 (wall)", "1.358633", sw["1.0"]["ratio_b_over_a"], V + " Facts used; claim_under_review")
    H("P32 S' dgbf/dgb w=1 per curve t89141 (wall)", "1.3547", sw["1.0"]["per_curve"]["icv1-fp32-t89141-95879fe3"], V + " Facts used")
    H("P32 S' dgbf/dgb w=1 per curve tm68303 (wall)", "1.3626", sw["1.0"]["per_curve"]["icv1-fp32-tm68303-f902d2f0"], V + " Facts used")
    H("P32 S' dgbf/dgb w=0.1 per curve tm68303 (wall)", "1.0509", sw["0.1"]["per_curve"]["icv1-fp32-tm68303-f902d2f0"], V + " Facts used")
    H("P32 S' without bit-set term dgbf/dgb w=1", "1.0", sw["1.0"]["ratio_without_bitset_term"], V + " Facts used")
    H("P32 S' filter cost at w=0.1, percent", "5.05", 100 * (sw["0.1"]["ratio_b_over_a"] - 1), V + " (A) band")
    H("P32 S' filter cost at w=1, percent", "35.86", 100 * (sw["1.0"]["ratio_b_over_a"] - 1), V + " (A) band")
    for w, p1, p2, p3 in (("0.0", "31.4719", "31.4719", "1.0"), ("0.1", "29.9858", "31.4997", "1.0505"), ("1.0", "23.2775", "31.6255", "1.3586")):
        H(f"P32 counted S' dgb/rho-neg w={w}", p1, sc_["dgb/rho-neg"][w]["ratio_b_over_a"], V + " Facts used table; claim_under_review")
        H(f"P32 counted S' dgbf/rho-neg w={w}", p2, sc_["dgbf/rho-neg"][w]["ratio_b_over_a"], V + " Facts used table; claim_under_review")
        H(f"P32 counted S' dgbf/dgb w={w}", p3, sc_["dgbf/dgb"][w]["ratio_b_over_a"], V + " Facts used table")
    for pair, c1, c2 in (("dgb/rho-neg", "21.81", "24.97"), ("dgbf/rho-neg", "29.55", "34.02"), ("dgbf/dgb", "1.3547", "1.3626")):
        H(f"P32 counted S' {pair} w=1 per curve t89141", c1, sc_[pair]["1.0"]["per_curve"]["icv1-fp32-t89141-95879fe3"], V + " Facts used table")
        H(f"P32 counted S' {pair} w=1 per curve tm68303", c2, sc_[pair]["1.0"]["per_curve"]["icv1-fp32-tm68303-f902d2f0"], V + " Facts used table")
    allpc = [sc_[p][w]["per_curve"][cv] for p in ("dgb/rho-neg", "dgbf/rho-neg") for w in ("0.0", "0.1", "1.0") for cv in sc_[p][w]["per_curve"]]
    H("P32 smallest per-curve IC/rho-neg ratio in the band", "21.81", min(allpc), V + " Facts used")
    agg = [sc_[p][w]["ratio_b_over_a"] for p in ("dgb/rho-neg", "dgbf/rho-neg") for w in ("0.0", "0.1", "1.0")]
    H("P32 rho-neg cheaper by factor, low", "23.3", min(agg), V + " (A) against matched rho")
    H("P32 rho-neg cheaper by factor, high", "31.6", max(agg), V + " (A) against matched rho")
    for w, p1, p2 in (("0.0", "31.47", "31.47"), ("0.1", "29.99", "31.50"), ("1.0", "23.28", "31.63")):
        H(f"P32 (D)4 dgb/rho-neg w={w}", p1, sc_["dgb/rho-neg"][w]["ratio_b_over_a"], V + " (D) 4")
        H(f"P32 (D)4 dgbf/rho-neg w={w}", p2, sc_["dgbf/rho-neg"][w]["ratio_b_over_a"], V + " (D) 4")
    g32 = j2["P32-counted"]["gate"]["below_l2_from_gate"]
    H("P32 counted below_l2 entries", "14", g32["count"], V + " Facts used", exact=True)
    H("P32 counted below_l2 on rho-neg", "7", g32["per_arm"]["rho-neg"], V + " Facts used", exact=True)
    H("P32 counted below_l2 on rho-neg-aa", "7", g32["per_arm"]["rho-neg-aa"], V + " Facts used", exact=True)
    for k in ("P32-wall", "P32-counted"):
        H(f"{k} in-pod audit replays identical", "1152", j2[k]["audit"]["replays_identical"], V + " Facts used", exact=True)
    c10 = j3["P32-wall-cond10"]
    H("P32 condition 10 p max", "3965125361", c10["p_max"], V + " Facts used", exact=True)
    H("P32 condition 10 signed points max", "2048", c10["signed_points_max"], V + " Facts used", exact=True)
    # P24
    d = j1[24]["decision"]; r, a, c = d["dgb->dgbf"], d["aa dgb->dgb-aa"], d["corrected"]
    V24 = V + " Facts used, P24"
    H("P24 corrected_ratio dgbf/dgb", "1.003200", c["ratio"], V24)
    H("P24 corrected ci95 lower", "1.000564", c["ci95_log"][0], V24)
    H("P24 corrected ci95 upper", "1.005842", c["ci95_log"][1], V24)
    H("P24 aa_ratio", "1.000506", a["ratio"], V24)
    H("P24 aa ci95 lower", "0.999012", a["ci95_log"][0], V24)
    H("P24 aa ci95 upper", "1.002003", a["ci95_log"][1], V24)
    pc = j1[24]["dgb->dgbf-edge"]
    H("P24 position control dgbf-edge/dgb", "1.005304", pc["ratio"], V24)
    H("P24 position control ci95 lower", "1.002677", pc["ci95_log"][0], V24)
    H("P24 position control ci95 upper", "1.007938", pc["ci95_log"][1], V24)
    H("P24 (D)2 corrected", "1.0032", c["ratio"], V + " (D) 2")
    H("P24 (D)2 corrected ci95 lower", "1.0006", c["ci95_log"][0], V + " (D) 2")
    H("P24 (D)2 corrected ci95 upper", "1.0058", c["ci95_log"][1], V + " (D) 2")
    s24 = j4["P24-counted"]
    for w, p1, p2, p3 in (("0.0", "12.1159", "12.1159", "1.0"), ("0.1", "11.6498", "12.2388", "1.0506"), ("1.0", "9.4528", "12.8184", "1.3560")):
        H(f"P24 counted S' dgb/rho-neg w={w}", p1, s24["dgb/rho-neg"][w]["ratio_b_over_a"], V24)
        H(f"P24 counted S' dgbf/rho-neg w={w}", p2, s24["dgbf/rho-neg"][w]["ratio_b_over_a"], V24)
        H(f"P24 counted S' dgbf/dgb w={w}", p3, s24["dgbf/dgb"][w]["ratio_b_over_a"], V24)
    for w, p1, p2 in (("0.0", "12.12", "12.12"), ("0.1", "11.65", "12.24"), ("1.0", "9.45", "12.82")):
        H(f"P24 (D)4 dgb/rho-neg w={w}", p1, s24["dgb/rho-neg"][w]["ratio_b_over_a"], V + " (D) 4")
        H(f"P24 (D)4 dgbf/rho-neg w={w}", p2, s24["dgbf/rho-neg"][w]["ratio_b_over_a"], V + " (D) 4")
    # in-pod instrument cross-check (full precision; the native A/A admission file)
    for b in (32, 24):
        x = xc[f"P{b}"]
        for k in ("ratio_b_over_a", "aa_ratio", "corrected_ratio", "corrected_ci95_lo", "corrected_ci95_hi", "corrected_dof"):
            heads.append({"name": f"P{b} control-comparison.json wall.{k} (in-pod instrument output)", "claimed": x["file"][k],
                          "claimed_as_printed": repr(x["file"][k]), "recomputed": x["mine"][k], "rel_tol": 1e-9,
                          "source": f"P{b}/wall/eks-results/control-comparison.json",
                          "reproduced": x["rel_diff"][k] <= 1e-9})

    descriptive_discrepancies = [{
        "name": "P32 S' dgbf/dgb w=0.1 per curve icv1-fp32-t89141-95879fe3",
        "claimed_as_printed": "1.0500", "recomputed": sw["0.1"]["per_curve"]["icv1-fp32-t89141-95879fe3"],
        "source": V + " Facts used, S' for dgb->dgbf (line 61); also the counted session gives the same value",
        "reproduced": False,
        "reading": "1.0500605 rounds to 1.0501 at the printed precision; 1.0500 is a truncation. Descriptive per-curve figure; not in (A), (D) or the claim under review; the aggregate 1.050488 and every claim-under-review number reproduce. Not counted as a defect of the evidence; the Coordinator may rule otherwise.",
    }]

    def cond(name, ok, value):
        return {"condition": name, "reproduced": bool(ok), "recomputed": value}

    six1 = {}
    for b in (32, 24):
        w2, w1 = j2[f"P{b}-wall"], j1[b]
        w3, w4 = j3[f"P{b}-wall-cond8"], j4[f"P{b}-wall"]
        cc = j2[f"P{b}-counted"]["gate"]
        dd = w1["decision"]
        six1[f"P{b}"] = {"wall": [
            cond("1 exit 0 from a single pod", w2["cond1_exit"], {"exit_code": w2["job_exit"]["exit_code"], "pod_prefixes": s3["jobs"][f"P{b}-wall"]["pod_prefixes"]}),
            cond("2 audit ok, complete, 0 problems, every measured replay identical", w2["cond2_audit"], w2["audit"]),
            cond("3 counted A/A ops exactly 1 at w=0,0.1,1 (dgb/dgb-aa, dgbf-edge/dgbf)", w2["cond3"], w2["cond3_counted_aa"]),
            cond("4 input-spec JSON-equal frozen spec; build-info revision and ecbench sha256", w2["cond4"], w2["spec_build"]),
            cond("5 l2-gate accepted, every execution L2/L3, invalid_outcomes empty", w2["cond5"], w2["l2_gate"]),
            cond("6 native A/A admission (control-comparison.json)", w2["cond6"], w2["cond6_control_comparison"]),
            cond("7 decision block facts (recomputed from records; the analysis file itself was not read)",
                 dd["dgb->dgbf"]["n"] == 288 and dd["dgb->dgbf"]["pairs_below_level"] == 0 and dd["aa dgb->dgb-aa"]["n"] == 288,
                 {"pairs": dd["dgb->dgbf"]["n"], "pairs_below_level": dd["dgb->dgbf"]["pairs_below_level"], "aa_pairs": dd["aa dgb->dgb-aa"]["n"],
                  "aa_arms": ["dgb", "dgb-aa"], "corrected_ci95": dd["corrected"]["ci95_log"], "weight_dependence": "none: the wall block reads charged total_gae only"}),
            cond("8 identical charged work; table_filter_bit_sets; bitop identity; ops dgbf/dgb exactly 1",
                 not w3["mismatches"] and w3["tfbs"]["absent_on_dgb_and_dgb_aa"] and w3["tfbs"]["present_once_in_oracle_setup_on_filtered"]
                 and w3["tfbs"]["one_value_per_curve"] and not w3["bitops"]["identity_failures"] and not w3["bitops"]["reprice_mismatches"]
                 and all(w4["dgbf/dgb"][w]["ratio_without_bitset_term"] == 1.0 for w in ("0.0", "0.1", "1.0")),
                 {"matched": w3["matched"], "mismatches": len(w3["mismatches"]), "tfbs": w3["tfbs"],
                  "bitop_identity_checked": w3["bitops"]["identity_checked"], "bitop_identity_failures": len(w3["bitops"]["identity_failures"]),
                  "records_repriced": w3["bitops"]["records_repriced"], "reprice_mismatches": len(w3["bitops"]["reprice_mismatches"]),
                  "ops_dgbf_over_dgb": [w4["dgbf/dgb"][w]["ratio_without_bitset_term"] for w in ("0.0", "0.1", "1.0")]}),
            cond("9 identity: dgbf/dgb-aa ratio equals corrected_ratio within 1e-9", w1["cond9"]["rel_diff_vs_corrected"] <= 1e-9 and w1["cond9"]["dgb-aa->dgbf"]["n"] == 288,
                 {"ratio": w1["cond9"]["dgb-aa->dgbf"]["ratio"], "corrected": dd["corrected"]["ratio"], "rel_diff": w1["cond9"]["rel_diff_vs_corrected"],
                  "sum_wall_ratio": w1["cond9"]["sum_wall_ratio_dgbf_over_dgbaa"], "gae_sums_equal": w1["cond9"]["gae_sums_equal"]}),
            cond("10 packed layout: p <= 2^32-1 and signed points <= 65536 on every IC record", not j3[f"P{b}-wall-cond10"]["failures"], j3[f"P{b}-wall-cond10"]),
        ], "counted": [
            cond("1 audit", cc["cond1_audit"], j2[f"P{b}-counted"]["audit"]),
            cond("2 rho-neg-aa/rho-neg exactly 1 at w=0,0.1,1", cc["cond2"], cc["cond2_rho_aa"]),
            cond("3 spec and build", cc["cond3_spec_build"], j2[f"P{b}-counted"]["spec_build"]),
            cond("4 every ic.pipeline execution L2/L3", cc["cond4_ic_all_L2_L3"], cc["records_levels"]),
            cond("5 rho executions L2/L3, or L1 with only too-short L2 blockers", cc["cond5_rho"], cc["below_l2_from_gate"]),
            cond("6 invalid_outcomes empty", cc["cond6_invalid_outcomes_empty"], []),
            cond("exit-42 clause (from l2-gate.json and from records.jsonl)", cc["EXIT42_CLAUSE"],
                 {"exit_code": j2[f"P{b}-counted"]["job_exit"]["exit_code"], "below_l2_from_records_equal_gate": cc["below_l2_from_records_equal_gate"]}),
            cond("counted dgb and dgbf records equal wall records (condition 8 fields, table_filter_bit_sets included)", j3[f"P{b}-counted-equals-wall"]["equal"],
                 {a_: j3[f"P{b}-counted-equals-wall"][a_]["matched"] for a_ in ("dgb", "dgbf")}),
            cond("condition 10 on counted IC records", not j3[f"P{b}-counted-cond10"]["failures"], j3[f"P{b}-counted-cond10"]),
        ]}
    all_six1 = all(x["reproduced"] for s in six1.values() for part in s.values() for x in part)
    p28 = j2["P28-counted"]["gate"]
    p28_ok = (p28["EXIT42_CLAUSE"] and j3["P28-counted-equals-wall"]["equal"] and not j3["P28-counted-cond10"]["failures"])
    proves_too_much = {
        "P32 A/A dgb-aa/dgb interval contains 1": j1[32]["decision"]["aa dgb->dgb-aa"]["ci95_log"][0] < 1 < j1[32]["decision"]["aa dgb->dgb-aa"]["ci95_log"][1],
        "P32 A/A dgbf/dgbf-edge interval contains 1": j1[32]["dgbf-edge->dgbf"]["ci95_log"][0] < 1 < j1[32]["dgbf-edge->dgbf"]["ci95_log"][1],
        "P24 corrected lower bound > 1 (regression reproduced, no improvement found)": j1[24]["decision"]["corrected"]["ci95_log"][0] > 1,
        "counted A/A exactly 1 (dgb/dgb-aa, dgbf-edge/dgbf, rho-neg-aa/rho-neg) at every weight": all(
            [j2["P32-wall"]["cond3"], j2["P24-wall"]["cond3"], j2["P32-counted"]["gate"]["cond2"], j2["P24-counted"]["gate"]["cond2"]]),
        "label-swap null (dgb-aa, dgbf) centred on 1, observed outside it": all(
            abs(x["null_median"] - 1) < 1e-3 and x["null_fraction_le_observed"] == 0.0 for x in j1[32]["label_swap_null_dgbaa_dgbf"].values()),
    }
    j1_ok = all(h["reproduced"] for h in heads if any(h["name"].startswith(p) for p in ("P32 ns_per", "P32 ratio", "P32 aa", "P32 corrected", "P32 condition 9", "P24 corrected", "P24 aa")))
    j1_ok &= j1[32]["decision"]["corrected"]["ci95_log"][1] < 1
    j2_ok = all(j2[f"P{b}-wall"][k] for b in (32, 24) for k in ("cond1_exit", "cond2_audit", "cond4", "cond5", "cond6")) \
        and all(not j2[f"P{b}-wall"]["raw_regrade"]["disagreements"] for b in (32, 24)) \
        and all(j2[f"P{b}-wall"]["positions"]["positions"][a_] == {"1": 144, "2": 144} for b in (32, 24) for a_ in ("dgbf", "dgb-aa"))
    j3_ok = all(x["reproduced"] for b in (32, 24) for x in six1[f"P{b}"]["wall"] if x["condition"].startswith(("8", "10"))) \
        and all(j3[f"P{b}-counted-equals-wall"]["equal"] for b in (32, 24, 28))
    j4_ok = all(h["reproduced"] for h in heads if "S'" in h["name"] or "rho-neg" in h["name"]) \
        and all(j4[f"P{b}-counted"][p]["strict_same_order_across_band_every_curve"] for b in (32, 24, 28) for p in ("dgb/rho-neg", "dgbf/rho-neg")) \
        and all(j2[f"P{b}-counted"]["gate"]["EXIT42_CLAUSE"] for b in (32, 24, 28))
    reps = {t: J(f"replay/{t}.replay-receipt.json") for t in ("P32-wall", "P32-counted", "P24-wall", "P24-counted", "P28-counted", "P28-wall")}
    j5_ok = all(reps[t]["ok"] and len(reps[t]["problems"]) == 0 and sum(1 for x in reps[t]["replays"] if x["reproduced"] and x["detail"] == "identical") == 1152
                for t in ("P32-wall", "P32-counted", "P24-wall", "P24-counted")) \
        and all(sc[k]["all_ok"] for k in ("P32-wall", "P32-counted", "P24-wall", "P24-counted"))
    joints = {"J1 decision statistic": "holds" if j1_ok else "breaks",
              "J2 instrument": "holds" if j2_ok else "breaks",
              "J3 charged-work identity": "holds" if j3_ok else "breaks",
              "J4 counted route and band": "holds" if j4_ok else "breaks",
              "J5 independent replay": "holds" if j5_ok else "breaks",
              "proves_too_much": "holds" if all(proves_too_much.values()) else "breaks"}
    defects = [h for h in heads if not h["reproduced"]]
    status = "pass" if (not defects and all(v_ == "holds" for v_ in joints.values()) and all_six1 and st["all_pass"]) else "fail"

    rel_from_evidence = "../../research/ic_packed_filter_20261029/"
    raw_files = []
    for b, kind in ((32, "wall"), (32, "counted"), (24, "wall"), (24, "counted"), (28, "counted")):
        for name in ("session/records.jsonl", "job-exit.json", "audit.json", "l2-gate.json", "input-spec.json", "build-info.json", "SHA256SUMS") + (("control-comparison.json",) if kind == "wall" else ()):
            repo_rel = f"research/ic_packed_filter_20261029/P{b}/{kind}/eks-results/{name}"
            p = os.path.join(v.REPO, repo_rel)
            raw_files.append({"path": rel_from_evidence + f"P{b}/{kind}/eks-results/{name}", "repo_path": repo_rel, "sha256": v.sha256_file(p)})
    for spec in ("spec-P32.json", "spec-P32-rho.json", "spec-P24.json", "spec-P24-rho.json", "spec-P28-rho.json"):
        repo_rel = f"backlog/XP-20261029-packed-filter/{spec}"
        raw_files.append({"path": "../../" + repo_rel, "repo_path": repo_rel, "sha256": v.sha256_file(os.path.join(v.REPO, repo_rel))})

    report = {
        "schema": "validator-report/v1 (docs/VALIDATION_GATE.md)",
        "id": None,
        "id_provenance": "not allocated by the validator: tools/allocate_id.py has no validation-report type and the validator mints no identifier; the archiving session names this report by path and sha256, or allocates an id",
        "task_id": "TASK-20261029-654f73",
        "decision_ref": "DEC-20261029-5eb8e6",
        "study": "XP-20261029-packed-filter",
        "status": status,
        "independent": True,
        "independence": "validator's own code (validation/code/), sharing nothing with cards/tools/; no analysis/ file read; replay on a host of another class; blind_from respected",
        "raw_data_paths_relative_to": "a depth-2 evidence directory (benchmarks/<dir>/ or research/ic_packed_filter_20261029/); repo_path gives the repository-root path",
        "raw_data": raw_files,
        "headline": heads,
        "descriptive_discrepancies": descriptive_discrepancies,
        "section_6_1": six1,
        "section_6_1_all_reproduced": all_six1,
        "p28_counted_session_of_record": {
            "job": "ic-dgbf-rho-p28-20261029-r2", "route_satisfied": p28["route_satisfied"], "EXIT42_CLAUSE": p28["EXIT42_CLAUSE"],
            "below_l2": p28["below_l2_from_gate"], "counted_equals_wall": j3["P28-counted-equals-wall"]["equal"],
            "cond10": j3["P28-counted-cond10"], "original_prefix_objects_now": s3["original_p28_counted_prefix_objects"],
            "sprime": {p: [j4["P28-counted"][p][w]["ratio_b_over_a"] for w in ("0.0", "0.1", "1.0")] for p in ("dgb/rho-neg", "dgbf/rho-neg", "dgbf/dgb")},
            "ok": p28_ok, "note": "P28 wall used for counts only (equality check); no P28 wall field read; P28 stays undecidable"},
        "joints": joints,
        "proves_too_much": proves_too_much,
        "selftest_negative_controls": st,
        "s3_copy_check": {k: {"one_pod_prefix_equal_to_job_exit": x["one_pod_prefix_equal_to_job_exit"], "all_files_equal": x.get("all_files_equal"),
                              "launcher_exit_codes": [y["exit_code"] for y in x["launcher_results"]]} for k, x in s3["jobs"].items()},
        "launcher_requests": s3r,
        "c2_identity_facts": c2,
        "defects": [{"name": h["name"], "claimed": h["claimed_as_printed"], "recomputed": h["recomputed"]} for h in defects],
        "outputs": "validation/code/out/",
    }
    v.dump(report, os.path.join(vdir, "validator-report.json"))

    runs = []
    for t, job, req in (("P32-wall", "ic-dgbf-p32-20261029", True), ("P32-counted", "ic-dgbf-rho-p32-20261029", True),
                        ("P24-wall", "ic-dgbf-p24-20261029", True), ("P24-counted", "ic-dgbf-rho-p24-20261029", True),
                        ("P28-counted", "ic-dgbf-rho-p28-20261029-r2", False), ("P28-wall", "ic-dgbf-p28-20261029", False)):
        b, kind = int(t[1:3]), t.split("-")[1]
        rr = reps[t]
        rpath = os.path.join(OUT, "replay", f"{t}.replay-receipt.json")
        run = J(f"replay/{t}.replay-run.json")
        sess = json.load(open(os.path.join(v.session_dir(b, kind), "session", "session.json")))
        runs.append({
            "run": t, "job": job, "session_id": sess["session_id"], "required_by_handoff": req,
            "descriptive_only": t == "P28-wall",
            "records_sha256": sess["records_sha256"],
            "replay_receipt": {"path": f"code/out/replay/{t}.replay-receipt.json", "sha256": v.sha256_file(rpath)},
            "command": f"ecbench verify --dir <copy of P{b}/{kind}/eks-results/session> --replay-all --out <receipt>",
            "ecbench_sha256": run["ecbench_sha256"], "started": run["started"], "finished": run["finished"], "exit": run["exit"],
            "ok": rr["ok"], "session_status": rr["session_status"], "problems": len(rr["problems"]),
            "replay_selection": rr["replay_selection"], "replay_candidates": rr["replay_candidates"],
            "replays": len(rr["replays"]), "identical": sum(1 for x in rr["replays"] if x["reproduced"] and x["detail"] == "identical"),
            "warmup_records_not_replayable_by_binary": run["warmup_records_not_replayed"],
            "replay_env_class_id": rr["auditor_env_class_id"], "replay_hardware_class_id": rr["auditor_hardware_class_id"],
            "pod_env_class_id": sess["env_class_id"],
            "own_scalar_check": {k_: sc[t if t != "P28-wall" else "P28-wall(descriptive)"][k_] for k_ in ("records", "warmups", "scalar_ok", "status_verified", "matches_target_true")},
            "own_scalar_check_failures": len(sc[t if t != "P28-wall" else "P28-wall(descriptive)"]["failures"]),
            "curve_binding": sc[t if t != "P28-wall" else "P28-wall(descriptive)"]["curves"],
        })
    required = [x for x in runs if x["required_by_handoff"]]
    cert_ok = all(x["ok"] and x["problems"] == 0 and x["identical"] == x["replays"] == 1152 and x["own_scalar_check"]["scalar_ok"] == 1344
                  and x["own_scalar_check_failures"] == 0 for x in required)
    cert = {
        "schema": "replay-certificate/v1 (docs/VALIDATION_GATE.md)",
        "task_id": "TASK-20261029-654f73",
        "status": "verified" if cert_ok else "failed",
        "independent": True,
        "scalars_verified": cert_ok,
        "binary": {"path": "/dev/shm/pf-bundle/ecbench (local copy of the image binary)", "sha256": "21701be6c420c3507a81767ed20f759e230f1fe5ec002ef3f7bb63a9c0b64503",
                   "image": "104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:32144a29e437376b9bf8b902aef4ba6b9cc723a15f01d296fb7b09eb131b817b",
                   "source_revision": "3271a25d02dcef6da148293fb8f9e2149ce59858"},
        "replay_host": open(os.path.join(OUT, "replay", "replay-host.txt")).read().strip().splitlines(),
        "pod_host": "c8a.2xlarge, AMD EPYC 9R45 (session/host.json)",
        "method": "every measured deterministic run re-executed by `ecbench verify --replay-all` on a byte-identical copy of the session directory (copy checked against SHA256SUMS before and after); warm-ups are refused by the binary's replay modes and are covered by the validator's own scalar check, which verifies [recovered]G = Q on every record (warm-ups included) with its own short-Weierstrass arithmetic, the curve bound by its ICV1 j-invariant, p + 1 - t = r and [r]G = O",
        "runs": required,
        "additional_runs": [x for x in runs if not x["required_by_handoff"] and not x["descriptive_only"]],
        "descriptive_runs": [x for x in runs if x["descriptive_only"]],
        "descriptive_note": "P28 wall replay is descriptive only: it never stands in for the missing in-pod audit (RULING-P28 (a), 'No post-hoc cure'), and P28 stays undecidable",
        "first_attempt": "an earlier attempt that added --replay-seq for each warm-up seq was refused for those seqs ('not replayable (a warm-up, ...)'); its P24-wall summary is in code/out/replay/attempt1-with-warmup-seqs/; the 1152 measured replays of that attempt were identical",
    }
    v.dump(cert, os.path.join(vdir, "replay-certificate.json"))
    print("status", status, "cert", cert["status"], "joints", joints, "6.1", all_six1, "defects", len(defects), "headlines", len(heads))
    for h in heads:
        if not h["reproduced"]:
            print("NOT REPRODUCED", h)


if __name__ == "__main__":
    main()
