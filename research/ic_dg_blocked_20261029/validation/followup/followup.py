#!/usr/bin/env python3
"""Follow-up review for VAL-20261029-33a286 at commit 313ec5d2af (validator's own code).

Checks, against my own recomputation and the S3 originals:
  1. the commit only adds files after bf899979a6, and validation/ equals my output;
  2. the committed launch/ folders bind to the S3 launcher receipts;
  3. README.md numbers (decision, edge control, phase sums, counted band, validation summary);
  4. the figure reproduces from the committed analysis files (re-rendered in scratch);
  5. README.pdf carries the README's numbers and the figure;
  6. the README's causal sentence on the prediction miss (phase counterfactual).
Writes followup/followup.json.  Reads the repository with read-only git commands;
writes only under /dev/shm/dgb-validation/followup and /dev/shm/dgb-val/plotcheck.
"""
import glob
import hashlib
import json
import os
import re
import shutil
import subprocess
import sys

import numpy as np

sys.path.insert(0, "/dev/shm/dgb-validation/code")
import dgbstats as st  # noqa: E402
import dgbval as dv  # noqa: E402

WT = "/dev/shm/wt-blk"
EVREL = "research/ic_dg_blocked_20261029"
EV = f"{WT}/{EVREL}"
SNAP0 = "bf899979a6a6304fbbfbbc27204e50acfc52f6b8"
SNAP1 = "313ec5d2af38fe7994a6eab34aa1ef98e374f309"
V = "/dev/shm/dgb-validation"
S3L = "/dev/shm/dgb-val/s3-launcher"
OUT = f"{V}/followup/followup.json"
SIZES = (24, 28, 32)


def git(*a):
    return subprocess.run(["git", "-C", WT, *a], check=True, capture_output=True, text=True).stdout


def sha(p):
    return hashlib.sha256(open(p, "rb").read()).hexdigest()


def close(x, y, nd):
    """x (as printed with nd decimals) equals y rounded to nd decimals."""
    return abs(x - round(y, nd)) < 0.5 * 10 ** (-nd) + 1e-12


res = {"snapshot": SNAP1}

# ---------------------------------------------------------------- 1. snapshot
res["head"] = git("rev-parse", "HEAD").strip()
res["remote_tip"] = git("ls-remote", "origin", "claude/ic-dg-blocked-20261029").split()[0]
changed = git("diff", "--name-status", "--diff-filter=MDRT", SNAP0, SNAP1).split("\n")
res["files_modified_or_deleted_since_bf899979a6"] = [x for x in changed if x]
res["files_added_since_bf899979a6"] = len([x for x in git("diff", "--name-status", "--diff-filter=A", SNAP0, SNAP1).split("\n") if x])
res["working_tree_evidence_dir_clean"] = subprocess.run(["git", "-C", WT, "diff", "--quiet", SNAP1, "--", EVREL]).returncode == 0
res["untracked_in_evidence_dir"] = [x for x in git("status", "--short", "--", EVREL).split("\n") if x]
vs = subprocess.run(["sha256sum", "-c", "--quiet", "SHA256SUMS"], cwd=f"{EV}/validation", capture_output=True, text=True)
res["validation_dir_matches_my_SHA256SUMS"] = vs.returncode == 0
res["validation_SHA256SUMS_equals_mine"] = sha(f"{EV}/validation/SHA256SUMS") == sha(f"{V}/SHA256SUMS")
res["VALIDATION_json_equals_my_draft"] = sha(f"{EV}/VALIDATION.json") == sha(f"{V}/VALIDATION.draft.json")
# artifact-store files of validation/ and figures/ hydrated and equal to the committed manifests
man_ok = {}
for m in sorted(x for x in git("ls-tree", "-r", "--name-only", SNAP1, EVREL + "/").split() if x.endswith(".artifacts.json")):
    d = json.loads(git("show", f"{SNAP1}:{m}"))
    for fn, ent in d["files"].items():
        p = f"{WT}/{os.path.dirname(m)}/{fn}"
        man_ok[f"{os.path.dirname(m)[len(EVREL) + 1:]}/{fn}"] = os.path.isfile(p) and sha(p) == ent["sha256"] and os.path.getsize(p) == ent["bytes"]
res["artifact_manifest_files"] = len(man_ok)
res["artifact_manifest_mismatches"] = [k for k, v in man_ok.items() if not v]

# ---------------------------------------------------------------- 2. launch/
launch = {}
for b in SIZES:
    for s in ("wall", "counted"):
        k = f"P{b}-{s}"
        d = f"{EV}/P{b}/{s}/launch"
        rq = json.load(open(f"{d}/receipts/request.json"))
        rs = json.load(open(f"{d}/receipts/result.json"))
        bj = json.load(open(f"{d}/receipts/build.json"))["builds"][0]
        bs = json.load(open(f"{d}/receipts/build-start.json"))["build"]
        sr = json.load(open(f"{d}/receipts/start-request.json"))
        pre = json.load(open(glob.glob(f"{d}/request-P*.json")[0]))
        log = open(glob.glob(f"{d}/launch-P*.log")[0]).read()
        fz = json.load(open(f"{V}/frozen/spec-P{b}.json" if s == "wall" else f"{V}/frozen/spec-P{b}-rho.json"))
        lid = open(f"{S3L}/{k}/PREFIX").read().strip().split("/")[-1]
        rsha = sha(f"{d}/receipts/request.json")
        launch[k] = {
            "tracked_at_313ec5d2af": all(f"{EVREL}/P{b}/{s}/launch/{x}" in git("ls-tree", "-r", "--name-only", SNAP1, f"{EVREL}/P{b}/{s}/launch/") for x in ("receipts/request.json", "receipts/result.json")),
            "request_equals_s3": rsha == sha(f"{S3L}/{k}/request.json"),
            "result_equals_s3": sha(f"{d}/receipts/result.json") == sha(f"{S3L}/{k}/result.json"),
            "producer_request_spec_equals_frozen": pre["job"]["spec"] == fz,
            "producer_request_equals_receipt_except_memory": {x: v for x, v in pre["job"].items() if x != "memory"} == {x: v for x, v in rq["job"].items() if x != "memory"} and pre.get("provenance") == rq.get("provenance"),
            "codebuild_id_is_s3_launcher_id": bj["id"].endswith(":" + lid) and bs["id"] == bj["id"],
            "start_token_equals_request_sha256": sr["idempotencyToken"] == rsha == rs["request_sha256"],
            "codebuild_status": bj["buildStatus"], "launcher_state": rs["state"], "launcher_exit": rs["exit_code"],
            "codebuild_status_consistent": (bj["buildStatus"], rs["state"]) in {("SUCCEEDED", "succeeded"), ("FAILED", "failed")},
            "log_names_launcher_and_state": lid in log and rs["state"] in log,
        }
res["launch"] = launch
res["launch_all_hold"] = all(all(v for x, v in L.items() if isinstance(v, bool)) for L in launch.values())

# ---------------------------------------------------------------- 3. README numbers
R = json.load(open(f"{V}/work/recompute.json"))["sizes"]
readme = open(f"{EV}/README.md").read()
claims = []


def claim(name, shown, value, nd, src):
    claims.append({"claim": name, "readme": shown, "validator_value": value, "decimals": nd, "source": src,
                   "agrees": close(shown, value, nd)})


dec = {24: (79.34, 72.03, 1.0004, 0.9075, 0.9045, 0.9105), 28: (73.83, 68.66, 1.0001, 0.9300, 0.9266, 0.9333),
       32: (90.32, 81.07, 1.0001, 0.8975, 0.8939, 0.9011)}
for b, (na, nb, aa, c, lo, hi) in dec.items():
    d = R[f"P{b}"]["recompute"]["decision_dg__dgb"]
    claim(f"P{b} dg ns/GAE", na, d["ns_per_gae_a"], 2, "recompute decision")
    claim(f"P{b} dgb ns/GAE", nb, d["ns_per_gae_b"], 2, "recompute decision")
    claim(f"P{b} A/A ratio", aa, d["aa_ratio"], 4, "recompute decision")
    claim(f"P{b} corrected", c, d["corrected_ratio"], 4, "recompute decision")
    claim(f"P{b} corrected lo", lo, d["corrected_ci95"][0], 4, "recompute decision")
    claim(f"P{b} corrected hi", hi, d["corrected_ci95"][1], 4, "recompute decision")
d28 = R["P28"]["recompute"]["decision_dg__dgb"]
claim("P28 A/A lo", 0.9991, d28["aa_ci95"][0], 4, "recompute decision")
claim("P28 A/A hi", 1.0012, d28["aa_ci95"][1], 4, "recompute decision")
g28 = R["P28"]["wall_gate"]
res["P28_executions_all_L2"] = g28["5_gate"]["levels_recomputed"] == {"L2": 1344}
res["P28_native_aa"] = {k: g28["6_native_aa"][k] for k in ("aa_selection", "aa_pairs", "status")}

# edge control, A/A-corrected the way the committed compare file does (dg-aa / dg)
edge = {}
for b in SIZES:
    recs = dv.records(b, "wall")
    out, _ = dv.compare_wall(recs, "dg", "dgb-edge", aa=("dg", "dg-aa"))
    com = json.load(open(f"{EV}/P{b}/wall/analysis/compare-dg__dgb-edge-w0.json"))["wall"]
    edge[f"P{b}"] = {"corrected": out["corrected_ratio"], "ci95": out["corrected_ci95"],
                     "committed_corrected": com["corrected_ratio"], "committed_ci95": com["corrected_ci95"],
                     "committed_aa_arms": com.get("aa_arms"),
                     "rel_diff": abs(out["corrected_ratio"] - com["corrected_ratio"]) / com["corrected_ratio"]}
res["edge_corrected"] = edge
claim("P28 edge corrected", 0.9302, edge["P28"]["corrected"], 4, "own compare_wall dg vs dgb-edge, A/A dg-aa/dg")
claim("P28 edge lo", 0.9268, edge["P28"]["ci95"][0], 4, "own")
claim("P28 edge hi", 0.9336, edge["P28"]["ci95"][1], 4, "own")
claim("P24 edge corrected", 0.9077, edge["P24"]["corrected"], 4, "own")
claim("P32 edge corrected", 0.8977, edge["P32"]["corrected"], 4, "own")

# phase sums (ms, 0 decimals) and PDP rises
PH = {24: (262, 156, 228, 239, 744, 752, 1084, 984), 28: (1769, 1000, 1062, 1428, 3800, 4161, 5766, 5363),
      32: (13627, 6150, 10998, 13353, 36367, 38616, 50848, 45639)}
cf = {}
for b, vals in PH.items():
    x, y = R[f"P{b}"]["phase_sums_wall"]["dg"], R[f"P{b}"]["phase_sums_wall"]["dgb"]
    got = (x["phase_wall_ms"]["oracle_setup"], y["phase_wall_ms"]["oracle_setup"], x["online_phases_ms"]["target_PDP"],
           y["online_phases_ms"]["target_PDP"], x["phase_wall_ms"]["relations"], y["phase_wall_ms"]["relations"],
           x["solve_wall_ms"], y["solve_wall_ms"])
    for nm, sh, v in zip(("setup dg", "setup dgb", "PDP dg", "PDP dgb", "relations dg", "relations dgb", "solve dg", "solve dgb"), vals, got):
        claim(f"P{b} phase {nm} ms", sh, v, 0, "recompute phase sums")
    rise = got[3] / got[2] - 1
    # Counterfactual (descriptive): ratio of summed solve wall if dgb's target-PDP phase had kept dg's
    # time, and if additionally the set-up had fallen to 0.5 of dg's (the top of the predicted range).
    # The 288 measured runs are all admitted at 28 bits, where sum-solve ratio = the decision ratio.
    no_pdp = (got[7] - (got[3] - got[2])) / got[6]
    no_pdp_setup05 = (got[6] - 0.5 * got[0]) / got[6]
    cf[f"P{b}"] = {"pdp_rise": rise, "setup_ratio": got[1] / got[0], "solve_ratio": got[7] / got[6],
                   "ratio_if_pdp_unchanged": no_pdp, "ratio_if_pdp_unchanged_and_setup_0.5": no_pdp_setup05,
                   "setup_share_of_dg_solve": got[0] / got[6]}
res["phase_counterfactual"] = cf
for b, pct in ((24, 5), (28, 34), (32, 21)):
    claim(f"P{b} PDP rise %", pct, 100 * cf[f"P{b}"]["pdp_rise"], 0, "recompute phase sums")
claim("P28 set-up ratio", 0.565, cf["P28"]["setup_ratio"], 3, "recompute phase sums")

# counted pooled band: README vs committed compare ops block vs own point estimate + own cluster bootstrap
CB = {24: {"0": (12.96, 11.97, 14.08), "0.1": (12.45, 11.48, 13.53), "1": (10.03, 9.22, 10.96)},
      28: {"0": (21.79, 20.12, 23.69), "0.1": (20.66, 19.07, 22.46), "1": (15.48, 14.25, 16.88)},
      32: {"0": (33.07, 30.36, 36.17), "0.1": (31.49, 28.90, 34.44), "1": (24.35, 22.34, 26.66)}}
band = {}
rng = np.random.default_rng(2026102999)
for b in SIZES:
    recs = dv.records(b, "counted")
    meas = [r for r in recs if not r["warmup"] and r["outcome"]["status"] == "verified"]
    by = {}
    for r in meas:
        if r["arm"] in ("dgb", "rho-neg"):
            by.setdefault((r["workload"]["workload_id"], r["round"]), {})[r["arm"]] = r
    keys = sorted(k for k, v in by.items() if len(v) == 2)
    curve_of = {k: by[k]["dgb"]["workload"]["curve"]["slug"] for k in keys}
    band[f"P{b}"] = {}
    for w in ("0", "0.1", "1"):
        sb = np.array([dv.priced_s(by[k]["dgb"], float(w)) for k in keys])
        sr = np.array([dv.priced_s(by[k]["rho-neg"], float(w)) for k in keys])
        point = sb.sum() / sr.sum()
        # own cluster bootstrap: resample targets (workloads) within each curve, 20,000 draws
        wl = sorted({k[0] for k in keys})
        idx = {u: [i for i, k in enumerate(keys) if k[0] == u] for u in wl}
        strata = {}
        for u in wl:
            strata.setdefault(curve_of[keys[idx[u][0]]], []).append(u)
        sums = {u: (sb[idx[u]].sum(), sr[idx[u]].sum()) for u in wl}
        draws = []
        for _ in range(20000):
            nb = nr = 0.0
            for us in strata.values():
                for u in rng.choice(len(us), size=len(us), replace=True):
                    nb += sums[us[u]][0]
                    nr += sums[us[u]][1]
            draws.append(nb / nr)
        lo, hi = np.percentile(draws, [2.5, 97.5])
        # unstratified: resample the pooled workloads (targets of both curves) together
        sbw = np.array([sums[u][0] for u in wl])
        srw = np.array([sums[u][1] for u in wl])
        pick = rng.integers(0, len(wl), size=(20000, len(wl)))
        ud = sbw[pick].sum(axis=1) / srw[pick].sum(axis=1)
        ulo, uhi = np.percentile(ud, [2.5, 97.5])
        com = json.load(open(f"{EV}/P{b}/counted/analysis/compare-rho-neg__dgb-w{w}.json"))["ops"]
        rd, rlo, rhi = CB[b][w]
        band[f"P{b}"][w] = {"pairs": len(keys), "workloads": len(wl), "own_point": point,
                            "own_bootstrap_stratified_by_curve_ci95": [lo, hi], "own_bootstrap_unstratified_ci95": [ulo, uhi],
                            "committed_ratio": com.get("ratio_b_over_a"), "committed_ci95_linearised_t": com.get("ci95"),
                            "committed_ci95_bootstrap": com.get("ci95_bootstrap"), "committed_bootstrap_method": com.get("ci_bootstrap_method"),
                            "committed_resamples": com.get("resamples"), "readme": [rd, rlo, rhi]}
        claim(f"P{b} counted w={w} ratio", rd, point, 2, "own priced S, pooled")
        claim(f"P{b} counted w={w} lo", rlo, com["ci95_bootstrap"][0], 2, "committed compare ops.ci95_bootstrap (cluster, 4000)")
        claim(f"P{b} counted w={w} hi", rhi, com["ci95_bootstrap"][1], 2, "committed compare ops.ci95_bootstrap (cluster, 4000)")
        band[f"P{b}"][w]["readme_ci_contains_own_bootstraps"] = bool(rlo <= min(lo, ulo) and rhi >= max(hi, uhi))
res["counted_band"] = band

# validation-summary sentences: hashes cited in the README
res["readme_cites_report_sha"] = "9e5e1d34" in readme and sha(f"{EV}/validation/validator-report.json").startswith("9e5e1d34")
res["readme_cites_cert_sha"] = "d75487e8" in readme and sha(f"{EV}/validation/replay-certificate.json").startswith("d75487e8")
res["card_hashes"] = {c: sha(f"{EV}/cards/{c}") for c in ("card-launch.md", "card-analysis.md")}
res["readme_card_hashes_agree"] = (res["card_hashes"]["card-launch.md"].startswith("edc0da03") and res["card_hashes"]["card-launch.md"].endswith("7ccd")
                                   and res["card_hashes"]["card-analysis.md"].startswith("64754a62") and res["card_hashes"]["card-analysis.md"].endswith("cfda"))
appr = git("show", f"{SNAP1}:backlog/XP-20261029-dg-blocked/APPROVAL.md")
res["approval_E1_E10"] = "edits: [E1, E2, E3, E4, E5, E6, E7, E8, E9, E10]" in appr
res["approval_residual_risk_reported_not_rerun"] = "It would be reported and not rerun." in appr
res["approval_E6_probe_risk"] = "probe cost taken as equal" in appr
lin = {r["method"]["params"].get("linalg") for r in dv.records(28, "wall") if r["method"]["id"] == "ic.pipeline"}
res["linalg_values"] = sorted(lin)
# PROTOCOL section 10: "The README's position table gives, per size, each arm's block positions
# (from records.jsonl) and the position-control figures of section 6.2."
tables = [ln for ln in readme.split("\n") if ln.startswith("|") and re.search(r"position", ln, re.I)]
res["readme_position_table_rows"] = tables
res["readme_has_position_table"] = bool(tables)
res["readme_has_position_control_figures"] = all(s in readme for s in ("0.9302 [0.9268, 0.9336]", "0.9077", "0.8977"))
# The causal sentence in "Where the time went" against the phase counterfactual
sent = "The probe cost gives back part of the set-up gain, which is why every ratio missed its prediction."
pred_hi = {24: 0.87, 28: 0.84, 32: 0.85}
res["readme_causal_sentence"] = {
    "text": sent, "present": sent in readme,
    "misses_remain_with_pdp_unchanged": {f"P{b}": cf[f"P{b}"]["ratio_if_pdp_unchanged"] > pred_hi[b] for b in SIZES},
    "setup_ratio_above_predicted_0.5": {f"P{b}": cf[f"P{b}"]["setup_ratio"] > 0.5 for b in SIZES},
    "reading": ("not supported as stated: with the target-PDP phase held at dg's time the summed-solve ratios would be "
                + ", ".join(f"{b}: {cf[f'P{b}']['ratio_if_pdp_unchanged']:.3f}" for b in SIZES)
                + ", still above every predicted range (upper ends 0.87, 0.84, 0.85); at 24 and 28 bits the set-up fell only to "
                + f"{cf['P24']['setup_ratio']:.3f} and {cf['P28']['setup_ratio']:.3f} (predicted 0.25-0.5), so the smaller set-up saving "
                "and the slower target-PDP phase together explain the misses; only at 32 bits (set-up 0.451) is the PDP rise nearly the whole miss")}
res["claims"] = claims
res["claims_disagreeing"] = [c for c in claims if not c["agrees"]]

# ---------------------------------------------------------------- 4. figure re-render
pc = "/dev/shm/dgb-val/plotcheck"
shutil.rmtree(pc, ignore_errors=True)
os.makedirs(f"{pc}/figures")
for b in SIZES:
    os.makedirs(f"{pc}/P{b}/wall/analysis")
    for fn in ("compare-dg__dgb-w0.json", "compare-dg__dgb-edge-w0.json", "phase_sums.json"):
        shutil.copyfile(f"{EV}/P{b}/wall/analysis/{fn}", f"{pc}/P{b}/wall/analysis/{fn}")
shutil.copyfile(f"{EV}/figures/plot.py", f"/dev/shm/dgb-val/plot.py")
rr = subprocess.run([sys.executable, "-I", "/dev/shm/dgb-val/plot.py"], cwd=pc, capture_output=True, text=True)


def svg_norm(p):
    t = open(p).read()
    t = re.sub(r"<dc:date>[^<]*</dc:date>", "", t)
    ids = {}
    for m in re.finditer(r'id="([a-z]+[0-9a-f]{6,})"', t):
        ids.setdefault(m.group(1), f"ID{len(ids)}")
    for k, v in ids.items():
        t = t.replace(k, v)
    return t


res["figure"] = {"plot_rc": rr.returncode,
                 "png_rerender_equals_committed": rr.returncode == 0 and sha(f"{pc}/figures/ratios.png") == sha(f"{EV}/figures/ratios.png"),
                 "svg_rerender_equal_modulo_ids_and_date": rr.returncode == 0 and svg_norm(f"{pc}/figures/ratios.svg") == svg_norm(f"{EV}/figures/ratios.svg"),
                 "png_sha256": sha(f"{EV}/figures/ratios.png"), "svg_sha256": sha(f"{EV}/figures/ratios.svg"),
                 "benchmarks_png_equal": sha(f"{WT}/benchmarks/20261029-ic-dg-blocked/ratios.png") == sha(f"{EV}/figures/ratios.png"),
                 "benchmarks_svg_equal": sha(f"{WT}/benchmarks/20261029-ic-dg-blocked/ratios.svg") == sha(f"{EV}/figures/ratios.svg"),
                 "plots_from": "committed compare-dg__dgb-w0.json, compare-dg__dgb-edge-w0.json (wall block) and phase_sums.json, all equal to my recompute"}
shutil.rmtree(pc, ignore_errors=True)

# ---------------------------------------------------------------- 5. PDF
txt = subprocess.run(["pdftotext", "-layout", f"{EV}/README.pdf", "-"], capture_output=True, text=True).stdout
nums = set(re.findall(r"\d+\.\d+", readme))
imgs = subprocess.run(["pdfimages", "-list", f"{EV}/README.pdf"], capture_output=True, text=True).stdout.strip().split("\n")[2:]
res["pdf"] = {"sha256": sha(f"{EV}/README.pdf"),
              "benchmarks_REPORT_pdf_equal": sha(f"{WT}/benchmarks/20261029-ic-dg-blocked/REPORT.pdf") == sha(f"{EV}/README.pdf"),
              "readme_decimal_numbers": len(nums), "missing_in_pdf_text": sorted(n for n in nums if n not in txt),
              "embedded_images": len([x for x in imgs if x.strip()])}

os.makedirs(os.path.dirname(OUT), exist_ok=True)
json.dump(res, open(OUT, "w"), indent=1, sort_keys=True, default=float)
print(json.dumps({k: res[k] for k in ("head", "remote_tip", "files_modified_or_deleted_since_bf899979a6", "files_added_since_bf899979a6",
                                      "working_tree_evidence_dir_clean", "untracked_in_evidence_dir", "validation_dir_matches_my_SHA256SUMS",
                                      "validation_SHA256SUMS_equals_mine", "VALIDATION_json_equals_my_draft", "artifact_manifest_files",
                                      "artifact_manifest_mismatches", "launch_all_hold", "P28_executions_all_L2", "P28_native_aa",
                                      "readme_cites_report_sha", "readme_cites_cert_sha", "readme_card_hashes_agree", "approval_E1_E10",
                                      "approval_residual_risk_reported_not_rerun", "approval_E6_probe_risk", "linalg_values",
                                      "figure", "pdf")}, indent=1, default=float))
print("claims:", len(claims), "disagreeing:", json.dumps(res["claims_disagreeing"], indent=1, default=float))
print("edge:", json.dumps(edge, default=float))
print("counterfactual:", json.dumps(cf, indent=1, default=float))
print("band:", json.dumps(band, default=float))
