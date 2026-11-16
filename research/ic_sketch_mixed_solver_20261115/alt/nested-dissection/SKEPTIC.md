# Skeptic review: nested-dissection lane [pcc:unverified]
Reproduced: re-ran the release example (6.8 s); raw JSONL identical to committed raw_output.jsonl except the `ms` timing field; summarize.py output identical to summary_output.txt. 576 records, all correct, 192/192 lib_matches_reimpl.
Verdict: negative result stands; lever native_only, beats_baseline false is honest.
Issues:
- Headline "none beats filter+lanczos at |F|>=128" covers only |F|=128..512, m=2,3 (no Lanczos data for 1024 or m=4).
- Lanczos row_ops (Part A) charging was not re-audited against this lane's arith/book split; comparison is across different counters.
- summary header labels Part A figures "means" but they are medians.
- Markowitz vs Part A median gap (0.1-1.1%) unexplained.
- AMD simplified, ND BFS-only: negative result applies to these implementations, not to AMD/ND in general.
