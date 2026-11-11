# Resume checkpoint — cubic norms and balanced genus2

Read `QUEUE.md`, the latest dated session-log entries and actual Git status
before extending code. Do not repeat completed work solely because a prior
session ended. Claims remain unsigned/unverified. [pcc:unverified]

## Durable sources

- Crypto branch `research/chained-jacobian-transfer-20261110`, existing PR3877;
  last runtime-changing commit75a737b8366ec6c78ed8672886423fcbe69853a2.
  The cubic checkpoint adds an independent example/proofs/evidence, without
  changing the seven native runtime inputs in small-prime-inputs.sha256.
- Cryptanalysis cubic source commitf91a5e061ca5cfd777688afb65ab6972306618d1,
  branch `research/chained-transfer-cubic-norm-20261111`.
  Its base is merged PR1474,852a464a7310d733a472e4550e1fe4070614c538.
- Latest balanced source28ba661017cdba4b132c2dcbc1615a3fd92a2a48,
  branch `research/chained-transfer-balanced-genus2-20261111`, based on
  merged cubic PR1475,04fb06c27274990b8cadac7c356c6a2e40dc90b7.
  BALANCED_CHECKPOINT.md pins source digests, proof scope and producer PDF.
- `cubic-checkpoint.json` pins the producer, independent replay and witness
  digest. Earlier native/reference source pins and all raw receipts remain.
- Crypto-autoresearcher8edec422a60d87b2cbc656fb02f99f42eba959c7 is vault
  locked. No plaintext was written there; use INTEGRATION_HANDOFF.md on unlock.

## Finished checks: reuse when inputs match

- Native25/reference33 tests, strict Clippy, formatting, all source guards:
  cryptanalysis evidence/cubic-pinned-02.txt, claim29610e. One pre-existing
  benchmark is ignored. New arithmetic itself has4 passing tests.
- Independent130-witness root replay: evidence/cubic-independent-01.txt,
  claim2d5c08. No new producer norm code is imported by the replay.
- Four structural Lean lemmas: evidence/lean-cubic-01.txt, claime1a827;
  12-declaration gate passes. They do not formalize Picard/finite-field geometry.
- Consumer three-page and crypto two-page cubic PDFs were rebuilt and every
  page visually inspected. No overflow or clipped content observed.
- Latest complete balanced replay: native25/reference37 tests, strict Clippy,
  formatting and source guards; producer balanced-pinned-01.txt/PCC8b7ca0.
  Both10,404-class old/translated controls match the existing engine;
  E109/F101 source scalar/norm[2]/inverse55 controls and all11,663 paired norm
  additions pass. Its separate two-page PDF was rebuilt and inspected.
- Six balanced Lean bookkeeping/expansion lemmas pass6-declaration gate and
  local PCC check1cadd0. Signed standing remains unverified; actual geometry
  is primary-source input. Total35 written lemmas, gates22+8+7+12+6.
- Focused GitHub run42999610365 succeeds on exactdf2b78a2, merged PR1474.
  Broad C++ analysis jobs later failed; this is not an all-CI-green verdict.
  [pcc:unverified]

The completed root library attempt04 is6184 pass/13 fail/133 ignored, claim
8ffeb3. It ran about two hours and is finished. Restored tracked files fix6
failing tests;5 need unavailable archived raw data and2 need absent kernel
clock metadata. Full-suite status stays failed. Do not rerun it without a
change that resolves those causes. [pcc:unverified]

For changed inputs, use installed toolchains on PATH:

```sh
export PATH="$HOME/.cargo/bin:$HOME/.elan/bin:$PATH"
# From cryptanalysis, with the pinned crypto runtime inputs available:
experiments/chained-transfer-reference/check.sh
# From crypto:
cargo test --release --example chained_cubic_norm_reference
tools/lean/proof-gate.sh research/chained_jacobian_transfers_20261110/lean/CubicPullback.lean
```

## Next unblocked task: CT13

CT12's bounded balanced arithmetic and full old-chart controls are complete;
reuse their receipts when inputs match. Production/catalog integration remains
CT14, with large-prime/extension/normalization boundaries still unsupported.

Assess a Frobenius-stable unordered Richelot splitting. The
individually-rational-factor obstruction does not exclude it. Use the
predeclared E19/F17 source y²=x³+2x+2 and quadratic sextic. Validate its cubic
residue field and square roots, enumerate all Frobenius-stable pairings,
preserve every degenerate determinant, compute normalized descended targets
and audit curve/model equivalences. No target or transfer is already accepted.
Then implement an actual correspondence on the balanced classes and check
descent, additivity, odd-subgroup preservation and both dual norms before
calling it an operational transfer. [pcc:unverified]

The E109 sextic may supply a contrasting Frobenius-cycle control, but no
universal kernel exclusion has been established. Keep mathematical target
construction distinct from an implemented map or a measured solving advantage.
[pcc:unverified]

## Continuing constraints

- S3 checkpoint gate and live IsoLab budget/capacity are missing. No benchmark,
  collector experiment or paid allocation before successful admission.
- Crypto's append-only PCC ledger fork is unresolved. Extend PR3877; do not
  choose a history, rebase away records or bypass required checks. The pending
  synchronization request is already on the PR.
- Local statement receipts remain not_covered without independent signatures;
  local Lean proof pass also does not itself create signed standing.
- No new Python research/crypto arithmetic. New arithmetic Rust/Lean only.
- KB/ecdb unavailable, atlas locked. Keep prior-art uncertainty explicit.
- Use the guarded GitHub object-publishing procedure if ordinary Git push
  authentication still fails. Verify remote head before updating; never force.
- Check PRs on repository cadence (about50min absent new activity), or when
  actual merged state is required for a new publishing decision.
