# Interruption recovery — Chained ECDLP Transfers, 2026-11-11

Read this before older checkpoint paragraphs. The execution workspace became
unavailable around14:29UTC. A final local apply_patch failed after35seconds;
a subsequent read-only health call stalled and was terminated after the
environment was reported unavailable. No further shell command, compiler,
proof, benchmark or CT20 gluing test was run. The GitHub connector remained
available and was used to rescue this state. This is an interruption, not
completion of the user's research objective. [pcc:unverified]

## Requirement coverage at interruption

| User requirement | State and evidence |
|---|---|
| Nontrivial DLP-preserving transfers | Bounded maps implemented; full19/109 source preservation and CT19 elliptic7 controls published; unsigned. |
| Chained Jacobian maps | Earlier exact-fixture chains retained; accepted full-source targets force Richelot backtracking. Actual odd Jacobian continuation is CT20, planned/unexecuted. |
| Prime-field descent | Same-prime coefficient descent checked on complete285/12317-class maps. No cross-prime transfer claimed. |
| Independent validation and proofs | Separate old field/kernel/oracle engines replay bounded certificates; feasible Lean helpers partial; full geometry and signed standing remain open. |
| Computational advantage and benchmarks | Unmeasured; S3 admission and live budget/capacity unavailable. Complete construction/transport/solver/recovery controls remain required. |
| Persistent queue/GitHub commits | Existing3877/3904 plus this recovery preserve exact sources, negatives, CI and next actions. |
| Cross-repository integration | crypto/cryptanalysis published; crypto-autoresearcher vault locked, integration handoff retained. |
[pcc:unverified]

## Durable revisions

- Crypto parent before this recovery:
  ad10b21a3578dadf2366dbd793ebed2f7aea319a, branch
  research/chained-jacobian-transfer-20261110, existing PR3877.
  Its tree6a9231a81679f40c7e097280fb723b06f692c68f includes the CT18
  module assessment and separately authored CT16 oracle custody.
  This recovery extends that parent with an expected-head, nonforced update;
  it does not touch frozen PCC records or the ledger.
- Latest consumer:
  ef9a542598c1b0ebfde03cf27f77929b928391cd, ready PR1483,
  research/chained-transfer-seven-20261111.
  https://github.com/isogeny-labs/cryptanalysis/pull/1483 .
  All four actual elliptic7 maps, duals, certificates, independent replay,
  reports/diagram/inspected PDF and source manifests are published there.
- Exact-head focused run43038507853:
  https://github.com/isogeny-labs/cryptanalysis/actions/runs/43038507853 .
  Completed success14:33:50Z, job130583694992:
  native25 pass; reference51 unit passes plus two separate integration
  passes; one pre-existing benchmark ignored. Formatting, strict Clippy,
  all source guards and test steps succeeded. This operational CI
  observation has not been registered as a new frozen/signed PCC record.
- Previous consumer PR1482 merged29c4d720660297b30eff74ba735160b99b0d18da
  at13:48:49Z, after75e5064d1 exact-head43030710539 success:
  native25/reference50 unit+one integration, one old benchmark ignored.
  Its raw receipt/PCCceb332 are durable in consumer ef9a54259.
- PR1481 mergedee3f177b4a2d66544887d0307d5d042aef04f6b4;
  PR1480 merged57ab3ad058f77f4d28b6d1db8831bab6d1b33dad;
  PR1479 merged3e76a274fadbd40e4ad686aa8fa4ec5e0926ac31.
  Their original exact-head passing receipts remain retained.
- crypto-autoresearcher8edec422a60d87b2cbc656fb02f99f42eba959c7
  remains AES-SIV locked without KEY; no plaintext or guessed atlas update.
[pcc:unverified]

## CT19 implemented result and limits

Four actual degree7 elliptic maps over F101 and normalized duals preserve
the entire109/113-point groups. Source E109=(1,46) has kernels
[9,76,20,1]→(5,39),j91,dual[49,69,62,1] and
[72,100,35,1]→(26,58),j87,dual[58,26,58,1].
The other source has long coefficients[5,33,13,1], shift X=x+38,
short(44,21); kernels
[59,46,8,1]→(69,50),j92,dual[29,88,45,1] and
[77,86,71,1]→(13,42),j89,dual[42,13,8,1].
Dual normalization u=29; inverse7 recovery78/97. See published consumer
PCCa51d74/beeac7/45f608/2f70a0, all unsigned. [pcc:unverified]

Tests include actual kernel/division-polynomial membership and closure,
coprimality, smoothness, exact cleared curve identities, both global
rational-x compositions[7], all source/codomain points, both signed dual
norms[7], bijections, all pair additions/scalar relations, recovery and
actual coefficient-scale/j controls. The independent verifier uses an
unchanged pre-existing Montgomery field/poly/kernel implementation; it
imports no producer Kohel or new transfer code. Root and consumer use
the SAME independent harness through two entrypoints, not two independent
implementations or signed verdicts. Both arithmetic engines are unchanged
at crypto75a737b8366ec6c78ed8672886423fcbe69853a2. The separate102-input
construction/five-input verifier manifests and the original seven native
runtime guards must remain distinct. [pcc:unverified]

Frozen consumer fixture:
experiments/chained-transfer-reference/evidence/elliptic-seven-witness-01.tsv,
SHA25678f0b33e611113f8b6c8340874c39f153c0620b91a86c8242c606c675a927615.
Consumer harness tests/seven_independent.rs:
SHA2562173d13ef0b2d1fdbbeb459f9ec4a61c87fa0cfd4a116c90855b72c274319a2e.
Producer src/seven.rs:
SHA256bb640465bfe9406e5b5e275b5127c73417b48cec5a90006d2f2bb87f72f40ae9.
These are recorded prior custody digests, not newly calculated in the
unavailable workspace. [pcc:unverified]

Preserved correction: old isogeny_algos::curve::iso_exists_over returns
an unchecked scale candidate, not a certified isomorphism. Development
attempt02 mislabeled its Some results as Fp-isomorphisms despite differing
j. The checked adapter now requires BOTH coefficient equations and rejects
the false equivalences; the independent engine repeats this. Raw02 is
retained, later03/04 and independent02/03 supersede it. No accepted frozen
claim relied on the wrong diagnostic. No old arithmetic or lint exemption
was changed. These are known Vélu/Kohel/division-polynomial algorithms,
not a new kernel algorithm or novelty result. [pcc:unverified]

No actual Jacobian7-map is implemented yet. CT20_PLAN.md specifies compatible
gluing, geometric2-kernel inclusion, global factoring through[2], full
dual[7], geometric2-torsion via64 product4-torsion lifts, and a genuine
odd hop beyond the previous-dual Richelot obstruction. It is prospective.
No computational advantage, DLP solve, benchmark or cross-prime transfer
has been established. All maps so far use the SAME prime field; extension
arithmetic followed by coefficient descent is not a cross-prime attack.
[pcc:unverified]

## Exactly what was rescued and what remains local

This recovery copies the published consumer TSV and reconstructs
examples/chained_elliptic_seven_reference.rs from published
tests/seven_independent.rs by ONLY replacing its crate import with
crypto_lib::cryptanalysis::isogeny_walk and its fixture include path with
the root research path. The consumer harness is directly CI-tested at ef9;
the newly recovered root entrypoint was not rerun after workspace loss.
The pre-loss root example had passed locally, but no new execution is
claimed for this recovered file. Do not infer a frozen original-file
hash from the reconstruction. [pcc:unverified]

lean/EllipticSeven.lean is transcribed from the exact source preserved in
the session handoff. Before loss, local gate4 and local PCC verification
reported passes on four coordinate/scalar helpers under Lean4.35.0:
short_translation_cleared, isomorphism_scaling, recover_order109,
recover_order113. Its raw local receipt was not committed before loss.
This recovery preserves the source and adds it to the existing manual/
main proof workflow, but does not pretend a new proof run occurred.
The prior68 Lean lemmas have durable source/receipts; these four make72
written sources with the recovered module requiring a fresh retained gate
receipt if the original one cannot be restored. None formalizes elliptic
isogeny construction, finite fields or the full Jacobian geometry.
[pcc:unverified]

The old local worktree at HEADad10b21a had:
- examples/chained_elliptic_seven_reference.rs;
- study evidence/elliptic-seven-witness-01.tsv;
- study evidence/independent-elliptic-seven-01.txt,02,03;
- study evidence/lean-elliptic-seven-01.txt;
- study evidence/pcc-elliptic-seven-01.json;
- study lean/EllipticSeven.lean and CT20_PLAN.md;
- pcc/claims/PCC-20261111-7e9547.json and PCC-20261111-58dbb8.json;
- two additional local ledger rows,611→613.
The claim JSON, exact two rows and original local raw logs are NOT rescued
by this commit. Do not manufacture their IDs, hashes or historical receipts.
If the original worktree returns, inspect and preserve the originals before
any reset/stash/recovery. Verify the original hashes before reusing records.
Otherwise rerun the changed root entrypoint and four-helper gate, retain
fresh outputs and register FRESH PCC records with the existing CLI.
The published consumer report references crypto7e9547/58dbb8; treat those
as presently unavailable local references, not durable root evidence.
The unchanged complete consumer independent-seven-ci-03.txt is durable
evidence for the same harness. [pcc:unverified]

The failed final patch attempted both the workflow addition and a new
ELLIPTIC_SEVEN_CHECKPOINT.md. Neither original write is assumed applied.
This recovery supplies the equivalent checkpoint and workflow addition
through GitHub; no attempted local write is counted as a completed publish.
No known local test, solver or paid allocation was running at loss.
The stalled read-only health exec was terminated. [pcc:unverified]

## CI and merge handoff

PR1483 focused CI is successful. Active cryptanalysis ruleset25796910
requires store/gate/records, merge commits and resolved review threads:
all three named checks passed on ef9. Read protection endpoint returned403
(Resource not accessible by integration); ruleset read succeeded and has
no bypass actors, current_user_can_bypass=never. PR had no submitted reviews
or inline review comments, was mergeable=true, mergeable_state=unstable.
The last check snapshot still had Analyze(rust) in progress; a running
check is not stuck. No merge or auto-merge was attempted during recovery
while applicable analysis remained pending. [pcc:unverified]

Five general-CI jobs failed before their analysis/tests because apt could
not resolve security.ubuntu.com/archive.ubuntu.com:
clang-format changed-lines(job130583690271),
coverage(130583690367), cppcheck(130583690324),
clang-tidy(130583690215), Valgrind memcheck(130583688587).
Logs were individually read through the connector; each ends in package
fetch DNS errors/exit100. The PR changes Rust/reference/docs/PCC and two
sparse-checkout lines, not C++ source or those package-install steps.
These failures are infrastructure observations, not mathematical negatives
or test passes. Prior1482 also had cppcheck/clang-tidy failures; unlike
those historical failures, the current five have explicit DNS evidence.
Follow current rules and report source-independent failures with evidence;
never disable jobs, change filters, waive required checks or rerun unchanged
failures merely to get green. [pcc:unverified]

Focused job log has81,588 characters and format job log207,244 characters;
full logs were not newly committed as oversized run data without S3.
Public job URLs/log endpoints and an exact outcome excerpt are retained in
evidence/elliptic-seven-ci-observation.md. The Actions artifact upload
succeeded, but its retention is finite; archive the complete logs/artifacts
through the existing artifact-store tool once S3 access returns.
This operational observation is not a signed execution receipt.
[pcc:unverified]

Do not busy poll. Check actual PR state on the repository cadence (~50min
after activity, then spaced) or next restored session before a merge
decision. When all actual required/applicable checks and review obligations
permit, merge PR1483 with expected head
ef9a542598c1b0ebfde03cf27f77929b928391cd and merge_method=merge; confirm the
merge commit. If the head moved, inspect the diff and new CI first.
CryptoPR3877 stays blocked by the frozen ledger fork/missing archives;
do not merge, force-push, rebase away evidence or choose an alternate lineage.
[pcc:unverified]

## Exact next session

1. Restore a working execution workspace and read applicable AGENTS/skills.
   Inspect actual refs, worktree/index and untracked files; preserve original
   pending PCC records/raw logs if present. Read QUEUE.md and this checkpoint.
2. Collect PR1483 analysis/merge status and archive the exact focused CI log
   when admitted. Check prior frozen receipts, do not rerun unchanged baseline
   corpora or the completed two-hour root library attempt.
3. If originals were not recoverable, validate only the restored root
   example/four Lean helpers, retaining new outputs and fresh PCC records:
   cargo test --release --example chained_elliptic_seven_reference -- --nocapture
   tools/lean/proof-gate.sh research/chained_jacobian_transfers_20261110/lean/EllipticSeven.lean
   python3 tools/pcc/pcc.py claim new --help
   Existing consumer full command:
   experiments/chained-transfer-reference/check.sh .
   Rust1.99.1/Lean4.35.0; inspect source guards before any reuse.
4. CT20 first discriminating step: implement four compatible image2-torsion
   gluing inventories using unchanged FpR/genus2::glue; check descent,
   square-leading normalization, actual quotient component swap/isomorphisms
   and full point counts. Read CT20_PLAN.md for the global factor proof,
  64-lift torsion action and subsequent actual Jacobian7-chain.
5. Continue to CT17a CRT direct controls if gluing has a concrete unsupported
   chart. Supplied scalars remain unexecuted: p17 projectors210/76, halves10/8;
   p101 projectors9266/3052, halves55/57. Do not reopen already closed
   rational nonbacktracking Richelot kernels on the exact accepted targets.
[pcc:unverified]

## Continuing obligations

- S3 checkpoint gate exact headings were fixed, but transport still lacks
  configured AWS profile ecc-challenges-new/checkpoint token. Live IsoLab
  allowance/capacity is unknown. No experiment/collector/benchmark/paid job
  before successful admission and live budget/capacity. Local development
  tests/proofs are distinct; GitHub recovery is not an S3 admission bypass.
- Frozen crypto PCC ledger fork after569 common rows remains unresolved.
  Existing Copilot synchronization request6789021128 was unanswered at last
  observation; extendPR3877, preserve both histories, never hand-rechain.
- Root library attempt04:6184pass/13fail/133ignored. Restored tracked files
  fixed6; five absent archive records and two missing kernel-clock metadata
  remain. Global benchrec reports189 problems/106 records from missing
  historical artifacts; consumer's scoped16-record gate passed.
  No full-root suite or global benchrec green claim; do not repeat without
  addressing the known causes.
- Research-KB MCP/CLI, ecdb and vault integration remain unavailable.
  No new novelty claim/canonical graph promotion; lead-atlas handoff remains
  INTEGRATION_HANDOFF.md. Reports/diagrams/PDFs for completed components
  are already durable in the consumer.
- Mathematical standing is unsigned. CI, producer-run independent replay,
  local Lean compilation, hashing and PR merges do not create a signed
  scientific verdict. Preserve failed and negative outcomes at exact scope.
- New research arithmetic is Rust/Lean. No subagents were requested.
  No Slack/email messages, cloud allocations or secrets were emitted.
[pcc:unverified]

Unblock condition: a usable checkout/compiler/proof environment, with
separate admission/budget and ledger/archive access restored when those
specific workstreams are reached. No background research continuation is
scheduled or promised by this checkpoint. The user objective remains open.
