# Review-packet validation

This is a deterministic conversion of frozen study grids, not a new solver.
Run from the repository root:

```
python3 research/four_followups_review_20261022/build.py
python3 -m py_compile research/four_followups_review_20261022/build.py
tectonic research/four_followups_review_20261022/REPORT.tex --outdir research/four_followups_review_20261022
sha256sum -c research/four_followups_review_20261022/SHA256SUMS
```

The builder checks each copied priced ratio against its copied candidate and
comparator cost at relative tolerance `1e-9`. The source studies separately
check recovered public scalars, matched rho, raw counter repricing, and
archive-input replay. For this packet the two priced studies contribute
2,016 rows (seven arms, eight slices, 36 cells each); seven unpriced Kummer
size rows contain empty cost fields. An unpriced row is not a failed DLP
trial and cannot support a cost comparison.

The graph is rendered from the JSON representative rows. The PDF table is
an editorial selection of the pooled representative rows; reviewers should
use the CSV for per-size patterns and the linked raw JSONL for target-level
analysis. The original pencil request remains open and its partial evidence
is linked in `REPORT.md` until the sparse continuation has its final replay
certificate and can be mapped into this packet.
