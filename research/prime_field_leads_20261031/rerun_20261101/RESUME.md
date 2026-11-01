# Resume: prime-field leads 20261031 reruns (branch research/prime-field-leads-reruns-20261101)

## Goal
Finish the three open cells of the prime-field leads round (PR #3103, merged): L2a at s = 12, the frozen L2b ladder, the C2 msolve leg on M. Land results, reports, graphs and PDFs, update §5 of research/notes/ecdlp-general/RESEARCH_PRIME_FIELD_LEADS_20261031.md and SYNTHESIS.md, in one PR.

## Lead / hypothesis
Leads L2a, L2b (catalogue M008) and C2 of the note above. Null predictions: L2a e ≥ 1; L2b real and control trajectories grow alike; C2 crater and floor msolve degrees not separated.

## Spec
research/prime_field_leads_20261031/rerun_20261101/RERUN_PLAN.md (frozen before data), driver run_stage.sh in the same directory. Inputs: committed C2 .ms files (sha256 in each stage's inputs.sha256), seeded generators.

## State
Instruments ported to Linux (L2b run_cell.sh env overrides, rsswatch Linux branch); binaries built (binaries.sha256); generator reproduces all original instance parameters. No stage has run yet.

## Next steps
cd research/prime_field_leads_20261031/rerun_20261101; for s in parity_c2 parity_l2b l2b c2m l2a l2a_fit; do ./run_stage.sh $s; done (one at a time; ./run_stage.sh build first in a fresh container; apt-get install msolve). Then write REPORT.md, graphs, PDF; update lead reports, note §5, SYNTHESIS.md; PR with auto-merge.

## Runs in flight
None yet. Each stage writes under rerun_20261101/<stage dir>/; logs there.

## Stop and success conditions
Stop rules per RERUN_PLAN.md (caps 300/600/1500/3600 s, 6 GiB, 20-min L2a guard). Success = every cell attempted and reported, decisions read off the frozen rules; no speedup is expected or claimed.
