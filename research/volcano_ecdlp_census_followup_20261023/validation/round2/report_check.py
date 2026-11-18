#!/usr/bin/env python3
"""Round-2 validator: check REPORT.md tables and verdict sentences against analysis/results.json.

Validator-written. Each numeric cell is compared with the results.json value formatted to the
cell's own number of decimals. Usage: report_check.py STUDY_DIR OUT_JSON
"""
import json, re, sys

study, out = sys.argv[1:3]
R = json.load(open(f"{study}/analysis/results.json"))
md = open(f"{study}/REPORT.md").read()
NUM = re.compile(r"-?\d+\.\d+|-?\d+")
checks, bad = [], []


def same(cell, val):
    cell = cell.strip().strip("*").strip()
    m = re.fullmatch(r"-?\d+(?:\.(\d+))?", cell)
    if not m:
        return None
    dec = len(m.group(1) or "")
    return f"{val:.{dec}f}" == cell or (cell.startswith("-") is False and f"{val:.{dec}f}" == "-" + cell)


def chk(where, cell, val):
    ok = same(cell, val)
    checks.append((where, cell, val, ok))
    if ok is not True:
        bad.append({"where": where, "report": cell, "results_json": val})


def table_after(anchor):
    i = md.index(anchor)
    lines = md[i:].split("\n")
    rows, started = [], False
    for l in lines[1:]:
        if l.startswith("|"):
            started = True
            rows.append([c.strip() for c in l.strip().strip("|").split("|")])
        elif started:
            break
    return rows[0], rows[2:]


def ci(cell):
    a, b = cell.strip().strip("[]").split(",")
    return a.strip(), b.strip()


# s4 level table
hdr, rows = table_after("| V | arm | L | curves |")
for r in rows:
    V, arm, L = r[0], r[1], r[2]
    c = R["level_effect"][f"{V}|{arm}"]["R"][L]["S"]
    w = f"s4 {V} {arm} L{L}"
    chk(w + " R", r[4], c["ratio"])
    a, b = ci(r[5]); chk(w + " ci95lo", a, c["ci95"][0]); chk(w + " ci95hi", b, c["ci95"][1])
    a, b = ci(r[6]); chk(w + " ci90lo", a, c["ci90"][0]); chk(w + " ci90hi", b, c["ci90"][1])
    chk(w + " halfwidth", r[7], c["half_width95"])
    if r[9] != "-":
        chk(w + " holm", r[9], c["p_holm"])
    if r[10] != c["verdict"]:
        bad.append({"where": w + " verdict", "report": r[10], "results_json": c["verdict"]})
    checks.append((w + " verdict", r[10], c["verdict"], r[10] == c["verdict"]))

# s5 curve-effect table
hdr, rows = table_after("| arm | F | permutation p | Holm p |")
for r in rows:
    c = R["curve_effect"][f"V6|{r[0]}"]
    if c.get("F") is None:
        continue
    chk(f"s5 {r[0]} F", r[1], c["F"]); chk(f"s5 {r[0]} p", r[2], c["p_perm"]); chk(f"s5 {r[0]} holm", r[3], c["p_perm_holm"])
    chk(f"s5 {r[0]} flagged", r[4], float(c["n_flagged"]))

# s7 H-T table
hdr, rows = table_after("| stratum | vertices |")
for r in rows:
    if r[0].startswith("**overall"):
        node = R["H-T"]["overall"]
    else:
        node = R["H-T"]["by_level"][r[0].split()[1]]
    w = f"s7 {r[0]}"
    for col, (key, p) in zip(r[2:], [("T", "0.0"), ("T", "0.1"), ("T", "1.0"), ("T0", "0.0"), ("T0", "1.0")]):
        val, rest = col.split(" ", 1)
        a, b = ci(rest)
        chk(f"{w} {key}@{p}", val, node[key][p]["value"])
        chk(f"{w} {key}@{p} lo", a, node[key][p]["ci95"][0]); chk(f"{w} {key}@{p} hi", b, node[key][p]["ci95"][1])

# s8 H-M table
hdr, rows = table_after("| curve | level | `ic` F |")
for r in rows:
    c = R["H-M"]["curves"][r[0]]["arms"]
    w = f"s8 {r[0]}"
    chk(w + " ic F", r[2], c["ic"]["F"])
    hp = r[3].replace("*", "").strip()
    chk(w + " ic holm", hp, c["ic"]["p_holm"])
    star = "*" in r[3]
    checks.append((w + " ic star", star, c["ic"]["significant"], star == c["ic"]["significant"]))
    if star != c["ic"]["significant"]:
        bad.append({"where": w + " ic star", "report": star, "results_json": c["ic"]["significant"]})
    v, rest = r[4].split(" ", 1); a, b = ci(rest)
    chk(w + " ic maxmin", v, c["ic"]["max_over_min_model_mean"]["value"])
    chk(w + " ic maxmin lo", a, c["ic"]["max_over_min_model_mean"]["ci95"][0]); chk(w + " ic maxmin hi", b, c["ic"]["max_over_min_model_mean"]["ci95"][1])
    chk(w + " rho F", r[5], c["rho-neg"]["F"]); chk(w + " rho holm", r[6], c["rho-neg"]["p_holm"])
    v, rest = r[7].split(" ", 1); a, b = ci(rest)
    chk(w + " rho maxmin", v, c["rho-neg"]["max_over_min_model_mean"]["value"])
    chk(w + " rho maxmin lo", a, c["rho-neg"]["max_over_min_model_mean"]["ci95"][0]); chk(w + " rho maxmin hi", b, c["rho-neg"]["max_over_min_model_mean"]["ci95"][1])

# s9 IC/rho table and pooled table
hdr, rows = table_after("| V | level | matched rho |")
for r in rows:
    V, L = r[0], r[1]
    if V == "V5" and L == "0":
        node = {p: {"r": R["ic_vs_rho_cubic_crater"]["prices"][p]["ratio_ic_over_rho_cubic"], "ci": R["ic_vs_rho_cubic_crater"]["prices"][p]["ci95"]} for p in ("0.0", "0.1", "1.0")}
    else:
        node = {p: {"r": R["ic_vs_rho"][V]["levels"][L]["prices"][p]["ratio_ic_over_rho"], "ci": R["ic_vs_rho"][V]["levels"][L]["prices"][p]["ci95"]} for p in ("0.0", "0.1", "1.0")}
    for col, p in zip(r[3:6], ("0.0", "0.1", "1.0")):
        v, rest = col.split(" ", 1); a, b = ci(rest)
        chk(f"s9 {V} L{L} @{p}", v, node[p]["r"]); chk(f"s9 {V} L{L} @{p} lo", a, node[p]["ci"][0]); chk(f"s9 {V} L{L} @{p} hi", b, node[p]["ci"][1])
hdr, rows = table_after("| pooled | at 0 GAE |")
for r in rows:
    V = r[0][:2]
    node = R["ic_vs_rho"][V]["pooled_levels_ge1" if V == "V5" else "pooled"]
    for col, p in zip(r[1:4], ("0.0", "0.1", "1.0")):
        v, rest = col.split(" ", 1); a, b = ci(rest)
        chk(f"s9 pooled {V} @{p}", v, node[p]["ratio_ic_over_rho"]); chk(f"s9 pooled {V} @{p} lo", a, node[p]["ci95"][0]); chk(f"s9 pooled {V} @{p} hi", b, node[p]["ci95"][1])

# s10 wall table
hdr, rows = table_after("| V | A/A ratio | 95% CI |")
for r in rows:
    node = R["wall_admission"][r[0]]
    chk(f"s10 {r[0]} aa", r[1], node["aa_ratio"])
    a, b = ci(r[2]); chk(f"s10 {r[0]} aa lo", a, node["ci95"][0]); chk(f"s10 {r[0]} aa hi", b, node["ci95"][1])
    for part in r[4].split(","):
        arm, pc = part.strip().rsplit(" ", 1)
        chk(f"s10 {r[0]} L2 {arm}", pc.rstrip("%"), 100 * node["arms"][arm]["frac_rows_L2"])

# s1 / s3 sentences: key numbers
pc = R["positive_control"]
sent = [("s1.1 ratio", "0.6113", pc["row_bootstrap"]["ratio"]), ("s1.1 lo", "0.5995", pc["row_bootstrap"]["ci95"][0]),
        ("s1.1 hi", "0.6237", pc["row_bootstrap"]["ci95"][1]), ("s1.1 slug lo", "0.5967", pc["slug_cluster_sensitivity"]["ci95"][0]),
        ("s1.1 slug hi", "0.6288", pc["slug_cluster_sensitivity"]["ci95"][1]), ("s1.1 n_pairs", "4000", float(pc["n_pairs"])),
        ("s1.4 gain", "0.6148", R["H-T"]["overall"]["cubic_over_neg_S_on_Tcr"]["value"]),
        ("s1.4 gain lo", "0.5895", R["H-T"]["overall"]["cubic_over_neg_S_on_Tcr"]["ci95"][0]),
        ("s1.4 gain hi", "0.6420", R["H-T"]["overall"]["cubic_over_neg_S_on_Tcr"]["ci95"][1]),
        ("s1.4 transport share %", "0.21", 100 * R["H-T"]["overall"]["transport_share_of_T_numerator"]["1.0"]["value"]),
        ("s1.6 V6 @0", "60.4", R["ic_vs_rho"]["V6"]["pooled"]["0.0"]["ratio_ic_over_rho"]),
        ("s1.6 V6 @1", "90.8", R["ic_vs_rho"]["V6"]["pooled"]["1.0"]["ratio_ic_over_rho"]),
        ("s1.6 V5 @0", "53.3", R["ic_vs_rho"]["V5"]["pooled_levels_ge1"]["0.0"]["ratio_ic_over_rho"]),
        ("s1.6 V5 @1", "81.8", R["ic_vs_rho"]["V5"]["pooled_levels_ge1"]["1.0"]["ratio_ic_over_rho"]),
        ("s1.6 crater @0", "89.3", R["ic_vs_rho_cubic_crater"]["prices"]["0.0"]["ratio_ic_over_rho_cubic"]),
        ("s1.6 crater @0.1", "98.2", R["ic_vs_rho_cubic_crater"]["prices"]["0.1"]["ratio_ic_over_rho_cubic"]),
        ("s1.6 crater @1", "141.2", R["ic_vs_rho_cubic_crater"]["prices"]["1.0"]["ratio_ic_over_rho_cubic"]),
        ("s1.7 V5 aa", "1.042", R["wall_admission"]["V5"]["aa_ratio"]), ("s1.7 V6 aa", "1.001", R["wall_admission"]["V6"]["aa_ratio"]),
        ("s1.7 V5 L2 all rows %", "87", 100 * R["isolation_fraction_L2"]["V5"]), ("s1.7 V6 L2 all rows %", "84", 100 * R["isolation_fraction_L2"]["V6"]),
        ("s2 rows measured", "428976", float(R["data_checks"]["rows_measured"])),
        ("s2 warmup", "241712", float(R["data_checks"]["rows_all_incl_warmup"] - R["data_checks"]["rows_measured"])),
        ("s1.5 ic sig", "5", float(R["H-M"]["summary"]["ic"]["n_significant_holm"])),
        ("s1.5 rho sig", "0", float(R["H-M"]["summary"]["rho-neg"]["n_significant_holm"])),
        ("s1.5 bsgs sig", "0", float(R["H-M"]["summary"]["bsgs"]["n_significant_holm"]))]
for w, cell, val in sent:
    chk(w, cell, val)
text_checks = {
    "s1.1 text": "mean S(`rho-cubic`) / mean S(`rho-neg`) = **0.6113**, 95% CI [0.5995, 0.6237] over 4000 paired runs" in md,
    "s1.4 text": "T = 0.5946 [0.5684, 0.6220] at 0.0 GAE, 0.5917 [0.5655, 0.6193] at 0.1 GAE, 0.5781 [0.5512, 0.6063] at 1.0 GAE" in md,
    "s1.4 T0 text": "0.9672 [0.9227, 1.0145], 0.9672 [0.9223, 1.0149], 0.9670 [0.9204, 1.0164]" in md,
    "s2 counts text": "428,976 measured runs after dropping 241,712 warm-up runs" in md,
    "verdict H-T": R["H-T"]["verdict"] == "supported" and "**Verdict: supported.**" in md,
    "verdict H-M": R["H-M"]["verdict"] == "inconclusive" and "**Verdict: inconclusive.**" in md,
    "verdict Q2a": R["positive_control"]["pass"] is True and "**Pass.**" in md,
}
for k, v in text_checks.items():
    checks.append((k, None, None, v))
    if not v:
        bad.append({"where": k})
res = {"n_checked": len(checks), "n_ok": sum(1 for c in checks if c[3] is True), "mismatches": bad}
json.dump(res, open(out, "w"), indent=1, default=str)
print(json.dumps({k: v for k, v in res.items() if k != "mismatches"}), json.dumps(bad[:30], default=str))
