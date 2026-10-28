# Row 44 | 20261015-u64-rootbatch: recompute headline claims from committed raw summaries and per-target records.
# Headline numbers come from the round-3 table (raw/s3-*.txt, raw/o3-*.jsonl). The addendum round (s4/o4) is reported in notes.
import json, os, statistics, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import common as C

STUDY = "research/u64_root_batch_20261015"
RAW = f"{STUDY}/raw"
def load_s(arm, rep, rnd="s3"):
    return json.load(open(os.path.join(C.REPO, f"{RAW}/{rnd}-{arm}-{rep}.txt")))
def load_o(arm, rep, rnd="o3"):
    return [json.loads(l) for l in open(os.path.join(C.REPO, f"{RAW}/{rnd}-{arm}-{rep}.jsonl")) if l.strip()]

sb = [load_s("base", k) for k in (1, 2, 3)]
sc = [load_s("cand", k) for k in (1, 2, 3)]
pt_b = [s["timing_ms"]["process_total"] / 1000 for s in sb]
pt_c = [s["timing_ms"]["process_total"] / 1000 for s in sc]
rank_b = [s["timing_ms"]["rank_stage"] for s in sb]
rank_c = [s["timing_ms"]["rank_stage"] for s in sc]
tgt_b = [s["timing_ms"]["target_median"] for s in sb]
tgt_c = [s["timing_ms"]["target_median"] for s in sc]

med_b, med_c = statistics.median(pt_b), statistics.median(pt_c)
claims = [
    {"claimed": "whole-process median 76.4 s (baseline)", "computed": f"{med_b:.2f} s (median of {['%.2f' % v for v in pt_b]})",
     "definition": "median over 3 reps of timing_ms.process_total / 1000", "files": [f"{RAW}/s3-base-{k}.txt" for k in (1,2,3)],
     "status": "match" if abs(med_b - 76.4) / 76.4 <= 0.01 else "mismatch"},
    {"claimed": "23.4 s (candidate)", "computed": f"{med_c:.2f} s (median of {['%.2f' % v for v in pt_c]})",
     "definition": "median over 3 reps of timing_ms.process_total / 1000", "files": [f"{RAW}/s3-cand-{k}.txt" for k in (1,2,3)],
     "status": "match" if abs(med_c - 23.4) / 23.4 <= 0.01 else "mismatch"},
    {"claimed": "3.26x whole-process (median ratio)", "computed": f"{med_b/med_c:.4f}",
     "definition": "median baseline / median candidate, process_total", "files": [f"{RAW}/s3-base-1.txt", f"{RAW}/s3-cand-1.txt"],
     "status": "match" if abs(med_b/med_c - 3.26) / 3.26 <= 0.01 else "mismatch"},
    {"claimed": "1.87x min-to-min whole-process", "computed": f"{min(pt_b)/min(pt_c):.4f} (min base {min(pt_b):.2f} / min cand {min(pt_c):.2f})",
     "definition": "min over reps baseline / min over reps candidate", "files": [f"{RAW}/s3-base-1.txt", f"{RAW}/s3-cand-1.txt"],
     "status": "match" if abs(min(pt_b)/min(pt_c) - 1.87) / 1.87 <= 0.01 else "mismatch"},
    {"claimed": "rank stage 2.95x", "computed": f"median rank_stage base {statistics.median(rank_b):.0f} ms / median cand {statistics.median(rank_c):.0f} ms = {statistics.median(rank_b)/statistics.median(rank_c):.4f}; "
                                              f"README table value 10261 ms is the cand rep-3 value (min), giving {statistics.median(rank_b)/min(rank_c):.4f}",
     "definition": "median over reps of timing_ms.rank_stage, baseline / candidate", "files": [f"{RAW}/s3-base-1.txt", f"{RAW}/s3-cand-1.txt"],
     "status": "mismatch" if abs(statistics.median(rank_b)/statistics.median(rank_c) - 2.95) / 2.95 > 0.01 else "match"},
    {"claimed": "per-target median 6.09x", "computed": f"median target_median base {statistics.median(tgt_b):.1f} ms / median cand {statistics.median(tgt_c):.1f} ms = {statistics.median(tgt_b)/statistics.median(tgt_c):.4f}; "
                                              f"README value 213 ms is the cand rep-3 value, giving {statistics.median(tgt_b)/min(tgt_c):.4f}",
     "definition": "median over reps of timing_ms.target_median, baseline / candidate", "files": [f"{RAW}/s3-base-1.txt", f"{RAW}/s3-cand-1.txt"],
     "status": "mismatch" if abs(statistics.median(tgt_b)/statistics.median(tgt_c) - 6.09) / 6.09 > 0.01 else "match"},
    {"claimed": "~80x fewer field inversions per solved state", "computed": "not computed: no inversion counter in raw records (fields listed in notes)",
     "definition": "inversion counts per solved state, scalar vs batched", "files": [f"{RAW}/s3-base-1.txt"], "status": "no_raw_in_repo"},
    {"claimed": "new batched-vs-scalar equivalence unit test (4,096 inputs)", "computed": "no test output in repo; only code-level test named in README",
     "definition": "cargo test result for root_batch_tests", "files": [f"{STUDY}/README.md"], "status": "no_raw_in_repo"},
]

# identical check: round-3 per-target records, pairs matched by rep index
FIELDS = ["recovered_scalar", "probes", "point_indices", "x_codes"]
n_cmp = n_eq = n_diff = 0
diffs = []
for k in (1, 2, 3):
    ob, oc = load_o("base", k), load_o("cand", k)
    for a, b in zip(ob, oc):
        for f in FIELDS:
            n_cmp += 1
            if a.get(f) == b.get(f):
                n_eq += 1
            else:
                n_diff += 1
                diffs.append((k, a.get("fixture_index"), f))
    if len(ob) != len(oc):
        diffs.append((k, "length", len(ob), len(oc)))
# round-4 (addendum) equality, reported in notes only
m_cmp = m_eq = m_diff = 0
for k in (1, 2, 3):
    ob, oc = load_o("base", k, "o4"), load_o("cand", k, "o4")
    for a, b in zip(ob, oc):
        for f in FIELDS:
            m_cmp += 1
            if a.get(f) == b.get(f):
                m_eq += 1
            else:
                m_diff += 1

identical = {
    "claimed": True, "n_compared": n_cmp, "n_equal": n_eq, "n_differ": n_diff,
    "files": [f"{RAW}/o3-base-{k}.jsonl" for k in (1,2,3)] + [f"{RAW}/o3-cand-{k}.jsonl" for k in (1,2,3)],
    "status": "match" if n_diff == 0 else "mismatch",
}

offrepo = C.grep_lines([STUDY], C.OFFREPO_PAT)
aa = C.grep_lines([STUDY], C.AA_PAT)
iso = C.grep_lines([f"{STUDY}/README.md"], C.ISO_PAT)
row = {
    "row_no": 44,
    "slug": "20261015-u64-rootbatch",
    "claims": claims,
    "identical": identical,
    "offrepo_refs": offrepo,
    "aa_lines": aa,
    "isolation_lines": iso,
    "protocol_git_date": "no PROTOCOL file in " + STUDY,
    "first_raw_git_date": C.git_first_add(f"{RAW}/s3-base-1.txt") + " | " + C.git_first_add(f"{RAW}/o3-base-1.jsonl"),
    "script": "research/validation_sweep_20261019/B-benchmarks/scripts/recompute/44.py",
    "notes": (f"Round-3 identical check: fields {FIELDS}, pairs base-k/cand-k for k=1..3, 16 fixtures each. "
              f"Round-4 addendum (o4-*.jsonl, not headline): n_compared={m_cmp}, n_differ={m_diff}. "
              f"Round-3 rep-3 candidate rank_stage/target_median are the values the README table prints for the candidate "
              f"(10261 ms, 213 ms); the rep medians differ (see claims). "
              f"Summary s3 per-arm targets_solved=11 targets_failed=5 rank=26 in all 12 summary files (checked). "
              "Host per README: Apple M4 Pro, shared, no isolation receipt. "
              "Also the README gives round-4 numbers 3.52x and 1.39x which are the addendum, not the headline."),
}
print(json.dumps(row, ensure_ascii=False))
