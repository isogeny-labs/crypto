# Review-packet validation

This is a deterministic native Rust conversion of frozen study grids, not a new solver.
Run from the repository root:

```
cd research/four_followups_review_20261022 && cargo run --release --locked --offline && cd ../..
uvx --from cairosvg==2.8.2 cairosvg -d 150 -s 1.5625 research/four_followups_review_20261022/cost-ratios.svg -o research/four_followups_review_20261022/cost-ratios.png
uvx --from cairosvg==2.8.2 cairosvg -d 150 -s 1.5625 research/four_followups_review_20261022/pencil-ratios.svg -o research/four_followups_review_20261022/pencil-ratios.png
rsvg-convert -d 150 -p 150 research/four_followups_review_20261022/pencil-coverage.svg -o research/four_followups_review_20261022/pencil-coverage.png
SOURCE_DATE_EPOCH=1792627200 tectonic research/four_followups_review_20261022/REPORT.tex --outdir research/four_followups_review_20261022
sha256sum -c research/four_followups_review_20261022/SHA256SUMS
python3 scripts/validation_gate.py evidence research/pencil_sparse_20261022
python3 scripts/validation_gate.py evidence research/pencil_markowitz_20261022
python3 scripts/validation_gate.py evidence research/kummer_differential_collector_20261022
```

The native Rust converter checks each copied priced ratio against its copied candidate and
comparator cost at relative tolerance `1e-9`. The source studies separately
check recovered public scalars, matched rho, raw counter repricing, and
archive-input replay. For this packet the factor-base and law studies contribute
2,016 rows (seven arms, eight slices, 36 cells each), ten online/peel pencil pairs
contribute 720 rows (two arms, 36 cells each), one p=739 Markowitz-priority
arm contributes 36 rows, and the same-process Kummer
collector contributes 288 rows (eight slices, 36 cells each). Seven Kummer
coordinate-gate size rows have empty cost fields. The archived rho comparison
and separately run cached arm are retained only in the source study.

The converter replaces an earlier Python reporting script to comply with
`AGENTS.md`'s native implementation rule. It regenerates both standardized
CSVs, the pencil coverage CSV, the summary JSON, and all three SVGs from unchanged source evidence. Its
checks include 240/360 peel cells below online, 120/360 above, zero below
rho, 0/36 Markowitz cells below peel or rho, 1,506 verified pencil arm-target
rows, four cap failures, and exact joins
across all 1,260 workload keys.

The companion target index has 12,850 rows: 6,300 factor-base, 2,520
projective-law, 1,510 pencil, 1,260 Kummer differential collector, and 1,260
Kummer coordinate records. The converter verifies
one shared workload ID for each of the 1,260 exact `(bits,p,target_index)`
keys, unique standardized observation identities including `source_record`,
all source checks, seven Kummer zero outputs, zero coordinate failures,
and 1,260 matching source-checked signed Recover rows. It additionally checks
1,506 source-verified pencil arm-target solves and four null-candidate-cost
cap records with measured rho costs.
The separate pencil coverage matrix has one row for each of the 1,260
frozen workload keys. The converter checks 543 keys with verified
candidate rows, one with capped rows but no verified candidate, and
716 with no indexed pencil row; the graph groups those statuses by bits.
The two pencil study validators check their source scalars, replays, paired
grids, and matched rho controls independently. The Kummer study validator checks its raw candidate
and same-process rho source scalars, unmodified candidate trajectory, per-target
cold costs, and native price band. This packet does not replace
the separate source studies' arithmetic and scalar validators.
The pencil manifests are generation-time provenance records rather than
checksums of a later checkout or mutable ignored `target/` output. Their
historical hashes remain unchanged; the exact interpretation and current-file
boundary are recorded in
`research/pencil_sparse_20261022/MANIFEST_NOTES.md` and
`research/pencil_markowitz_20261022/MANIFEST_NOTES.md`.

The factor-base/law/Kummer graph is rendered from the same representative grid rows as the JSON summary; the
paired pencil graph uses all 36 price cells per measured curve. The coverage graph groups all 1,260 pencil workload keys by status. Their PNGs are
rendered from the same SVGs at 150/96 pixel scale for the 150-dpi PDF embed.
The three-page PDF includes all three graphs. Its table is
an editorial selection of the pooled representative rows; reviewers should
use the CSV for per-size patterns and the linked raw JSONL for target-level
analysis. The original pencil 1,260-distinct-target request remains open: 543
distinct candidate targets solved and 717 have no complete candidate solve.
