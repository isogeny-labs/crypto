# First-hit versus all-hit rootfinding relation screen

Frozen question, 2026-11-16. Does continuing the ordered negation-folded
pair-table scan after its first valid triple yield independent rows soon
enough to reduce the number of distinct target queries needed for full
rank on the fixed K13/K19/K23 signed-orbit bases? This is a deterministic
source-level screen, not a complete-call timing experiment or an attack-cost
claim. The source parent is `7b7afe6c35594b92378da5a581a0858beeb0357f`.

## Mechanism and falsifier

The current batch m=3 oracle in `src/bin/ecbench/pdp3_koblitz.rs` returns
after the first verified `P_i+P_j+P_k=R`. The relation collector in
`src/cryptanalysis/ic_boundary.rs` accepts a vector of rows per target.
Further valid `k` values may supply additional independent rows without
another target construction. A row from the same target can also be
dependent. The candidate enumerates every `k` in the production factor-base
order and retains the pair table's first witness for each key. The control
retains only the first verified hit from that identical scan.
[pcc:unverified]

If all-hit reaches full rank in no fewer target queries, or its saved
queries do not repay its additional scan probes under the stated
operation model, the screen does not support integration on that cell.
If the extra rows help, guarded production integration and a complete
same-target timing comparison remain separate obligations. No novelty
is asserted: the KB search for multiple same-target decompositions was
non-exhaustive, and the current source is the construction reference.

## Fixed inputs and controls

Build `KoblitzCurve::new(0,n)` at n=13/19/23. Use standard-subspace
dimensions 5/7/8, signed Frobenius-orbit columns, and the same torsion
batch pair-table builder as the source-pinned inline-index parity test.
Pin each ICV1 curve ID, factor-base digest, point count, pair-key count,
collision count, and seed rank against that committed test and the
collision census. Every reported triple must independently re-add to
the residual and every row must satisfy the planted scalar equation.

Use public `Q=[641]G`, `r` from the registered curve, and residuals
`R_t=Q+[a_t]G` with `a_t=(117+37t) mod r`, `b=1`. Both arms receive this
identical order; skip `R_t=O`. K13 enumerates all `r` distinct residuals;
K19 and K23 take the first 1024 distinct residuals. The step 37 must be
coprime to each `r`. The stride is a deterministic diagnostic stream, not
a model of a random collector. Both arms start with the same verified
collision rows and the torsion seed row. The original pair builder and
first-hit arm are controls; the planted scalar is a positive check.
[pcc:unverified]

## Measures and decision

Record per cell: target count, first-hit and all-hit decomposition counts,
extra hits, independent extra rows, seed rank, final rank, first full-rank
target index, first-hit scan probes and all-hit scan probes through each
arm's full-rank target, and whether Gaussian recovery yields scalar 641.
The scan model counts one subtraction and one pair probe per examined
factor-base point, plus one pair re-addition per hit. It excludes native
field arithmetic, matrix updates, allocation, setup, process wall and
the F4/F5/F6 arms; it cannot establish a complete cost advantage.

The primary local signal is strictly fewer target queries to full rank
with scalar verification and an explicit extra-scan cost. A cell that
does not reach full rank within its fixed limit is censored. Do not
select favorable targets, increase the fixed limits, or drop failures.
One source-pinned release harness run is planned. Preserve exact command,
toolchain, binary and source hashes, UTC/resource log, stdout/stderr,
machine-readable result, verifier, editable graph and PDF. Independent
replay is required before a promoted quantitative advantage claim.

`dominated_by`: the existing first-hit batch oracle and the existing
collision-harvest seed rows. `sota_delta`: unknown until this screen, and
no asymptotic change is predicted. A later integration must compare the
same target stream, factor base, source revision, answer, complete cost,
memory and matched F4/F5/F6 baselines.
