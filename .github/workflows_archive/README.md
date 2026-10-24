# Archived historical workflows

This directory holds 198 GitHub Actions workflow files that used to live in `.github/workflows/`. GitHub only reads workflows from that exact directory, so these files are no longer registered, can no longer be dispatched, and consume no runner time. Nothing here is deleted: every file was moved with `git mv` as a pure rename (R100, byte-identical), so `git log --follow` still shows its full history.

Policy and selection rules: [`../ci-archive-policy.md`](../ci-archive-policy.md). Machine-readable list: [`MANIFEST.json`](MANIFEST.json) (`moved` and `left_in_place_with_reason`).

## Why these were safe to move

Each file was re-checked against `origin/main` at `da3a1b5d53323a4fb14ed761ca269e84b6018089` and meets all of:

- Its only trigger was `workflow_dispatch` (parsed with PyYAML). No `push`, `pull_request`, `schedule`, `workflow_call`, `workflow_run` or `repository_dispatch`.
- No other workflow calls it (`uses: ./.github/workflows/...`) and no workflow, script, test, fixture or evidence seal reads, hashes or dispatches it by path. A search of the whole of `origin/main` for each `<name>.yml` found hits only in the places listed under "Prose and list references" below.
- It is not byte-hash-bound to any evidence seal, and it is not a provenance label in generated registry data.
- Their own headers already said so: 76 are headed "Retired 2026-10-11 ... run it on demand", 117 "Historical stage control. Manual only", 3 say "Archived ... preserve for explicit replay", and the other 2 are manual-only replays of finished studies.

Because every moved file was already manual-only, moving it changes no PR, push or scheduled behaviour. The only loss is the ability to start one with `gh workflow run <file>` from the default branch, which is the point.

## Restore one

```sh
git mv .github/workflows_archive/<file>.yml .github/workflows/<file>.yml
git commit -m "ci: restore <file>.yml"
```

Restoring through `git mv` keeps the rename chain intact. To dispatch it with `gh workflow run <file>` the restore has to be merged to the default branch.

## Run one from a branch without restoring it to main

GitHub registers a `workflow_dispatch` workflow only from the default branch, so a file that exists only on a feature branch cannot be dispatched. To run an archived workflow without putting it back on `main`:

1. Branch from `main` and `git mv` the file back into `.github/workflows/` on that branch.
2. On that branch only, add a temporary `push:` trigger restricted to the branch (`push: { branches: [<branch>] }`) next to `workflow_dispatch:`. A push trigger runs the file as it exists at the pushed commit, so the file does not have to be on `main`.
3. Push the branch, read the run, then delete the branch. Do not merge the temporary trigger.

This route has not been exercised from this change (it was prepared without network access); treat it as the documented approach and adjust if GitHub behaves differently. If you want the workflow permanently dispatchable, use the `git mv` restore above and merge it.

Do not edit an archived file in place expecting it to run from here: nothing in this directory is executed.

## Known side effects

- `taskq/deploy/eks-karpenter/arc/first-wave-workflows.txt` and `deferred-workflows.txt` still list many of these filenames. Nothing validates those lists against the workflows directory, and the `|filename|` runner-routing markers only matter for files in `.github/workflows`, so the stale entries are harmless. They were intentionally left as a historical rollout record.
- The research notes below mention a few of these files by path. Research notes are evidence and were not edited, so these links and paths now point at a file that has moved here:

  - `f5-mainline-rank-segments.yml`: `research/f5_mainline_port_20261002/RESULT.md`
  - `f5-support-cut19-call.yml`: `research/f5_support_cut19_call_20261002/RESULT.md`
  - `f5-support-rank6-call.yml`: `research/f5_support_rank_bits_20261002/LINUX_READY.md`
  - `n83-online-k3114.yml`: `research/prime61_disk_walk_20261017/results/prime61-disk-fastforward.log`

- `scripts/build_curve_registry.py` scans every tracked `*.yml` for curve mentions and records the first path in sorted order as a source label. This directory is named `workflows_archive` (underscore) so that its paths sort after `.github/workflows/...` and no generated label changes. Do not rename it to `workflows-archive`: a hyphen sorts before the slash and would flip labels.

## Index

`trigger` is the original `on:` block, which was `workflow_dispatch` only for every file. `inputs` lists its dispatch inputs. `ARC` is the filename-routing list that mentions it: F is first-wave, D is deferred.

### boolean (22)

| file | workflow name | trigger | inputs | ARC |
|---|---|---|---|---|
| `boolean-affine-substitution.yml` | Standalone Boolean affine and enumeration | workflow_dispatch | - | F |
| `boolean-basis-transport.yml` | Standalone Boolean basis transport | workflow_dispatch | - | F |
| `boolean-byte-sieve.yml` | Standalone Boolean projected syndromes | workflow_dispatch | - | F |
| `boolean-degree-replay.yml` | Boolean degree evidence replay | workflow_dispatch | - | F |
| `boolean-envelope-indexed.yml` | Indexed Boolean support envelope | workflow_dispatch | phase | F |
| `boolean-f4-batched-pairs.yml` | Boolean F4 batched pair updates experiment | workflow_dispatch | - | F |
| `boolean-f4-bitmap.yml` | Boolean F4 bitmap membership experiment | workflow_dispatch | - | F |
| `boolean-f4-wide-tables.yml` | Boolean F4 wider lookup tables experiment | workflow_dispatch | - | F |
| `boolean-f5-affine-signature.yml` | Boolean F5 affine signature | workflow_dispatch | - | F |
| `boolean-f5-amortized-ceiling.yml` | Boolean F5 graded-reuse ceiling | workflow_dispatch | phase | F |
| `boolean-f5-call-only-ceiling.yml` | Boolean F5 call-only ceiling | workflow_dispatch | phase | F |
| `boolean-f5-graded-batch.yml` | Boolean F5 exact graded batch | workflow_dispatch | discovery_run_attempt, discovery_run_id, phase | F |
| `boolean-graded-tail-reuse.yml` | Graded Boolean tail reuse | workflow_dispatch | phase | F |
| `boolean-halfword.yml` | Standalone Boolean half-word study | workflow_dispatch | phase | F |
| `boolean-implicit-minors.yml` | Standalone Boolean implicit minors | workflow_dispatch | - | F |
| `boolean-initial-restriction.yml` | Standalone Boolean initial restriction | workflow_dispatch | - | F |
| `boolean-linear-fibers.yml` | Standalone Boolean linear fibers | workflow_dispatch | - | F |
| `boolean-pivot-screen.yml` | Boolean sibling pivot screen | workflow_dispatch | - | F |
| `boolean-projected-fibers.yml` | Standalone Boolean projected fibers | workflow_dispatch | - | F |
| `boolean-quadratic-frontier.yml` | Standalone Boolean quadratic frontier | workflow_dispatch | - | F |
| `boolean-representation-audit.yml` | Boolean representation audit | workflow_dispatch | - | F |
| `boolean-schedule-construction.yml` | Standalone Boolean construction cost bound | workflow_dispatch | - | F |

### compact (2)

| file | workflow name | trigger | inputs | ARC |
|---|---|---|---|---|
| `compact-base-sweep-replay.yml` | Compact base sweep replay | workflow_dispatch | - | F |
| `compact-four-sum-membership-replay.yml` | Compact four-sum membership replay | workflow_dispatch | - | F |

### ecc2k130 (38)

| file | workflow name | trigger | inputs | ARC |
|---|---|---|---|---|
| `ecc2k130-263-dual-composition.yml` | ECC2K-130 degree-263 dual composition | workflow_dispatch | - | F |
| `ecc2k130-263-order-certificate.yml` | ECC2K-130 degree-263 kernel order certificate | workflow_dispatch | - | F |
| `ecc2k130-base-window-input-freeze.yml` | ECC2K-130 base-window input freeze | workflow_dispatch | - | F |
| `ecc2k130-base-window-outcome.yml` | ECC2K-130 base-window sealed outcome replay | workflow_dispatch | - | F |
| `ecc2k130-challenge-point-import.yml` | ECC2K-130 direct public challenge point import | workflow_dispatch | - | F |
| `ecc2k130-decision-gates.yml` | ECC2K-130 decision gates | workflow_dispatch | - | F |
| `ecc2k130-disjoint-cold-outcome.yml` | ECC2K130 disjoint-Q cold outcome replay | workflow_dispatch | - | F |
| `ecc2k130-disjoint-cold-q.yml` | ECC2K130 disjoint-Q compact cold CPU panel | workflow_dispatch | - | F |
| `ecc2k130-leaf-m10-column-bound.yml` | ECC2K-130 leaf m10 column-budget bound | workflow_dispatch | - | F |
| `ecc2k130-leaf7-metered-freeze.yml` | ECC2K-130 leaf7 metered route freeze | workflow_dispatch | - | F |
| `ecc2k130-m10-export-capacity.yml` | ECC2K-130 m10 export capacity preflight | workflow_dispatch | - | F |
| `ecc2k130-m3-base-selector-outcome.yml` | ECC2K-130 m3 base-selector outcome replay | workflow_dispatch | - | F |
| `ecc2k130-m3-base-selector-protocol.yml` | ECC2K-130 m3 base-selector protocol freeze | workflow_dispatch | - | F |
| `ecc2k130-m3-probe-order-bound.yml` | ECC2K-130 m3 probe-order bound replay | workflow_dispatch | - | F |
| `ecc2k130-m83-solver-matrix.yml` | ECC2K-130 m83 solver evidence replay | workflow_dispatch | - | F |
| `ecc2k130-n131-f0-census.yml` | ECC2K130 n131 beta3 F0 exact census | workflow_dispatch | - | F |
| `ecc2k130-n37-native-capacity.yml` | ECC2K-130 n37 native-column capacity replay | workflow_dispatch | - | F |
| `ecc2k130-native-benchmark.yml` | ECC2K-130 native candidate benchmark | workflow_dispatch | - | D |
| `ecc2k130-native-m3-counting.yml` | ECC2K-130 native m3 counting admission | workflow_dispatch | - | F |
| `ecc2k130-oaware-paired-first-attempt.yml` | ECC2K-130 n13 paired SAT first-attempt archive | workflow_dispatch | - | F |
| `ecc2k130-pinned-x-witness.yml` | ECC2K-130 pinned x-only witness regression | workflow_dispatch | - | F |
| `ecc2k130-point-sum-cold.yml` | ECC2K130 point-sum frozen-Q cold CPU panel | workflow_dispatch | - | F |
| `ecc2k130-point-sum-cold-outcome.yml` | ECC2K130 point-sum cold outcome replay | workflow_dispatch | - | F |
| `ecc2k130-point-sum-query.yml` | ECC2K-130 compact point-sum query | workflow_dispatch | - | F |
| `ecc2k130-rotated-arity-screen.yml` | ECC2K130 rotated arity screen | workflow_dispatch | - | F |
| `ecc2k130-rotated-beta-portfolio.yml` | ECC2K130 rotated beta portfolio | workflow_dispatch | - | F |
| `ecc2k130-rotated-beta-sweep.yml` | ECC2K-130 rotated normal-beta sweep replay | workflow_dispatch | - | F |
| `ecc2k130-rotated-four-base-joint-rank.yml` | ECC2K130 rotated four-base joint rank | workflow_dispatch | - | F |
| `ecc2k130-rotated-m56-export-gate.yml` | ECC2K130 rotated m5/m6 exporter semantic gate | workflow_dispatch | - | F |
| `ecc2k130-rotated-pdp-corpus.yml` | ECC2K-130 rotated PDP corpus replay | workflow_dispatch | - | F |
| `ecc2k130-rotated-rank-holdout.yml` | ECC2K-130 rotated row rank and point-only holdout replay | workflow_dispatch | - | F |
| `ecc2k130-rotated-row-certificate.yml` | ECC2K-130 rotated compressed-row certificate | workflow_dispatch | - | F |
| `ecc2k130-rotated-s3-candidate.yml` | ECC2K-130 rotated recursive S3 candidate gate | workflow_dispatch | - | F |
| `ecc2k130-rotated-subspace-support.yml` | ECC2K-130 rotated subspace support replay | workflow_dispatch | - | F |
| `ecc2k130-rotated-unequal-arity.yml` | ECC2K130 rotated unequal arity | workflow_dispatch | - | F |
| `ecc2k130-s3-fibre-theorem.yml` | ECC2K-130 finite-fibre recursive S3 theorem | workflow_dispatch | - | F |
| `ecc2k130-s3-o-branch.yml` | ECC2K-130 O-aware recursive S3 CNF gate | workflow_dispatch | - | F |
| `ecc2k130-sigma-shared-scratch.yml` | ECC2K fused sigma shared scratch | workflow_dispatch | - | F |

### f5 (4)

| file | workflow name | trigger | inputs | ARC |
|---|---|---|---|---|
| `f5-echelon.yml` | Matrix-F5 echelon output experiment | workflow_dispatch | - | F |
| `f5-mainline-rank-segments.yml` | Matrix-F5 mainline row-basis x86 qualification | workflow_dispatch | - | F |
| `f5-support-cut19-call.yml` | Matrix-F5 fixed cut-19 complete call | workflow_dispatch | - | F |
| `f5-support-rank6-call.yml` | Matrix-F5 cut-19 rank-width-6 complete call | workflow_dispatch | - | F |

### koblitz (3)

| file | workflow name | trigger | inputs | ARC |
|---|---|---|---|---|
| `koblitz-isogeny-descent-37.yml` | Koblitz degree-73 descent at 2^37 | workflow_dispatch | - | D |
| `koblitz-isogeny-descent-37-archive.yml` | Koblitz degree-73 descent archive replay | workflow_dispatch | - | F |
| `koblitz-structural-fields.yml` | Koblitz structural field survey | workflow_dispatch | - | F |

### koblitz-stage (117)

| file | workflow name | trigger | inputs | ARC |
|---|---|---|---|---|
| `koblitz-stage100-fixed-base-validation.yml` | Koblitz Stage 100 n53 fixed-base validation | workflow_dispatch | - | F |
| `koblitz-stage101-selected-panel.yml` | Koblitz Stage 101 n53 compact fixed-base five-pair panel | workflow_dispatch | - | F |
| `koblitz-stage102-host-class-panel.yml` | Koblitz Stage 102 n53 host-class panel | workflow_dispatch | - | F |
| `koblitz-stage103-host-panel-archive.yml` | Koblitz Stage 103 host-panel archive | workflow_dispatch | - | F |
| `koblitz-stage105-sharded-support.yml` | Koblitz Stage 105 n53 sharded support table | workflow_dispatch | - | F |
| `koblitz-stage106-shard-routing.yml` | Koblitz Stage 106 n53 shard routing | workflow_dispatch | - | F |
| `koblitz-stage108-routing-archive.yml` | Koblitz Stage 108 routing selection archive | workflow_dispatch | - | F |
| `koblitz-stage109-current-gate-audit.yml` | Koblitz Stage 109 current gate audit | workflow_dispatch | - | F |
| `koblitz-stage110-dense-support.yml` | Koblitz Stage 110 n53 dense support | workflow_dispatch | - | F |
| `koblitz-stage111-streamed-compaction.yml` | Koblitz Stage 111 n53 streamed compaction | workflow_dispatch | - | F |
| `koblitz-stage114-single-core-dense.yml` | Koblitz Stage 114 n53 single-core dense support | workflow_dispatch | - | F |
| `koblitz-stage115-packed-dense.yml` | Koblitz Stage 115 n53 packed dense support | workflow_dispatch | - | F |
| `koblitz-stage116-single-core-unsharded.yml` | Koblitz Stage 116 n53 single-core unsharded support | workflow_dispatch | - | F |
| `koblitz-stage118-fused-packed.yml` | Koblitz Stage 118 n53 fused packed compaction | workflow_dispatch | - | F |
| `koblitz-stage119-unknown-ab.yml` | Koblitz Stage 119 n53 unknown support A/B | workflow_dispatch | - | F |
| `koblitz-stage121-dense-scan.yml` | Koblitz Stage 121 n53 occupied-only dense scan | workflow_dispatch | - | F |
| `koblitz-stage122-derived-slots.yml` | Koblitz Stage 122 n53 derived dense slots | workflow_dispatch | - | F |
| `koblitz-stage123-three-window-filter.yml` | Koblitz Stage 123 n53 three-window x filter | workflow_dispatch | - | F |
| `koblitz-stage124-current-gate-audit.yml` | Koblitz Stage 124 current gate audit | workflow_dispatch | - | F |
| `koblitz-stage125-current-gate-audit.yml` | Koblitz Stage 125 current gate audit | workflow_dispatch | - | F |
| `koblitz-stage126-current-gate-audit.yml` | Koblitz Stage 126 current gate audit | workflow_dispatch | - | F |
| `koblitz-stage127-current-gate-audit.yml` | Koblitz Stage 127 current gate audit | workflow_dispatch | - | F |
| `koblitz-stage128-current-gate-audit.yml` | Koblitz Stage 128 historical gate audit | workflow_dispatch | - | F |
| `koblitz-stage129-current-gate-audit.yml` | Koblitz Stage 129 current gate audit | workflow_dispatch | - | F |
| `koblitz-stage130-current-gate-audit.yml` | Koblitz Stage 130 current gate audit | workflow_dispatch | - | F |
| `koblitz-stage131-current-gate-audit.yml` | Koblitz Stage 131 current gate audit | workflow_dispatch | - | F |
| `koblitz-stage133-current-gate-audit.yml` | Koblitz Stage 133 current gate audit | workflow_dispatch | - | F |
| `koblitz-stage134-current-gate-audit.yml` | Koblitz Stage 134 current gate audit | workflow_dispatch | - | F |
| `koblitz-stage135-current-gate-audit.yml` | Koblitz Stage 135 current gate audit | workflow_dispatch | - | F |
| `koblitz-stage136-current-gate-audit.yml` | Koblitz Stage 136 current gate audit | workflow_dispatch | - | F |
| `koblitz-stage137-current-gate-audit.yml` | Koblitz Stage 137 current gate audit | workflow_dispatch | - | F |
| `koblitz-stage138-current-gate-audit.yml` | Koblitz Stage 138 current gate audit | workflow_dispatch | - | F |
| `koblitz-stage139-current-gate-audit.yml` | Koblitz Stage 139 current gate audit | workflow_dispatch | - | F |
| `koblitz-stage140-current-gate-audit.yml` | Koblitz Stage 140 current gate audit | workflow_dispatch | - | F |
| `koblitz-stage141-current-gate-audit.yml` | Koblitz Stage 141 current gate audit | workflow_dispatch | - | F |
| `koblitz-stage142-current-gate-audit.yml` | Koblitz Stage 142 current gate audit | workflow_dispatch | - | F |
| `koblitz-stage143-current-gate-audit.yml` | Koblitz Stage 143 current gate audit | workflow_dispatch | - | F |
| `koblitz-stage144-current-gate-audit.yml` | Koblitz Stage 144 current gate audit | workflow_dispatch | - | F |
| `koblitz-stage145-current-gate-audit.yml` | Koblitz Stage 145 current gate audit | workflow_dispatch | - | F |
| `koblitz-stage146-current-gate-audit.yml` | Koblitz Stage 146 current gate audit | workflow_dispatch | - | F |
| `koblitz-stage147-current-gate-audit.yml` | Koblitz Stage 147 current gate audit | workflow_dispatch | - | F |
| `koblitz-stage148-current-gate-audit.yml` | Koblitz Stage 148 current gate audit | workflow_dispatch | - | F |
| `koblitz-stage149-current-gate-audit.yml` | Koblitz Stage 149 current gate audit | workflow_dispatch | - | F |
| `koblitz-stage150-current-gate-audit.yml` | Koblitz Stage 150 current gate audit | workflow_dispatch | - | F |
| `koblitz-stage151-current-gate-audit.yml` | Koblitz Stage 151 current gate audit | workflow_dispatch | - | F |
| `koblitz-stage152-current-gate-audit.yml` | Koblitz Stage 152 current gate audit | workflow_dispatch | - | F |
| `koblitz-stage153-current-gate-audit.yml` | Koblitz Stage 153 current gate audit | workflow_dispatch | - | F |
| `koblitz-stage154-current-gate-audit.yml` | Koblitz Stage 154 current gate audit | workflow_dispatch | - | F |
| `koblitz-stage155-current-gate-audit.yml` | Koblitz Stage 155 current gate audit | workflow_dispatch | - | F |
| `koblitz-stage156-current-gate-audit.yml` | Koblitz Stage 156 current gate audit | workflow_dispatch | - | F |
| `koblitz-stage157-current-gate-audit.yml` | Koblitz Stage 157 current gate audit | workflow_dispatch | - | F |
| `koblitz-stage158-current-gate-audit.yml` | Koblitz Stage 158 current gate audit | workflow_dispatch | - | F |
| `koblitz-stage159-current-gate-audit.yml` | Koblitz Stage 159 historical gate audit | workflow_dispatch | - | F |
| `koblitz-stage160-current-gate-audit.yml` | Koblitz Stage 160 historical gate audit | workflow_dispatch | - | F |
| `koblitz-stage161-current-gate-audit.yml` | Koblitz Stage 161 historical gate audit | workflow_dispatch | - | F |
| `koblitz-stage162-current-gate-audit.yml` | Koblitz Stage 162 historical gate audit | workflow_dispatch | - | F |
| `koblitz-stage163-current-gate-audit.yml` | Koblitz Stage 163 current gate audit | workflow_dispatch | - | F |
| `koblitz-stage164-current-gate-audit.yml` | Koblitz Stage 164 current gate audit | workflow_dispatch | - | F |
| `koblitz-stage165-current-gate-audit.yml` | Koblitz Stage 165 current gate audit | workflow_dispatch | - | F |
| `koblitz-stage166-current-gate-audit.yml` | Koblitz Stage 166 current gate audit | workflow_dispatch | - | F |
| `koblitz-stage167-current-gate-audit.yml` | Koblitz Stage 167 current gate audit | workflow_dispatch | - | F |
| `koblitz-stage168-current-gate-audit.yml` | Koblitz Stage 168 current gate audit | workflow_dispatch | - | F |
| `koblitz-stage169-current-gate-audit.yml` | Koblitz Stage 169 current gate audit | workflow_dispatch | - | F |
| `koblitz-stage170-current-gate-audit.yml` | Koblitz Stage 170 current gate audit | workflow_dispatch | - | F |
| `koblitz-stage26-affinity-matrix.yml` | Koblitz Stage 26 one-CPU Phase-B matrix | workflow_dispatch | - | F |
| `koblitz-stage27-direct-mitm.yml` | Koblitz Stage 27 direct MITM matrix | workflow_dispatch | - | F |
| `koblitz-stage28-relation-yield-replay.yml` | Koblitz Stage 28 relation-yield replay | workflow_dispatch | - | F |
| `koblitz-stage29-public-target.yml` | Koblitz Stage 29 public unknown target | workflow_dispatch | - | F |
| `koblitz-stage30-n31-factor-search.yml` | Koblitz Stage 30 n31 factor search | workflow_dispatch | - | F |
| `koblitz-stage31-n31-unknown-scalar.yml` | Koblitz Stage 31 n31 unknown scalar | workflow_dispatch | - | F |
| `koblitz-stage32-wdsat-capacity.yml` | Koblitz Stage 32 WDSat capacity correction | workflow_dispatch | - | F |
| `koblitz-stage33-n41-public.yml` | Koblitz Stage 33 n41 public unknown scalar | workflow_dispatch | - | F |
| `koblitz-stage35-algebraic-walk.yml` | Koblitz Stage 35 algebraic walked descent | workflow_dispatch | - | F |
| `koblitz-stage38-algebraic-collection.yml` | Koblitz Stage 38 algebraic collection windows | workflow_dispatch | - | F |
| `koblitz-stage39-fixed-algebraic-holdout.yml` | Koblitz Stage 39 fixed algebraic holdout | workflow_dispatch | - | F |
| `koblitz-stage40-parallel-precompute.yml` | Koblitz Stage 40 parallel precompute | workflow_dispatch | - | F |
| `koblitz-stage42-n53-same-target.yml` | Koblitz Stage 42 n53 same target | workflow_dispatch | - | F |
| `koblitz-stage43-gate-audit.yml` | Koblitz Stage 43 gate audit | workflow_dispatch | - | F |
| `koblitz-stage44-n53-parallel.yml` | Koblitz Stage 44 n53 parallel query | workflow_dispatch | - | F |
| `koblitz-stage45-gate-audit.yml` | Koblitz Stage 45 gate audit | workflow_dispatch | - | F |
| `koblitz-stage47-n53-width.yml` | Koblitz Stage 47 n53 parallel width | workflow_dispatch | - | F |
| `koblitz-stage48-n53-width.yml` | Koblitz Stage 48 n53 parallel width | workflow_dispatch | - | F |
| `koblitz-stage50-gate-audit.yml` | Koblitz Stage 50 gate audit | workflow_dispatch | - | F |
| `koblitz-stage52-lazy-result.yml` | Koblitz Stage 52 lazy-label result | workflow_dispatch | - | F |
| `koblitz-stage53-gate-audit.yml` | Koblitz Stage 53 gate audit | workflow_dispatch | - | F |
| `koblitz-stage55-n53-pclmul.yml` | Koblitz Stage 55 n53 PCLMUL A/B | workflow_dispatch | - | F |
| `koblitz-stage56-pclmul-result.yml` | Koblitz Stage 56 PCLMUL result | workflow_dispatch | - | F |
| `koblitz-stage57-n53-unknown.yml` | Koblitz Stage 57 n53 unknown scalar | workflow_dispatch | - | F |
| `koblitz-stage58-unknown-result.yml` | Koblitz Stage 58 n53 unknown-scalar result | workflow_dispatch | - | F |
| `koblitz-stage59-gate-audit.yml` | Koblitz Stage 59 gate audit | workflow_dispatch | - | F |
| `koblitz-stage61-production-rank.yml` | Koblitz Stage 61 n53 production rank | workflow_dispatch | - | F |
| `koblitz-stage62-rank-aware.yml` | Koblitz Stage 62 n53 rank-aware scan | workflow_dispatch | - | F |
| `koblitz-stage64-parallel-expansion.yml` | Koblitz Stage 64 n53 parallel expansion | workflow_dispatch | - | F |
| `koblitz-stage65-n53-reduction.yml` | Koblitz Stage 65 n53 fixed reduction | workflow_dispatch | - | F |
| `koblitz-stage66-pipelined-expansion.yml` | Koblitz Stage 66 n53 pipelined expansion | workflow_dispatch | - | F |
| `koblitz-stage67-fused-pcl.yml` | Koblitz Stage 67 n53 fused PCLMUL reduction | workflow_dispatch | - | F |
| `koblitz-stage68-fused-result.yml` | Koblitz Stage 68 fused result | workflow_dispatch | - | F |
| `koblitz-stage69-n53-unknown-optimized.yml` | Koblitz Stage 69 n53 optimized unknown scalar | workflow_dispatch | - | F |
| `koblitz-stage71-single-core.yml` | Koblitz Stage 71 n53 single core | workflow_dispatch | - | F |
| `koblitz-stage72-current-results.yml` | Koblitz Stage 72 current results | workflow_dispatch | - | F |
| `koblitz-stage73-current-audit.yml` | Koblitz Stage 73 current gate audit | workflow_dispatch | - | F |
| `koblitz-stage74-dual-sign.yml` | Koblitz Stage 74 n53 dual-sign scan | workflow_dispatch | - | F |
| `koblitz-stage75-split-bloom.yml` | Koblitz Stage 75 n53 split-index Bloom filter | workflow_dispatch | - | F |
| `koblitz-stage76-insert-hash-reuse.yml` | Koblitz Stage 76 n53 insert-hash reuse | workflow_dispatch | - | F |
| `koblitz-stage77-compact-scratch.yml` | Koblitz Stage 77 n53 compact query scratch | workflow_dispatch | - | F |
| `koblitz-stage78-combined-fast-path.yml` | Koblitz Stage 78 n53 combined field fast path | workflow_dispatch | - | F |
| `koblitz-stage79-specialized-pair-batch.yml` | Koblitz Stage 79 n53 specialized pair batch | workflow_dispatch | - | F |
| `koblitz-stage80-specialized-support-expansion.yml` | Koblitz Stage 80 n53 specialized support expansion | workflow_dispatch | - | F |
| `koblitz-stage81-specialized-pair-sums.yml` | Koblitz Stage 81 n53 specialized initial pair sums | workflow_dispatch | - | F |
| `koblitz-stage90-current-audit.yml` | Koblitz Stage 90 current selected panel audit | workflow_dispatch | - | F |
| `koblitz-stage93-current-audit.yml` | Koblitz Stage 93 current CPU-0 audit | workflow_dispatch | - | F |
| `koblitz-stage94-filter-inverse.yml` | Koblitz Stage 94 n53 filter and inverse | workflow_dispatch | - | F |
| `koblitz-stage95-itoh-width.yml` | Koblitz Stage 95 n53 Itoh width sweep | workflow_dispatch | - | F |
| `koblitz-stage96-blocked-filter.yml` | Koblitz Stage 96 n53 blocked x filter | workflow_dispatch | - | F |
| `koblitz-stage97-selected-panel.yml` | Koblitz Stage 97 n53 selected five-pair panel | workflow_dispatch | - | F |
| `koblitz-stage98-compact-evidence.yml` | Koblitz Stage 98 n53 compact evidence | workflow_dispatch | - | F |
| `koblitz-stage99-optimization-chain.yml` | Koblitz Stage 99 optimization chain archive | workflow_dispatch | - | F |

### n37 (2)

| file | workflow name | trigger | inputs | ARC |
|---|---|---|---|---|
| `n37-shared-rank-folded.yml` | n37 target-blind folded rank replay | workflow_dispatch | - | F |
| `n37-shared-rank-targets.yml` | n37 shared-rank point-only target replay | workflow_dispatch | - | F |

### n53 (3)

| file | workflow name | trigger | inputs | ARC |
|---|---|---|---|---|
| `n53-combined-l384.yml` | n53 combined L384 one-table control | workflow_dispatch | - | F |
| `n53-native-cyclic-l384-outcome-replay.yml` | n53 native-cyclic L384 first outcome archive replay | workflow_dispatch | - | F |
| `n53-shared-log-batch.yml` | n53 true shared-log batch | workflow_dispatch | - | F |

### n83 (2)

| file | workflow name | trigger | inputs | ARC |
|---|---|---|---|---|
| `n83-online-k3114.yml` | N83 K3114 online capacity-frontier screen | workflow_dispatch | - | F |
| `n83-tier1-final.yml` | N83 fresh-point Tier 1 paired IC versus rho | workflow_dispatch | - | F |

### toy (5)

| file | workflow name | trigger | inputs | ARC |
|---|---|---|---|---|
| `toy-binary-elimination.yml` | Toy binary elimination audit | workflow_dispatch | - | F |
| `toy-binary-formula-audit.yml` | Toy binary formula audit | workflow_dispatch | - | F |
| `toy-f5-neighbors.yml` | Toy F5 neighbor measurements | workflow_dispatch | - | F |
| `toy-quotient-controls.yml` | Toy quotient and representation controls | workflow_dispatch | - | F |
| `toy-symmetry-quotient.yml` | Toy symmetry quotient | workflow_dispatch | - | F |

## Deliberately left in `.github/workflows`

See `left_in_place_with_reason` in `MANIFEST.json` for the full list with reasons. The ones that matter most:

- `weak-curve-lean.yml`: Never touched: bytes are sha256-sealed by check-pr-budgets.py and research/weak_curve_properties_lean_20261011/EVIDENCE_SHA256.tsv; also has a pull_request trigger.
- `weak-curve-conjectures.yml`: Never touched: bytes are sha256-sealed by check-pr-budgets.py and research/weak_curve_conjectures_20261012/EVIDENCE_SHA256SUMS; also has a pull_request trigger.
- `weak-curve-database.yml`: Has a pull_request trigger in addition to workflow_dispatch, so it is not manual-only; out of scope.
- `koblitz-stage25-single-core.yml`: Functional reference: scripts/run_koblitz_stage25_single_core.py custody-hashes this path as a source binding. Also keeps the koblitz-stage*.yml glob in scripts/test_workflows_parse.py and scripts/ci_disable_historical_koblitz_pr_triggers.py non-empty.
- `koblitz-stage132-current-gate-audit.yml`: Its path is a provenance label in generated docs/curves/registry.json and docs/browser/data.json and is embedded in immutable research tarballs; moving it would change generated output and force a registry.json regeneration (an embedded isogeny-cli input / release payload in ci-scope.mjs). Also keeps the koblitz-stage*.yml glob non-empty.
- `n83-hosted-paired-gate.yml`: Functional reference: research/koblitz_n83_factor_base_sweep_20261008/verification/hosted-paired-gate-20261015/run.py sha256-hashes this path into hosted_source_sha256 and raises if it is absent.
- `n83-tier1-github.yml`: Live protocol: research/n83_tier1_online_20261016/GITHUB_PROTOCOL.md prescribes gh workflow run n83-tier1-github.yml; created 2026-10-16.
- `ecc2k130-controlplane-integration.yml`: Policy hold: the real Postgres/Redis/S3 control-plane integration gate (fixture ecc2k130/aws/integration/docker-compose.yml), not a historical stage. The owner may move it.
- `ecc2k130-base-window-source-lock.yml`: Sparse-checked-out by archived workflows (input-freeze/outcome/probe-order-bound); must stay in .github/workflows.
- `ecc2k130-m3-four-policy.yml`: Sparse-checked-out by archived workflows (input-freeze/outcome/probe-order-bound); must stay in .github/workflows.
- `ecc2k130-m3-base-selector-source.yml`: Sparse-checked-out by archived workflows (input-freeze/outcome/probe-order-bound); must stay in .github/workflows.
