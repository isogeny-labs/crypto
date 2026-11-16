# Before-data amendment 2: x-only binary pair-table build

Status: specified before any study execution or outcome row, 2026-11-15.
This amendment follows [`AMENDMENT-1.md`](AMENDMENT-1.md) and changes only
the `rootfind-negfold-batch` candidate implementation. The exact executable
source revision must be bound by `publication-amendment-2.json` before a
study job is admitted. Earlier protocols, amendments and publication
receipts remain immutable and identify their own source revisions.

## Source change and prediction

The binary negation-folded pair table stores a key derived solely from the
sum's abscissa and retains the first `(i, j)` witness for each key. The
candidate builder now forms rows with `FastCurve::add_many_lazy` and inserts
each finite sum under `x + 1`, the same key as `FastPoint::pack() >> 1`.
It leaves ordinates unfinished during construction. Exceptional additions
are completed by the existing primitive. The row, pair and insertion order,
table representation, and abstract addition charge are unchanged.

The mechanism predicts less native field work in pair-table setup. It does
not predict an end-to-end result without the fixed paired study. The
candidate's m=3 scan remains as specified in amendment 1. The scalar arms,
F4/F5/F6 study, target IDs, seeds, six arms, 336 planned executions,
complete cold `bitop/v1` metric, exact relation controls, L2/A/A wall rule,
stopping limits, budget gate and inference policy are unchanged.

## Gate and outcome handling

Before publication, require the release parity test to match the scalar
builder's full key-to-first-witness map, entry and insertion counts,
abstract charges and m=2/m=3 decomposition answers on K13 and K17. The
test is an implementation check, not study data. A failed check blocks this
revision's publication until repaired. A passing check does not establish
an advantage. Execute only with a pushed source/spec/amendment receipt,
preserving all failures. Apply the original protocol's controls and
decision rule to the resulting records; do not pool earlier source
revisions or infer complete cost from setup alone.
