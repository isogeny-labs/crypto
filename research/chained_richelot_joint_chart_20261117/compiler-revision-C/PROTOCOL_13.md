# Joint charts for the eight CT49 nondegenerate Richelot pairs

Prospective finite correctness unit, frozen before new Rust source/compiler or
native calls. This uses the actual four CT49 source sextics and eight j1/j2
target presentations. It is a model/factor bridge into the existing generic
Richelot constructor. The complete operational divisor map and dual[2] are the
next task. No performance experiment or new field-law implementation is planned.
[pcc:unverified]

## Hypothesis and primary sources

For each frozen pair f=lambda*product(q_i), H=product(B_i)/(lambda*Delta^3),
there is a rational a with nonzero squares b^2=f(a), c^2=H(a). A common shift
then gives monic source F=w^6 f(a+1/w)/b^2 and monic target
G=w^6 H(a+1/w)/c^2. Reusing the exact checked cubic-field/FpPoly backend,
the normalized factors should produce raw Richelot target (bc)^2*G.

Primary review: full CT49_NONSPLIT_RICHELOT.md and CT50_BIDUAL_RETURN.md,
exact ConsumerE079 nonsplit.rs/CubicField and nonsplit_map.rs constructor,
RootEB3 RichelotAlgebra and balanced-chart sources, and Galbraith corrected
chapters8.3.8/10 and10.4 (complete relevant arguments read in unit12).
The raw recipe, determinant signs and eight literal targets are hypotheses to
check from those sources, not accepted because an earlier producer wrote them.
No original JS/Python/C++ research driver is run. [pcc:unverified]

The enabled tool registry has no research-KB capability. Root's referenced
controlled-experiment/queue-experiment skill paths are absent in this checkout's
HEAD; no missing skill is treated as approval or prior-art absence. Conductor
check is attempted on this exact new path; when unavailable, use the documented
unconfigured fallback with a local scoped task card. No novelty claim is made.

## Frozen inputs and deterministic policy

`fixtures-13.csv` has exactly8 ordered rows and21 fields each: name,lambda,
g0,g1,g2,j,Delta,seven f coefficients,seven H coefficients; all ascending.
The four moduli are g=t^3+g2*t^2+g1*t+g0 over F101. Define
a_i=t^(101^i) and q_i=(x-a_i)(x+a_(i+j mod3)). All coefficients and model
names are fixed before execution; no replacement fixture or twist selection.

For every row, enumerate all101 a in increasing order and print f(a),H(a),
the least nonzero square root of each (or absence), and eligibility. Choose
the least eligible a only after completing that row's full search. Preserve all
808 candidate records. If a row has none, retain its bounded absence and
continue the remaining searches; report the bridge incomplete at the end.
Do not widen the field, choose separate shifts, change a model or call that an
isogeny obstruction in this unit.

## Positive obligations

1. The existing checked cubic constructor accepts each literal modulus; its
   branch orbit has length3. Reconstruct q_i and check f=lambda*product(q_i),
   the literal nonzero Delta, all cyclic Frobenius relations, every coefficient
   of the original raw target H, and squarefreeness of f/H. These are exact
   coefficient checks, not point-count or zeta comparisons.
2. Put alpha_i=q_i(a), S=product(alpha_i)=b^2/lambda. Form
   r_i=w^2 q_i(a+1/w), Q_i=r_i/alpha_i. Compare this coefficient transform to
   the monic polynomial with roots 1/(a_i-a),1/(-a_(i+j)-a).
   Check all6 distinct transformed branch roots, their source equation and
   their full Frobenius permutation. No denominator is inverted before its
   nonzero check.
3. Reconstruct the full source model independently by binary substitution in
   the literal base sextic. Check product(Q_i)=F, both model inverse
   substitutions, Delta'=-Delta/S, and the3 full bracket polynomials
   B'_i=-w^2 B_i(a+1/w)/(alpha_(i+1)*alpha_(i+2)).
4. Check raw_target_new=product(B'_i)/(Delta'^3)=b^2*w^6*H(a+1/w).
   Every coefficient must descend to F101, leading coefficient must be
   (bc)^2, and division by this fixed square must give G coefficient by
   coefficient. Check squarefreeness/degree6/monicity of F/G.
5. All6 factor permutations are valid positive reorderings. For every one,
   the determinant has its parity sign and the complete raw target remains
   unchanged (48 reordering checks). Do not call an odd permutation invalid.
   All4 choices of signs of b,c are valid model normalizations (32 checks);
   record their scale bc, without claiming a divisor-level dual orientation.

## Negative controls and failures

- For each of the four distinct sources, j0 gives determinant0; reject before
  inversion (4 controls). It is a degenerate kernel, not a failed infrastructure
  call.
- For every nondegenerate row, compute the original recipe with omitted lambda
  and require its whole target to differ from the fixed literal H.
- Flip the required sign of Delta' while keeping the prescribed b,c/model
  normalization fixed; require the whole computed target to differ from G.
  This detects the specified normalization error, not an intrinsically invalid
  curve or all possible alternative isomorphisms.
- Add1 to Q_0's constant coefficient; the factorization validator must reject
  its complete source product (24 nondegenerate semantic controls total).

Accepting the bridge requires8 completed rows,808 candidates,48 permutations,
32 legitimate sign choices,4 degenerate and24 other detected controls. A
missing joint point is a preserved bounded negative result. An arithmetic
mismatch is a failed normalization/implementation claim. Compiler/download,
timeout or resource failure is infrastructure, never semantic rejection.

## Native implementation and custody

Reuse the entire unchanged FpPoly/Fp2 modules from unit12 and the exact
CubicField struct/impl from ConsumerE079. Preserve full primary files and a
byte-pinned extraction receipt; wrappers/new chart code remain separate.
No rederived general field laws, new Lean declarations or modified verifier.
Compile a standalone Rust library test qualifier with the four existing locked
registry packages. The new test is the only executed test; inherited backend
tests remain compiled/Clippy-checked and already have their unit12 result.

Publish exact new source and lockfile normally before any compiler/native
call. Rust1.99.1/release, one compiler worker/codegen unit,120s per phase, one
test worker. Vendor the locked dependencies offline, hash every vendor file
before build, and use explicit Cargo source replacement to those guarded files.
Use distinct build/pre-call/post-call manifests; never replace a pre-call one.
Compile without running, copy a separate chmod555 qualifier, hash it before
calls; execute only the frozen test. Retain all stdout/stderr/exits and complete
traces on every outcome, then strict Clippy/fmt/source/dependency/qualifier
guards. No timing result is a benchmark or a cold-cost claim.

Retain the complete declared custody packet: source inputs, vendored dependency
manifest/tree, separate qualifier, drivers, all logs/exits and traces. Build
cache is explicitly disposable and is excluded from the scientific packet;
record that exclusion. Guarded dependency bytes are also retained in the exact
unit12 archive if a duplicate loose copy is compacted. Freeze/report outcomes
without a corrected native rerun. A later revision needs a new prospective
record. Before build, require20MB of private RAM storage available; record
actual space and preserve any resource failure without increasing120s.

Only this bounded correctness component is authorized locally. No original
fullLP/full-class population, EKS build, benchmark, paid execution or locked
crypto-autoresearcher access. Existing budget/admission/store/PCC gates remain.
All records remain unsigned until the configured independent signer acts.
[pcc:unverified]
