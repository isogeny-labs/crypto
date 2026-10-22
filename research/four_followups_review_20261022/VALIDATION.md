# Review-packet validation

This is a deterministic conversion of frozen study grids, not a new solver.
Run from the repository root:

```
python3 research/four_followups_review_20261022/build.py
python3 -m py_compile research/four_followups_review_20261022/build.py
tectonic research/four_followups_review_20261022/REPORT.tex --outdir research/four_followups_review_20261022
sha256sum -c research/four_followups_review_20261022/SHA256SUMS
python3 scripts/validation_gate.py evidence research/pencil_sparse_20261022
```

The builder checks each copied priced ratio against its copied candidate and
comparator cost at relative tolerance `1e-9`. The source studies separately
check recovered public scalars, matched rho, raw counter repricing, and
archive-input replay. For this packet the factor-base and law studies contribute
2,016 rows (seven arms, eight slices, 36 cells each), and ten pencil pairs
contribute 720 rows (two arms, 36 cells each). Seven unpriced Kummer
size rows contain empty cost fields. An unpriced row is not a failed DLP
trial and cannot support a cost comparison.

The companion target index has 11,528 rows: 6,300 factor-base, 2,520
projective-law, 1,448 pencil, and 1,260 Kummer coordinate records. The builder verifies
one shared workload ID for each of the 1,260 exact `(bits,p,target_index)`
keys, all source checks, seven Kummer zero outputs, zero coordinate failures,
and 1,260 matching source-checked signed Recover rows. It additionally checks
1,446 source-verified pencil solves and two null-cost cap records. The pencil
study validator checks its source scalars, replays, paired grids, and matched
rho control independently. This packet does not replace
the separate source studies' arithmetic and scalar validators.

The factor-base/law graph is rendered from the JSON representative rows; the
paired pencil graph uses all 36 price cells per measured curve. The PDF table is
an editorial selection of the pooled representative rows; reviewers should
use the CSV for per-size patterns and the linked raw JSONL for target-level
analysis. The original pencil 1,260-distinct-target request remains open: 543
distinct candidate targets solved and 717 have no complete candidate solve.
