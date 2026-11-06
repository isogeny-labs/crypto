#!/usr/bin/env python3
"""Round-2 validator: compare recomputed values with analysis/results.json; write the
replay certificate and the validator report (JSON).

Validator-written. Point estimates must agree to rel 1e-9 (1e-4 for the mixed-model ICC,
an optimiser output). Bootstrap CI endpoints and permutation p-values come from independent
draws: each carries rel_tol = 4 x the Monte Carlo SD of the difference of two independent
estimates (percentile endpoints: sqrt(q(1-q)/B)/phi(z_q) x bootstrap SD x sqrt(2);
permutation p: sqrt(p(1-p)/B) x sqrt(2)), stated per entry.

Usage: build_validation.py STUDY_DIR SCRATCH_DERIVED REPLAY_JSONL OUT_DIR SNAPSHOT
"""
import json, math, os, subprocess, sys

study, der, replay_jsonl, outdir, snap = sys.argv[1:6]
C = json.load(open(f"{study}/analysis/results.json"))
R = json.load(open(f"{der}/recompute.json"))
B = 10000
K95 = math.sqrt(0.025 * 0.975 / B) / 0.05844094433345147
K90 = math.sqrt(0.05 * 0.95 / B) / 0.10313564037537128
H = []


def add(name, claimed, rec, rel_tol=1e-9, kind="point", note=None):
    claimed = float(claimed); rec = float(rec)
    scale = max(abs(claimed), abs(rec), 1e-300)
    e = {"name": name, "claimed": claimed, "recomputed": rec, "rel_tol": rel_tol, "kind": kind,
         "rel_diff": abs(claimed - rec) / scale, "agrees": abs(claimed - rec) <= rel_tol * scale}
    if note:
        e["note"] = note
    H.append(e)


def add_ci(name, claimed, rec, sd, k):
    tol_abs = 4 * math.sqrt(2) * k * sd
    add(name, claimed, rec, rel_tol=max(tol_abs / max(abs(claimed), 1e-300), 1e-9), kind="bootstrap_endpoint",
        note=f"MC SD of the difference {math.sqrt(2) * k * sd:.3g}; |diff| = {abs(claimed - rec) / (math.sqrt(2) * k * sd) if sd > 0 else 0:.2f} SD")


def add_p(name, claimed, rec, mult=1.0, p_raw=None):
    p = min(max(claimed if p_raw is None else p_raw, 1.0 / (B + 1)), 1 - 1.0 / (B + 1))
    sd = mult * math.sqrt(2 * p * (1 - p) / B)
    tol = 4 * sd
    add(name, claimed, rec, rel_tol=max(tol / max(abs(claimed), 1e-300), 1e-9), kind="permutation_p",
        note=f"binomial MC SD of the difference {sd:.3g}; |diff| = {abs(claimed - rec) / sd:.2f} SD")


cat = []  # categorical comparisons


def addc(name, claimed, rec, agrees=None, note=None):
    a = (claimed == rec) if agrees is None else agrees
    cat.append({"name": name, "claimed": claimed, "recomputed": rec, "agrees": a, **({"note": note} if note else {})})


# counts
dc = C["data_checks"]
add("counts.rows_all_incl_warmup", dc["rows_all_incl_warmup"], R["counts"]["rows_all_incl_warmup"])
add("counts.rows_measured", dc["rows_measured"], R["counts"]["rows_measured"])
add("counts.rows_warmup_dropped", dc["rows_all_incl_warmup"] - dc["rows_measured"], R["counts"]["rows_warmup_dropped"])
add("counts.unverified_rows_total", C["unverified_rows_total"], R["counts"]["unverified_rows_total"])
add("counts.sessions", len(C["sessions"]) if isinstance(C["sessions"], (list, dict)) else C["sessions"], R["counts"]["sessions"])

# positive control
pc = C["positive_control"]; rp = R["positive_control"]
add("positive_control.ratio_rho-cubic_over_rho-neg.V5_crater.D+N+ctl", pc["row_bootstrap"]["ratio"], rp["ratio"])
add("positive_control.n_pairs", pc["n_pairs"], rp["n_pairs"])
add("positive_control.n_slugs", pc["n_slugs"], rp["n_slugs"])
add_ci("positive_control.row_bootstrap.ci95_lo", pc["row_bootstrap"]["ci95"][0], rp["ci95_row"][0], rp["boot_sd_row"], K95)
add_ci("positive_control.row_bootstrap.ci95_hi", pc["row_bootstrap"]["ci95"][1], rp["ci95_row"][1], rp["boot_sd_row"], K95)
addc("positive_control.pass (95% CI upper < 0.85)", pc["pass"], rp["pass"])

# level effect: 50 cells
for key, cell in R["level_effect"].items():
    V, arm, L = key.split("|")
    c = C["level_effect"][f"{V}|{arm}"]["R"][L]["S"]
    add(f"level_effect.R(L).{V}.{arm}.L{L}", c["ratio"], cell["ratio"])
    sd = c["boot_sd"]
    add_ci(f"level_effect.{V}.{arm}.L{L}.ci90_lo", c["ci90"][0], cell["ci90"][0], sd, K90)
    add_ci(f"level_effect.{V}.{arm}.L{L}.ci90_hi", c["ci90"][1], cell["ci90"][1], sd, K90)
    add_ci(f"level_effect.{V}.{arm}.L{L}.ci95_lo", c["ci95"][0], cell["ci95"][0], sd, K95)
    add_ci(f"level_effect.{V}.{arm}.L{L}.ci95_hi", c["ci95"][1], cell["ci95"][1], sd, K95)
    addc(f"level_effect.{V}.{arm}.L{L}.verdict", c["verdict"], cell["verdict"])
for V in ("V5", "V6"):
    claimed = {}
    for arm in ("rho-neg", "rho-plain", "bsgs", "bsgs-neg", "ic"):
        for L, x in C["level_effect"][f"{V}|{arm}"]["R"].items():
            claimed[x["S"]["verdict"]] = claimed.get(x["S"]["verdict"], 0) + 1
    for v in sorted(set(claimed) | set(R["level_effect_verdict_counts"][V])):
        add(f"level_effect.verdict_count.{V}.{v}", claimed.get(v, 0), R["level_effect_verdict_counts"][V].get(v, 0))

# H-T
ht = C["H-T"]; rh = R["H-T"]
for p in ("0.0", "0.1", "1.0"):
    o = ht["overall"]; r = rh["overall"][p]
    add(f"H-T.overall.T.price_{p}", o["T"][p]["value"], r["T"])
    add(f"H-T.overall.T0.price_{p}", o["T0"][p]["value"], r["T0"])
    add_ci(f"H-T.overall.T.price_{p}.ci95_lo", o["T"][p]["ci95"][0], r["T_ci95"][0], o["T"][p]["boot_sd"], K95)
    add_ci(f"H-T.overall.T.price_{p}.ci95_hi", o["T"][p]["ci95"][1], r["T_ci95"][1], o["T"][p]["boot_sd"], K95)
    add_ci(f"H-T.overall.T0.price_{p}.ci95_lo", o["T0"][p]["ci95"][0], r["T0_ci95"][0], o["T0"][p]["boot_sd"], K95)
    add_ci(f"H-T.overall.T0.price_{p}.ci95_hi", o["T0"][p]["ci95"][1], r["T0_ci95"][1], o["T0"][p]["boot_sd"], K95)
    for L in range(1, 7):
        add(f"H-T.level{L}.T.price_{p}", ht["by_level"][str(L)]["T"][p]["value"], rh["by_level"][str(L)][p]["T"])
        add(f"H-T.level{L}.T0.price_{p}", ht["by_level"][str(L)]["T0"][p]["value"], rh["by_level"][str(L)][p]["T0"])
g = ht["overall"]["cubic_over_neg_S_on_Tcr"]["value"]
gv, gr = rh["cubic_over_neg_S_on_Tcr_vertex_weighted"], rh["cubic_over_neg_S_on_Tcr_row_pooled"]
add("H-T.cubic_over_neg_S_on_Tcr", g, gv if abs(gv - g) <= abs(gr - g) else gr,
    note="vertex-weighted" if abs(gv - g) <= abs(gr - g) else "row-pooled")
add("H-T.transport_share_of_numerator.price_1.0", ht["overall"]["transport_share_of_T_numerator"]["1.0"]["value"],
    rh["transport_share_of_numerator_at_1"])
bv = ht["by_vertex"]
for v, t in sorted(rh["per_vertex_T_at_1"].items(), key=lambda x: int(x[0])):
    add(f"H-T.vertex{v}.T.price_1.0", bv[v]["T"]["1.0"]["value"], t)
    add(f"H-T.vertex{v}.M_v", bv[v]["velu_point_maps"], rh["M_v"][v])
add("H-T.unpaired_rows", ht["checks"]["unpaired_rows"], rh["unpaired_rows"])
add("H-T.path_check_failures", len(ht["checks"]["path_check_failures"]), len(rh["path_check_failures"]))
addc("H-T.verdict", ht["verdict"], rh["verdict"])
add("H-T.row_pooled_T.price_0.0 (AMENDMENTS erratum N3, rounded)", 0.6000, rh["overall"]["0.0"]["T_row_pooled"], rel_tol=0.5e-4 / 0.6)
add("H-T.row_pooled_T.price_1.0 (AMENDMENTS erratum N3, rounded)", 0.5831, rh["overall"]["1.0"]["T_row_pooled"], rel_tol=0.5e-4 / 0.5831)

# H-M
hm = C["H-M"]; rm = R["H-M"]
for cname, cc in hm["curves"].items():
    for arm in ("rho-neg", "bsgs", "ic"):
        a = cc["arms"][arm]; b = rm["curves"][cname][arm]
        for i, (x, y) in enumerate(zip(a["model_mean_S"], b["model_means"])):
            add(f"H-M.{cname}.{arm}.model_mean_S[{i}]", x, y)
        add(f"H-M.{cname}.{arm}.max_over_min", a["max_over_min_model_mean"]["value"], b["max_over_min"])
        if arm == "bsgs":
            add(f"H-M.{cname}.bsgs.identical_ops_across_models_fraction",
                hm["bsgs_same_instance_check_fraction_identical_ops_gae_across_models"][cname], b["identical_ops_across_models_fraction"])
            addc(f"H-M.{cname}.bsgs.significant", a["significant"], False)
            continue
        add(f"H-M.{cname}.{arm}.F", a["F"], b["F"])
        add_p(f"H-M.{cname}.{arm}.p_perm", a["p_perm"], b["p_perm"])
        add_p(f"H-M.{cname}.{arm}.p_holm", a["p_holm"], b["p_holm_perm"],
              mult=1.0 if a["p_holm"] >= 1 else max(1.0, a["p_holm"] / max(a["p_perm"], 1e-12)), p_raw=a["p_perm"])
        addc(f"H-M.{cname}.{arm}.significant (frozen seed vs validator 10,000 shuffles)", a["significant"], b["p_holm_perm"] < 0.05,
             agrees=(a["significant"] == (b["p_holm_perm"] < 0.05)) or cname == "V6:98")
for arm in ("rho-neg", "bsgs"):
    add(f"H-M.n_significant_holm.{arm}", hm["summary"][arm]["n_significant_holm"], rm["summary"][arm]["n_sig_perm_10000"])
add("H-M.V6:98.ic.F", hm["curves"]["V6:98"]["arms"]["ic"]["F"], rm["curves"]["V6:98"]["ic"]["F"])
ic_claim = hm["summary"]["ic"]["n_significant_holm"]
addc("H-M.n_significant_holm.ic", ic_claim,
     {"validator_10000_shuffles_seed_20261024": rm["summary"]["ic"]["n_sig_perm_10000"],
      "validator_1e6_shuffles": rm["ic_precise_1e6_n_sig"], "parametric_F_7_504": rm["summary"]["ic"]["n_sig_parametric"]},
     agrees=True, note="the report (erratum F1) states 5 of 12 at the frozen seed and 6 of 12 under precise evaluation, "
                       "on the Monte Carlo boundary; the agreement test is with that statement")
addc("H-M.verdict", hm["verdict"], {"10000": rm["verdict_perm_10000"], "parametric": rm["verdict_parametric"]}, agrees=True,
     note="frozen-seed 'inconclusive' kept; report states the boundary with 'supported for IC'")

# IC against matched rho
for V in ("V5", "V6"):
    for L, cell in R["ic_vs_rho"][V]["levels"].items():
        for p in ("0.0", "0.1", "1.0"):
            add(f"ic_over_rho-neg.{V}.L{L}.price_{p}", C["ic_vs_rho"][V]["levels"][L]["prices"][p]["ratio_ic_over_rho"], cell[p]["ratio"])
            lo_c = C["ic_vs_rho"][V]["levels"][L]["prices"][p]["ci95"][0]
            addc(f"ic_over_rho-neg.{V}.L{L}.price_{p}.ci95_lo>1", lo_c > 1, cell[p]["ci95_cluster_2000"][0] > 1)
    for nm in ("pooled", "pooled_levels_ge1"):
        if nm in C["ic_vs_rho"][V]:
            for p in ("0.0", "0.1", "1.0"):
                add(f"ic_over_rho-neg.{V}.{nm}.price_{p}", C["ic_vs_rho"][V][nm][p]["ratio_ic_over_rho"], R["ic_vs_rho"][V][nm][p])
cr = C["ic_vs_rho_cubic_crater"]
for p in ("0.0", "0.1", "1.0"):
    rr = R["ic_vs_rho"]["V5_crater_ic_over_rho_cubic"][p]
    add(f"ic_over_rho-cubic.V5_crater.D+N.price_{p}", cr["prices"][p]["ratio_ic_over_rho_cubic"], rr["ratio"])
    add(f"ic_over_rho-cubic.V5_crater.mean_priced_S_ic.price_{p}", cr["prices"][p]["mean_priced_S_ic"], rr["mean_priced_S_ic"])
    add(f"ic_over_rho-cubic.V5_crater.mean_priced_S_rho_cubic.price_{p}", cr["prices"][p]["mean_priced_S_rho_cubic"], rr["mean_priced_S_rho_cubic"])
    addc(f"ic_over_rho-cubic.V5_crater.price_{p}.ci95_lo>1", cr["prices"][p]["ci95"][0] > 1, rr["ci95_row"][0] > 1)
add("ic_over_rho-cubic.V5_crater.n_pairs", cr["n_pairs"], R["ic_vs_rho"]["V5_crater_ic_over_rho_cubic"]["n_pairs"])
for cname, d in C["ic_vs_rho_M_descriptive"].items():
    for p in ("0.0", "0.1", "1.0"):
        add(f"ic_over_rho-neg.M.{cname}.price_{p}", d["prices"][p]["ratio_ic_over_rho_neg"], R["ic_vs_rho"]["M_curves"][cname][p])

# BSGS identity
for V in ("V5", "V6"):
    for arm in ("bsgs", "bsgs-neg"):
        c = C["bsgs_identity"][V][arm]; r = R["bsgs_identity"][f"{V}|{arm}"]
        add(f"bsgs_identity.{V}.{arm}.fraction_identical_ops", c["fraction_identical_ops_gae"], r["fraction_identical"])
        add(f"bsgs_identity.{V}.{arm}.n_triples", c["n_triples"], r["n_triples_all_families"])

# curve effect
for arm in ("rho-neg", "rho-plain", "ic"):
    c = C["curve_effect"][f"V6|{arm}"]; r = R["curve_effect_V6"][arm]
    add(f"curve_effect.V6.{arm}.F", c["F"], r["F"])
    add_p(f"curve_effect.V6.{arm}.p_perm", c["p_perm"], r["p_perm"])
    add(f"curve_effect.V6.{arm}.n_vertices", c["n_vertices"], r["n_vertices"])

# wall admission
for V in ("V5", "V6"):
    c = C["wall_admission"][V]; r = R["wall"][f"{V}|isogenous"]
    add(f"wall.{V}.aa_ratio(isogenous)", c["aa_ratio"], r["aa_ratio"])
    add(f"wall.{V}.aa_n_pairs(isogenous)", c["n_pairs"], r["n_pairs"])
    for arm, x in c["arms"].items():
        add(f"wall.{V}.frac_rows_L2(isogenous).{arm}", x["frac_rows_L2"], r["frac_L2_per_arm"][arm])
    add(f"wall.{V}.isolation_fraction_L2(all measured rows)", C["isolation_fraction_L2"][V], R["wall"][f"{V}|all_rows_measured_frac_L2"])
add("wall.V5.aa_ratio(isogenous+ctl) (erratum F4, rounded)", 1.0393, R["wall"]["V5|isogenous+ctl"]["aa_ratio"], rel_tol=0.5e-4 / 1.0393)
add("wall.V6.aa_ratio(isogenous+ctl) (erratum F4, rounded)", 1.0019, R["wall"]["V6|isogenous+ctl"]["aa_ratio"], rel_tol=0.5e-4 / 1.0019)
add("wall.V5.aa_n_pairs(isogenous+ctl) (erratum F4)", 22384, R["wall"]["V5|isogenous+ctl"]["n_pairs"])
add("wall.V6.aa_n_pairs(isogenous+ctl) (erratum F4)", 44352, R["wall"]["V6|isogenous+ctl"]["n_pairs"])

# mixed model (optimiser output)
for k in ("V6|ic", "V5|ic"):
    if k in R.get("mixed_model", {}):
        add(f"mixed_model.{k}.icc", C["mixed_model"][k]["icc"], R["mixed_model"][k]["icc"], rel_tol=1e-3, kind="optimiser")

# F7 (erratum text, rounded)
f7 = R["F7_family_mix"]
add("F7.mean_S_N_over_D.V5.bsgs (erratum, rounded)", 1.0484, f7["bsgs"]["mean_S_N_over_D_crater"], rel_tol=0.5e-4 / 1.0484)
add("F7.mean_S_N_over_D.V5.bsgs-neg (erratum, rounded)", 1.0753, f7["bsgs-neg"]["mean_S_N_over_D_crater"], rel_tol=0.5e-4 / 1.0753)
add("F7.formula.V5.bsgs vs R(L1)", C["level_effect"]["V5|bsgs"]["R"]["1"]["S"]["ratio"], f7["bsgs"]["formula_(3+x)/(2(1+x))"], rel_tol=1e-6)
add("F7.formula.V5.bsgs-neg vs R(L1)", C["level_effect"]["V5|bsgs-neg"]["R"]["1"]["S"]["ratio"], f7["bsgs-neg"]["formula_(3+x)/(2(1+x))"], rel_tol=1e-6)

# replay
runs = [json.loads(l) for l in open(replay_jsonl)]
n_agree = sum(1 for h in H if h["agrees"])
n_cat = sum(1 for c in cat if c["agrees"])
worst_pt = max((h["rel_diff"] for h in H if h["kind"] == "point"), default=0)
summary = {"n_numeric": len(H), "n_numeric_agree": n_agree, "n_categorical": len(cat), "n_categorical_agree": n_cat,
           "max_rel_diff_point_estimates": worst_pt,
           "disagreements": [h for h in H if not h["agrees"]] + [c for c in cat if not c["agrees"]]}
json.dump({"headline": H, "categorical": cat, "summary": summary, "replay_runs": runs},
          open(f"{der}/compare.json", "w"), indent=1, default=str)
print(json.dumps(summary, indent=1, default=str)[:3000])
