# Separate-agent review and native binary replay

Reviewed 2026-11-23 UTC (2026-11-22 Pacific). Reviewer: `map_cost_audit`, a separate agent in the producing team, on the same host. This is unsigned source review and reproduction of development checks, not a signed independent PCC verdict. [pcc:unverified]

## Scope and outcome

Read the complete rank qualifier `coupled/native/main.rs`, its standalone Cargo manifest, the attempt-2 source/build/pre-call records, complete raw outputs, and the preregistered `coupled/PROTOCOL.md`. Reviewed the coefficient algebra independently before this implementation was written. No source was changed by this review. Concurrent certificate-mode work and its separate source file are outside this receipt.

No defect was found in the declared fixed-fixture rank calculation. The copied read-only attempt-2 qualifier and native unit-test executable were replayed sequentially with a 60-second timeout and `ulimit -v 2097152` for each invocation. This is a 2 GiB virtual-address-space ceiling, not a measured resident-memory result. Both exited zero. The complete qualifier output is byte-identical to attempt 2. The unit-test executable reports nine passed, zero failed, zero ignored. No rebuild was performed; these observations concern the already source-bound binaries. [pcc:unverified]

## Source binding

The four current files matched the original attempt-2 build manifest before replay and remained unchanged across the two calls:

| Input | SHA-256 |
| --- | --- |
| `coupled/native/main.rs` | `585172d4871b3b55591d1562eca95fb93ff44f9d08d0fb1bf92f2138f7e663ee` |
| `coupled/native/Cargo.toml` | `72fed26cf35c939e85781de83ebdfd3ec498b441be77da45448f933fa402a2d5` |
| `coupled/native/Cargo.lock` | `0756c7a19ceb2186aa2031934e34b29091bbc2ff526c3b9e52420aaa064ac2ac` |
| `src/cryptanalysis/semaev_prime_poly.rs` | `c8249ba825c946dbdb37fa5d6762f8f32d15320344693443e8af0d52f122784a` |

The immutable qualifier SHA-256 is `b7be1544faf395e62fea1461a54d356eec37175aedbeae953619e8fcde47aa1f`; the immutable test executable SHA-256 is `28054d0e58dd2f8ea1b7c1ad13c4f9680ed400673356dc640fa1419f7b3ce2a8`. Both had mode 0555 when inspected. The source-to-binary association relies on the retained producer build manifest; this review did not independently rebuild that association.

## Mathematical and implementation inspection

- `factor` expands the symmetric S3 polynomial correctly as coefficients indexed by x-degree and t-degree. Its repeated off-diagonal coefficient is `-2*a*z-4*b`; the mixed coefficient is `-2*(z*z+a)`.
- `partial_norm` multiplies those factors with x exponents reduced modulo d while retaining the complete t coefficient array. It implements the product over every selector root; it is explicitly an enumeration/product correctness oracle, not a faster extraction algorithm.
- `selector_roots` checks d divides p-1 and verifies exact cardinality and return to one. Its bounded primitive-root search is sufficient for the fixed inputs that passed; it is not a general availability theorem.
- `eliminate` tracks row-swap signs and pivot products without normalizing pivot rows. It gives the correct determinant when used on the square first-d-column minor. Rank of the full rectangular coefficient matrix is reported separately.
- The cusp columns have coefficients `(-1)^j binom(2d,j)` in row `-j mod d`. The resulting determinant sign is `(-1)^(d-1)`. The characteristic checks ensure the factorial denominators and these binomial coefficients are nonzero.
- The t^(2d) coefficient vanishes modulo x^d-1. For d>=2 the next coefficient is checked against `-4*d^2*(x+a*x^(d-1)+b*x^(d-2))` with colliding residue terms added, including d=2. At d=1 the routine checks the vanished top coefficient but does not separately assert this next-coefficient formula; this matches the protocol's explicit d>=2 scope.
- The nonsingular exception over F97 with a=-1, b=0, d=2 is retained: H=16*t^2 and rank one. This correctly prevents interpreting generic full rank as a universal nonsingular-curve assertion.

The nine unit tests comprise three qualifier tests and six unchanged imported-module tests. The altered-cusp-coefficient control fails its predicate as intended. The specialized Sylvester controls include degree-drop cases; they use the existing scalar S3 and formal-resultant code. The bounded scalar evaluations do not constitute a universal polynomial proof.

## Replayed observations

The qualifier emitted 40 rank cells, 2,690 scalar-product checks, 270 Sylvester checks, 269 cusp coefficient-column checks, 20 cusp determinant checks, 40 leading-coefficient-check invocations, and one explicit nonsingular rank exception. These are correctness check counts, not field-operation or relation-yield measurements. [pcc:unverified]

The ordinary project fixture has full rank d at exactly the twelve selected Prime31 degrees `1,2,3,6,7,9,14,18,21,31,42,63`. This does not mean every integer degree through 63 was tested. At d=63 the recorded first minor is 2016915671, t-degree is 125, and there are 7,307 nonzero coefficients. The qualifier did not execute d=1302. [pcc:unverified]

## Limits on interpretation

Coefficient separation rank is different from independent relation rank. No target decomposition, signed point lift, subgroup-order certification, complete DLP recovery, timing comparison or cost advantage is established by this qualifier. It checks nonsingularity of the noncusp fixtures; any broader ordinary-curve classification or group-order fact remains inherited evidence.

The written generic determinant argument applies to exact linear separation of the named polynomial representative. It does not rule out compact arithmetic circuits, alternative zero-set representatives, different selectors or target-specific quotients. Its Schwartz-Zippel coefficient-sampling bound is not automatically a probability statement for prime-order-curve selection or a guarantee about the fixed d=1302 project instance.

The source and outputs are stable for this narrow replay. The whole-repository build, formal Lean proof, signed independent validation, and any later certificate-mode checks remain separate gates.

## Receipts

- `receipt.json`: machine-readable provenance, limits and outcomes.
- `qualifier.pre-call.txt`, `qualifier.post-call.sha256`, `qualifier.stdout.jsonl`, `qualifier.stderr.log`, `qualifier.exit-status`: qualifier replay.
- `tests.pre-call.txt`, `tests.post-call.sha256`, `tests.stdout.log`, `tests.stderr.log`, `tests.exit-status`: native test replay.
- `source-binding-check.txt`, `source-binding-check.exit-status`, `replay-source-stability.exit-status`: source checks.
- `qualifier.byte-comparison.exit-status`: zero, byte equality with attempt 2.
- `output-summary.json`: structural selection/counting of the recorded JSONL; it does not recompute the algebra.
