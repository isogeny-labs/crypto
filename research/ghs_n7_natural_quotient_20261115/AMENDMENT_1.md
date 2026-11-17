# Protocol amendment 1: exact second Frobenius order

The first frozen source run at 07:46 UTC failed before producing a
coefficient result. Its assertion showed that the enumeration included
`gamma2=d in k*` when the V component was zero. Such an element has
sigma-order T+1, not `(T+1)p`, and is outside W1. The raw failed run
is retained in `reports/run-20261115T0746Z/`.

The source and protocol now explicitly require `gamma2=v+d` with
`v in V_p\{0}` and `d in k*`. This restores the published W1 domain;
all predictions and controls remain as originally stated. A new source
hash, checkpoint, and write-once run record are required.
