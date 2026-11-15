# Curve25519 torus/S4: native Rust qualification

## Current four-point follow-up

The [four-point report](reports/four-point-v1/REPORT.md),
[diagram](reports/four-point-v1/pipeline.svg) and
[inspected PDF](reports/four-point-v1/report.pdf) describe the new
[Rust summation-tree crate](../../tools/torus-s4-tree/README.md).
The actual-field, four-coset construction has 454 variables and 455 quadratic
constraints; its known witness and all eight raw cofactor sections pass
producer checks. No large-base solver was run. PCC-20261115-02fa5a.
[pcc:unverified]

The [frozen geometric draft](theory-v1/energy-lemma.md) proposes an average
four-sum collision bound under random admissible base shifts. Its
[source addendum](theory-v1/SOURCE_ADDENDUM.md) identifies the degree theorem
and related prior work. [Lean arithmetic](lean/PROOF_MAP.md) discharges only
four concrete numerical dependencies; geometric formalization and independent
review remain open. PCC-20261115-1ea4c1 and PCC-20261115-2deca4.
[pcc:unverified]

The conditional cost target is D=soft-O(B^gamma), gamma<1, plus complete
distribution/rank/family/phase bounds. The
[named proof/cost obstruction](../../problems/curve25519-torus-four-point-decomposition.md)
and [isolated runner gate](../../problems/curve25519-singular-isolated-runner.md)
give explicit acceptance conditions. The broader library check is
[incomplete](root-library-compatibility/STATUS.md), not a pass.
[pcc:unverified]

## Historical three-leaf qualification, revision 2

The text and original report below describe the preceding three-leaf scope.
Later root recovery is in the cryptanalysis consumer; the current report
above supersedes its earlier pending-implementation statements.

This change ports the compact Chebyshev/Edwards-y S4 circuit into a standalone
[reusable Rust crate](../../tools/torus-s4/README.md) and supplies development
correctness controls for the actual Curve25519 field. The consuming harness
and route rejection criteria live in `isogeny-labs/cryptanalysis`, study
`research/curve25519_torus_native_20261115`.

The producer release log reports six arithmetic/circuit tests passing; exact
coefficient dictionaries match the historical M=5,25,79 input serializers.
The actual-field planted witness checks every quadratic equation and its
signed curve-point sum. These are development observations pending signed
independent verification. [pcc:unverified]

The optional native Singular control reports expected nonunit/unit basis
classifications for the two M=1 actual-field controls. The preserved output
is a backend observation, not a checked ideal-membership absence certificate.
No full-M timing or solver-root recovery was run in this change. [pcc:unverified]

| Input | Field bits | Variables | Equations | Purpose |
|---|---:|---:|---:|---|
| M=1 | 255 | 17 | 18 | Functional control |
| M=25606137078333158825 | 255 | 395 | 396 | Circuit and planted-witness qualification |

Counts above are producer structural observations. [pcc:unverified]

Evidence: [final native tests](native-final-tests.log),
[backend test](backend-tests.log), [exact backend records](backend-control-v1/),
[frozen historical inputs](../../tools/torus-s4/tests/fixtures/),
[diagram](reports/pipeline.svg), [PDF](reports/report.pdf),
[report source](reports/report.tex). PCC observations: `PCC-20261115-29168c`
(initial source revision), `PCC-20261115-1f2703` (native backend); final source
revision is recorded by its separate claim in this PR. All remain unverified
unless `pcc standing` reports otherwise.

Domain: E: v^2=u^3+486662*u^2+u over p=2^255-19; recognition and point-sum work
here occurs before cofactor clearing. The full-M pilot uses one torus coset.
The complete four-coset construction, all cofactor translates, relation yield,
root extraction, and cost/scaling campaign remain separate obligations.
No independent novelty determination is made by this port.

The future full-M native solver run requires a supported EKS image/command,
live capacity and budget remainder, committed immutable inputs, and the
existing isolated execution gates. No EKS/AWS client is available in this
workspace; the inspected GitHub workflow image contains ecbench, not this
Singular workload. No substitute integration fixture was dispatched.

Sources informing this implementation:

- `src/bin/curve25519_family.rs` and `src/ecc/field.rs`: repository parameter
  and public-input arithmetic conventions; existing machine-word F4 APIs do
  not accept this actual-field input.
- [RFC 7748](https://www.rfc-editor.org/rfc/rfc7748): curve and subgroup parameters.
- [RFC 9380](https://www.rfc-editor.org/rfc/rfc9380): Elligator map conventions.
- [Petit–Kosters–Messeng, PKC 2016](https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf):
  algebraic decomposition and explicit solver assumptions.
- [Buresh-Oppenheim et al.](https://www.cs.toronto.edu/~toni/Papers/homo-icalp.pdf):
  static Nullstellensatz bounds must not be transferred to iterative polynomial
  calculus or arbitrary Groebner solving degree.
- The current campaign's historical torus/circuit fixtures were imported as
  public regression data. Python execution was not extended or rerun.

KB receipt: no research-KB MCP is exposed and the wrapper exits 127 because its
executable/catalog is absent. Lead-atlas directory listing succeeded, but
membership/tower YAML fetches returned decoding errors. The files and primary
references above were the fallback; absence of search hits is not novelty evidence.

Canonical leaderboard/progress graphs are unchanged: this PR provides native
qualification and no admitted performance comparison or graph edge.
