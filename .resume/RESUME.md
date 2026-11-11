# Chained ECDLP Transfers — exact resume 2026-11-11T13:11Z

## Goal
Continue user-authorized autonomous research across crypto,cryptanalysis,
crypto-autoresearcher,starting from cryptoPR3877. Prioritize genuine DLP-
preserving maps,Jacobian chains,prime-field descent and actual solver costs.
Canonical durable queue: research/chained_jacobian_transfers_20261110/QUEUE.md
and GitHub isogeny-labs/crypto issue3904. Read actual HEAD/diffs first.

## Lead / hypothesis
CT17 implements both elliptic quotient maps on accepted odd bielliptic
Jacobian rational groups. Sources:p17 coefficients[2,0,2,0,0,0,1],order285;
p101 shifted109[56,0,13,0,6,0,1],order12317. Quotient orders19/15,109/113.
Reciprocal T=s/x,Y=b*y/x³,b=s² gives second monic elliptic cubic
[b²,a2*b,a4,1]. Both full class tests and source subgroup coordinates
([4]P,0) pass; this is unsigned correctness,no solver/advantage.
CT16 found target6-cycles,only previous-dual stable Richelot kernel and
Fp3-two-torsion16/4,obstructing unpolarizedFp-Jacobian isomorphism using
written standard geometry. Forced backtrack retained. Next CT18 assesses
rational odd-prime maximal-isotropic kernels without pre-accepting a map.

## Spec
See crypto study CHECKPOINT.md,QUEUE.md,TARGET_FROBENIUS_CHECKPOINT.md;
cryptanalysis experiments/chained-transfer-reference/SPLIT_QUOTIENTS.md,
TARGET_FROBENIUS.md and frozen split-quotients-proposal-01.txt. Strict
source custody: seven upstream runtime files remain pinned to crypto
75a737b8366ec6c78ed8672886423fcbe69853a2. New arithmetic Rust/Lean only.
Tests are local development correctness,not experiments or benchmarks.
Any actual experiment requires successful S3 admission and live IsoLab
allowance/capacity; neither currently available. No bypass.

## State
Crypto branch research/chained-jacobian-transfer-20261110,PR3877,local and
published8ef9baecf6a4288cb0eb0596af26187d30ff04a8 before pending CT17 proof
checkpoint. Local seven-lemma SplitQuotients.lean passes gate7/PCCfffa35;
total62 written lemmas. Failed broad tactic attempts01/02 retained,03pass.
PCC is unsigned; local proof pass is not signed scientific standing.
Consumer branch research/chained-transfer-split-quotients-20261111,HEAD
da66ab9d953d2ba96c4ac9d6e8379c3478b3f277. Targeted all285/12317 fullsplit,
all19/109 target components and strict Clippy pass,PCC036d81/96b1e2.
Ready PR1481;current full49-test focused43026964101 running; do not claim
that whole current matrix passed. Prior63cf80560 passes exact-head
43024621372:native25/reference47,one old benchmark ignored;PR1480 merged
57ab3ad058f77f4d28b6d1db8831bab6d1b33dad at13:05:29Z.
Reports/diagrams/two-page PDFs inspected. Root library attempt04 remains
6184pass/13fail/133ignored; six restored-file reruns pass,five need absent
archives,two missing kernel clock metadata. Do not repeat two-hour suite.
Crypto frozen ledger diverges after569 shared rows; ownerCopilot request
6789021128 pending. Extend existing PR3877,no rebase/alternate lineage/force.
Globalbenchrec historical189 problems/106records retained; consumer16passes.
Autoresearcher8edec422a60d87b2cbc656fb02f99f42eba959c7 remains AES-SIV
locked,no KEY binding/no plaintext. Use INTEGRATION_HANDOFF.md when unlocked.
KB MCP/CLI/ecdb unavailable,no novelty claim. No paid allocation/solver run.

## Next steps
1. Inspect actual heads,diffs and this file versus current CHECKPOINT.md;
   collect focused43026964101 on exactda66ab9d9/readyPR1481.
   Hold auto-merge until focused49-test pass plus actual branch requirements.
2. Publish root CT17 proof/source/queue/report checkpoint,update3877/3904.
   Object publishing workaround uses exact40hex objects and guarded connector
   ref update,neverforce. See /workspace/scratch/publish_checkpoint_objects.py.
3. CT18: read primary odd-prime isotropic-isogeny/ellipticFrobenius input;
   freeze3/5 (and prospective7) kernel criteria before bounded correctness
   tests. Preserve negative/equivalent outcomes,no new map pre-accepted.
4. Do not re-run completed unchanged arithmetic or root full suite. Reuse
   immutable receipts only with matching inputs. Poll PRs on~50min cadence
   unless actual state is necessary for next publishing/merge decision.

## Runs in flight
No benchmark,solver,collector,paid/cloud job. No full-library test running.
Consumer immutable Git object upload may still run:
python3 /workspace/scratch/publish_checkpoint_objects.py isogeny-labs/cryptanalysis
57ab3ad058f77f4d28b6d1db8831bab6d1b33dad
da66ab9d953d2ba96c4ac9d6e8379c3478b3f277
Log:/workspace/scratch/ct-github-split-quotients-publication-01.txt.
Checkps/log before restarting; uploader moves no refs. Prior CT16 focused CI
43024621372 is finished success,rawlogcommitted in consumerda66ab9d9.
No other local work needs waiting. Any newly dispatched focusedCI is visible
in gh run list by exactbranch/head; record actualrun ID and outcome.

## Stop and success conditions
Continue next unblocked correctness/proof task after each durable checkpoint.
Accept only actual full-domain/source-subgroup/duals/descent checks in frozen
scope. Record failures,uncertainty,negative results and exactsource/CI pins.
No signed standing fabricated,no immutable ledger rewrite,no CI bypass or
plaintext vault write. No benchmark until S3 gate and live budget succeed.
If interrupted,commit complete state and updateGitHubqueue/this resume file.
