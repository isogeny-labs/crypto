# Native finite-size extensions: implementation receipt

2026-10-30. This implements the user's request to improve the code along
review items 1–5. The earlier review/report in the parent directory is a
historical planning snapshot and remains unchanged. This receipt covers
native development correctness, not a scientific timing or scaling study.

![Implementation and the remaining packet proof](implementation.svg)

| Item | Delivered and checked | Status and remaining obligation |
| --- | --- | --- |
| 1. Selector inputs | Native generic-domain search; final-slot derivation; bounded decode cache; slot-preserving domain order; actual selector-input tensor graph | Implemented and development-verified; search remains generic, and decoder work is not free |
| 2. Solver/encoding choice | Exact greedy rank planning, checked tables/cutsets, witness-only support propagation; optional ripple schoolbook product | Implemented and development-verified; no optimal-width, SAT/CNF-XOR or performance claim |
| 3. Checked local plans | Recomputed edge partition, pins, intermediate ranks, exact cutset children and resource policy | Partial relative to the original proposal: these are finite exact conditioning plans, **not** a new small-parameter Appendix D packet schedule/proof |
| 4. Nested bases | Two-threshold validation, up-to-two-outer-point admission, outer-before-inner sparse elimination, raw-row provenance, bounded query collector and independent certificate replay | Implemented and development-verified; all storage, failures and rank costs still need measurement |
| 5. Binary specialization | Checked polynomial fields, odd-degree Koblitz groups, invertible XOR hashes, derived/verified Frobenius eigenvalues, coefficient-carrying representatives and complete orbit-selector expansion | Implemented components and development-verified; no inherited coverage theorem or full official ECC2K-130 setup/solve claim |

The public contract and examples are in
[`FINITE_SIZE.md`](../../../crates/hashed-ecdl-tensor/FINITE_SIZE.md). The
file runner adds `solve-selectors` and `solve-width`; the original `solve`
retains its backend, RNG behavior and certificate path. `solve-selectors`
produces source-v1 receipts independently checked by the existing `verify`
command. The width route can exhaust its rank/slice/work envelope even on
small arithmetic networks; that is an unknown-result error, never UNSAT.
The partial collector has a separate library certificate/verifier and does
not impersonate a source-v1 receipt.

## Correctness boundaries

Selectors invert the existing affine hash and derive canonical points with
a separate deterministic nonresidue-search stream. The last point can be
derived from the group equation. The solver sees public bases and a query,
never the random labels or their RNG. Every supplied point witness receives
membership/subgroup/sum replay before the unchanged reduction admits it.
The selector graph counts distinct decoded tuples; it does not preserve the
extra cofactor-preimage multiplicity of the original coordinate language.
Both graph languages remain explicitly selectable. No lexicographic-witness
equivalence is claimed for these new opt-in search orders.

Contraction uses exact BigUint factor arithmetic. Caller-edited plans are
rechecked against the actual graph and pins. Predicates are tabled only
after rank/vector admission; opaque high-rank predicates are not assumed
cheaply factorable. Witness search propagates exact local support, delays
high-arity predicates until sufficiently assigned, and does not allocate
count precision. Limits propagate as errors; None requires full exhaustion.

Partial rows retain slot/point column identities and all original labelled
rows. Elimination uses coefficients modulo the certified odd subgroup order;
an explicit r=7 regression needs coefficient 6, demonstrating why XOR cycle
cancellation would be wrong. The independent verifier checks original
memberships, group equations, every column cancellation and scalar replay.
The bounded collector and oracle remain separate from any performance claim.

Binary fields verify Rabin irreducibility rather than trusting a polynomial.
Koblitz construction verifies subgroup primality (including supplied checked
prime proofs), the generator, and full cardinality by Hasse uniqueness or
bounded exact counting. Odd-degree half-trace decoding is canonical. The
Frobenius eigenvalue is derived from known Koblitz polynomial candidates and
checked on the generator; representative coefficients are replayed by scalar
multiplication. Orbit domains hash canonical representatives and enumerate
their full selected orbit, including short orbits. Duplicate selectors are
allowed here; this witness domain is not a parsimonious counting promise.
XOR/orbit base distributions require their own coverage argument. The binary
API does not imply that prime-field curves inherit these specializations.

## Exact development fixtures and validation

- Prime-field fixture: `p=5, a=2, b=1, r=7, h=1, G=(0,1)`. Library tests use
  `Q=[3]G=(3,3)`; the existing CLI fixture has `Q=(3,2)=[4]G`. Hash prime 11,
  explicit thresholds/seeds in the tests; exhaustive target scalars 0..6.
- Cofactor fixture: `p=5, a=0, b=1, r=3, h=2, G=(0,1)`.
- Binary fixtures: polynomial `x^5+x^2+1` (37), model
  `y^2+xy=x^3+a*x^2+1`, `r=11`. At `a=0`, `h=4`, `G=(4,22)`, verified
  Frobenius eigenvalue 4; at `a=1`, `h=2`, `G=(10,18)`, eigenvalue 5.
- The official binary-field polynomial `x^131+x^13+x^2+x+1` passes the native
  irreducibility check. This is field validation only, not official complete
  curve/order/generator/target validation.
- Exact graph controls cover every pair of two-port truth tables and selected
  pins under unsliced and sliced plans; forged, incomplete, duplicate and
  understated plans; isolated false predicates; a 70-edge witness under a
  one-bit precision policy; and explicit exhaustion.
- Selector controls compare exhaustive actual base sets/targets, both final-slot
  policies and slot-order policies, invalid codes, cofactor/subgroup checks,
  alternative graph counts, edge forgery, full IC recovery and forged oracles.
  The CLI roundtrip also verifies exclusive receipt creation.
- Binary controls include independent small carryless products, all small-field
  inverses, canonical decompression/cardinality, subgroup associativity,
  Frobenius compatibility, XOR inversion, singular matrices, complete orbit
  enumeration and coefficient replay. Schoolbook products retain all bits.

See [VALIDATION.md](VALIDATION.md), raw development transcripts and source
manifests alongside this file for the actual final commands and outcomes.
These are bounded local correctness checks. Their durations and diagnostic
counts are not admitted runtime results or comparable performance units.
No scientific profile, finite-size ladder or frozen matched A/B panel ran.

## Sources and decisions

The [earlier review](../README.md) records primary-source and lead-atlas checks:
Bochum v0.2 `1ee44e562b1b9a50ea339f3b8f81112b5a24b96f`, TensorOrder,
WDSat and atlas `fc26d24369e56ad20995380475989c40a5e0e84f`. Prior negative
SAT/enumeration scaling and generic-oracle cost scopes remain in force;
none of the new wrappers, domains or exact algorithms proves a sub-rho
advance. No new mathematical theorem or comprehensive novelty claim is made.

The manuscript's default packet guards are untouched. A smaller-parameter
packet schedule still needs checked packing, coefficient/mode coverage,
precision/valuation accounting, reconstruction, child shrinkage and complete
cost bounds. Exact cutset admission does not discharge those obligations.
The [independent audit](../../../problems/hashed-ecdl-tensor-backend.md) stays
open. Original frozen evidence, whole seeded default-certificate controls,
strict JSON/point/reader checks and the old matched panel are preserved.

Earlier S3 checkpoint upload/readback succeeded. Native authenticated image/
execution and measurement admission remain distinct unverified prerequisites;
no chat credentials were reused. The zero-funded
[measurement card](../../../backlog/XP-20261030-hashed-ecdl-finite-size/README.md)
stays draft until its driver, exact identities/protocol, independent checker,
cost envelope and current execution-host checkpoint/admission exist.
Both Certicom challenge scalars remain unresolved.

Canonical scoreboard/progress, curve and performance graphs receive no new
measured quantity or transport edge, and remain unchanged. This new diagram
records implementation status only; there is no numerical performance graph.
