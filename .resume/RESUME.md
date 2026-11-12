# Resume: crypto @ claude/zealous-meitner-85ipb5

## Goal
Owner (2026-11-12): "Speed it all up" (M4GB and Gröbner-basis decomposition engines). Two tracks:
(A) M4GB patch 0004, divisibility signatures in update() (this run); (B) inherited-f4 per-call
speed-up by an Opus subagent working in this checkout (koblitz_groebner.rs, gf2_elim.rs, pq_f4_f2.rs,
inherited part of ic_framework/solvers.rs; engine name inherited-f4-next for a paired A/B).

## Lead / hypothesis
Callgrind: update() is 70 % of a 24-variable M4GB solve, divisibility on degrevlex-ranked monomials
decoded per variable. Exact two-mask signatures cut instructions to 0.47x / 0.43x. Expectation and
falsifier: research/m4gb_update_signatures_20261112/README.md.

## Spec
research/m4gb_update_signatures_20261112/ab_corpus.py under tools/benchrec.py run (record benchmarks/records/20261112T0517Z-m4gb-update-signatures.json):
corpus of 90 captured descent systems (capture.sh; scratchpad corpus/), arms old (patches 0001-0003,
scratchpad m4gb-prefix-v3), new (0001-0004, m4gb-prefix-v4), oldAA; 5 repeats; --serve; one thread.

## State
Patch 0004 written, built clean from pinned commit, byte-identical outputs on 15 inputs. PR #3936 open
(merged main 2026-11-12). Corpus run starting.

## Runs in flight
Local corpus A/B (this record). Subagent B (inherited-f4) in the background.

## Stop and success conditions
Fixed sample (90 systems x 5 repeats x 3 arms). Any disagreement = failure. Report per cell ratio vs A/A.

## Next steps
1. summarize/make_report; close record; benchmarks entry; session log; commit; push.
2. Integrate track B result (paired A/B under its own record), then PR update.
