#!/usr/bin/env python3
"""Second follow-up for VAL-20261029-33a286 (validator's own code): reviews the Coordinator's
working-tree corrections on top of 313ec5d2af (README.md, README.pdf, benchmarks REPORT.pdf,
PLACEMENT.md, the regenerated leaderboard and the benchmarks row) against the validator's numbers,
and pins the sha256 of every reviewed file.  The clearance it supports holds only for those bytes.
Writes followup/followup2.json.  Read-only on the repository (git diff, git show, pdftotext).
"""
import hashlib
import json
import re
import subprocess

WT = "/dev/shm/wt-blk"
STUDY = "research/ic_dg_blocked_20261029"
EV = f"{WT}/{STUDY}"
SNAP1 = "313ec5d2af38fe7994a6eab34aa1ef98e374f309"
V = "/dev/shm/dgb-validation"
OUT = f"{V}/followup/followup2.json"


def git(*a):
    return subprocess.run(["git", "-C", WT, *a], check=True, capture_output=True, text=True).stdout


def sha(p):
    return hashlib.sha256(open(p, "rb").read()).hexdigest()


R = json.load(open(f"{V}/work/recompute.json"))["sizes"]
F1 = json.load(open(f"{V}/followup/followup.json"))
cf = F1["phase_counterfactual"]
res = {"base_commit": SNAP1, "head": git("rev-parse", "HEAD").strip(),
       "remote_tip": git("ls-remote", "origin", "claude/ic-dg-blocked-20261029").split()[0]}

# ------------------------------------------------------------------ README.md
readme = open(f"{EV}/README.md").read()
old = git("show", f"{SNAP1}:{STUDY}/README.md")
removed = [ln[1:] for ln in git("diff", SNAP1, "--", f"{STUDY}/README.md").split("\n") if ln.startswith("-") and not ln.startswith("---")]
res["readme_lines_removed"] = removed
# every numeric token of the committed README that the diff did not remove is still present
kept_nums = set(re.findall(r"\d+\.\d+", "\n".join(ln for ln in old.split("\n") if ln not in removed)))
res["readme_numbers_dropped"] = sorted(n for n in kept_nums if n not in readme)
# 1. position table: rows equal the recomputed positions at every size
pos = {P: R[P]["positions"] for P in ("P24", "P28", "P32")}
same = all(pos[P]["position_measured"] == pos["P24"]["position_measured"] for P in pos)
want = {a: " and ".join(sorted(v)) for a, v in pos["P24"]["position_measured"].items()}
rows = {m.group(1): (m.group(2), m.group(3)) for m in re.finditer(r"^\| `([a-z-]+)` \| ([0-9 and]+) \| (\d+) each \|$", readme, re.M)}
res["position_table"] = {
    "rows": rows, "identical_at_all_sizes": same,
    "rows_match_recompute": same and set(rows) == set(want) and all(rows[a][0] == want[a] and
                                                                    all(c == int(rows[a][1]) for c in pos["P24"]["position_measured"][a].values())
                                                                    for a in want),
    "directions_match": all(pos[P]["direction_measured"] == {"forward": 144, "reversed": 144} for P in pos)
                         and "144 forward and 144 reversed blocks" in readme,
    "precedence_match": all(pos[P]["dgb_vs_dg-aa_precedence_measured"] == {"dg-aa_before_dgb": 144, "dgb_before_dg-aa": 144} for P in pos)
                         and "`dgb` follows `dg-aa` in 144 blocks and precedes it in 144" in readme,
    "cites_validator_file": "validation/work/recompute.json" in readme}
# 2. causal sentence replaced; numbers equal the validator's counterfactual
bad = "which is why every ratio missed its prediction"
nums = {"0.594": cf["P24"]["setup_ratio"], "0.565": cf["P28"]["setup_ratio"], "0.451": cf["P32"]["setup_ratio"],
        "0.898": cf["P24"]["ratio_if_pdp_unchanged"], "0.867": cf["P28"]["ratio_if_pdp_unchanged"], "0.851": cf["P32"]["ratio_if_pdp_unchanged"]}
res["causal_sentence"] = {"old_sentence_gone": bad not in readme,
                          "numbers_present_and_equal": {k: (k in readme) and abs(float(k) - round(v, 3)) < 1e-12 for k, v in nums.items()},
                          "two_causes_stated": "Together with a set-up saving smaller than predicted at 24 and 28 bits" in readme,
                          "32_bit_reading": "Only at 32 bits, where the set-up fell to 0.451, is the PDP rise nearly the whole miss" in readme}
# minor points
res["minor"] = {"provenance_line_reworded": "Nothing here is computed by hand" not in readme and "descriptive arithmetic" in readme,
                "next_table_fixed": "(see the next table)" not in readme and '(see "Counted units" below)' in readme
                                    and "## Counted units" in readme,
                "pr2844_marked": "outside this evidence, not checked by the validator" in readme}

# ------------------------------------------------------------------ PDFs
txt = subprocess.run(["pdftotext", "-layout", f"{EV}/README.pdf", "-"], capture_output=True, text=True).stdout
flat = re.sub(r"\s+", " ", txt)
imgs = [x for x in subprocess.run(["pdfimages", "-list", f"{EV}/README.pdf"], capture_output=True, text=True).stdout.split("\n")[2:] if x.strip()]
rnums = set(re.findall(r"\d+\.\d+", readme))
res["pdf"] = {"readme_pdf_sha256": sha(f"{EV}/README.pdf"),
              "benchmarks_report_pdf_equal": sha(f"{WT}/benchmarks/20261029-ic-dg-blocked/REPORT.pdf") == sha(f"{EV}/README.pdf"),
              "readme_decimal_numbers": len(rnums), "missing_in_pdf": sorted(n for n in rnums if n not in txt),
              "old_sentence_absent": bad not in flat,
              "new_sentence_present": "Together with a set-up saving smaller than predicted" in flat,
              "position_table_present": all(re.search(rf"{a}\s+0 and 3|{a}\s+1 and 2", txt) for a in ("dg-aa", "dgb-edge")),
              "embedded_images": len(imgs)}

# ------------------------------------------------------------------ PLACEMENT.md
pl = open(f"{EV}/PLACEMENT.md").read()
pt_src = git("show", f"{SNAP1}:scripts/build_ic_leaderboard.py")
m = re.search(r"def pair_table_component\(\).*?\n\n\n", pt_src, re.S)
res["placement"] = {"names_section": "`dg_blocked_component`" in pl,
                    "names_rows": all(r in pl for r in ("icv1-fp28-t9805-a382b8a8", "icv1-fp28-tm3463-6d2f5699")),
                    "outside_tables_A_C": "outside tables A–C" in pl,
                    "pair_table_component_is_setup_only_at_313ec5d2af": bool(m) and "ic_oracle_setup_blocked_20261020" in m.group(0)
                                                                         and "set-up" in m.group(0),
                    "no_24_32_rows": "24 and 32 bits are undecidable and get no row" in pl,
                    "tracked": bool(git("ls-files", f"{STUDY}/PLACEMENT.md").strip())}

# ------------------------------------------------------------------ leaderboard and benchmarks row
lb = json.load(open(f"{WT}/docs/ic/leaderboard.json"))["dg_blocked_component"]
row = lb["rows"]
d = R["P28"]["recompute"]["decision_dg__dgb"]
band = F1["counted_band"]["P28"]


def rel(a, b):
    return abs(a - b) / max(abs(b), 1e-300)


res["leaderboard"] = {
    "rows_bits": [r["bits"] for r in row], "validated_flag": lb.get("validated"),
    "max_rel_diff_vs_recompute": max(rel(row[0]["ns_dg"], d["ns_per_gae_a"]), rel(row[0]["ns_dgb"], d["ns_per_gae_b"]),
                                     rel(row[0]["aa_ratio"], d["aa_ratio"]), rel(row[0]["corrected"], d["corrected_ratio"]),
                                     *[rel(x, y) for x, y in zip(row[0]["corrected_ci95"], d["corrected_ci95"])],
                                     *[rel(x, y) for x, y in zip(row[0]["aa_ci95"], d["aa_ci95"])],
                                     *[rel(row[0]["dgb_over_rho"][i]["ratio"], band[w]["own_point"]) for i, w in enumerate(("0", "0.1", "1"))],
                                     *[rel(row[0]["dg_over_rho"][i]["ratio"], band[w]["own_point"]) for i, w in enumerate(("0", "0.1", "1"))]),
    "counted_dgb_over_dg": [x["ratio"] for x in row[0]["counted"]],
    "markdown_has_row": "**0.9300 [0.9266, 0.9333]**" in open(f"{WT}/docs/ic/LEADERBOARD.md").read(),
    "html_has_row": "<b>0.9300 [0.9266, 0.9333]</b>" in open(f"{WT}/docs/ic-leaderboard.html").read()}
brow = [ln for ln in open(f"{WT}/benchmarks/README.md").read().split("\n") if "20261029-ic-dg-blocked" in ln]
res["benchmarks_row"] = {"rows": len(brow), "edge_corrected_0.9302": bool(brow) and "edge control (A/A-corrected) 0.9302" in brow[0],
                         "decision": bool(brow) and "**0.9300 [0.9266, 0.9333]**" in brow[0]}

# ------------------------------------------------------------------ pins of every reviewed working-tree file
files = [f"{STUDY}/README.md", f"{STUDY}/README.pdf", f"{STUDY}/PLACEMENT.md", "benchmarks/20261029-ic-dg-blocked/REPORT.pdf",
         "benchmarks/README.md", "docs/ic/LEADERBOARD.md", "docs/ic/leaderboard.json", "docs/ic-leaderboard.html",
         "scripts/build_ic_leaderboard.py"]
res["reviewed_files_sha256"] = {f: sha(f"{WT}/{f}") for f in files}
res["status_short"] = [x for x in git("status", "--short").split("\n") if x]

checks = [res["readme_numbers_dropped"] == [], res["position_table"]["rows_match_recompute"], res["position_table"]["directions_match"],
          res["position_table"]["precedence_match"], res["causal_sentence"]["old_sentence_gone"],
          all(res["causal_sentence"]["numbers_present_and_equal"].values()), res["causal_sentence"]["two_causes_stated"],
          all(res["minor"].values()), res["pdf"]["benchmarks_report_pdf_equal"], not res["pdf"]["missing_in_pdf"],
          res["pdf"]["old_sentence_absent"], res["pdf"]["new_sentence_present"], res["pdf"]["position_table_present"],
          res["pdf"]["embedded_images"] > 0, res["placement"]["names_section"], res["placement"]["names_rows"],
          res["placement"]["pair_table_component_is_setup_only_at_313ec5d2af"], res["leaderboard"]["rows_bits"] == [28],
          res["leaderboard"]["max_rel_diff_vs_recompute"] < 1e-9, res["leaderboard"]["counted_dgb_over_dg"] == [1.0, 1.0, 1.0],
          res["leaderboard"]["markdown_has_row"], res["leaderboard"]["html_has_row"], res["benchmarks_row"]["edge_corrected_0.9302"]]
res["all_checks_hold"] = all(checks)
res["n_checks"] = len(checks)
json.dump(res, open(OUT, "w"), indent=1, sort_keys=True)
print(json.dumps({k: v for k, v in res.items() if k not in ("readme_lines_removed",)}, indent=1))
