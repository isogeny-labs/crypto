# Native complete-model adapter

`tools/torus-spectrum-solver` consumes the unchanged compact compiler and
provides a public Spec/R/Q/limits solve entry point. It requests every semantic
input only after SAT; checks binding uniqueness, modulus and canonical range;
rebuilds/checks every gate; then applies the unchanged numeric support and
original sign/index/group replay. No supplied unknown enters a solve query.
Native statement **PCC-20261123-ab3e5b**, unsigned/not_covered.

| Fixed control | Observed result |
| --- | --- |
| Complete supplied models, including actual prime field | All 12 synthetic responses pass full gate/original replay; zero inverse model variants refuse |
| Two-input nonlinear system, F101 | Native SAT model passes full native gate check |
| Inconsistent variant | Native UNSAT; get-value is not sent |
| Native output cap | Truncation is explicit and the process refuses |
| Public p101/q3/k8 section target, R=(89,27), Q=[8]R | No status/model before the frozen 5000ms limit; backend reports timeout |

The synthetic response controls are **supplied models, not discovery**. The
public-target query receives no coefficients/labels/signs/scalar/witness. It
has 3511 variables/3519 equations/44 semantic inputs. Its exact query hash is
7730af2ae05f6c8b0bdd64bd43b5ed97a2acc8a528792cd6a3405c417ffb6944.
Captured stderr is `cvc5 interrupted by timeout.`; the subsequent pipe write
refused, and no get-value request was sent. The adapter records
resource_or_backend_refusal. There was no retry and no discovered curve root.
[pcc:PCC-20261123-ab3e5b]

Every captured transcript, including the exact SMT text, is in
native-v1/native-run-v1.tar.xz; RAW_MANIFEST.json supplies stream sizes/hashes
and a lossless readback record. The intentional output-cap control retains
only the declared bounded output with an explicit truncation flag. It is not
described as a complete uncapped backend response. [pcc:PCC-20261123-ab3e5b]

The deadline is an admission limit, not a measured solver rate or hardness
result. UNSAT without an independently checked certificate is an observation;
caps, parser errors and unsupported projection are never absence or route
disproof. No comparative campaign, full actual-base solve, fitted scaling,
total work/RSS or attack exponent is inferred. [pcc:unverified]

Source identity is composite: unchanged 76-input compiler archive plus the
new 9-input delta, all 85 current hashes matched; seven new crate inputs are
compiled in. Local release tests/fmt/strict Clippy pass. No scientific source
was edited or tests rerun after final qualification. Existing frozen source,
claim records, criteria and all 1834 parent ledger events are retained.
[pcc:PCC-20261123-ab3e5b]

See NEXT.md for the structural-elimination/point-comparator gate, FROZEN.md and
REPRODUCE.md for source/evidence scope. All producer standing is unverified.
