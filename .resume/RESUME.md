# Resume: crypto @ claude/gifted-goodall-r3sb7t

## Goal
Does prime-field Semaev S3/S4 relation finding depend on curve coefficients or model?
Study: research/coeff_model_semaev_20261109 (PROTOCOL.md frozen, FROZEN.sha256, commit e23b0bf).
Cells: C40-S3 (19 arms, T=1000), C40-S4 (8 arms, T=40), C48-S3 (9 arms, T=200), side study M (C40-SIDE).

## Lead / hypothesis
Null expected: yield is a set statistic |F|^2/#E independent of a,b; root-finder per-element cost has
no coefficient product after precomputation. Montgomery/Edwards impossible in prime-order class.

## Spec
Build: cd research/coeff_model_semaev_20261109/tool && cargo build --release.
Curves: python3 tool/setup.py <cell> <bits> <B> <T> <mode> <xcheck> <arms|side> runs
Run: tool/target/release/cmsem run < runs/<cell>.cell > runs/<cell>.jsonl
Analyze: python3 tool/analyze.py runs/<cell>.jsonl runs/<cell>.meta.json runs/<cell>.analysis.json

## State
Preregistration committed. Runs launched locally (L0); counted metrics decide.
