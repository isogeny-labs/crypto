# Corpus coverage reconciliation

Source inspection on 2026-10-30, branch `codex/ml-dataset-sat-prefix-20261030`,
runtime correction `356e6af19ff7`. This is an implementation inventory, not
a generated-cell census or evidence that every supported profile completes.
The full requested cross-product remains Koblitz/binary/prime, curve and twist,
n23..31 and47..61 including p=2^61-1, m2..15, multiple generators and keys.
Unsupported, capped, interrupted and failed cells remain explicit outcomes.
Verified cloud dataset cells: **0**.

## Current method boundaries

The authoritative machine-readable description is `crypto dataset capabilities`,
implemented by `src/ml_dataset/solvers.rs::capabilities`. Current source says:

| Adapter | Current domain | Completion or measurement boundary |
|---|---|---|
| enumerate | All three families, m2..15 | Bounded signed finite-domain PDP; a relation is not a scalar recovery. |
| bsgs | All three families | Independent ECDLP baseline; setup/baby table and giant probes retained, subject to caps. |
| SAT | Binary/Koblitz, m2..15 | Chained S3 with native semantic events; new exact finite-coordinate gate retention and causal export await adapter integration and successful executable validation. |
| Weil | Binary/Koblitz, m2 | Complete selector ANF and lift map; construction-only adapter explicitly sets solver_completed=false. |
| F4 | Binary/Koblitz m2; prime m2..15 below2^32 | Supervised wide-prime profile currently m2 only through2^61-1. Affine exceptional branches prevent general negative PDP conclusions. |
| F5 | Binary/Koblitz m2 | Boolean matrix row criterion plus splitting; per-row signature/rejection witnesses missing. |
| F6 | Binary/Koblitz m4 | Native geometric four-sum pair index; this name does not denote a general Groebner engine. |
| root | All three families, m2 | Root/lift certificates and bounded internal traces; higher-arity elimination and complete native cost verification missing. |
| PKM | Prime m2/3, p<2^31 | Kummer-tower F4 plus finite-grid extraction; private elimination operations and larger profiles missing. |
| double-large-prime | All three families, m2..15 | Verified two-residual elimination, admitted signed Frobenius or identity/sign actions, sparse filtering and determined-column recovery; private filter/pivot stream incomplete. |

T887's separate wide-prime dynamic work is not presumed integrated into these
branches. Its source changes and receipts require explicit reconciliation.

## Factor bases, symmetry and learning axes

`CaseConfig` currently chooses factor-base size, not a versioned construction
policy. `curves.rs` constructs small-x prime-primary projections with cyclic
membership and both signs. It retains candidate provenance, negation and
Frobenius action indices, complete point-orbit witnesses and stabilizers, while
restricting computed membership logs. An orbit witness does not mean the selected
factor-base population is closed under Frobenius.

After completing SAT integration, the next bounded extension should add explicit
policy identities and verified orbit-union selection. Freeze that contract before
edits. Bind policy, exact point population, action/domain witnesses and any caps
through curve verification, solver inputs, relation transcripts and feature IDs.
Compare model choices only on matched public inputs; construction logs and labels
remain restricted. New policies must preserve historical plan/cell identity or
introduce an explicit schema version.

## Labels and measurements that remain separate

- Constructing Q=[k]G proves a known answer, not independent ECDLP recovery. Keep
  instance_constructed, known_answer_verified, pdp_witness_verified,
  solver_completed, scalar_recovered_without_witness and scalar_replay_verified
  distinct. Retain attempts that fail to recover any scalar.
- Retired instructions require admitted hardware counters and explicit thread,
  privilege, multiplexing and interruption coverage. Abstract literal/parity
  visits and field operations are different units. Missing hardware counts stay
  null with reasons; no elapsed-time conversion is valid.
- A checked polynomial identity is a local consequence certificate. Witness
  cancellation, maximum observed degree and solver caps do not establish exact
  first or last fall degree. Keep exact labels unknown without the required
  closure/span or mathematical certificate.
- SAT decisions are observed imitation targets. Certified learned clauses may
  enter later causal contexts; future actions, outcomes, scalars, provenance and
  costs may not enter public model inputs. No optimal-policy claim follows.
- S3 controller readback proves custody of the specific saved objects. It does
  not establish worker IAM admission, a generated cloud corpus, model training,
  a solver improvement or a cryptanalytic theorem.

## Delivery gates still open

The consequence CLI has passed retained curve/twist and corruption checks; its
complete library rerun is active after exact missing-fixture restoration. SAT
source and review corrections are committed, but the first execution attempt
stopped before compilation on a missing local dependency manifest. Complete
dependency restoration, scoped tests, adapter coordination, actual nonzero
PDP learning-transition fixture, full checks and PR delivery remain open.
Worker S3 access, fresh cumulative resource/budget admission and independent
worker output restore/replay remain separate prerequisites to a cloud wave.
