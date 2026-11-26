# Independent curve and target selection for the native PKM panel

Frozen 2026-11-25 before looking at native F4 performance on any panel
curve. This is a selection rule; the generated roster, its source commit,
and its checksum must be committed before a timed arm starts. It implements
the `t=3,4,5`, three-curve, sixteen-target requirement in `PROTOCOL.md`
and `AMENDMENT_NATIVE_F4_20261125.md`.

For each `t` in increasing order, put `B=2^t` and
`p_min=ceil(B^3/6)`. Select the first three distinct primes `p>=p_min`
with `p ≡ 1 (mod B)`, ordered by `p`. The threshold keeps the simple
three-summand density proxy `B^3/(6p)` near one while field size grows
with map degree. It is chosen from that proxy, without measuring the
candidate, a control, or rho. Do not replace a selected prime or curve
because a later solver arm is slow or censored.

For each selected prime, scan `(a,b)` lexicographically with
`1<=a,b<p`. Take the first pair for which `4a^3+27b^2 != 0 (mod p)`,
the curve `E: y^2=x^3+ax+b` has prime order `r=#E(F_p)`, and `r!=p`.
The nonzero coefficients avoid the special `j=0,1728` families. Choose
the lexicographically first affine point `(x,y)` as `G`; every nonidentity
point has order `r` because the full group order is prime. Retain the
prime, rejected `(a,b)` count, exact order certificate, chosen generator,
`v2(p-1)`, primitive `2^t` root, `g=1`, all `B` Kummer abscissae, and
the exact lifted factor-base points. A selection program may use Sage
10.10.rc0 for point counting and independent group-law validation. It
must record its version and source hash. Curve selection and order proofs
are fixture validation. Building the Kummer abscissae and lifting the
factor base inside each cold attack process are charged as setup; the
precomputed roster is not supplied to the candidate target solver.

For each curve, select sixteen distinct nonzero scalar targets before
timing. For `j=0..15`, hash UTF-8
`pkm-native-f4-v1|t|curve_index|j|counter` with SHA-256, starting
`counter=0`, and set `q=1+(int_be(digest) mod (r-1))`. If `q` repeats
an earlier scalar, increment `counter` and retry. Store each `q`,
`Q=qG`, digest, and counter. The candidate, both equal-size controls,
and rho receive the identical ordered `(G,Q)` pairs. A runner must
verify each point and scalar relation independently before the first
timed call. These are random-looking, unplanted targets; any planted
positive control belongs in a separate labelled row.

The generator of relation attempts and mirror order is seeded by
`sha256("pkm-native-f4-relations-v1|t|curve_index|j")`; record the
32-byte seed in every raw arm. The source-pinned binary, complete process
wall and CPU, peak RSS, every failed query, matrix rank, linear algebra,
and target descent are charged to each cold target. A timeout or memory
cap is retained as censored. The same-target rho run receives the same
`Q`; no reuse credit enters the primary comparison.

The roster must be reviewed for exact factor-base size and curve order
before benchmarking. If the selected base has zero lifted points or cannot
support full rank, retain that row as a construction failure under this
rule; do not substitute another curve. The three fields at one height
are independent curve instances, but their costs are reported separately
and never pooled across hosts.
