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
- Latest kernel/codomain source0348e90f0654255917c0d6878168ebbda782ffd5,
  branch `research/chained-transfer-nonsplit-kernels-20261111`, ready PR1477,
  based on merged balanced PR1476,1bef68b5736dc359a884f7d64391fb26f3f74d84.
  NONSPLIT_KERNEL_CHECKPOINT.md pins outcomes, scope and detailed next-map plan.
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
- Latest kernel guard: native25/reference40,1 existing ignored benchmark,
  formatting/strict Clippy/guards pass; producer nonsplit-pinned-02/PCC3c51f5.
  Five new Lean table/algebra lemmas pass5-declaration gate/PCC465142 locally;
  total40 written lemmas, gates22+8+7+12+6+5. Signed standing remains unverified.
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

## Next unblocked task: CT13b actual non-split map

CT12's bounded balanced arithmetic and full old-chart controls are complete;
reuse their receipts when inputs match. Production/catalog integration remains
CT14, with large-prime/extension/normalization boundaries still unsupported.

CT13a is complete: E19/F17 has3 stable pairings,1 degenerate/2 equivalent smooth
targets; both source/target Jacobians have285 rational classes. The full PGL2
curve audit distinguishes source from target, without unpolarized-Jacobian
inference. E109/F101 unshifted cover has only the degenerate stable pairing;
this is a fixture-specific direct-neighbor restriction, not universal closure.
[pcc:unverified]

Implement the frozen determinant11 correspondence, raw target w=3Y, in the
F17⁶ compositum; see NONSPLIT_KERNEL_CHECKPOINT.md for exact finite/infinity
formulas and acceptance criteria. Validate both dual norms[2] on all285
rational classes, additivity, coefficient descent and all19 source scalar
relations/recovery[4]/inverse5. Target construction is not an accepted map.
[pcc:unverified]

After that pipeline passes, CT15 predeclares shift2 of the E109 cover, rather
than interpreting the unshifted negative as closure of shifted covers.
Its exact proposed input is in QUEUE.md; do not claim a shifted map or
performance edge before implementation and independent checks.
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
