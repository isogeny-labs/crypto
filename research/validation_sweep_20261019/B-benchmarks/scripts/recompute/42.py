# Row 42 | 20261016-binary-mitm-ladder-l11-l12: recompute headline claims from committed raw records.
import json, os, sys, statistics
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import common as C

STUDY = "research/binary_mitm_oracle_l11_l12_20261016"
BASE = os.path.join(C.REPO, STUDY)
FILES = {"l11": f"{STUDY}/l11n33/results.json", "l12": f"{STUDY}/l12n37/results.json"}
R = {k: json.load(open(os.path.join(C.REPO, v))) for k, v in FILES.items()}

def stats(rung):
    pt = R[rung]["decomposition"]["per_target"]
    mitm = [t["mitm_ms"] for t in pt]
    pairs = [t["pairs_ms"] for t in pt]
    return {
        "n_targets": len(pt),
        "median_mitm_ms": statistics.median(mitm),
        "median_pairs_ms": statistics.median(pairs),
        "ratio_median": statistics.median(pairs) / statistics.median(mitm),
        "max_mitm_ops": max(t["mitm_ops"] for t in pt),
        "max_pairs_pairs": max(t["pairs_pairs"] for t in pt),
        "n_agree": sum(1 for t in pt if t["agree"]),
        "n_disagree": sum(1 for t in pt if not t["agree"]),
        "n_found_mitm": sum(1 for t in pt if t["mitm_found"]),
        "n_found_witness_checked": sum(1 for t in pt if t["mitm_found"] and t["witness_group_verified"]),
        "n_found_witness_unverified": sum(1 for t in pt if t["mitm_found"] and not t["witness_group_verified"]),
        "stored_disagreements": R[rung]["decomposition"]["disagreements"],
        "stored_median_ms": R[rung]["decomposition"]["median_ms_per_target"],
        "stored_all_witnesses_group_verified": R[rung]["decomposition"]["all_witnesses_group_verified"],
    }

s11, s12 = stats("l11"), stats("l12")
claims = [
    {"claimed": "median 1.746 ms (l=11, MITM)", "computed": f"{s11['median_mitm_ms']:.6f} ms",
     "definition": "median over 64 targets of per_target.mitm_ms", "files": [FILES["l11"]],
     "status": "match" if abs(s11['median_mitm_ms'] - 1.746) / 1.746 <= 0.01 else "mismatch"},
    {"claimed": "1.62 s (l=11, pairs-and-solve median)", "computed": f"{s11['median_pairs_ms']/1000:.6f} s",
     "definition": "median over 64 targets of per_target.pairs_ms, divided by 1000", "files": [FILES["l11"]],
     "status": "match" if abs(s11['median_pairs_ms']/1000 - 1.62) / 1.62 <= 0.01 else "mismatch"},
    {"claimed": "928.0x (l=11)", "computed": f"{s11['ratio_median']:.2f}x",
     "definition": "median pairs_ms / median mitm_ms over 64 targets", "files": [FILES["l11"]],
     "status": "match" if abs(s11['ratio_median'] - 928.0) / 928.0 <= 0.01 else "mismatch"},
    {"claimed": "8.566 ms (l=12, MITM)", "computed": f"{s12['median_mitm_ms']:.6f} ms",
     "definition": "median over 64 targets of per_target.mitm_ms", "files": [FILES["l12"]],
     "status": "match" if abs(s12['median_mitm_ms'] - 8.566) / 8.566 <= 0.01 else "mismatch"},
    {"claimed": "18.7 s (l=12, pairs-and-solve median)", "computed": f"{s12['median_pairs_ms']/1000:.6f} s",
     "definition": "median over 64 targets of per_target.pairs_ms, divided by 1000", "files": [FILES["l12"]],
     "status": "match" if abs(s12['median_pairs_ms']/1000 - 18.7) / 18.7 <= 0.01 else "mismatch"},
    {"claimed": "2,187.2x (l=12)", "computed": f"{s12['ratio_median']:.2f}x",
     "definition": "median pairs_ms / median mitm_ms over 64 targets", "files": [FILES["l12"]],
     "status": "match" if abs(s12['ratio_median'] - 2187.2) / 2187.2 <= 0.01 else "mismatch"},
    {"claimed": "max 8,068 counted point-adds (l=11)", "computed": f"{s11['max_mitm_ops']}",
     "definition": "max per_target.mitm_ops over 64 targets", "files": [FILES["l11"]],
     "status": "match" if s11['max_mitm_ops'] == 8068 else "mismatch"},
    {"claimed": "max 16,092 counted point-adds (l=12)", "computed": f"{s12['max_mitm_ops']}",
     "definition": "max per_target.mitm_ops over 64 targets", "files": [FILES["l12"]],
     "status": "match" if s12['max_mitm_ops'] == 16092 else "mismatch"},
    {"claimed": "2,098,176 / 8,390,656 quartic solves (pairs-and-solve, max)",
     "computed": f"l11 max pairs_pairs={s11['max_pairs_pairs']}; l12 max pairs_pairs={s12['max_pairs_pairs']}",
     "definition": "max per_target.pairs_pairs over 64 targets per rung",
     "files": [FILES["l11"], FILES["l12"]],
     "status": "match" if (s11['max_pairs_pairs'], s12['max_pairs_pairs']) == (2098176, 8390656) else "mismatch"},
]

agree_n = s11["n_targets"] + s12["n_targets"]
agree_eq = s11["n_agree"] + s12["n_agree"]
agree_diff = s11["n_disagree"] + s12["n_disagree"]
identical = {
    "claimed": True,
    "n_compared": agree_n, "n_equal": agree_eq, "n_differ": agree_diff,
    "files": [FILES["l11"], FILES["l12"]],
    "status": "match" if agree_diff == 0 else "mismatch",
}

offrepo = C.grep_lines([STUDY], C.OFFREPO_PAT)
aa = C.grep_lines([f"{STUDY}/REPORT.typ"], C.AA_PAT)
iso = C.grep_lines([f"{STUDY}/REPORT.typ", f"{STUDY}/provenance.json"], C.ISO_PAT)

row = {
    "row_no": 42,
    "slug": "20261016-binary-mitm-ladder-l11-l12",
    "claims": claims,
    "identical": identical,
    "offrepo_refs": offrepo,
    "aa_lines": aa,
    "isolation_lines": iso,
    "protocol_git_date": "no PROTOCOL file in study directory " + STUDY + " (none to date)",
    "first_raw_git_date": C.git_first_add(FILES["l11"]) + " | " + C.git_first_add(FILES["l12"]),
    "script": "research/validation_sweep_20261019/B-benchmarks/scripts/recompute/42.py",
    "notes": (f"l11: found-by-MITM {s11['n_found_mitm']} of {s11['n_targets']}, witnesses group-verified "
              f"{s11['n_found_witness_checked']}, unverified {s11['n_found_witness_unverified']}. "
              f"l12: found-by-MITM {s12['n_found_mitm']} of {s12['n_targets']}, witnesses group-verified "
              f"{s12['n_found_witness_checked']}, unverified {s12['n_found_witness_unverified']}. "
              f"Stored decomposition.disagreements l11={s11['stored_disagreements']} l12={s12['stored_disagreements']}. "
              f"Stored median_ms_per_target l11={s11['stored_median_ms']} l12={s12['stored_median_ms']}. "
              "Study REPORT.typ states verdicts 21/43 and 24/40 (found/not found) at l11 and l12; the "
              "recomputed found counts are in the fields above. The identical field compares the per-target "
              "agree flag (MITM verdict vs pairs-and-solve verdict) on 128 targets. Host is unpinned and shared "
              "(provenance.json host.isolation: L0), recorded as stated."),
}
print(json.dumps(row, ensure_ascii=False))
