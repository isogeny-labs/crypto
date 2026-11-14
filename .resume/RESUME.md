# Resume: crypto @ claude/ecstatic-edison-cgo57l

## Goal
Owner (2026-11-14): work on hyperelliptic index calculus and make cross-genus statistical experiments possible: HCDLP at genus 2, 3, 4, 5 on one field, difficulty differences, Smith's isogeny, non-hyperelliptic curves.

## Spec
Frozen protocol: research/genus_ladder_20261114/PROTOCOL.md (frozen.sha256 beside it). Code: src/cryptanalysis/genus_ladder.rs, src/cryptanalysis/quartic_ladder.rs, src/bin/genus_ladder.rs, src/bin/genus_ladder_analyze.rs.
Run (section 6 of the protocol): two `genus_ladder run` invocations -> part_a.jsonl, part_b.jsonl -> ladder.jsonl; then `genus_ladder_analyze ladder.jsonl analysis.json`; then `python3 render.py analysis.json` in the study directory.

## State
2026-11-14: code and unit tests done (5 ladder tests pass), protocol frozen, grid launched locally (4 vCPU container, no AWS identity, so no isolab-eks and no S3 push).

## Next
Collect part_a/part_b, analyze, RESULTS.md with the H1..H9 scorecard, figures, PDF, bench record, session log, PR.
