# Compact certificates: current result and 15x runtime target

The new opt-in Rust path retains an acyclic chain of local polynomial
identities instead of repeatedly expanding every proof into original-input
cofactors. At source 96f0c814d928f07e96e15361cc43a35fe7306377, the existing
six-variable seed-7 case stores 442 local cofactor terms, including all
definition nodes and final rows, versus 20852 terms in the expanded final-basis
certificates. This occurs on both primes, with identical complete reduced
bases. It is a producer storage/correctness observation, not a runtime ratio
`[pcc:unverified]`.

| Fixed n=6, seed=7 | Compact nodes | Compact stored proof terms | Expanded final-basis proof terms | Local certificate product degree | Expanded-original product-degree upper bound |
|---|---:|---:|---:|---:|---:|
| p=2147483647 | 6 | 442 | 20852 | 14 | 40 |
| p=9007199254740881 | 6 | 442 | 20852 | 14 | 40 |

The units differ deliberately: local cofactor terms reference earlier checked
polynomials. Those polynomials are also retained and checked. These counts
are not bytes, peak memory, minimal expanded cofactors or a solving-degree
observation. The final basis remains degree two; the expanded-original bound
is not reduced to 14. `observed_solving_degree` stays null for this constructor
`[pcc:unverified]`.

## Preserved mathematical obligations

For definition j, references must be original inputs or definitions numbered
strictly before j. The verifier rejects a self/future reference before
arithmetic. Each exact local identity is checked, then its polynomial becomes
an admissible earlier generator. Final rows have checked membership in that
augmented ideal; the chain supplies an inductive original-membership argument.
All original/auxiliary inputs must reduce to zero by the final basis, and
monicity, reducedness and every S-pair remain checked. The native F4 boundary
checker also checks the exported representation `[pcc:unverified]`.

The polynomial coordinate, rewrite, Bezout and change-of-order kernels are
shared with the old constructor. Its expanded arithmetic and caps are unchanged.
The compact route has its separately frozen global node, stored-term, local
degree and identity-work limits; these are not relabelled as the old expanded
proof budget. No unchecked auxiliary row, completion oracle, witness or hidden
fallback is introduced.

Public entry points are `f4_fp::certified::try_compact_quotient_basis` and
`verify_compact_basis`. The existing native-root-worker adds the `compact`
arm alongside `f4` and `certified`, so the same executable can drive the future
comparison. Old API and worker contracts remain historical source snapshots.

## Development qualification

Separate frozen executables with pre/post hashes retain:

- 30 shared release tests, including exact expanded/compact basis equality on
  existing seed-7 n=3/4/6 inputs on both fields and adversarial chain checks;
- 31 counter-feature release tests; these counters do not price total work;
- six native API checks, including both-field n=6 compact certificates and
  native F4 checking; three CLI checks, whose complete-case test compares all
  three worker arms;
- three additional CLI correctness checks against the separately frozen
  optimized release worker, covering the same contract without clock comparisons;
- strict shared normal/counter and actual-root qualifier clippy passes.

These are producer checks, recorded in PCC-20261115-39655c,
PCC-20261115-e806a2, PCC-20261115-a014f5 and PCC-20261115-3f31f7. They have no independent execution
signature. Read compact-development/ for distinct manifests/logs and immutable
copies; no unit-test elapsed times are compared `[pcc:unverified]`.

The initial clippy warning was an Option::map used for test mutation; it was
fixed before source freeze without changing an assertion. No correctness
failure or resource-limit increase was hidden. Full root unit-suite
qualification remains incomplete as documented in INTEGRATION_REPORT.md.

## What would establish the requested runtime result

COMPACT_SPEED_PROTOCOL.md freezes a three-arm comparison against actual
optimized F4 and the expanded constructor, with paid validation-inclusive
scope, complete-basis equality, A/A admission, retained failures and adjusted
paired lower endpoints above 15 against both controls. The primary family is
six variables with declared exploratory seeds and conditional untouched fresh
seeds. A storage reduction cannot satisfy this runtime rule
`[pcc:unverified]`.

No scientific timing is admitted yet: the current environment has no configured
S3 checkpoint upload transport. The maintained EKS route also requires its
commit-image, isolation, receipt and known-budget checks. The user requested
restoration here. CLI installation and a standard identity probe are complete,
but the current environment has no credential binding and its offered tools
cannot write configuration. Read compact-development/CHECKPOINT_ACCESS.md.
Continue the actual timing after those dependencies are available; do
not promote the earlier standalone denominator or development clocks.

Sources checked (KB): unavailable, with the fallback primary-source review
and its limitations in COMPACT_CERTIFICATE_HYPOTHESIS.md. The separate errata
clarifies standing of its frozen explanatory paragraph. Neither a novel proof
system nor a sparsest-representation theorem is claimed.
