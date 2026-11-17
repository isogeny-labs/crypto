# Native Rust integration result

The quotient/trace code is integrated with the actual Rust F4 codebase through
`crypto_lib::cryptanalysis::f4_fp::certified`. The algorithms live once in
`crates/certified-relations`; the original study reexports that crate. The
default F4 route is unchanged. The separately copied native API qualifier
passes all five checks at source 0eb68e7d47b34793b3700ad3619a573e848508cb
`[pcc:unverified]`.

```rust
use crypto_lib::cryptanalysis::f4_fp::certified;
let report = certified::try_quotient_basis(&equations, variables, prime)?;
assert!(certified::verify_basis(&report, &equations));
// report.basis uses the regular F4 polynomial representation.
// report.observed_solving_degree is None.
```

The native control test compares complete reduced bases with `f4_fp::f4` on
the fixed three-/four-variable 31-bit systems and rejects truncated/timed-out
controls. The 53-bit checks preserve exact coefficients, duplicate terms,
scaled equations and certificates, including malformed/tampered-report
rejection. They do not extend the regular F4 engine's documented small-prime
range. The root-input tracer preserves null status on an incomplete run.
These are producer correctness checks, not signed execution proofs
`[pcc:unverified]`.

## Failure-driven hypothesis refinement

The first native qualifier's six-variable case exceeded the 50000-term proof
cap during original-input rewrite: 55976 proof terms, 20 polynomial terms and
degree 9. Polynomial-only copies of already-certified rules remove discarded
provenance from that containment check while retaining all mathematical
reductions and final membership checks. That alone still failed later, during
change of order, at 63952 proof terms with 8 polynomial terms and degree 4.
Both failures remain recorded `[pcc:unverified]`.

The next hypothesis selected shorter certificate paths. Input-order scanning
immediately reused each newly derived coordinate and reached dependency depth
4 on the six-cycle. Breadth-first waves use only the previously known frontier
and reach depth 3. The input-order ablation remains available as
`quotient::solve_input_order`; the six-variable regression reproduces its
budget failure with the same equations and caps `[pcc:unverified]`.

The resulting fixed seed-7 six-variable systems complete on both primes:

| Prime | Dependency depths | Final basis degree | Stored certificate product degree | Largest stored final-basis proof |
|---|---|---:|---:|---:|
| 2147483647 | [1,2,3,2,1,0] | 2 | 40 | 8336 terms |
| 9007199254740881 | [1,2,3,2,1,0] | 2 | 40 | 8336 terms |

All original-input membership, containment, reducedness and S-pair checks
remain, including the native F4 representation checker. No cap was raised.
Stored proof size is not a peak-memory measurement. Graph dependency depth,
final basis degree and stored certificate product degree are separate from
Macaulay solving degree or regularity; no reduction of those invariants is
inferred `[pcc:unverified]`.

## Evidence and limits

Sources checked (KB): unavailable in this runtime; no KB MCP tools were loaded,
and `scripts/research-kb status` exited 127 for the missing executable. The
fallback was the actual `src/cryptanalysis/f4_fp.rs` and `f4_fp/reuse.rs`, the
prior `research/jv_cover_sparse_fglm_20261018/README.md`, and the
Sage/Singular change-of-order documentation already recorded in
QUOTIENT_PROTOCOL.md. This led to reusing the existing F4 checker and keeping
quotient construction, certificate work and change of order in the paid scope.
No novelty claim is made.

The shared crate passes 27 normal tests and 28 counter-feature tests. The
study compatibility crate has its own ten CLI/driver checks. Strict shared
and root-library clippy checks are retained. `integration-development/` holds
build/pre-call/post-call manifests for three distinct immutable native
qualifiers, their raw logs and archived executable copies. Round 01 is the
initial qualification, round 02 the fixed-case diagnosis, and round 03 the
complete breadth-first qualification. Earlier receipts are not overwritten
`[pcc:unverified]`.

The focused qualifier links the real root library; it contains no copied F4
implementation. Initial broad build failures caused by sparse-checkout
historical fixtures and unrelated CLI source are archived. This is not a pass
of the entire root unit-test suite. Compiler/toolchain and feature selections
are recorded, with Redis's unused native-TLS default disabled in the qualifier.

Hypotheses were written before their respective tests in
INTEGRATION_HYPOTHESIS.md, INPUT_REWRITE_HYPOTHESIS.md and
PROPAGATION_HYPOTHESIS.md. The next runtime hypothesis is full-cost comparison
with optimized native F4 and supported mature solvers. It must charge
admission, conversion, all construction and matched checking. Scientific
timing remains gated by missing S3 checkpoint upload transport; unit-test
elapsed times supply no speed ratio. Earlier standalone timing ratios are
not reused for the root-codebase route `[pcc:unverified]`.

Factor-base collection and P/Q-labelled dependency integration remain follow-up
work. The original claim subject LEADS.md has been restored to its exact pinned draft bytes;
current evolving notes are LEAD_STATUS.md. Claim files and ledger history
were not rewritten. Producer record PCC-20261115-ce42a6 binds the final native
qualification receipt and remains unverified.

The same-day root session log and append-only PCC ledger also changed in the
parent branch. The focused follow-on PR preserves its qualified source
commits; it does not reconcile competing frozen ledger histories. That merge
dependency is separate from the recorded local correctness checks.

## Actual-root worker follow-up

Source 6ff21fd8711d33d84fa8c3eb2856e2d71899ac07 adds native-root-worker in
the focused qualifier crate. It reads the same admitted System JSON and calls
the actual optimized F4 or certified API. Common admission/conversion and
canonical output stay charged, along with the certified API's internal
provenance/native checks. This supplies the root-codebase worker needed for a
future comparison; it does not replace the missing runtime protocol/gates.

Three separately frozen CLI tests pass: identical complete 31-bit fixed-case
bases; capped F4 remains incomplete with null basis/observed solving degree;
and the 53-bit F4 control is explicitly unsupported. The worker and test
executable both retain their pre-call hashes afterward. The original five
API checks are the separate earlier qualification, not a new combined-suite
run. See root-worker-development/ and producer record PCC-20261115-461551;
standing remains unverified `[pcc:unverified]`.

F4's highest productive degree is a run observation and is exposed only after
complete termination. Certified construction still reports no Macaulay/F4
solving degree. The worker's single-call clocks exclude launch/printing and
cannot provide a speed ratio; a matched driver must pay output consumption
and identical external basis checks as well.
