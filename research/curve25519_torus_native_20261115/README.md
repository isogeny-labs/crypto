# Curve25519 torus/S4: native Rust qualification

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
