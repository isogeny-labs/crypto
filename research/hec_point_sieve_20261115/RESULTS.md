# Point sieve with sparse public labels: real-stream correctness gate

Protocol350ab8dce8was committed before implementation. Class: known-method
baseline and engineering prerequisite; no timing, novelty or10xclaim.

## Complete terminal outcome

Six reused family0fixtures, seed indices8/9/10,128batches per target.
Worker target() takes curve,D1,D2,ell,FB,H,public projected places and seed;
the expected k is confined to main's harness and never passed to target().
The jump walk carries only two public scalars a,b; every batch checks
D=aD1+bD2. It carries no growing factor-base-step vector or per-batch unknown.

|g|p|H|Recovered /3|Admitted rows|Emitted pure rows|Final rank range|
|---:|---:|---:|---:|---:|---:|---:|
|2|1009|32|3|11738|10298|33|
|2|16381|128|3|49088|25118|129|
|3|127|26|3|1089|975|27|
|3|1009|101|3|3024|1772|102|
|4|31|14|3|94|88|15|
|4|127|38|2|203|142|36--39|

All65236admissions pass their unreduced whole-Jacobian principal identity.
All65236lazy/dense insertion prefixes have identical emitted Option<Row> and
union/pin counts. All38393emitted modular rows pass the projected group gate.
Every returned k passes kD1=D2and the separate expected-scalar comparison.

The g4,p127,seed10target is **capped**, not solved:57admissions,21unions,
36pure rows,rank36with39columns. No cap extension, target omission or seed
replacement was made. This falsifies guaranteed128batch recovery across this
specific cohort; it does not contradict the principal-divisor identity.
Low-degree D batches are explicitly skipped and counted in the raw output.
All128batches are consumed even after recovery for this stream gate; it is
not an end-to-end stopping-time benchmark and cannot be priced as one.

## Small-field oracle and negative controls

396cases compare touched-bucket outputs to exhaustive lambda/root enumeration
on g2/3/4thread curves over11/13with valid c1/2, full and half base. The
independent enumeration uses the SAME>=g-1distinct non-u-root admission policy;
no completeness claim covers cases outside it. All237admitted relations pass
the raw group identity. Observed coverage:64low-degree skips,594skipped
u(x)=0points,92repeated-root admissions,30ramified admissions.

Separate default-build controls exercise all three genera over101. Six negative
checks flip the public challenge sign or a non-2-torsion residual sign, and
the group gate rejects each. These controls supplement the output-set test;
they are not a substitute for the exactness of group arithmetic itself.

## Cost evidence and limitations

Sieve uses canonical point pairs, one u/v evaluation and u inverse per x,
both signs except y0, touched ordered buckets, and exact removal of distinct
known roots. Residual degree<=2uses explicit roots; reconstruction checks the
full degree-g+1polynomial including multiplicities. The on-row carries both
constant and target coefficients through all forest substitutions. Only after
emission does the constant move to RHS with a minus sign.

A conservative Hasse interval supplies a candidate N=h*ell; the gate checks
ell²>upperbound, h!=0modell and Nannihilates EVERYcanonical FBplace. This is an
annihilating-multiple certificate, not an independently counted Jacobian order.
Each projected small place,D1,D2is separately checked for ellannihilation.
The unreduced principal gate remains necessary and is performed first.

Real-stream retained vector estimates favor lazy by1.18..4.17across targets,
including the capped one. These are not peak RSS or runtime ratios. Total
expansion can still be large: g2p16381seed8visits787342expression nodes and
929975arcs across8366emissions. The sparse representation avoids H-wide work
on each union; it does not make forest expansion free. Existing dense echelon
is deliberately retained, so sparse-LA scaling is not established.

The tiny fields, reused curves, fixed public-label walks and producer-only
replay establish this implementation's checks, not a general yield/rank law,
statistical timing confidence, mathematical novelty or a10xcomplete speedup.
No previous ratios are multiplied and no closed record/default/admitted row
is changed. The known sieve remains prior art, as recorded in PROTOCOL.md.

## Reproduction / next gate

`gate.sh` builds the native library and discovers its rlib through Cargo JSON,
compiles gate.rs optimized and controls.rs with default settings, executes both
and emits source/fixture/library hashes and native jq SUMMARY.json. Set
HEC_SIEVE_GATE_OUTto a scratch path, stable Rust on PATH and a separate
CARGO_TARGET_DIR. Six exact original fixture copies are included under fixtures/.
All scientific execution/reduction is native Rust/jq, with no cloud worker.

Next: freeze a complete cold solver protocol that stops at recovered target,
records rank caps as censored, charges setup/jump walk/sieve/maps/expansion/
algebra/verification and compares BOTHforest variants against strongest
existing same-target controls with A/A. It must expose the g4supply/rank tail,
not choose a larger cap after seeing it. Novel mathematics and the full10x
objective remain outstanding after this prerequisite.
