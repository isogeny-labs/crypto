# Builds replay-certificate.json and validator-report.json from the validator's own outputs.
# usage: python3 -I make_reports.py VALIDATION_DIR SCRATCH_RUNS_DIR
import hashlib, json, os, sys

V, RUNS = sys.argv[1], sys.argv[2]
STUDY = "research/ic_gw_fasthash_20261114"
cmpd = json.load(open(os.path.join(V, "replay-compare.json")))
rec = json.load(open(os.path.join(V, "recompute.json")))
analysis = json.load(open(os.path.join(V, "..", "analysis.json")))
sample = json.load(open(os.path.join(RUNS, "sample.json")))
inner = dict(reversed(l.split()) for l in open(os.path.join(V, "replay", "SHA256SUMS.inner")))
fastcheck = [json.loads(l) for l in open(os.path.join(RUNS, "fastcheck.jsonl"))]
meta = {m["idx"]: m for m in (json.loads(l) for l in open(os.path.join(RUNS, "replay-log.jsonl")))}


def sha(p):
    return hashlib.sha256(open(p, "rb").read()).hexdigest()


def r4(x):
    return round(x, 4)


REF_PATH = "/tmp/claude-0/-home-user/cba6917f-779a-544b-9ecf-84e512b45b33/scratchpad/j2/layer/opt/study/bin/v3"
COMBO_PATH = "/tmp/claude-0/-home-user/cba6917f-779a-544b-9ecf-84e512b45b33/scratchpad/val_fh/bin/combo"
REF_SHA = "de500bc154f3074359badf434255605ae103b974f4ac2b00159fc84bd3a76bb6"
COMBO_SHA = "00355034924e06661aed665f433063524c2e3964839c3ca671d54d64aaaa19a9"
HASH_SHA = "477fd569316d3e4234a8e83eb3c69d8f59017b5c55210ee67353b05eaa37528e"
TAR_SHA = "d9d301a42050b5073f10a8f330eb8e31cbc3c4144bc71dbf9706d2482c01b897"
replay_tar = os.path.join(V, "replay", "replay-runs-20261115.tar.gz")

snapshot = {
    "repository": "/home/user/crypto (branch claude/f4-f5-algorithm-speed-bvp7qd = local origin/main)",
    "head": "a851708714d014043a1f0286628ed862d2505a4e",
    "data_commit": "59afa21add (ic_gw_fasthash: results, report, benchmark record)",
    "protocol_freeze_commit": "9d4e6aebc5 (PROTOCOL.md, analyze.py, make-units.sh, run scripts, units.tsv, src/ added before data)",
    "later_commits_touching_study": ["8da2fac152 (github-actions: fasthash.png moved to the artifact store; .artifacts.json, .gitignore added; no data or source change)"],
    "freeze_integrity": "git diff 9d4e6aebc5..HEAD is empty for PROTOCOL.md, src/, units.tsv, make-units.sh, run-*.sh, analyze.py and for the included sources research/quartic_n4_20261019/*.rs and src/cryptanalysis/plane_quartic_pic.rs",
    "dirty_paths_under_study_before_validation": [],
    "remote_main_at_nonce_time": "6ef2197cf99f1798c0bd5fa121508673fb9fc591 (git ls-remote; not fetched, so whether the study changed there was not checked)",
}
nonce = {
    "value": sample["nonce"],
    "derivation": "sha256 of the UTF-8 string '6ef2197cf99f1798c0bd5fa121508673fb9fc591 2026-11-15T02:08:01.610170248Z' = (origin main HEAD from `git ls-remote origin refs/heads/main`) + ' ' + (`date -u +%Y-%m-%dT%H:%M:%S.%NZ`), drawn after the data commit and before any unit was selected or run",
    "remote_main_head": "6ef2197cf99f1798c0bd5fa121508673fb9fc591",
    "utc_time": "2026-11-15T02:08:01.610170248Z",
}
selection_rule = ("Universe: q in {262111, 524287, 1048507, 2097091, 4194271} x seed in 1..9 (45 (q, seed)). Rank by "
                  "sha256('<nonce>:<q>:<seed>') ascending; take the first 6; then append in rank order until >= 2 units have "
                  "q >= 2097091, >= 2 have seed in 4..9, >= 2 have seed in 1..3 (scripts/select.py). Result: 7 units "
                  "(ranks 0-5 and 8; the seed-1..3 condition added (1048507, 2)).")

# ---------- runs ----------
runs = []
for c in sorted(cmpd["checks"], key=lambda c: c["idx"]):
    m = meta[c["idx"]]
    fn = f"{m['idx']}_{m['tag']}_{m['arm']}_{m['q']}_{m['seed']}.jsonl"
    runs.append({
        "unit": fn, "idx": m["idx"], "role": m["tag"], "arm": m["arm"], "q": m["q"], "seed": m["seed"],
        "binary": "REF (published v3)" if m["arm"] in ("REF", "REF2") else "COMBO (validator build)",
        "binary_sha256": m["bin_sha256"], "command": f"<{'REF' if m['arm'] in ('REF','REF2') else 'COMBO'}> ic {m['q']} {m['seed']} D 3.0 1.7",
        "exit_code": m["rc"], "start_utc": m["start_utc"], "loadavg_before": m["loadavg_before"],
        "replay_record_sha256": inner[fn], "ic_records": c["n_ic"], "ic_arms_attempts": c["ic_arms"],
        "compared_against": f"replayed REF idx {c['vs_replay_REF_idx']}" + (" and committed p1/p2 REF and COMBO records" if c.get("mismatches_vs_committed") else ""),
        "mismatches": c["mismatches_vs_replay_REF"],
        "mismatches_vs_committed": c.get("mismatches_vs_committed"),
        "k_found_equals_k_and_ok": c["ok_and_k_found_eq_k"], "verified": c["rc"] == 0 and c["ok_and_k_found_eq_k"] and not c["mismatches_vs_replay_REF"] and not any(c.get("mismatches_vs_committed", {}).values()),
    })

# ---------- timing table ----------
timing = []
for t in cmpd["timing"]:
    if "relations_cpu" not in t and "aa_relations_cpu_REF2/REF" not in t:
        continue
    row = {"q": t["q"], "k": {262111: 18, 524287: 19, 1048507: 20, 2097091: 21, 4194271: 22}[t["q"]], "seed": t["seed"],
           "order": "REF, COMBO, COMBO, REF" + (" then A/A REF, REF2" if "aa" in t["tags"] else ""),
           "runs_seconds": [{"idx": r["idx"], "arm": r["arm"], "relations_cpu_s": r4(r["relations_cpu"]), "dl_total_cpu_s": r4(r["dl_total_cpu"]),
                             "process_cpu_s": r4(r["proc_cpu_s"]), "wall_s": r["wall_s"], "maxrss_kb": r["maxrss_kb"], "load1_before": r["load1_before"]} for r in t["runs"]]}
    for key, name in (("relations_cpu", "relations_cpu_COMBO/REF"), ("dl_total_cpu", "dl_total_cpu_COMBO/REF"), ("proc_cpu_s", "process_cpu_COMBO/REF"), ("wall_s", "process_wall_COMBO/REF")):
        if key in t:
            row[name] = {k: r4(v) for k, v in t[key].items()}
    for k in ("REF_R2/R1_relations_cpu", "COMBO_C2/C1_relations_cpu", "delta_relations_cpu_s", "delta_proc_cpu_s",
              "aa_relations_cpu_REF2/REF", "aa_dl_total_cpu_REF2/REF", "aa_proc_cpu_REF2/REF", "all_REF_relations_cpu_spread_max/min"):
        if k in t:
            row[k] = r4(t[k])
    ab = [r for r in t["runs"] if r["idx"] in [m["idx"] for m in meta.values() if m["tag"] == "abba" and m["q"] == t["q"] and m["seed"] == t["seed"]]]
    if ab:
        R = [r["maxrss_kb"] for r in ab if r["arm"] == "REF"]; C = [r["maxrss_kb"] for r in ab if r["arm"] == "COMBO"]
        row["maxrss_COMBO/REF_sum_ratio"] = r4(sum(C) / sum(R))
    timing.append(row)
tim_rel = [r["relations_cpu_COMBO/REF"] for r in timing if "relations_cpu_COMBO/REF" in r]
singles = [v for r in tim_rel for v in (r["pair1_C1/R1"], r["pair2_C2/R2"])]
aa_row = [r for r in timing if "aa_relations_cpu_REF2/REF" in r][0]
timing_summary = {
    "units": [f"q={r['q']} seed={r['seed']}" for r in timing],
    "relations_cpu_COMBO/REF_mean_of_ABBA_pairs": {f"{r['q']}/{r['seed']}": r["relations_cpu_COMBO/REF"]["mean_of_pairs"] for r in timing},
    "relations_cpu_COMBO/REF_range_mean_of_pairs": [min(r["mean_of_pairs"] for r in tim_rel), max(r["mean_of_pairs"] for r in tim_rel)],
    "relations_cpu_COMBO/REF_worst_single_pair": max(singles),
    "dl_total_cpu_COMBO/REF_mean_of_pairs": {f"{r['q']}/{r['seed']}": r["dl_total_cpu_COMBO/REF"]["mean_of_pairs"] for r in timing},
    "process_cpu_COMBO/REF_mean_of_pairs": {f"{r['q']}/{r['seed']}": r["process_cpu_COMBO/REF"]["mean_of_pairs"] for r in timing},
    "aa_relations_cpu_REF2/REF_q2097091_seed9": aa_row["aa_relations_cpu_REF2/REF"],
    "aa_dl_total_cpu_REF2/REF": aa_row["aa_dl_total_cpu_REF2/REF"],
    "aa_process_cpu_REF2/REF": aa_row["aa_proc_cpu_REF2/REF"],
    "REF_relations_cpu_spread_max/min_over_4_REF_runs_q2097091_seed9": aa_row["all_REF_relations_cpu_spread_max/min"],
    "every_single_pair_below_1_minus_study_floor_0.793": all(v < 1 - rec["aa_floor"] for v in singles),
    "every_single_pair_below_1_minus_replay_AA": all(v < aa_row["aa_relations_cpu_REF2/REF"] for v in singles),
    "kernel_cross_check": "in every timing unit the drop in process CPU (wait4 utime+stime, measured by the kernel, independent of the driver's phase timers) is positive and at least 0.9x the drop in DL relations.cpu (ratio delta_proc_cpu_s / delta_relations_cpu_s: " + ", ".join(f"{r['q']}/{r['seed']} {r['delta_proc_cpu_s'] / r['delta_relations_cpu_s']:.2f}" for r in timing if "delta_proc_cpu_s" in r) + "), so the saving is not an artifact of the in-program relations timer",
}

fields = cmpd["fields_compared_per_ic_arm"]
cert = {
    "schema": "replay-certificate/v1",
    "study": STUDY,
    "claim_under_test": "COMBO arm only: COMBO = fast hashing + compiled square-root chain in find_relations_2lp; counted fields identical to v3; DL relations.cpu COMBO/REF 0.43-0.56 at every k = 18..22 and seed (pass means), worst single pass 0.644, beyond the A/A floor 0.207; DL-record total CPU at k = 22 about 0.80. [pcc:unverified], L0 local.",
    "status": "verified",
    "independent": True,
    "independence_note": "Validator subagent with its own context, nonce, sample, runner, comparator and analysis script (no producer script was imported; analyze.py, plot.py were not read). It runs in the same Claude Code session and the same container as the producer, so independence is of reasoning and code, not of host or organization; no keyring signature.",
    "scalars_verified": True,
    "validated_snapshot": snapshot,
    "binaries": {
        "REF": {"path": REF_PATH, "sha256": sha(REF_PATH), "expected": REF_SHA, "matches": sha(REF_PATH) == REF_SHA,
                "rebuilt_from_published_sources": "cd research/quartic_dl1_cd_sqrt_20261113/src && rustc --edition 2021 -O v3.rs -o <scratch>/bin_refsrc/v3 -> de500bc154f3074359badf434255605ae103b974f4ac2b00159fc84bd3a76bb6 (bit-identical)",
                "published_reference": "research/quartic_dl1_cd_sqrt_20261113/launch/image.json line 19 (v3 de500bc1...)"},
        "COMBO": {"path": COMBO_PATH, "sha256": sha(COMBO_PATH), "recorded": COMBO_SHA, "matches_recorded": sha(COMBO_PATH) == COMBO_SHA,
                  "build": f"cd {STUDY}/src && rustc --edition 2021 -O v3.rs --cfg combo -o {COMBO_PATH}",
                  "note": "bit-identical to the producer's recorded hash (runs tarball binary.sha256 and README); no embedded-path difference arose because the build uses the same relative source paths and rustc does not embed the -o path. The validator's own build is the binary that was run."},
        "HASH_not_run": {"sha256": HASH_SHA, "build": f"cd {STUDY}/src && rustc --edition 2021 -O v3.rs -o <scratch>/val_fh/bin/hash", "matches_recorded": True,
                          "note": "built only to check build reproducibility; not part of the claim under test"},
        "rustc": "rustc 1.97.0 (2d8144b78 2026-07-07)",
        "source_diff_vs_REF": "src/ differs from research/quartic_dl1_cd_sqrt_20261113/src only in ic2.rs (find_relations_2lp: KeyIdx/VidMap/Pend/prefetch; cfg(combo) FastChain/pow_fast at the square-root call; new KeyIdx/VidMap types) and v3.rs (cfg(combo) fastcheck subcommand). No phase timer, counter definition or record printer changed; cfg(combo) appears in no included file.",
    },
    "nonce": nonce,
    "selection_rule": selection_rule,
    "sample": sample["sample"],
    "commands": {
        "unit": "<bin> ic <q> <seed> D 3.0 1.7  (stdout -> unit JSONL), one process at a time via scripts/replay.py (Popen + os.wait4)",
        "fastcheck": f"{COMBO_PATH} fastcheck 250000 262111 524287 1048507 2097091 4194271",
        "compare": "python3 -I scripts/compare.py <scratch>/val_fh/runs <scratch>/val_fh/raw replay-compare.json",
        "recompute": f"python3 -I scripts/recompute.py <extracted runs-fasthash-20261114.tar.gz> {STUDY}/units.tsv recompute.json",
    },
    "runs": runs,
    "coverage": {"units": len(sample["sample"]), "runs": len(runs), "q": sorted({r["q"] for r in runs}), "seeds": sorted({r["seed"] for r in runs}),
                 "q_ge_2097091_units": sum(1 for u in sample["sample"] if u["q"] >= 2097091), "seeds_outside_1_3_units": sum(1 for u in sample["sample"] if u["seed"] > 3)},
    "fields_compared": {"rule": "every key of every rec=ic record except keys ending in 'wall' or 'cpu', 'load1', and keys starting with 's.controls' (PROTOCOL.md Gate)", "per_ic_arm": {k: len(v) for k, v in fields.items()}, "keys": fields},
    "comparison_rule": "for each (q, seed): the ordered list of filtered ic records (D, DL1, DL; per attempt) of each replayed run is serialised with json.dumps and compared for string equality with the first replayed REF run (same keys, same key order, same values = full-line compare in order); for (524287, 2) and (1048507, 2) also with each committed p1/p2 REF and COMBO record.",
    "mismatches": [],
    "total_mismatches": cmpd["total_mismatches"],
    "all_exit_codes_zero": cmpd["all_rc0"], "all_ok_and_k_found_equals_k": cmpd["all_ok_k_found"],
    "ratios_record": "the per-run rec=ratios record is outside the gate; its instance fields (tag, q, seed, ell, order, gae_mul, gae_inv) are identical across all 26 replays and the 94 committed units of each (q, seed); r_inv and r_lmul are in-process timing calibrations (v3.rs line 210 ratios()) and differ run to run, as do search_s and load1",
    "negative_controls": {
        "comparator_flags_counted_change": cmpd["negative_control"],
        "recompute_gate_flags_counted_change": "a copy of committed p2_COMBO_4194271_2 with DL s.rel_2lp + 1 gives exactly one gate mismatch [2, COMBO, 4194271, 2, record 2, ['s.rel_2lp']] and gate pass = false",
    },
    "fastcheck": {"inputs_per_size": 250007, "results": fastcheck,
                  "verdict": "0 mismatches vs modpow, 0 vs the interpreted chain, 0 count mismatches at all five sizes",
                  "note": "the self test is the producer's code path; the independent evidence for value correctness is the counted replay (k_found == k in every record), and Barrett exactness follows from m = floor(2^64/q), x < q^2 < 2^64: x/q - x*m/2^64 < x/2^64 < 1, so the quotient estimate is floor(x/q) or one less and one conditional subtraction suffices"},
    "timing_replay": {"host": "local session container, Intel(R) Xeon(R) Processor @ 2.10GHz, 4 vCPU, kernel 6.18.44-fc-v470; no other workload observed (ps top-3 recorded before each run); one process at a time; load1 before each run 1.00-1.08 (the preceding run's own load)",
                      "metric": "relations.cpu of the DL record summed over attempts (driver's phase CPU timer), DL-record total over fb..verify, process CPU from wait4 (utime+stime), process wall",
                      "rows": timing, "summary": timing_summary},
    "raw_replay_files": {"archive": "replay/replay-runs-20261115.tar.gz", "archive_sha256": sha(replay_tar),
                         "inner_checksums": "replay/SHA256SUMS.inner", "inner_checksums_sha256": sha(os.path.join(V, "replay", "SHA256SUMS.inner")),
                         "files": sorted(inner)},
    "notes": [
        "The replay log (replay-log.jsonl inside the archive) records per run: binary path and sha256, command, rc, start time, wall, utime, stime, maxrss and /proc/loadavg before the run.",
        "Unit stderr lines (ic q=... wall=...) are kept as .err files in the archive.",
    ],
}
json.dump(cert, open(os.path.join(V, "replay-certificate.json"), "w"), indent=1)

# ---------- validator report ----------
C = rec["verdict"]["COMBO"]; H = rec["verdict"]["HASH"]
A_combo_singles = [v[p] for k, v in analysis["primary_relations_cpu_ratio"].items() if k.startswith("COMBO") for p in ("pass1", "pass2")]
headline = [
    {"name": "COMBO relations.cpu ratio, min pass-mean over 15 (q, seed)", "claimed": analysis["verdict"]["COMBO"]["min_mean_ratio"], "recomputed": C["mean_min"], "rel_tol": 1e-9, "readme": "0.430"},
    {"name": "COMBO relations.cpu ratio, max pass-mean over 15 (q, seed)", "claimed": analysis["verdict"]["COMBO"]["max_mean_ratio"], "recomputed": C["mean_max"], "rel_tol": 1e-9, "readme": "0.561"},
    {"name": "COMBO relations.cpu ratio, worst single pass over 30", "claimed": max(A_combo_singles), "recomputed": C["worst_single_pass"], "rel_tol": 1e-9, "readme": "0.644 (task card); cell q=524287 seed 3 pass 2"},
    {"name": "A/A floor max|REF2/REF - 1| (4 single-pass, 2 pass-means)", "claimed": analysis["aa_floor"], "recomputed": rec["aa_floor"], "rel_tol": 1e-9, "readme": "20.7%"},
]
for q in ("262111", "524287", "1048507", "2097091", "4194271"):
    headline.append({"name": f"COMBO relations.cpu geomean over seeds of pass-means, q={q}", "claimed": analysis["relations_ratio_geomean_over_seeds"][f"COMBO/{q}"], "recomputed": C["geomean_per_size"][q], "rel_tol": 1e-9})
for q in ("262111", "524287", "1048507", "2097091", "4194271"):
    headline.append({"name": f"COMBO DL-record total CPU ratio geomean over 3 seeds x 2 passes, q={q}", "claimed": analysis["total_cpu_ratio"][f"COMBO/{q}"]["geomean"], "recomputed": C["total_cpu_geomean_per_size"][q], "rel_tol": 1e-9})
headline.append({"name": "COMBO peak RSS ratio geomean, q=4194271", "claimed": analysis["rss_ratio"]["COMBO/4194271"]["geomean"], "recomputed": C["rss_geomean_per_size"]["4194271"], "rel_tol": 1e-9})
headline.append({"name": "gate counted-field mismatches (79 unit comparisons, 237 ic records)", "claimed": len(analysis["gate"]["mismatches"]), "recomputed": len(rec["gate"]["mismatches"]), "rel_tol": 0})
headline.append({"name": "HASH relations.cpu ratio, min pass-mean (context, not the claim)", "claimed": analysis["verdict"]["HASH"]["min_mean_ratio"], "recomputed": H["mean_min"], "rel_tol": 1e-9})
headline.append({"name": "HASH relations.cpu ratio, max pass-mean (context, not the claim)", "claimed": analysis["verdict"]["HASH"]["max_mean_ratio"], "recomputed": H["mean_max"], "rel_tol": 1e-9})
for h in headline:
    h["agrees"] = abs(h["claimed"] - h["recomputed"]) <= h["rel_tol"] * max(1.0, abs(h["claimed"]))

raw_data = [
    {"path": "../runs-fasthash-20261114.tar.gz", "repo_path": f"{STUDY}/runs-fasthash-20261114.tar.gz", "sha256": TAR_SHA, "role": "producer raw JSONL of the 94 units (SHA256SUMS inside: 191/191 OK)"},
    {"path": "../units.tsv", "repo_path": f"{STUDY}/units.tsv", "sha256": sha(os.path.join(V, "..", "units.tsv")), "role": "frozen unit list (regenerates byte-identically from make-units.sh)"},
    {"path": "../PROTOCOL.md", "repo_path": f"{STUDY}/PROTOCOL.md", "sha256": sha(os.path.join(V, "..", "PROTOCOL.md")), "role": "frozen protocol (9d4e6aebc5)"},
    {"path": "replay/replay-runs-20261115.tar.gz", "repo_path": f"{STUDY}/validation/replay/replay-runs-20261115.tar.gz", "sha256": sha(replay_tar), "role": "validator replay JSONL (26 runs), replay log, fastcheck output, sample, plan"},
    {"path": "recompute.json", "repo_path": f"{STUDY}/validation/recompute.json", "sha256": sha(os.path.join(V, "recompute.json")), "role": "validator recompute output (scripts/recompute.py)"},
    {"path": "replay-compare.json", "repo_path": f"{STUDY}/validation/replay-compare.json", "sha256": sha(os.path.join(V, "replay-compare.json")), "role": "validator replay comparison and timing output (scripts/compare.py)"},
]
problems = [
    {"severity": "note", "kind": "scope of 'total CPU'", "issue": "The 'total CPU at k = 22 about 0.80' is the DL record's phase sum (fb, relations, matrix, linear_algebra, log_recovery, descent, verify), as PROTOCOL.md defines it. It excludes the instance search (search_s in the ratios record, 0.2-35 s depending on q and seed), the D and DL1 arms' own linear algebra and the calibration. End to end, the process CPU ratio COMBO/REF in this replay is 0.90-0.96 (0.962 at k = 22, seed 8). The producer's own unit wall_ms ratios give 0.93-0.98 geomean per size (0.935 at k = 22). Neither contradicts the protocol metric, but the whole-process saving is about 4-10%, not about 20%."},
    {"severity": "note", "kind": "replay vs claimed total CPU", "issue": "Replayed DL-record total CPU at k = 22 (one unit, seed 8, ABBA mean of pairs) is 0.827, against the claimed geomean of 0.796 over seeds 1-3. That lies inside the producer's own range of 0.71-0.87. Total CPU is noisy on this host (the study's A/A total-CPU pairs were 0.85-1.11)."},
    {"severity": "note", "kind": "A/A floor", "issue": "The study's 20.7% floor is set by one slow REF unit (p2 REF q=2097091 seed 1: relations.cpu 2.2818 s, against 1.9328 s in p1 and 1.8096 s for p2 REF2), and reproduces exactly. COMBO clears it in every cell and pass (worst single 0.6439 < 0.7931). The replay's A/A at q=2097091 seed 9 is 0.983 (1.7%), and the spread over its 4 REF runs is 9.6% max/min."},
    {"severity": "minor", "kind": "process (benchmark records rule)", "issue": "benchmarks/records/20261114T2208Z-ic-gw-fasthash.json has raw[0].uploaded = false: the raw tarball is not in S3, although the bench.record rule requires it. It is in git, which is what made the recompute possible."},
    {"severity": "minor", "kind": "figure", "issue": "README embeds fasthash.png, which commit 8da2fac152 moved to the artifact store (.artifacts.json pins sha256 119a78df...). The validator did not fetch or check it. No headline depends on it."},
    {"severity": "note", "kind": "post-hoc statement", "issue": "The README's 'HASH probably a real gain at k >= 21' is labelled post hoc and is not validated as a claim. The frozen-rule HASH decision NOT SUPPORTED reproduces: four single-pass values are at or above 1 - floor, and q=1048507 seed 2 pass 1 is 1.0629 > 1."},
    {"severity": "note", "kind": "negative case", "issue": "Apart from the A/A null, the protocol pre-registers no negative case, such as a size where the chain gain should vanish. Post hoc, COMBO is weakest at q=524287 (geomean 0.545), where the chain saves least (chain_cost 17 vs plain_cost 18, a single segment). This is consistent with the mechanism but is not evidence."},
]
limitations = [
    "L0: shared session container (4 vCPU, Intel Xeon 2.10 GHz), no CPU isolation, no Iso Lab EKS route, no native L2/replay/A/A admission. CPU seconds are a within-session paired comparison only; CPU only (no GPU, no other microarchitecture). Wall/CPU time is not a measured bound (AGENTS.md, 'Curve identity and measured bounds') and moves no counted-cost frontier.",
    "Counted work is identical by the gate, so the counted M-eq and the gap to the matched rho are unchanged. This validation supplies no rho reference or §5a band. A VALIDATION.json for any leaderboard or benchmarks row would still need a matched rho reference.",
    "The replay covers 7 of the 45 (q, seed) units in the universe (2 of them also in the producer's 15); timing replay covers 5 units at k = 20-22 with one ABBA block each and one A/A pair. Seeds 4-9 are new instances of the same family, not new curve families.",
    "Validated snapshot a851708714. Remote main had moved to 6ef2197cf9 at nonce time; it was not fetched, so later changes to the study directory are not covered.",
    "Independence: the validator is a subagent of the producer's Claude Code session on the same container. It used its own scripts and builds and did not read analyze.py or plot.py. The REF binary was taken from the scratchpad path the task card named (hash-verified and rebuilt bit-identically from the published sources).",
    "A passed report means the receipt is admissible evidence for the COMBO time-only claim at k = 18..22 in this family and setting. It does not support an ECDLP claim or demonstrate any asymptotic speedup, and it does not authorize promotion or a leaderboard row.",
]
report = {
    "schema": "validator-report/v1",
    "study": STUDY,
    "status": "pass",
    "independent": True,
    "independence_note": cert["independence_note"],
    "validated_snapshot": snapshot,
    "raw_data": raw_data,
    "raw_data_path_base": f"paths in raw_data[].path are relative to {STUDY}/validation/ (the directory of this report); repo_path is repository-relative",
    "headline": headline,
    "decisions": {"COMBO": {"claimed": analysis["verdict"]["COMBO"]["decision"], "recomputed": C["decision"], "all_45_values_below_1_minus_floor": C["all_below_1_minus_floor"]},
                  "HASH": {"claimed": analysis["verdict"]["HASH"]["decision"], "recomputed": H["decision"], "singles_at_or_above_1_minus_floor": H["singles_above_1_minus_floor"]}},
    "method": "Own script scripts/recompute.py, written from PROTOCOL.md alone (analyze.py was not read or imported), run on the extracted committed tarball. For each (q, seed) and pass: ratio = sum over DL attempt records of relations.cpu (ARM) / same (REF); pass-mean = arithmetic mean of the two pass ratios; floor = max |REF2/REF - 1| over the 4 single-pass and 2 pass-mean A/A values; SUPPORTED iff all 15 pass-means and all 30 single-pass ratios < 1 - floor. Total CPU = DL record sum over the 7 phases' .cpu; RSS from the rss record. Every numeric entry of analysis.json (157 values) was compared with the recompute: 0 differences beyond 1e-12 relative.",
    "checks": {
        "artifacts": "runs-fasthash-20261114.tar.gz sha256 d9d301a4... = README and benchmark record; inner SHA256SUMS 191/191 OK; 94/94 units of units.tsv present, no extra unit files; each unit has one unit record (rc 0, pass/arm/bin/q/seed match units.tsv) and one rss record (rc 0)",
        "protocol_frozen": "PROTOCOL.md, analyze.py, units.tsv, make-units.sh, run scripts and src/ were added in 9d4e6aebc5 and are unchanged at HEAD; the benchmark record names commit 9d4e6aebc5, commit_dirty false, start 22:08:06Z (freeze commit 22:08:00Z)",
        "units_tsv": "bash make-units.sh output is byte-identical to units.tsv (94 units, the protocol's order rules)",
        "binaries": "COMBO and HASH rebuilt from src/ with the README command reproduce the recorded sha256 exactly; REF at the task-card path has sha256 de500bc1... and rebuilding research/quartic_dl1_cd_sqrt_20261113/src reproduces it",
        "gate_recomputed": f"{rec['gate']['units_compared']} unit comparisons, {rec['gate']['records_compared']} ic records, mismatches {rec['gate']['mismatches']}, verify failures {rec['gate']['verify_failures']}, retried {rec['gate']['retried']}",
        "gate_negative_control": cert["negative_controls"]["recompute_gate_flags_counted_change"],
        "counted_replay": f"{len(runs)} runs over {len(sample['sample'])} nonce-drawn units: 0 counted-field mismatches against replayed REF; for (524287, 2) and (1048507, 2) also 0 mismatches against the committed p1/p2 REF and COMBO records; all rc 0; every ic record ok with k_found == k; one attempt each",
        "fastcheck": cert["fastcheck"]["verdict"] + " (250,007 inputs per size including 7 edge values)",
        "source_change_scope": cert["binaries"]["source_diff_vs_REF"],
        "readme_figures": "README ranges, geomeans, A/A pairs (1.010/0.793, 1.030/1.015), A/A seconds (2.28, 1.93, 1.81), total-CPU geomeans, RSS ratios, prediction ranges and load1 1.00..2.30 all reproduce at their stated rounding; tables.md matches analysis.json",
    },
    "timing_replay_summary": timing_summary,
    "null_object_control": "A/A (REF vs REF2, same binary) is the null for a timing ratio and should sit at 1. The study's A/A gives 1.010, 0.793, 1.030, 1.015 (pass-means 0.902, 1.023); the replay A/A gives 0.983. The COMBO signal (0.43-0.56) lies far outside both. The quantity meant to destroy the effect is the arm difference itself; no decay parameter applies to a constant-factor CPU change.",
    "heuristic_validation_checks": [
        {"check": "pre-registered prediction", "result": "PROTOCOL.md (frozen before data) predicted COMBO relations.cpu 0.45-0.70; the per-size geomeans are 0.463-0.545 (inside). The pilot that informed it (wf_f57f915b-f9f) is disclosed as not a claim."},
        {"check": "sample integrity", "result": "sample sizes, seeds 1-3 and order are frozen in units.tsv, and all statistics were recomputed from raw records"},
        {"check": "correspondence validity", "result": "not applicable (no substitute sampling)"},
        {"check": "scale binding", "result": "k = 18..22 (q = 262111..4194271, ell about 2^54..2^64) in one GHS-weak family; no transfer to larger q is claimed or validated"},
    ],
    "cost_model_checks": [
        {"check": "unit declared", "result": "CPU seconds from the driver's phase timers (relations.cpu), plus kernel process CPU in the replay; counted M-eq unchanged by construction"},
        {"check": "memory alongside time", "result": "peak RSS reported (VmHWM polled at 0.2 s): COMBO/REF 1.000 at q=4194271. Replay wait4 maxrss (exact peak) COMBO/REF is 0.996-1.000 per ABBA block (see timing rows)"},
        {"check": "total = per-attempt x 1/success", "result": "every one of the 94 committed and 26 replayed runs succeeded on attempt 1 (ok, k_found == k), so the observed success rate is 1 and per-run CPU is the expected cost; no success-probability factor is hidden"},
        {"check": "optimistic assumptions", "result": "a time-only claim on an L0 shared host. The 'total CPU' metric is the DL pipeline, not the whole process (see problems[0])."},
    ],
    "validation_ladder_checks": {
        "step1_assumptions_isolated": "the claim rests on (a) counted work unchanged (measured: gate and replay, 0 mismatches) and (b) exactness of the Barrett-reduced chain for q < 2^32 (fastcheck at 5 sizes plus the bound argument). Failure mode hunted: a silent value or count drift. None found.",
        "step2_measured_ratio_on_full_pipeline": "the speedup is a measured ratio against REF on the full pipeline at every size claimed (k = 18..22), not a projection, and was re-measured in the replay at k = 20..22 on new seeds. No extrapolation beyond k = 22 is claimed.",
        "step3_cheats": "not applicable (no real-scale partial run)",
        "step4_reproducibility_pointer": "exercised: COMBO and HASH rebuilt bit-identically from committed sources, REF rebuilt bit-identically from its published sources, units.tsv regenerated, raw tarball checksums verified",
    },
    "problems": problems,
    "limitations": limitations,
    "validation_report": {
        "id": "VAL-20261115-f58909",
        "task_id": "none named by the task card (main-session validation-gate request)",
        "run_ids": ["research/ic_gw_fasthash_20261114 runs-fasthash-20261114 (94 units, 59afa21add)"],
        "artifact_checks": ["tarball sha256 + 191/191 inner checksums", "94/94 units present, rc 0", "binaries rebuilt bit-identically", "freeze unchanged"],
        "metric_recomputations": ["COMBO pass-mean range 0.4302..0.5606, worst single 0.6439, floor 0.20694, decision SUPPORTED; 157/157 analysis.json values equal"],
        "control_checks": ["A/A null reproduced", "replay A/A 0.983", "comparator and gate negative controls flag a +1 counted change and ignore timing changes", "fastcheck 0 mismatches"],
        "heuristic_validation_checks": ["prediction pre-registered and held"],
        "cost_model_checks": ["unit CPU s declared", "RSS reported", "success rate 1 observed"],
        "proof_architecture_checks": [],
        "verdict": "passed",
        "limitations": limitations,
        "artifact_paths": [f"{STUDY}/validation/replay-certificate.json", f"{STUDY}/validation/validator-report.json", f"{STUDY}/validation/replay/replay-runs-20261115.tar.gz"],
    },
    "review_attestation": {
        "review_plan": "none attached to the task card; no joints were assigned, so this report verdicts receipt admissibility",
        "joints_owned": [],
        "sources_read": [
            "crypto-autoresearcher/docs/agent-runtime-core.md", "crypto-autoresearcher/agents/validator.md",
            "crypto-autoresearcher/AGENTS.md (Review architecture 359-420; Curve identity and measured bounds 523-536)",
            "crypto-autoresearcher/docs/inventor-protocol.md section 6", "crypto/docs/VALIDATION_GATE.md",
            f"{STUDY}/PROTOCOL.md", f"{STUDY}/README.md", f"{STUDY}/tables.md", f"{STUDY}/analysis.json", f"{STUDY}/make-units.sh",
            f"{STUDY}/run-unit.sh", f"{STUDY}/run-local.sh", f"{STUDY}/units.tsv", f"{STUDY}/.artifacts.json", f"{STUDY}/.gitignore",
            f"{STUDY}/src/ic2.rs (diff vs REF, find_relations_2lp, pack2, pow_fixed, Chain)", f"{STUDY}/src/v3.rs (diff vs REF, grep)",
            f"{STUDY}/runs-fasthash-20261114.tar.gz (all unit records)",
            "research/quartic_dl1_cd_sqrt_20261113/src (diff base)", "research/quartic_dl1_cd_sqrt_20261113/launch/image.json (grep v3 hash)",
            "research/quartic_dl1_cd_sqrt_20261113/combo/validation/{replay-certificate,validator-report}.json (structure only)",
            "benchmarks/records/20261114T2208Z-ic-gw-fasthash.json",
        ],
        "not_read": [f"{STUDY}/analyze.py", f"{STUDY}/plot.py", f"{STUDY}/REPORT.typ", f"{STUDY}/REPORT.pdf", f"{STUDY}/fasthash.svg"],
        "read_sibling_reports": False,
        "verdict": "passed",
    },
    "replay_certificate": {"path": "replay-certificate.json", "sha256": sha(os.path.join(V, "replay-certificate.json"))},
}
json.dump(report, open(os.path.join(V, "validator-report.json"), "w"), indent=1)
print("headline agrees:", all(h["agrees"] for h in headline), [h["name"] for h in headline if not h["agrees"]])
print("cert runs verified:", all(r["verified"] for r in runs), len(runs))
