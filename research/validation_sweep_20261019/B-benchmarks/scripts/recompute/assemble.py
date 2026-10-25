import json, os
REPO = "/home/user/crypto"
SC = "/home/user/crypto/research/validation_sweep_20261019/B-benchmarks/scripts/recompute"
OUT = "/home/user/crypto/research/validation_sweep_20261019/B-benchmarks/recompute/SWB-RECOMPUTE-18.jsonl"
def ln(ref):
    """ref = 'relative/path:line' -> verbatim line text (stripped of trailing newline)."""
    path, n = ref.rsplit(":", 1)
    with open(os.path.join(REPO, path), encoding="utf-8", errors="replace") as f:
        lines = f.read().split("\n")
    return f"{path}:{n}: {lines[int(n) - 1].rstrip()}"
S = {
 1021: "research/isogeny_fibers_20261010/round4/16_2_fast_4e",
 1022: "research/isogeny_fibers_20261010/round4/16_3_vertical_margin",
}
GIT_ALL = "ca00c151764f7c3221b22a43a138887f171e051d 2026-10-17T10:19:32-07:00"
rows = []
def claim(c, comp, d, files, st): return {"claimed": c, "computed": comp, "definition": d, "files": files, "status": st}
# ---------- row 1021
rows.append({
 "row_no": 1021, "slug": "20261011-koblitz-4e-historical",
 "claims": [
  claim("0.124 us (n=131 table check)", "0.124 us (124.039 ns)", "median contended_wall_ns_per_check_median, set members_of_G, method closed_form_table, n=131; /1000", ["research/isogeny_fibers_20261010/round4/16_2_fast_4e/sample_n131.json"], "match"),
  claim("0.122 us (n=233 table check)", "0.122 us (122.198 ns)", "as above, n=233", ["research/isogeny_fibers_20261010/round4/16_2_fast_4e/sample_n233.json"], "match"),
  claim("160 us (tau-NAF, n=131)", "160.294 us (160293.739 ns)", "median, members_of_G, method tnaf_ld_rP, n=131; /1000", ["research/isogeny_fibers_20261010/round4/16_2_fast_4e/sample_n131.json"], "match"),
  claim("98 us (tau-NAF, n=233)", "98.231 us (98230.691 ns)", "median, members_of_G, method tnaf_ld_rP, n=233; /1000", ["research/isogeny_fibers_20261010/round4/16_2_fast_4e/sample_n233.json"], "match"),
 ],
 "identical": {"claimed": False, "n_compared": None, "n_equal": None, "n_differ": None, "files": [], "status": "not_claimed"},
 "offrepo_refs": [ln("research/isogeny_fibers_20261010/round4/16_2_fast_4e/RESULTS.md:72")],
 "aa_lines": [],
 "isolation_lines": [ln("research/isogeny_fibers_20261010/round4/16_2_fast_4e/RESULTS.md:6"), ln("benchmarks/20261011-koblitz-4e-historical/README.md:7")],
 "protocol_git_date": "no PROTOCOL/spec file in study directory",
 "first_raw_git_date": "2026-10-17T10:19:32-07:00 (exhaustive.json, commit ca00c151764f7c3221b22a43a138887f171e051d)",
 "script": f"{SC}/1021.py",
 "notes": "Headline values match the members_of_G set; the uniform set gives 0.081 us (table) and 0.081 us (table, n=233 uniform 80.868 ns). Headline does not name the set. exhaustive.json was git-added 2026-10-17, after the 2026-10-11 measurement date. Identical claim absent: MEASURED text says byte-identical file copies, not arm outputs.",
})
# ---------- row 1022
rows.append({
 "row_no": 1022, "slug": "20261011-isogeny-vertical-margin-historical",
 "claims": [
  claim("100,663,797 Meq (Velu, l~2^24)", "100663797 Meq (velu_1pt, ell=16777259)", "rational.json row ell=16777259, field velu_1pt.Meq", ["research/isogeny_fibers_20261010/round4/16_3_vertical_margin/rational.json"], "match"),
  claim("24,454,729 Meq (sqrt-elu, l~2^24)", "24454729 Meq (sqrtvelu_1pt, ell=16777259)", "rational.json row ell=16777259, field sqrtvelu_1pt.Meq", ["research/isogeny_fibers_20261010/round4/16_3_vertical_margin/rational.json"], "match"),
  claim("4.12x", "4.1163 (100663797/24454729)", "velu_1pt.Meq / sqrtvelu_1pt.Meq at ell=16777259", ["research/isogeny_fibers_20261010/round4/16_3_vertical_margin/rational.json"], "match"),
  claim("0.822 [0.805, 0.838] exponent, ell>=2^16", "0.8217 [0.8052, 0.8382] (log2 c 4.93, n=17)", "OLS of log2(sqrtvelu_1pt.Meq) on log2(ell), ell>=2^16, one point per distinct ell (3 instances per ell, identical values), 95% t-interval df=15", ["research/isogeny_fibers_20261010/round4/16_3_vertical_margin/rational.json"], "match"),
 ],
 "identical": {"claimed": False, "n_compared": None, "n_equal": None, "n_differ": None, "files": [], "status": "not_claimed"},
 "offrepo_refs": [ln("research/isogeny_fibers_20261010/round4/16_3_vertical_margin/RESULTS.md:83"), ln("research/isogeny_fibers_20261010/round4/16_3_vertical_margin/RESULTS.md:91")],
 "aa_lines": [],
 "isolation_lines": [ln("benchmarks/20261011-isogeny-vertical-margin-historical/README.md:7"), ln("research/isogeny_fibers_20261010/round4/16_3_vertical_margin/plot.py:114")],
 "protocol_git_date": "no PROTOCOL/spec file in study directory",
 "first_raw_git_date": "2026-10-17T10:19:32-07:00 (analysis.json, commit ca00c151764f7c3221b22a43a138887f171e051d)",
 "script": f"{SC}/1022.py",
 "notes": "Exponent interval reproduced with n=17 (one point per distinct ell); the per-ell aggregation (mean of Meq, mean of log, or first instance) gives identical values because the three instances per ell carry identical Meq fields. The 0.822 row in the fit list uses the sqrtvelu_1pt series. analysis.json was git-added 2026-10-17.",
})
# ---------- row 1023
rows.append({
 "row_no": 1023, "slug": "20261011-glv-gls-lattice-cost-model",
 "claims": [
  claim("1.220x (D=-619 cost-model ratio)", "1.2197 (embedding s); 1.2275 (embedding -s)", "glvgls_lattice.json rows with D=-619, field model.ratio; embedding not named in the headline", ["research/isogeny_loop_glv_20261011/glvgls_lattice.json"], "cannot_define"),
  claim(">=2.5 (characteristic-2 Verschiebung correction, cost ratio)", "2.5 (2*(c_dbl+2)/c_dbl at c_dbl=8.0)", "formula stated in README.md:36 evaluated at c_dbl from degree_bound.json; no per-run record of a corrected ratio", ["research/isogeny_loop_glv_20261011/README.md", "research/isogeny_loop_glv_20261011/degree_bound.json"], "partial"),
 ],
 "identical": {"claimed": False, "n_compared": None, "n_equal": None, "n_differ": None, "files": [], "status": "not_claimed"},
 "offrepo_refs": [ln("research/isogeny_loop_glv_20261011/README.md:192"), ln("research/isogeny_loop_glv_20261011/REPORT.typ:1")],
 "aa_lines": [],
 "isolation_lines": [ln("benchmarks/20261011-glv-gls-lattice-cost-model/README.md:7")],
 "protocol_git_date": "no PROTOCOL/spec file in study directory",
 "first_raw_git_date": "2026-10-17T10:19:32-07:00 (degree_bound.json, commit ca00c151764f7c3221b22a43a138887f171e051d)",
 "script": f"{SC}/1023.py",
 "notes": "Headline 1.220 equals the s-embedding row (1.21972); the -s row gives 1.22752 (which would print 1.228). Headline does not name the embedding, so the choice is left undefined. The >=2.5 figure is the README formula, not a record. glvgls_lattice.json was git-added 2026-10-17.",
})
# ---------- row 1024
rows.append({
 "row_no": 1024, "slug": "20261011-glv-gls-4d-benchmark",
 "claims": [
  claim("71,172 ns (D=-619 projective GLV-GLS-4 median)", "71171.688 ns (run 1, bench_run1.json)", "median over 30 rounds of arm 'glvgls4 (projective loop)'", ["research/isogeny_loop_glv_20261012/bench_run1.json"], "match"),
  claim("81,665 ns (GLS-2 median)", "81664.672 ns (run 1)", "median over 30 rounds of arm 'gls2'", ["research/isogeny_loop_glv_20261012/bench_run1.json"], "match"),
  claim("1.147x", "1.1474 (81664.672/71171.688)", "GLS-2 median / GLV-GLS-4 projective median, run 1", ["research/isogeny_loop_glv_20261012/bench_run1.json"], "match"),
  claim("1.062-1.214 across D projective ratios", "1.0619-1.2136 (six bench files: D=-23 1.2136, -3019 1.0619, -431 1.1149, -71 1.1203, -619 run1 1.1474, -619 run2 1.0907)", "GLS-2 median / GLV-GLS-4 projective median per bench file, min and max", ["research/isogeny_loop_glv_20261012/bench_D23.json", "research/isogeny_loop_glv_20261012/bench_D3019.json", "research/isogeny_loop_glv_20261012/bench_D431.json", "research/isogeny_loop_glv_20261012/bench_D71.json", "research/isogeny_loop_glv_20261012/bench_run1.json", "research/isogeny_loop_glv_20261012/bench_run2.json"], "match"),
 ],
 "identical": {"claimed": False, "n_compared": None, "n_equal": None, "n_differ": None, "files": [], "status": "not_claimed"},
 "offrepo_refs": [ln("research/isogeny_loop_glv_20261012/binary_validate_cmd.txt:5"), ln("research/isogeny_loop_glv_20261012/REPORT.typ:1")],
 "aa_lines": [ln("research/isogeny_loop_glv_20261012/README.md:108"), ln("research/isogeny_loop_glv_20261012/README.md:122"), ln("research/isogeny_loop_glv_20261012/README.md:148"), ln("research/isogeny_loop_glv_20261012/bench_run1.md:13"), ln("research/isogeny_loop_glv_20261012/reports/run-20261011T2333Z-bench/report.typ:32")],
 "isolation_lines": [ln("research/isogeny_loop_glv_20261012/README.md:95"), ln("research/isogeny_loop_glv_20261012/README.md:127"), ln("research/isogeny_loop_glv_20261012/README.md:128"), ln("benchmarks/20261011-glv-gls-4d-benchmark/README.md:7")],
 "protocol_git_date": "no PROTOCOL/spec file in study directory",
 "first_raw_git_date": "2026-10-17T10:19:32-07:00 (bench_D3019.json, commit ca00c151764f7c3221b22a43a138887f171e051d)",
 "script": f"{SC}/1024.py",
 "notes": "Headline D=-619 values are run 1 (bench_run1.json). Run 2 (bench_run2.json, a different host) gives 1.0907 at D=-619, which the headline does not state. The 1.062-1.214 range is computed over six bench files including both D=-619 runs; the study's report.typ:32 lists the same five-ratio set without run 2. Identical claim absent.",
})
# ---------- row 1025
rows.append({
 "row_no": 1025, "slug": "20261011-fiber-ic-scaling",
 "claims": [
  claim("8.4 to 8.8 (h=4, 18 bits, envelope/rho per curve)", "8.43, 8.83", "min over closed-MITM arms (mean cost.s) divided by mean cost.s of rho-neg, per curve, 18-bit curves, h=4 session", ["research/fiber_ic_scaling_20261011/sessions/scaling_h4/records.jsonl"], "match"),
  claim("35.1 to 37.0 (h=4, 30 bits)", "35.10, 36.99", "as above, 30-bit curves, h=4", ["research/fiber_ic_scaling_20261011/sessions/scaling_h4/records.jsonl"], "match"),
  claim("13.4 to 20.1 (h=72, 18 bits)", "13.44, 20.12", "as above, h=72 sessions pooled (B=4..256, stated rule), 18-bit curves", ["research/fiber_ic_scaling_20261011/sessions/scaling_h72/records.jsonl", "research/fiber_ic_scaling_20261011/sessions/scaling_h72_small_b/records.jsonl"], "match"),
  claim("60.4 to 63.1 (h=72, 30 bits)", "60.36, 63.06", "as above, 30-bit curves, h=72 pooled", ["research/fiber_ic_scaling_20261011/sessions/scaling_h72/records.jsonl", "research/fiber_ic_scaling_20261011/sessions/scaling_h72_small_b/records.jsonl"], "match"),
  claim("816 audited records", "816 (336 + 336 + 144)", "line count of the three records.jsonl files", ["research/fiber_ic_scaling_20261011/sessions/scaling_h4/records.jsonl", "research/fiber_ic_scaling_20261011/sessions/scaling_h72/records.jsonl", "research/fiber_ic_scaling_20261011/sessions/scaling_h72_small_b/records.jsonl"], "match"),
 ],
 "identical": {"claimed": False, "n_compared": None, "n_equal": None, "n_differ": None, "files": [], "status": "not_claimed"},
 "offrepo_refs": [ln("research/fiber_ic_scaling_20261011/sessions/scaling_h4/session.json:15"), ln("research/fiber_ic_scaling_20261011/sessions/scaling_h72/session.json:15"), ln("research/fiber_ic_scaling_20261011/sessions/scaling_h72_small_b/session.json:15")],
 "aa_lines": [],
 "isolation_lines": [ln("research/fiber_ic_scaling_20261011/README.md:99"), ln("benchmarks/20261011-fiber-ic-scaling/README.md:7")],
 "protocol_git_date": "2026-10-17T10:19:32-07:00 (PROTOCOL.md, commit ca00c151764f7c3221b22a43a138887f171e051d)",
 "first_raw_git_date": "2026-10-17T10:19:32-07:00 (sessions/scaling_h4/records.jsonl, commit ca00c151764f7c3221b22a43a138887f171e051d)",
 "script": f"{SC}/1025.py",
 "notes": "Envelope rule (README.md:36) is the cheapest fiber-closed MITM arm over B=4..256 divided by rho.negation, both as mean S; S reproduced as the mean of cost.s over non-warmup records (matches TABLES.md values). The h=72 headline uses both h=72 sessions pooled; scaling_h72 alone (B>=16) gives 91.04-146.94 at 18 bits, which is informational only. Protocol and first raw record share the same git add commit.",
})
# ---------- row 1026
rows.append({
 "row_no": 1026, "slug": "20261011-f4-f5-row-allocation",
 "claims": [
  claim("42.47% (F4 cumulative requested bytes reduction)", "42.4684% (allocated_bytes sum, f4 arm)", "1 - sum(candidate allocated_bytes)/sum(baseline allocated_bytes), arm f4, 24 records per side", ["research/f4_f5_row_build_20261011/records/baseline.jsonl", "research/f4_f5_row_build_20261011/records/candidate.jsonl"], "match"),
  claim("45.47% (F5)", "45.4662%", "as above, arm f5", ["research/f4_f5_row_build_20261011/records/baseline.jsonl", "research/f4_f5_row_build_20261011/records/candidate.jsonl"], "match"),
  claim("50.30% (criterion)", "50.3034%", "as above, arm criterion", ["research/f4_f5_row_build_20261011/records/baseline.jsonl", "research/f4_f5_row_build_20261011/records/candidate.jsonl"], "match"),
  claim("0.25% (F4 requests reduction)", "0.2479% (allocations sum)", "1 - sum(candidate allocations)/sum(baseline allocations), arm f4", ["research/f4_f5_row_build_20261011/records/baseline.jsonl", "research/f4_f5_row_build_20261011/records/candidate.jsonl"], "match"),
  claim("7.70% (F5 requests reduction)", "7.6961%", "as above, arm f5", ["research/f4_f5_row_build_20261011/records/baseline.jsonl", "research/f4_f5_row_build_20261011/records/candidate.jsonl"], "match"),
  claim("35.12% (criterion requests reduction)", "35.1166%", "as above, arm criterion", ["research/f4_f5_row_build_20261011/records/baseline.jsonl", "research/f4_f5_row_build_20261011/records/candidate.jsonl"], "match"),
 ],
 "identical": {"claimed": True, "n_compared": 96, "n_equal": 96, "n_differ": 0, "files": ["research/f4_f5_row_build_20261011/records/baseline.jsonl", "research/f4_f5_row_build_20261011/records/candidate.jsonl"], "status": "partial"},
 "offrepo_refs": [ln("research/f4_f5_row_build_20261011/records/baseline-sha256.txt:1"), ln("research/f4_f5_row_build_20261011/records/baseline-aa-sha256.txt:1"), ln("research/f4_f5_row_build_20261011/records/baseline-aa-sha256.txt:2"), ln("research/f4_f5_row_build_20261011/REPORT.md:151"), ln("research/f4_f5_row_build_20261011/REPORT.md:156"), ln("research/f4_f5_row_build_20261011/REPORT.md:163"), ln("research/f4_f5_row_build_20261011/REPORT.md:164")],
 "aa_lines": [ln("research/f4_f5_row_build_20261011/REPORT.md:27"), ln("research/f4_f5_row_build_20261011/REPORT.md:73"), ln("research/f4_f5_row_build_20261011/REPORT.md:135"), ln("research/f4_f5_row_build_20261011/REPORT.md:146"), ln("research/f4_f5_row_build_20261011/PROTOCOL.md:40")],
 "isolation_lines": [ln("research/f4_f5_row_build_20261011/REPORT.md:89"), ln("benchmarks/20261011-f4-f5-row-allocation/README.md:7")],
 "protocol_git_date": "2026-10-17T10:19:32-07:00 (PROTOCOL.md, commit ca00c151764f7c3221b22a43a138887f171e051d)",
 "first_raw_git_date": "2026-10-17T10:19:32-07:00 (records/baseline.jsonl, commit ca00c151764f7c3221b22a43a138887f171e051d)",
 "script": f"{SC}/1026.py",
 "notes": "Identical: output_digest compared for f4 (24) and f5 (24), word_ops for f4 (24) and criterion (24), 96 comparisons, 0 differ. Not compared: criterion output_digest (absent from raw records), f5 word_ops and rank (absent). Headline says 'requested bytes'; raw field is allocated_bytes and comparison.json calls it allocated_byte_reduction_percent. The 'store:' matches in records/library-*.txt are test names, not store references. Per-arm allocation totals and reductions match comparison.json.",
})
# ---------- row 1027
rows.append({
 "row_no": 1027, "slug": "20261011-eigen-seta-1958z-pke-l3-l5",
 "claims": [
  claim("L3 Encrypt 1.1051 s", "1.1051 s", "encrypt_s from records/run_output.txt, L3 line", ["research/eigen_seta_rust_20261011/reports/run-20261011T1958Z-pke-l3-l5/records/run_output.txt"], "match"),
  claim("L5 Encrypt 2.2990 s", "2.2990 s", "encrypt_s, L5 line", ["research/eigen_seta_rust_20261011/reports/run-20261011T1958Z-pke-l3-l5/records/run_output.txt"], "match"),
  claim("L3 Decrypt 47.138 s", "47.138 s", "decrypt_s, L3 line", ["research/eigen_seta_rust_20261011/reports/run-20261011T1958Z-pke-l3-l5/records/run_output.txt"], "match"),
  claim("L5 Decrypt 104.223 s", "104.223 s", "decrypt_s, L5 line", ["research/eigen_seta_rust_20261011/reports/run-20261011T1958Z-pke-l3-l5/records/run_output.txt"], "match"),
  claim("one sample per size", "1 line per set (L3: 1, L5: 1)", "count of keygen lines per set in run_output.txt", ["research/eigen_seta_rust_20261011/reports/run-20261011T1958Z-pke-l3-l5/records/run_output.txt"], "match"),
 ],
 "identical": {"claimed": False, "n_compared": None, "n_equal": None, "n_differ": None, "files": [], "status": "not_claimed"},
 "offrepo_refs": [],
 "aa_lines": [],
 "isolation_lines": [ln("research/eigen_seta_rust_20261011/reports/run-20261011T1958Z-pke-l3-l5/README.md:15"), ln("research/eigen_seta_rust_20261011/reports/run-20261011T1958Z-pke-l3-l5/README.md:29"), ln("research/eigen_seta_rust_20261011/reports/run-20261011T1958Z-pke-l3-l5/report.typ:14"), ln("benchmarks/20261011-eigen-seta-1958z-pke-l3-l5/README.md:7")],
 "protocol_git_date": "2026-10-17T10:19:32-07:00 (reports/run-20261011T1958Z-pke-l3-l5/README.md, commit ca00c151764f7c3221b22a43a138887f171e051d; the README is the frozen protocol text, no separate PROTOCOL file)",
 "first_raw_git_date": "2026-10-17T10:19:32-07:00 (reports/run-20261011T1958Z-pke-l3-l5/records/run_output.txt, commit ca00c151764f7c3221b22a43a138887f171e051d)",
 "script": f"{SC}/1027.py",
 "notes": "Single sample per set, correct true both. README states the protocol was frozen 2026-10-11 19:58 UTC.",
})
# ---------- row 1028
rows.append({
 "row_no": 1028, "slug": "20261011-eigen-seta-1956z-pke-l1",
 "claims": [
  claim("KeyGen 25.044 s", "25.044 s", "keygen_s, L1 line", ["research/eigen_seta_rust_20261011/reports/run-20261011T1956Z-pke-l1/records/run_output.txt"], "match"),
  claim("Encrypt 0.3897 s", "0.3897 s", "encrypt_s, L1 line", ["research/eigen_seta_rust_20261011/reports/run-20261011T1956Z-pke-l1/records/run_output.txt"], "match"),
  claim("Decrypt 11.208 s", "11.208 s", "decrypt_s, L1 line", ["research/eigen_seta_rust_20261011/reports/run-20261011T1956Z-pke-l1/records/run_output.txt"], "match"),
  claim("1411-byte ciphertext for 39-byte message", "pke_ct_bytes 1411, message_bytes 39", "pke_ct_bytes and message_bytes fields, L1 line", ["research/eigen_seta_rust_20261011/reports/run-20261011T1956Z-pke-l1/records/run_output.txt"], "match"),
 ],
 "identical": {"claimed": False, "n_compared": None, "n_equal": None, "n_differ": None, "files": [], "status": "not_claimed"},
 "offrepo_refs": [],
 "aa_lines": [],
 "isolation_lines": [ln("research/eigen_seta_rust_20261011/reports/run-20261011T1956Z-pke-l1/README.md:14"), ln("research/eigen_seta_rust_20261011/reports/run-20261011T1956Z-pke-l1/report.typ:13"), ln("benchmarks/20261011-eigen-seta-1956z-pke-l1/README.md:7")],
 "protocol_git_date": "2026-10-17T10:19:32-07:00 (reports/run-20261011T1956Z-pke-l1/README.md, commit ca00c151764f7c3221b22a43a138887f171e051d)",
 "first_raw_git_date": "2026-10-17T10:19:32-07:00 (reports/run-20261011T1956Z-pke-l1/records/run_output.txt, commit ca00c151764f7c3221b22a43a138887f171e051d)",
 "script": f"{SC}/1028.py",
 "notes": "One sample; output shows exit 0 and correct true.",
})
# ---------- row 1029
rows.append({
 "row_no": 1029, "slug": "20261011-eigen-seta-1955z-verify",
 "claims": [
  claim("1,116,209,216 non-allocator secret-phase instructions (Toy16)", "1116209216 (rest column, all 12 Toy16 runs)", "rest = total - allocator per ctprobe_summary.txt; raw callgrind totals: lines match the summary total for all 12 runs; allocator split not recomputed (callgrind_annotate is not an allowed tool)", ["research/eigen_seta_rust_20261011/reports/run-20261011T1955Z-verify/records/toyprobe/ctprobe_summary.txt", "research/eigen_seta_rust_20261011/reports/run-20261011T1955Z-verify/records/toyprobe/callgrind/"], "partial"),
  claim("120,996,429,610 non-allocator secret-phase instructions (L1)", "120996429610 (rest column, all 4 L1 runs)", "as above, l1probe; raw totals match summary for all 4 runs", ["research/eigen_seta_rust_20261011/reports/run-20261011T1955Z-verify/records/l1probe/ctprobe_summary.txt", "research/eigen_seta_rust_20261011/reports/run-20261011T1955Z-verify/records/l1probe/callgrind/"], "partial"),
  claim("3,620/3,620 round trips", "3620/3620 (L1 20/20, Toy16 3000/3000, Toy32 600/600)", "sum of RESULT lines in failrate/l1.txt, toy16.txt, toy32.txt", ["research/eigen_seta_rust_20261011/reports/run-20261011T1955Z-verify/records/failrate/l1.txt", "research/eigen_seta_rust_20261011/reports/run-20261011T1955Z-verify/records/failrate/toy16.txt", "research/eigen_seta_rust_20261011/reports/run-20261011T1955Z-verify/records/failrate/toy32.txt"], "match"),
 ],
 "identical": {"claimed": False, "n_compared": None, "n_equal": None, "n_differ": None, "files": [], "status": "not_claimed"},
 "offrepo_refs": [ln("research/eigen_seta_rust_20261011/reports/run-20261011T1955Z-verify/records/verify_run.sh:2")],
 "aa_lines": [],
 "isolation_lines": [ln("research/eigen_seta_rust_20261011/reports/run-20261011T1955Z-verify/README.md:17"), ln("benchmarks/20261011-eigen-seta-1955z-verify/README.md:7")],
 "protocol_git_date": "2026-10-17T10:19:32-07:00 (reports/run-20261011T1955Z-verify/README.md, commit ca00c151764f7c3221b22a43a138887f171e051d)",
 "first_raw_git_date": "2026-10-17T10:19:32-07:00 (records/failrate/toy16.txt, commit ca00c151764f7c3221b22a43a138887f171e051d)",
 "script": f"{SC}/1029.py",
 "notes": "Rest column is constant within each set in the summary (1116209216 for Toy16, 120996429610 for L1). Allocator/rest split is defined by records/ctsummary.sh (callgrind_annotate plus function-name regex), which was not re-run. Headline says 'constant within recorded grids', which is not an identical-output claim.",
})
# ---------- row 1030
rows.append({
 "row_no": 1030, "slug": "20261011-eigen-seta-1955z-final-l1",
 "claims": [
  claim("KeyGen 31.050 s", "31.050 s", "keygen_s, L1 line", ["research/eigen_seta_rust_20261011/reports/run-20261011T1955Z-final-l1/records/run_output.txt"], "match"),
  claim("Encaps 0.3032 s", "0.3032 s", "encaps_s, trial 0", ["research/eigen_seta_rust_20261011/reports/run-20261011T1955Z-final-l1/records/run_output.txt"], "match"),
  claim("Decaps 11.464 s", "11.464 s", "decaps_s, trial 0", ["research/eigen_seta_rust_20261011/reports/run-20261011T1955Z-final-l1/records/run_output.txt"], "match"),
  claim("one sample, 1/1 correct", "1/1 decapsulations correct", "RESULT line", ["research/eigen_seta_rust_20261011/reports/run-20261011T1955Z-final-l1/records/run_output.txt"], "match"),
 ],
 "identical": {"claimed": False, "n_compared": None, "n_equal": None, "n_differ": None, "files": [], "status": "not_claimed"},
 "offrepo_refs": [],
 "aa_lines": [],
 "isolation_lines": [ln("research/eigen_seta_rust_20261011/reports/run-20261011T1955Z-final-l1/README.md:13"), ln("research/eigen_seta_rust_20261011/reports/run-20261011T1955Z-final-l1/report.typ:13"), ln("benchmarks/20261011-eigen-seta-1955z-final-l1/README.md:7")],
 "protocol_git_date": "2026-10-17T10:19:32-07:00 (reports/run-20261011T1955Z-final-l1/README.md, commit ca00c151764f7c3221b22a43a138887f171e051d)",
 "first_raw_git_date": "2026-10-17T10:19:32-07:00 (reports/run-20261011T1955Z-final-l1/records/run_output.txt, commit ca00c151764f7c3221b22a43a138887f171e051d)",
 "script": f"{SC}/1030.py",
 "notes": "README.md:17 reads '1/1 shared keys equal'; this is encapsulate vs decapsulate within one trial, not an equality between arms, so identical.claimed is false.",
})
os.makedirs(os.path.dirname(OUT), exist_ok=True)
with open(OUT, "w", encoding="utf-8") as f:
    for r in sorted(rows, key=lambda x: x["row_no"]):
        f.write(json.dumps(r, ensure_ascii=False) + "\n")
print("wrote", OUT, len(rows), "rows")
