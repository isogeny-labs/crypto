#!/usr/bin/env python3
"""Validator checks for research/quartic_diem_nofb_20261126 beyond replay, certificates and
the section-5 recompute: manifests, freeze order, binary-to-source binding, analyze.py byte
reproduction, structural identities, the section-5a field audit, documents and bench record,
and the seed manifest.

Usage:
  python3 -I checks_extra.py REPO SHA SNAP BIN REPLAY_DIR RECOMPUTE.json OUT.json

  REPO       repository root (git is read with `git -C REPO show SHA:...`, never the worktree,
             except for the cargo build directory REPO/target/release, read-only)
  SHA        the validated commit (cc7e5c5488a)
  SNAP       `git archive SHA research/quartic_diem_nofb_20261126 research/quartic_diem_la_20261123/raw`
             extracted (analyze.py is run there, read-only inputs)
  BIN        the binary copied at dispatch
  REPLAY_DIR the replay working directory (REPLAY_DIR/raw/run*.jsonl), or - to skip
Every record key is read with [] (lesson L-99ee952350b2).
"""
import datetime
import hashlib
import json
import math
import os
import re
import subprocess
import sys

STUDY = "research/quartic_diem_nofb_20261126"
FREEZE = "8297532db36"
SRC_COMMIT = "31a1e2f53ef"
DEV_COMMIT = "8bc5537a844"
DRIVER_COMMIT = "a8fa56ebb34"
SGE, NOFB = "Q_diem_2lp_sfb_sge", "Q_diem_2lp_sfb_sge_nofb"


def git(repo, *args, text=True):
    return subprocess.run(["git", "-C", repo, *args], check=True, capture_output=True, text=text).stdout


def show(repo, sha, path):
    return subprocess.run(["git", "-C", repo, "show", f"{sha}:{path}"], check=True,
                          capture_output=True).stdout


def sha256b(b):
    return hashlib.sha256(b).hexdigest()


def sha256f(p):
    with open(p, "rb") as fh:
        return sha256b(fh.read())


def jl_bytes(b):
    return [json.loads(l) for l in b.decode().splitlines() if l.strip()]


def ts(s):
    return datetime.datetime.fromisoformat(s.strip().replace("Z", "+00:00"))


def birth(path):
    out = subprocess.run(["stat", "-c", "%W %Y", path], capture_output=True, text=True).stdout.split()
    return {"birth_utc": datetime.datetime.fromtimestamp(int(out[0]), datetime.UTC).isoformat() if out and out[0] != "0" else None,
            "mtime_utc": datetime.datetime.fromtimestamp(int(out[1]), datetime.UTC).isoformat() if out else None}


def main():
    repo, sha, snap, binp, replay, recp, outp = sys.argv[1:8]
    st = f"{snap}/{STUDY}"
    out = {"schema": "validator-checks-extra/v1", "sha": git(repo, "rev-parse", sha).strip()}

    # ---------- A. manifests ----------
    man = {}
    for mf in ("frozen.sha256", "data.sha256"):
        res = {}
        for line in show(repo, sha, f"{STUDY}/{mf}").decode().splitlines():
            h, p = line.split(None, 1)
            res[p] = sha256b(show(repo, sha, f"{STUDY}/{p}")) == h
        man[mf] = {"entries": res, "ok": all(res.values())}
    bh = show(repo, sha, f"{STUDY}/binary.sha256").decode().split()[0]
    dbh = show(repo, sha, f"{STUDY}/dev/binary.sha256").decode().split()[0]
    man["binary"] = {"binary.sha256": bh, "dev/binary.sha256": dbh, "bound_copy": sha256f(binp),
                     "ok": bh == dbh == sha256f(binp)}
    src = {}
    for line in show(repo, sha, f"{STUDY}/source.sha256").decode().splitlines():
        h, p = line.split(None, 1)
        src[p] = {c: sha256b(show(repo, c, p)) == h for c in (SRC_COMMIT, FREEZE, sha)}
    man["source.sha256"] = {"entries": src, "ok": all(all(v.values()) for v in src.values())}
    out["manifests"] = man

    # ---------- B. freeze order ----------
    def cdate(c):
        return git(repo, "log", "-1", "--format=%cI", c).strip()
    commits = {c: cdate(c) for c in (SRC_COMMIT, DEV_COMMIT, FREEZE, DRIVER_COMMIT, sha)}
    stamps = {f: show(repo, sha, f"{STUDY}/raw/{f}").decode().strip()
              for f in ("main_start.txt", "main_end.txt", "r131101_start.txt", "r131101_end.txt")}
    done = {}
    for f in ("run.jsonl", "run131101.jsonl"):
        d = [r for r in jl_bytes(show(repo, sha, f"{STUDY}/raw/{f}")) if r["kind"] == "done"]
        done[f] = d[0]["wall_s"]
    raw_commits = git(repo, "log", "--format=%h", f"{sha}", "--", f"{STUDY}/raw/").split()
    frozen_files = [l.split(None, 1)[1] for l in show(repo, sha, f"{STUDY}/frozen.sha256").decode().splitlines()]
    frozen_unchanged = subprocess.run(["git", "-C", repo, "diff", "--quiet", FREEZE, sha, "--"]
                                      + [f"{STUDY}/{p}" for p in frozen_files]).returncode == 0
    # the decision rule and predictions: PROTOCOL sections 4-5 at the draft vs the freeze
    draft = show(repo, SRC_COMMIT, f"{STUDY}/PROTOCOL.md").decode()
    froz = show(repo, FREEZE, f"{STUDY}/PROTOCOL.md").decode()

    def sec(t, a, b):
        return t[t.index(a):t.index(b)]
    s4_same = sec(draft, "## 4.", "## 5.") == sec(froz, "## 4.", "## 5.")
    dec_d = sec(draft, "- **Decision, on the primary panel.**", "## 6.")
    dec_f = sec(froz, "- **Decision, on the primary panel.**", "## 6.")
    t_main = (ts(stamps["main_end.txt"]) - ts(stamps["main_start.txt"])).total_seconds()
    t_131 = (ts(stamps["r131101_end.txt"]) - ts(stamps["r131101_start.txt"])).total_seconds()
    out["freeze_order"] = {
        "commit_dates": commits, "raw_stamps": stamps, "done_wall_s": done,
        "stamp_intervals_s": {"main": t_main, "131101": t_131},
        "done_vs_stamps_abs_diff_s": {"main": abs(t_main - done["run.jsonl"]),
                                      "131101": abs(t_131 - done["run131101.jsonl"])},
        "freeze_before_main_start_s": (ts(stamps["main_start.txt"]) - ts(commits[FREEZE])).total_seconds(),
        "dev_smoke_after_draft_s": (ts(commits[DEV_COMMIT]) - ts(commits[SRC_COMMIT])).total_seconds(),
        "driver_committed_after_main_start_s": (ts(commits[DRIVER_COMMIT]) - ts(stamps["main_start.txt"])).total_seconds(),
        "commits_touching_raw": raw_commits,
        "frozen_files_unchanged_freeze_to_sha": frozen_unchanged,
        "protocol_section4_predictions_unchanged_draft_to_freeze": s4_same,
        "protocol_decision_rule_unchanged_draft_to_freeze": dec_d == dec_f,
        "analyze_py_hash": {c: sha256b(show(repo, c, f"{STUDY}/analyze.py")) for c in (SRC_COMMIT, FREEZE, sha)},
        "worktree_birth_times": {f: birth(f"{repo}/{STUDY}/raw/{f}") for f in ("run.jsonl", "run131101.jsonl")},
        "ok": (raw_commits == [git(repo, "rev-parse", "--short=11", sha).strip()]
               and ts(commits[FREEZE]) < ts(stamps["main_start.txt"]) and frozen_unchanged and s4_same
               and dec_d == dec_f),
    }

    # ---------- C. binary-to-source binding through cargo's fingerprints ----------
    bb = {"available": False}
    fp = f"{repo}/target/release/.fingerprint"
    try:
        exe = f"{repo}/target/release/genus_ladder"
        deps = f"{repo}/target/release/deps"
        bins = [f for f in os.listdir(deps) if f.startswith("genus_ladder-") and not f.endswith(".d")]
        match = [f for f in bins if sha256f(f"{deps}/{f}") == bh]
        bb = {"available": True, "target_release_genus_ladder_sha256": sha256f(exe), "deps_matching_binary": match}
        if match:
            hb = match[0].split("-", 1)[1]
            inv_bin = os.stat(f"{fp}/crypto-{hb}/invoked.timestamp").st_mtime
            # the lib the bin linked: the newest lib-crypto_lib fingerprint finished before the bin was invoked
            libs = []
            for d in os.listdir(fp):
                if d.startswith("crypto-") and os.path.exists(f"{fp}/{d}/lib-crypto_lib"):
                    libs.append((os.stat(f"{fp}/{d}/lib-crypto_lib").st_mtime, d))
            libs = sorted(x for x in libs if x[0] <= inv_bin + 1e-3)
            lib = libs[-1][1]
            lh = lib.split("-", 1)[1]
            inv_lib = os.stat(f"{fp}/{lib}/invoked.timestamp").st_mtime

            def depfiles(p):
                txt = open(p).read().split("\n\n")[0]
                _, d = txt.split(":", 1)
                return [x.replace("\\ ", " ") for x in re.split(r"(?<!\\) ", d.replace("\\\n", " ").strip()) if x]

            res = {}
            for name, dfile, inv in (("lib", f"{deps}/crypto_lib-{lh}.d", inv_lib),
                                     ("bin", f"{deps}/genus_ladder-{hb}.d", inv_bin)):
                fs = [f[len(repo) + 1:] if f.startswith(repo + "/") else f for f in depfiles(dfile)]
                outside = [f for f in fs if f.startswith("/")]
                late = [f for f in fs if not f.startswith("/") and os.stat(f"{repo}/{f}").st_mtime >= inv]
                newest = max(fs, key=lambda f: os.stat(f if f.startswith("/") else f"{repo}/{f}").st_mtime)
                # content at the build commit, per file, against the working tree
                differ = subprocess.run(["git", "-C", repo, "diff", "--name-only", SRC_COMMIT, "--"]
                                        + [f for f in fs if not f.startswith("/")],
                                        capture_output=True, text=True, check=True).stdout.split()
                tracked = subprocess.run(["git", "-C", repo, "ls-files", "--error-unmatch", "--"]
                                         + [f for f in fs if not f.startswith("/")],
                                         capture_output=True, text=True).returncode == 0
                res[name] = {"fingerprint": f"crypto-{lh if name == 'lib' else hb}",
                             "invoked_utc": datetime.datetime.fromtimestamp(inv, datetime.UTC).isoformat(),
                             "n_source_files": len(fs), "outside_repo": outside,
                             "modified_at_or_after_invocation": late,
                             "newest_source": newest,
                             "newest_source_mtime_utc": datetime.datetime.fromtimestamp(
                                 os.stat(f"{repo}/{newest}").st_mtime, datetime.UTC).isoformat(),
                             "all_tracked": tracked, "worktree_differs_from_build_commit": differ}
            bb["dep_info"] = res
            bb["cargo_lock_toml_equal_build_commit"] = subprocess.run(
                ["git", "-C", repo, "diff", "--quiet", SRC_COMMIT, sha, "--", "Cargo.lock", "Cargo.toml"]).returncode == 0
            bb["src_equal_build_commit_to_sha"] = subprocess.run(
                ["git", "-C", repo, "diff", "--quiet", SRC_COMMIT, sha, "--", "src", "crates"]).returncode == 0
            bb["exe_stat"] = birth(exe)
            bb["bin_fingerprint_finished_utc"] = datetime.datetime.fromtimestamp(
                os.stat(f"{fp}/crypto-{hb}/bin-genus_ladder").st_mtime, datetime.UTC).isoformat()
            bb["ok"] = all(not v["modified_at_or_after_invocation"] and not v["outside_repo"] and v["all_tracked"]
                           and not v["worktree_differs_from_build_commit"] for v in res.values()) \
                and bb["cargo_lock_toml_equal_build_commit"] and bb["src_equal_build_commit_to_sha"]
    except FileNotFoundError as e:
        bb["error"] = str(e)
    out["binary_binding"] = bb

    # ---------- D. analyze.py byte reproduction ----------
    rep = {}
    committed_analysis = show(repo, sha, f"{STUDY}/analysis.json")
    for label, rawdir in (("committed_raw", f"{st}/raw"), ("replay_raw", f"{replay}/raw" if replay != "-" else None)):
        if rawdir is None or not os.path.exists(f"{rawdir}/run131101.jsonl"):
            rep[label] = None
            continue
        r = subprocess.run([sys.executable, "-I", f"{st}/analyze.py", f"{rawdir}/run.jsonl",
                            f"{rawdir}/run131101.jsonl"], cwd=st, capture_output=True, check=True).stdout
        rep[label] = {"sha256": sha256b(r), "byte_identical": r == committed_analysis}
    rep["committed_sha256"] = sha256b(committed_analysis)
    out["analyze_py_reproduction"] = rep

    # ---------- E. structural identities on the rows ----------
    rows = jl_bytes(show(repo, sha, f"{STUDY}/raw/run.jsonl")) + jl_bytes(show(repo, sha, f"{STUDY}/raw/run131101.jsonl"))
    runs = [r for r in rows if r["kind"] == "run"]
    single = [r for r in runs if r["attempts"] == 1]
    cw = [r for r in single if r["la_spmv_add"] != (2 * r["sq_dim"] + 2) * r["sq_nnz"]]
    sel = [r for r in single if r["sge_rows_in"] != min(r["core_rows"], r["core_cols"] + 64)
           or r["sq_dim"] != r["sge_cols_out"]]
    out["structural_identities"] = {
        "single_attempt_runs": len(single),
        "coord_wiedemann_spmv_closed_form_violations": [[r["arm"], r["p"], r["curve_seed"], r["seed"]] for r in cw],
        "trim_and_square_dims_violations": [[r["arm"], r["p"], r["curve_seed"], r["seed"]] for r in sel],
        "ok": not cw and not sel,
    }

    # ---------- F. section 5a: every counted integer field accounted for ----------
    summed = {"rel_mul", "rel_add", "la_mul", "la_add", "idx_lookups", "idx_inserts", "group_ops"}
    la_sub = {f"la_{s}_{o}" for s in ("rows", "dsu", "peel", "tscan", "trim", "sge", "spmv", "bm", "vec")
              for o in ("add", "mul")}
    rel_sub = {"rel_enum_mul", "rel_enum_add", "rel_decomp_mul", "rel_decomp_add"}
    inside_group = {"cantor_mul", "cantor_inv"}
    sizes = {"p", "g", "h", "r", "order", "n_points", "curve_seed", "seed", "t_secret", "recovered", "attempts",
             "relations", "rels_line", "decomp_trials", "decomp_relations", "decomp_fallbacks", "inhom_linked",
             "inhom_rows", "lines_tried", "la_skipped", "la_skipped_attempts", "core_rows", "core_cols", "core_nnz",
             "sq_dim", "sq_nnz", "sge_rows_in", "sge_cols_in", "sge_nnz_in", "sge_rows_out", "sge_cols_out",
             "sge_nnz_out", "fb_cols", "fb_samples", "idx_points", "la_fallbacks", "la_fallbacks_skipped",
             "la_fallback_rescues", "trials", "wall_ns"}
    ints = set()
    for r in runs:
        for k, v in r.items():
            if isinstance(v, int) and not isinstance(v, bool):
                ints.add(k)
    unclassified = sorted(ints - summed - la_sub - rel_sub - inside_group - sizes)
    sub_ok = all(sum(r[f"la_{s}_add"] for s in ("rows", "dsu", "peel", "tscan", "trim", "sge", "spmv", "bm", "vec")) == r["la_add"]
                 and r["rel_enum_mul"] + r["rel_decomp_mul"] <= r["rel_mul"]
                 and r["rel_enum_add"] + r["rel_decomp_add"] <= r["rel_add"] for r in runs)
    out["section5a_fields"] = {"integer_fields": sorted(ints), "unclassified": unclassified,
                               "subsplits_within_summed_counters": sub_ok,
                               "ok": not unclassified and sub_ok}

    # ---------- G. figures, PDF, plot.py inputs ----------
    figs = {"fig_svg_equals_report_copy": sha256b(show(repo, sha, f"{STUDY}/fig_nofb.svg"))
            == sha256b(show(repo, sha, f"{STUDY}/reports/run-20261126T0333Z/fig_nofb.svg"))}
    plot = show(repo, sha, f"{STUDY}/plot.py").decode()
    figs["plot_py_opens"] = re.findall(r"open\(([^)]*)\)", plot)
    # its data input is argv[1] (analysis.json); the other opens are its own outputs
    figs["plot_py_data_inputs"] = [o for o in figs["plot_py_opens"]
                                   if not o.split(",")[0].strip().strip('"') in ("fig_nofb.svg", "fig_nofb.png")]
    figs["plot_py_reads_only_argv1"] = figs["plot_py_data_inputs"] == ["sys.argv[1]"]
    pdf = show(repo, sha, f"{STUDY}/reports/run-20261126T0333Z/report.pdf")
    tmp = f"{os.path.dirname(os.path.abspath(outp))}/.report_pdf_text.tmp"
    try:
        txt = subprocess.run(["pdftotext", "-layout", "-", "-"], input=pdf, capture_output=True, check=True).stdout.decode()
        typ = show(repo, sha, f"{STUDY}/reports/run-20261126T0333Z/report.typ").decode()
        body = "\n".join(l for l in typ.splitlines() if not l.startswith("#set"))  # skip layout directives
        nums_typ = re.findall(r"\d+\.\d+|\d{3,}", body)
        missing = sorted({n for n in nums_typ if n not in txt})
        figs["pdf_numbers_from_typ_missing_in_pdf_text"] = missing
        figs["pdf_text_ok"] = not missing
    except (FileNotFoundError, subprocess.CalledProcessError) as e:
        figs["pdf_text_ok"] = None
        figs["pdf_error"] = str(e)
    if os.path.exists(tmp):
        os.remove(tmp)
    out["figures_and_pdf"] = figs

    # ---------- H. bench record ----------
    a = json.loads(committed_analysis)
    rec = json.loads(show(repo, sha, "benchmarks/records/20261126T0333Z-quartic-diem-nofb.json"))
    m = {x["name"]: x for x in rec["metrics"]}
    pg = a["paired_nofb_over_sge"]["primary_seeds_3_4"]["gae_1.0"]["pooled"]
    ic = a["ic_over_rho"][NOFB]["all_1.0"]["per_prime"]["131101"]
    ics = a["ic_over_rho"][SGE]["all_1.0"]["per_prime"]["131101"]
    out["bench_record"] = {
        "metric_paired": m["paired_gae1_nofb_over_sge_heldout_pooled"]["value"] == pg["gm"]
        and m["paired_gae1_nofb_over_sge_heldout_pooled"]["spread"] == pg["ci95"],
        "metric_icr": m["ic_over_ideal_rho_gae1_p131101_nofb"]["value"] == ic["gm"]
        and m["ic_over_ideal_rho_gae1_p131101_nofb"]["spread"] == ic["ci95"]
        and m["ic_over_ideal_rho_gae1_p131101_nofb"]["baseline"] == ics["gm"],
        "raw_hashes": {x["path"]: x["sha256"] == sha256b(show(repo, sha, x["path"])) for x in rec["raw"]},
        "raw_uploaded": [x["uploaded"] for x in rec["raw"]],
        "commit": rec["commit"], "commit_dirty": rec["commit_dirty"],
        "command": rec["commands"][0]["argv"], "command_started_utc": rec["commands"][0]["started_utc"],
        "summary": rec["summary"],
        "summary_headline_displayed": re.findall(r"0\.8\d\dx", rec["summary"]),
        "summary_headline_correct_rounding": f"{pg['gm']:.3f}x",
    }

    # ---------- I. every displayed number in RESULTS.md and report.typ ----------
    R = show(repo, sha, f"{STUDY}/RESULTS.md").decode().splitlines()
    T = show(repo, sha, f"{STUDY}/reports/run-20261126T0333Z/report.typ").decode().splitlines()
    rc = json.load(open(recp))
    P = a["paired_nofb_over_sge"]
    I = a["ic_over_rho"]

    def pp(pan, met, p=None):
        x = P[pan][met]["pooled"] if p is None else P[pan][met]["per_prime"][str(p)]
        return [x["gm"]] + x["ci95"]

    def icr(arm, w, p):
        x = I[arm][f"all_{w}"]["per_prime"][str(p)]
        return [x["gm"]] + x["ci95"]
    prs = rc["pairs"]
    la_lo, la_hi = prs["la_total_ratio_range"]
    multi = rc["results_figures"]["nofb_multi_attempt_over_single_gae1"]
    # (file, line, text on the line, displayed numbers, values, decimals): the text must be on
    # the line, and each displayed number must equal its value rounded to that many decimals
    E = [
        ("RESULTS.md", 32, "0.833 [0.758, 0.907]", [0.833, 0.758, 0.907], pp("primary_seeds_3_4", "gae_1.0"), 3),
        ("RESULTS.md", 32, "0.826 [0.773, 0.881]", [0.826, 0.773, 0.881], pp("all_seeds_1_4", "gae_1.0"), 3),
        ("RESULTS.md", 33, "0.833 [0.761, 0.908]", [0.833, 0.761, 0.908], pp("primary_seeds_3_4", "gae_0.1"), 3),
        ("RESULTS.md", 33, "0.826 [0.771, 0.880]", [0.826, 0.771, 0.880], pp("all_seeds_1_4", "gae_0.1"), 3),
        ("RESULTS.md", 34, "0.9971 [0.9959, 0.9984]", [0.9971, 0.9959, 0.9984], pp("primary_seeds_3_4", "gae_0.0"), 4),
        ("RESULTS.md", 34, "0.9972 [0.9962, 0.9980]", [0.9972, 0.9962, 0.9980], pp("all_seeds_1_4", "gae_0.0"), 4),
        ("RESULTS.md", 42, "| GAE ratio | 1.000 |", [1.000], [P["all_seeds_1_4"]["gae_1.0"]["per_prime"]["4099"]["gm"]], 3),
        ("RESULTS.md", 42, "0.683 [0.571, 0.783]", [0.683, 0.571, 0.783], pp("all_seeds_1_4", "gae_1.0", 16411), 3),
        ("RESULTS.md", 42, "0.903 [0.814, 1.000]", [0.903, 0.814, 1.000], pp("all_seeds_1_4", "gae_1.0", 65537), 3),
        ("RESULTS.md", 42, "0.755 [0.623, 0.897]", [0.755, 0.623, 0.897], pp("all_seeds_1_4", "gae_1.0", 131101), 3),
        ("RESULTS.md", 43, "| 1.000 | 0.592 | 0.872 | 0.700 |", [1.000, 0.592, 0.872, 0.700],
         [P["all_seeds_1_4"]["la_total"]["per_prime"][str(p)]["gm"] for p in (4099, 16411, 65537, 131101)], 3),
        ("RESULTS.md", 45, "20.2–27.3%", [20.2, 27.3], [100 * la_lo, 100 * la_hi], 1),
        ("RESULTS.md", 51, "20.2–27.3%", [20.2, 27.3], [100 * la_lo, 100 * la_hi], 1),
        ("RESULTS.md", 52, "at 0.833", [0.833], [pg["gm"]], 3),
        ("RESULTS.md", 58, "| 4099 | 0.886 | 0.886 |", [0.886, 0.886], [icr(SGE, 0.1, 4099)[0], icr(NOFB, 0.1, 4099)[0]], 3),
        ("RESULTS.md", 58, "| 8.83 | 8.83 |", [8.83, 8.83], [icr(SGE, 1.0, 4099)[0], icr(NOFB, 1.0, 4099)[0]], 2),
        ("RESULTS.md", 59, "| 16411 | 0.756 |", [0.756], [icr(SGE, 0.1, 16411)[0]], 3),
        ("RESULTS.md", 59, "0.517 [0.472, 0.572]", [0.517, 0.472, 0.572], icr(NOFB, 0.1, 16411), 3),
        ("RESULTS.md", 59, "| 7.55 |", [7.55], [icr(SGE, 1.0, 16411)[0]], 2),
        ("RESULTS.md", 59, "5.16 [4.72, 5.73]", [5.16, 4.72, 5.73], icr(NOFB, 1.0, 16411), 2),
        ("RESULTS.md", 60, "| 65537 | 0.298 |", [0.298], [icr(SGE, 0.1, 65537)[0]], 3),
        ("RESULTS.md", 60, "0.269 [0.264, 0.275]", [0.269, 0.264, 0.275], icr(NOFB, 0.1, 65537), 3),
        ("RESULTS.md", 60, "| 2.98 |", [2.98], [icr(SGE, 1.0, 65537)[0]], 2),
        ("RESULTS.md", 60, "2.69 [2.63, 2.74]", [2.69, 2.63, 2.74], icr(NOFB, 1.0, 65537), 2),
        ("RESULTS.md", 61, "| 131101 | 0.263 |", [0.263], [icr(SGE, 0.1, 131101)[0]], 3),
        ("RESULTS.md", 61, "0.199 [0.184, 0.217]", [0.199, 0.184, 0.217], icr(NOFB, 0.1, 131101), 3),
        ("RESULTS.md", 61, "2.63 [2.04, 3.47]", [2.63, 2.04, 3.47], icr(SGE, 1.0, 131101), 2),
        ("RESULTS.md", 61, "1.98 [1.84, 2.16]", [1.98, 1.84, 2.16], icr(NOFB, 1.0, 131101), 2),
        ("RESULTS.md", 63, "about 2e-4", [2, 2], [icr(SGE, 0.0, 131101)[0] * 1e4, icr(NOFB, 0.0, 131101)[0] * 1e4], 0),
        ("RESULTS.md", 65, "1.98×", [1.98], [icr(NOFB, 1.0, 131101)[0]], 2),
        ("RESULTS.md", 65, "2.63×", [2.63], [icr(SGE, 1.0, 131101)[0]], 2),
        ("report.typ", 20, "[0.833 \\[0.758, 0.907\\]], [0.826 \\[0.773, 0.881\\]]",
         [0.833, 0.758, 0.907, 0.826, 0.773, 0.881], pp("primary_seeds_3_4", "gae_1.0") + pp("all_seeds_1_4", "gae_1.0"), 3),
        ("report.typ", 21, "[0.833 \\[0.761, 0.908\\]], [0.826 \\[0.771, 0.880\\]]",
         [0.833, 0.761, 0.908, 0.826, 0.771, 0.880], pp("primary_seeds_3_4", "gae_0.1") + pp("all_seeds_1_4", "gae_0.1"), 3),
        ("report.typ", 22, "[0.9971 \\[0.9959, 0.9984\\]], [0.9972 \\[0.9962, 0.9980\\]]",
         [0.9971, 0.9959, 0.9984, 0.9972, 0.9962, 0.9980], pp("primary_seeds_3_4", "gae_0.0") + pp("all_seeds_1_4", "gae_0.0"), 4),
        ("report.typ", 24, "20–27%", [20, 27], [100 * la_lo, 100 * la_hi], 0),
        ("report.typ", 31, "[8.83], [7.55], [2.98], [2.63]", [8.83, 7.55, 2.98, 2.63],
         [icr(SGE, 1.0, p)[0] for p in (4099, 16411, 65537, 131101)], 2),
        ("report.typ", 32, "[8.83], [5.16], [2.69], [1.98 \\[1.84, 2.16\\]]", [8.83, 5.16, 2.69, 1.98, 1.84, 2.16],
         [icr(NOFB, 1.0, p)[0] for p in (4099, 16411, 65537)] + icr(NOFB, 1.0, 131101), 2),
        ("report.typ", 34, "1.98×", [1.98], [icr(NOFB, 1.0, 131101)[0]], 2),
    ]
    docs = []
    for f, ln, text, shown, vals, nd in E:
        line = (R if f == "RESULTS.md" else T)[ln - 1]
        present = text in line
        correct = [f"{v:.{nd}f}" for v in vals]
        ok = present and len(shown) == len(vals) and all(f"{s:.{nd}f}" == c for s, c in zip(shown, correct))
        docs.append({"file": f, "line": ln, "text": text, "present_on_line": present,
                     "displayed": [f"{s:.{nd}f}" for s in shown], "values": vals, "decimals": nd,
                     "correct_rounding": correct, "ok": ok})
    out["documents"] = {
        "checked": len(docs), "failures": [d for d in docs if not d["ok"]],
        "all": docs,
        "multi_attempt_cost_claim": {
            "RESULTS.md:69": R[68], "report.typ:36": T[35],
            "measured_nofb_gae1_two_attempt_over_same_curve_single_attempt_gm":
                [min(x["ratio_to_single_gm"] for x in multi if x["attempts"] == 2),
                 max(x["ratio_to_single_gm"] for x in multi if x["attempts"] == 2)],
            "measured_three_attempt": [x["ratio_to_single_gm"] for x in multi if x["attempts"] == 3],
        },
        "analysis_json_rule_text": a["decision"]["rule"],
        "protocol_decision_section": "## 5. Analysis and decision rule" in froz,
    }

    # ---------- J. seed manifest ----------
    seeds = json.loads(show(repo, sha, f"{STUDY}/seeds.json"))
    ql = show(repo, SRC_COMMIT, "src/cryptanalysis/quartic_ladder.rs").decode().splitlines()
    rw = show(repo, SRC_COMMIT, "src/cryptanalysis/residual_walk.rs").decode().splitlines()
    tests_at = next(i for i, l in enumerate(ql) if l.startswith("mod tests"))
    sites = [(i + 1, l.strip()) for i, l in enumerate(ql) if "seed_from_u64" in l and i < tests_at]
    lock = show(repo, SRC_COMMIT, "Cargo.lock").decode()
    out["seed_manifest"] = {
        "schema": seeds["schema"], "master_seed": seeds["master_seed"],
        "has_prng_versions": "rand 0.8.8" in seeds["prng"]["library"] and "rand_chacha 0.3.1" in seeds["prng"]["algorithm"],
        "cargo_lock_rand_0_8_8": 'name = "rand"\nversion = "0.8.8"' in lock,
        "cargo_lock_rand_chacha_0_3_1": 'name = "rand_chacha"\nversion = "0.3.1"' in lock,
        "mix64_at_residual_walk_336": rw[335].startswith("pub fn mix64"),
        "streams_listed": sorted(seeds["streams"]),
        "non_test_seed_sites_quartic_ladder": sites,
        "panel_matches_protocol": seeds["panel"]["curve_seeds"] == {
            "4099": [35, 205, 220, 257, 295], "16411": [93, 158, 252, 366, 382],
            "65537": [13, 57, 156, 166, 369], "131101": [53, 177, 193, 228, 288]},
    }

    # ---------- K. repository policy ----------
    pol = {}
    for p in ("raw/run.jsonl", "raw/run131101.jsonl", "analysis.json", "dev/smoke_dev.jsonl"):
        n = len(show(repo, sha, f"{STUDY}/{p}"))
        pol[p] = {"bytes": n, "over_64KiB": n > 65536}
    pol["artifacts_manifest_present"] = subprocess.run(
        ["git", "-C", repo, "cat-file", "-e", f"{sha}:{STUDY}/raw/.artifacts.json"], capture_output=True).returncode == 0
    out["repository_policy"] = pol

    with open(outp, "w") as fh:
        json.dump(out, fh, indent=1, default=str)
        fh.write("\n")
    print(json.dumps({k: (v.get("ok") if isinstance(v, dict) and "ok" in v else "-") for k, v in out.items()}))
    print("document failures:", json.dumps([(d["file"], d["line"], d["text"], d["displayed"], d["correct_rounding"], d["present_on_line"])
                                            for d in out["documents"]["failures"]], ensure_ascii=False))


if __name__ == "__main__":
    main()
