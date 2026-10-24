import json, statistics, sys
ROOT = "/home/user/crypto/"
STUDY = ROOT + "research/pdp_oracle_speedup_20261011/ps2_scalar/"
R = STUDY + "raw/"
times = [json.loads(l) for l in open(R + "times.jsonl") if l.strip()]
ver = [json.loads(l) for l in open(R + "verify_arms.jsonl") if l.strip()]
def tag(rec):  # label is "<tag>-l<L>-r<R>"; baseline2 is the A/A copy of baseline
    return rec["label"].split("-l")[0]
def med(t, l, key):
    vals = [r[key] for r in times if tag(r) == t and r["l"] == l]
    return statistics.median(vals), len(vals)
ls = [8, 9, 10, 11, 12]
gs_ns = {}; ratio = {}; ratio_of_medians = {}; counts = {}
for l in ls:
    g, n1 = med("group-scalar", l, "ns_per_pair")
    b, n2 = med("baseline", l, "ns_per_pair")
    gs_ns[l] = g; ratio_of_medians[l] = g / b; counts[l] = (n1, n2)
    # study ab_summary.md rule ("B/A median"): median over rounds of the per-round B/A ratio, rounds paired by label suffix
    gr = {r["label"].split("-l")[1]: r["ns_per_pair"] for r in times if tag(r) == "group-scalar" and r["l"] == l}
    br = {r["label"].split("-l")[1]: r["ns_per_pair"] for r in times if tag(r) == "baseline" and r["l"] == l}
    ratio[l] = statistics.median(gr[k] / br[k] for k in gr if k in br)
claim1 = "%.1f-%.1f" % (min(gs_ns.values()), max(gs_ns.values()))
claim2 = "%.4f-%.4f" % (min(ratio.values()), max(ratio.values()))
rom = "%.4f-%.4f" % (min(ratio_of_medians.values()), max(ratio_of_medians.values()))
gs_checked = sum(r["root_sets_checked"] for r in ver if r["arm"] == "group-scalar")
gs_mism = sum(r["mismatches"] for r in ver)
claim3 = str(gs_checked)
# identical: per l, every arm's verdict digest vs the baseline arm's digest at the same l
n_cmp = n_eq = 0
for l in ls:
    base = [r for r in ver if r["l"] == l and r["arm"] == "baseline"][0]["digest_verdicts"]
    for r in ver:
        if r["l"] != l: continue
        n_cmp += 1
        if r["digest_verdicts"] == base and r["mismatches"] == 0: n_eq += 1
out = {
 "row_no": 1011, "slug": "20261011-pdp-ps2-scalar-group-law",
 "claims": [
  {"claimed": "29.8-40.8 ns per own pair", "computed": claim1 + " (group-scalar ns_per_pair median over 5 rounds per l=8..12: " + ", ".join("%d:%.2f" % (l, gs_ns[l]) for l in ls) + "; n per l " + str(counts) + ")",
   "definition": "min..max over l=8..12 of median over rounds of times.jsonl ns_per_pair, tag group-scalar, rounded to 0.1",
   "files": ["research/pdp_oracle_speedup_20261011/ps2_scalar/raw/times.jsonl"],
   "status": "match" if claim1 == "29.8-40.8" else "mismatch"},
  {"claimed": "group-scalar/baseline per-pair ratios 0.0668-0.1225", "computed": claim2 + " (per-l paired-round median: " + ", ".join("%d:%.4f" % (l, ratio[l]) for l in ls) + "; ratio of per-l medians: " + ", ".join("%d:%.4f" % (l, ratio_of_medians[l]) for l in ls) + ")",
   "definition": "min..max over l=8..12 of the median over the five paired rounds of (group-scalar ns_per_pair / baseline ns_per_pair), the study's ab_summary.md 'B/A median' rule; baseline = tag 'baseline' only, not baseline2. Alternative ratio of per-l medians = " + rom + " (0.1240 at l=8) is NOT the study's stated column and is reported only for transparency",
   "files": ["research/pdp_oracle_speedup_20261011/ps2_scalar/raw/times.jsonl"],
   "status": "match" if claim2 == "0.0668-0.1225" else "mismatch"},
  {"claimed": "400 corpus root sets agree", "computed": claim3 + " root_sets_checked summed over group-scalar records l=8..12; mismatches summed over all verify records = %d" % gs_mism,
   "definition": "sum of root_sets_checked where arm == group-scalar in verify_arms.jsonl",
   "files": ["research/pdp_oracle_speedup_20261011/ps2_scalar/raw/verify_arms.jsonl"],
   "status": "match" if (claim3 == "400" and gs_mism == 0) else "mismatch"},
 ],
 "identical": {"claimed": True, "n_compared": n_cmp, "n_equal": n_eq, "n_differ": n_cmp - n_eq,
   "files": ["research/pdp_oracle_speedup_20261011/ps2_scalar/raw/verify_arms.jsonl"],
   "status": "match" if n_cmp == n_eq else "mismatch"},
 "offrepo_refs": [],
 "aa_lines": [
   "research/pdp_oracle_speedup_20261011/ps2_scalar/README.md:69: * **A/A.**  `baseline` against `baseline2`, per target: medians of B/A 1.044, 1.023, 1.004, 1.009, 0.997 at `l = 8…12`; run-to-run",
   "research/pdp_oracle_speedup_20261011/ps2_scalar/README.md:70:   spread of the baseline 8.15%, 3.44%, 3.53%, 2.77%, 3.43%.  The `l = 8` row is the least stable (it contains the contended runs below); the",
   "research/pdp_oracle_speedup_20261011/ps2_scalar/README.md:14: (A/A spread at most 8.15%)",
   "research/pdp_oracle_speedup_20261011/reports/run-20261011T2255Z/report.typ:38: A/A (baseline against baseline), per target, B/A medians: 1.044, 1.023, 1.004, 1.009, 0.997 at $l = 8..12$; run-to-run spread of the baseline 8.15%, 3.44%, 3.53%, 2.77%, 3.43%."],
 "isolation_lines": [
   "research/pdp_oracle_speedup_20261011/ps2_scalar/README.md:4: Class: **engineering**.  (see line 47: Unit: wall nanoseconds on one reserved core (`tools/isolated_bench.py run --cpus 3`))",
   "research/pdp_oracle_speedup_20261011/ps2_scalar/README.md:77: * **Contention.**  6 of 125 timed runs were marked contended by the tool",
   "research/pdp_oracle_speedup_20261011/ps2_scalar/README.md:123: An oracle-stage figure, on this host class only (x86-64 Xeon, AVX-512F/BW, PCLMULQDQ, KVM guest; no Arm64, Metal or FPGA measurement), from `isolated_bench` runs on a shared",
   "research/pdp_oracle_speedup_20261011/ps2_scalar/README.md:124: container (not a receipt of the repository's CPU isolation gate).",
   "research/pdp_oracle_speedup_20261011/reports/run-20261011T2255Z/report.typ:13: timed runs on CPU 3 through `tools/isolated_bench.py run --cpus 3` (a shared container, not the repository's receipt service)."],
 "protocol_git_date": "ca00c151764f7c3221b22a43a138887f171e051d 2026-10-17T10:19:32-07:00 (protocols/PS-2.md added)",
 "first_raw_git_date": "ca00c151764f7c3221b22a43a138887f171e051d 2026-10-17T10:19:32-07:00 (raw/binary_hashes.txt, verify_arms.jsonl, times.jsonl, iso.jsonl all added in this commit)",
 "script": "research/validation_sweep_20261019/B-benchmarks/scripts/recompute/1011.py",
 "notes": "Aggregation as stated in study README section 4 (medians over five rounds). Contended runs kept (6 of 125 marked, per README). Study PDF text read from report.typ (source of the PDF). Protocol and raw files share one add commit, so protocol-before-raw ordering is not established by git dates. store refs: none in study files."
}
print(json.dumps(out))
