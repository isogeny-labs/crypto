# Independent HKY replay of the qualified closure panel

2026-10-25, after the 158 registered D2/D3 native attempts ended, before this
replay's outputs. Retain all producer rows. Reconstruct each selected query
from its full input, verify the input/process/raw-row hashes and SAT/UNSAT
class, build its equations by both independent library and u128 field-polynomial
builders, and require exact equation-set equality. Recompute closure with the
separate HKY round-and-parallel-elimination engine, over all monomials through
the same degree. Compare the fixed-point verdict and nonrefuting rank. A
refuted saturated space may use a different early-stop rank convention;
require both engines to detect 1 and prohibit any positive-control refutation.

Use one thread, HKY_CHUNK=1024, a 2-GiB address-space cap and 180 seconds per
case. Retain every mismatch/cap/failure. HKY independently checks mechanics
and verdict/rank; it does not reproduce or validate the producer's operation
count with a second cost model. Keep that cost limitation explicit. The native
summary recomputes per-degree/class coverage and solver-word totals from raw
data, leaves any incomplete total null, and records actual replay coverage.
The 55 negative and 24 positive selections preserve nine/eight quota shortages.
No additional degree-four, global minimum-guess, full-price or DLP claim follows.
