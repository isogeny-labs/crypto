# Symbolic signed-graph reduction for pencil relations

This is a mathematical implementation step toward the still-incomplete
1,260-target pencil cohort. It contains an exact finite-field row reducer,
independent unit tests, and four field-identity Lean lemmas. It has no curve
interface and returns symbolic linear constraints, not a target scalar.
No cohort, EKS job, or timing experiment was run for this packet. Candidate/rho
cost and the remaining 717 candidate outcomes are unmeasured here.

## Inputs and outputs

Each input equation is homogeneous, `sum a_i X_i + sum b_j S_j = 0`, with
at most two distinct surviving large variables `X_i` after duplicate terms
are combined. Small variables `S_j` remain symbolic. An arbitrary prime
modulus fitting `u64` is accepted, including characteristic two; integer
arithmetic uses `u128` intermediates. The constructor checks primality using
the deterministic seven-base Miller--Rabin test for 64-bit integers.

The forest stores `X_i = scale_i X_parent + offset_i(S)` and uses union by
component size. A root may be anchored to a symbolic expression. Inserting
an equation returns a component join, a new root anchor, or a constraint
among small variables only. Empty constraints are dependent rows. All
emitted constraints must be retained by the caller. Malformed indices and
more than two large variables return errors before modifying the forest.

No relation is silently dropped. A future pencil adapter must explicitly
account for the collection work of rows with three large variables and any
admission rule. Its small-base selection and stopping rules need a frozen
protocol before new performance data.

## Algebra and proof boundary

Substitution into an input row leaves zero, one, or two free root variables.
With two roots, solving for one nonzero coefficient creates an equivalent
substitution and decreases the number of free roots by one. With one root,
the equation determines an anchor. With no roots, the equation is exactly
the emitted small-variable constraint. These reversible row substitutions
preserve the projection of the solution space onto the small variables.
Equivalently, the small-only part of the input row space is spanned by the
emitted constraints. A signed cycle need not create a small-only relation:
when its residual root coefficient is nonzero, it instead anchors the root.

`SignedReduction.lean` checks the pivot equivalence, substitution identity,
balanced-cycle equation, and unbalanced-cycle anchor over any field, with
nonzero pivot assumptions explicit. The induction over forest state and
the bridge from Rust operations to those field identities remain open.
The full implementation is therefore tested, not formally verified.

The independent Rust reference uses dense RREF and brute-force inverses in
small fields; it does not call the graph's field or elimination helpers.
The fixed regression suite compares the emitted and eliminated row spaces
after each of 40 prefixes for 128 deterministic streams over each of
`F_2,F_3,F_5,F_7,F_101`: 25,600 prefix checks. Additional tests cover signed
cycles, duplicate cancellation, malformed inputs, composite rejection and
a prime near the upper `u64` limit. These are development tests, not cohort
measurements, random-sampling claims or performance evidence.

An additional targeted test builds 32-variable components in balanced merge
order, anchors one root, and queries all remaining variables while preserving
four independent symbolic small variables. It emits only zero constraints
until a final intentional rank-one constraint. This avoids the weakness of
generic streams whose small-column rank can saturate early. It also exercises
a union chain over the near-`u64`-limit prime; the dense reference remains
restricted to the five small fields.

## Sources and reuse assessment

- Gaudry, Theriault and Thome, *A double large prime variation for small
  genus hyperelliptic index calculus*, February 2005 version, sections
  2.3 and 3.1: [primary paper](https://cacr.uwaterloo.ca/techreports/2004/corr2004-29.pdf).
  Its graph construction distinguishes a full relation from a cycle that
  leaves one large prime. Its complexity result assumes genus at least
  three and its stated relation model; no complexity claim from that paper
  is transferred to this genus-2 pencil stream.
- `src/cryptanalysis/three_leads.rs`, `SignedGraph`: existing scalar-label
  affine potentials. This packet extends the label type to symbolic terms.
- `src/cryptanalysis/gaudry_cubic.rs`, `LargePrimeEliminator`: existing
  arbitrary-coefficient elimination with protected small columns. It uses
  pivot chaining; this packet uses size-balanced forest substitutions.
- `research/pencil_markowitz_20261022/PROOF.md` and `REPORT.md`: the prior
  upper gate and replay already ended at the 25-million-coefficient cap.
  This packet neither repeats those attempts nor overrides their stopping rule.

The deterministic Miller--Rabin base set is the established 64-bit witness
set `(2,325,9375,28178,450775,9780504,1795265022)`; its primality theorem is
not part of the Lean proof. Its use is an implementation dependency, tested
on the suite's listed primes and composite regressions.

## Reproduce

```
cargo test --locked --offline --manifest-path research/pencil_large_prime_20261029/Cargo.toml
cd research/pencil_large_prime_20261029
lake build
```

The Lean toolchain and mathlib revision are pinned in `lean-toolchain` and
`lakefile.toml`. The retained direct compiler log uses that same dependency
environment from the existing projective-law proof checkout.

## Remaining integration work

1. Freeze the small-base partition and admission policy on the original
   1,260 public workload identities. Preserve Jacobian relation checks.
2. Account for every vector traversal, coefficient operation, parent hop,
   allocation and peak retained payload, plus downstream small-row elimination.
   Union by size bounds tree depth, not symbolic payload density.
3. Add an independently checked collector adapter and matched rho control.
   The existing operation-price grid cannot be copied onto this new kernel.
4. Run admitted cohorts under their resource and recovery contracts and
   retain every cap, failure and null cost. Report full cost only after
   completion and independent source verification.

The report, graph and `checks.csv` expose these boundaries for review. No
claim of lower memory, lower runtime, sub-rho cost, or full-cohort completion
is made.
