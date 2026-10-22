# Review-packet validation

This is a deterministic native Rust conversion of frozen study grids, not a new solver.
Run from the repository root:

```
cd research/four_followups_review_20261022 && cargo run --release --locked --offline && cd ../..
uvx --from cairosvg==2.8.2 cairosvg -d 150 -s 1.5625 research/four_followups_review_20261022/cost-ratios.svg -o research/four_followups_review_20261022/cost-ratios.png
uvx --from cairosvg==2.8.2 cairosvg -d 150 -s 1.5625 research/four_followups_review_20261022/pencil-ratios.svg -o research/four_followups_review_20261022/pencil-ratios.png
tectonic research/four_followups_review_20261022/REPORT.tex --outdir research/four_followups_review_20261022
sha256sum -c research/four_followups_review_20261022/SHA256SUMS
python3 scripts/validation_gate.py evidence research/pencil_sparse_20261022
python3 scripts/validation_gate.py evidence research/kummer_differential_collector_20261022
```

The native Rust converter checks each copied priced ratio against its copied candidate and
comparator cost at relative tolerance `1e-9`. The source studies separately
check recovered public scalars, matched rho, raw counter repricing, and
archive-input replay. For this packet the factor-base and law studies contribute
2,016 rows (seven arms, eight slices, 36 cells each), ten pencil pairs
contribute 720 rows (two arms, 36 cells each), and the same-process Kummer
collector contributes 288 rows (eight slices, 36 cells each). Seven Kummer
coordinate-gate size rows have empty cost fields. The archived rho comparison
and separately run cached arm are retained only in the source study.

The converter replaces an earlier Python reporting script to comply with
`AGENTS.md`'s native implementation rule. It regenerates both standardized
CSVs, the summary JSON, and both SVGs from unchanged source evidence. Its
checks include 240/360 pencil cells below online, 120/360 above, zero below
rho, 1,446 verified pencil arm-target rows, three cap failures, and exact joins
across all 1,260 workload keys.

The companion target index has 12,789 rows: 6,300 factor-base, 2,520
projective-law, 1,449 pencil, 1,260 Kummer differential collector, and 1,260
Kummer coordinate records. The converter verifies
one shared workload ID for each of the 1,260 exact `(bits,p,target_index)`
keys, all source checks, seven Kummer zero outputs, zero coordinate failures,
and 1,260 matching source-checked signed Recover rows. It additionally checks
1,446 source-verified pencil solves and three null-cost cap records. The pencil
study validator checks its source scalars, replays, paired grids, and matched
rho control independently. The Kummer study validator checks its raw candidate
and same-process rho source scalars, unmodified candidate trajectory, per-target
cold costs, and native price band. This packet does not replace
the separate source studies' arithmetic and scalar validators.

The factor-base/law/Kummer graph is rendered from the same representative grid rows as the JSON summary; the
paired pencil graph uses all 36 price cells per measured curve. Their PNGs are
rendered from the same SVGs at 150/96 pixel scale for the 150-dpi PDF embed.
The two-page PDF includes both graphs. Its table is
an editorial selection of the pooled representative rows; reviewers should
use the CSV for per-size patterns and the linked raw JSONL for target-level
analysis. The original pencil 1,260-distinct-target request remains open: 543
distinct candidate targets solved and 717 have no complete candidate solve.
