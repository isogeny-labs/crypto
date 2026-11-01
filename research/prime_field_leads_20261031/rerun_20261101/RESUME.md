# Resume: prime-field leads 20261031 reruns (branch research/prime-field-leads-reruns-20261101)

## Goal
Finish the three open cells of the prime-field leads round (PR #3103, merged): L2a at s = 12, the frozen L2b ladder, the C2 msolve leg on M. Land results, reports, graphs and PDFs, update §5 of research/notes/ecdlp-general/RESEARCH_PRIME_FIELD_LEADS_20261031.md and SYNTHESIS.md, in one PR.

## Lead / hypothesis
Leads L2a, L2b (catalogue M008) and C2 of the note above. Null predictions: L2a e ≥ 1; L2b real and control trajectories grow alike; C2 crater and floor msolve degrees not separated.

## Spec
research/prime_field_leads_20261031/rerun_20261101/RERUN_PLAN.md (frozen before data), driver run_stage.sh in the same directory. Inputs: committed C2 .ms files (sha256 in each stage's inputs.sha256), seeded generators.

## State
Done 2026-11-01 20:50Z. Parity passed with msolve -l 1 (A5). L2b: 70 cells, 68 ok; ratio 4.00 vs 4.00, null_confirmed. C2 M: 32/32 cells complete, max degree 55 on both sides, null_confirmed. L2a s=6..12: e = 1.97 [1.95, 2.00] (G), 2.19 [2.16, 2.22] (S); does not reopen, G closes. REPORT.md, ASSESSMENT.md, receipts, lead reports, note §5 and SYNTHESIS.md updated.

## Next steps
Land the PR (auto-merge). Nothing in the round remains unrun.

## Runs in flight
None.

## Stop and success conditions
Stop rules per RERUN_PLAN.md (caps 300/600/1500/3600 s, 6 GiB, 20-min L2a guard). Success = every cell attempted and reported, decisions read off the frozen rules; no speedup is expected or claimed.
