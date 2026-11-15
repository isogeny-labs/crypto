# Inputs freeze (amendment A1): Stage 0 run, image receipt and nonce pending

Filed 2026-11-14, before any measured run (PROTOCOL.md section 3). Every number
here is `[pcc:unverified]`: produced by the Stage 0 session, no pcc claim id.
Raw logs: `research/ic_lead_queue_20261114/A_repsel/logs/` (with `SHA256SUMS`);
evidence for D2: `research/ic_lead_queue_20261114/A_repsel/DRIFT.md`.
The Stage 0 outputs under `inputs/` are untracked and stay so; the digests below
are the record.

## 1. Stage 0 commands and exit codes (repository root, branch claude/serene-davinci-ujoi58)

| step | command | exit |
|:--|:--|:--:|
| 0a | `run/00-build.sh` (cargo build --release --locked of volcano-rho and ecbench; repsel build; repsel tests) | 0 |
| tests | `cargo test --release --manifest-path research/ic-representative-selection_20261105/native/Cargo.toml`: 15 + 1 passed, 0 failed | 0 |
| 0b | `run/01-classes.sh 24` | 0 |
| 0b | `run/01-classes.sh 28` | 0 |
| 0c | `run/02-specs.sh` (invoked with a stray argument `describe`; the script ignores arguments) | 0 |

Stage 0 was run twice (second run to test determinism). Identical across runs:
`jobs.tsv`, both selection logs, every `curves.jsonl`, `transport.jsonl`,
`ascend.json` and every file under `inputs/specs/`. Different: the 96 class-level
`inputs/classes/t*/s*/spec*.json` files (see D3), hence the full manifest digest.

## 2. Tools (`inputs/tools.sha256`, sha256 `2b2dbd04c45a3030fca1d4759eeba2397fdeba8602d30fbd0d1fa85495c762df`)

```
0c2cc1b408e33a43e55ee31cb75307e90df9d619      (HEAD at build; not the frozen commit, see D2)
bf7c2e149afd01e06df0bf7ade1c5ddbbecf3a11fd466553659640150730fd88  target/release/volcano-rho
73c4f66bdbd1b7d795ab142fea18dafbb3476d876a8f9f81fde4aeb4f9e581e7  target/release/ecbench
28f69273899d617308a5fd50b74ffa9becbb41181799a537648761302905f37f  native/target/release/repsel
```

These are the local glibc builds that produced the inputs. The binary that
measures is the one in the Iso Lab image (separate build, receipt pending).

## 3. Classes

Tier 24: seeds 2401..2434 tried (34), 22 rejected, 12 accepted: 2401, 2409, 2414,
2415, 2416, 2418, 2421, 2424, 2426, 2428, 2433, 2434.
Tier 28: seeds 2801..2836 tried (36), 24 rejected, 12 accepted: 2801, 2803, 2804,
2806, 2811, 2819, 2823, 2830, 2831, 2833, 2834, 2836.
Arm C: all 12 at tier 24, the first 6 accepted (2801, 2803, 2804, 2806, 2811, 2819) at tier 28.
Logs: `inputs/classes/t24.selection.log` sha256
`522c420cb4758db71469c03c8da7a0950edb46eed45bf91f7547a22ce730efa2`,
`inputs/classes/t28.selection.log` sha256
`5aaaf0e364cdd9cdd2716066dad21701c122941089737b9214d2d89bc351853e`.

## 4. Digests

| item | sha256 |
|:--|:--|
| inputs digest of record (D3): `inputs/MANIFEST.sha256` without the 96 class-level `spec*.json` lines, i.e. `grep -vE '^[0-9a-f]+  classes/t[0-9]+/s[0-9]+/spec[^/]*\.json$' inputs/MANIFEST.sha256 \| sha256sum` | `7bbc0fd1a4d6638bb31a7c3f6f76e6d8d8adddb51332c096fa41b2f92ca1ae73` |
| full `inputs/MANIFEST.sha256`, run 2 (the one on disk; 578 lines) | `f5e7e64db409f84ccbe6efff635d6f969637f4a0bbd0d3760e6624a5f36d2e1f` |
| full manifest, run 1 (disclosed, superseded) | `9d4d5cbce666fbb583558897b0393a459cd0d6999c6532506ddecff7300f188b` |
| `inputs/jobs.tsv`: 205 jobs (tier 24: 100, tier 28: 105) | `038aaafc19377d9327cd082376d21643c2008f64bbc3cedbf6ff733243b241b8` |

Hold jobs (`jobs-hold.tsv`) do not exist until `run/05-hold.sh` runs per class;
about 42 are expected (estimate). If Stage 0 is re-run before launch, the full
manifest of the run actually launched replaces the run-2 line (D3).

## 5. Mask nonce

`02-specs.sh` does not produce one. It is drawn after this file is committed and
pushed, by the analysis owner, with `od -An -N8 -tx8 /dev/urandom`, and recorded
below as the line `mask nonce: <16 hex>` (that exact form; `run/06-analyse.sh`
reads it with `grep -oE 'mask nonce: [0-9a-f]{16}'`):

mask nonce: f0115f0e0192bd9a

## 6. Decisions of the main session, 2026-11-14

- **D1 Pricing.** The registered analysis stands unchanged: `repsel analyse` is the
  primary instrument as frozen (U from `cost.unpriced`). ecbench's default changed
  on 2026-11-08 to price the IC table's `table_insert_attempts`, after the freeze.
  The cross-check must price the same way. Resolution (main session, 2026-11-14): no
  frozen file is edited (their hashes are recorded in `protocol.json`
  `extensions.crypto`). `run/06-analyse.sh` runs as frozen; its band cross-check,
  priced with the newer default, is expected to differ from `repsel analyse` at
  w = 0.1 and 1 by the table-work term. The same band command is then run a second
  time with `--no-price-table-work` (the one-line difference below), expected to
  agree with `repsel analyse`; both outputs are kept and reported. Only `repsel
  analyse` enters the decision rule. The second command:

  ```
  - $ECBENCH band --dir "$s" --weights 0,0.1,1 --json "$out/band-$(basename "$(dirname "$s")").json" > /dev/null
  + $ECBENCH band --dir "$s" --weights 0,0.1,1 --no-price-table-work --json "$out/band-$(basename "$(dirname "$s")").json" > /dev/null
  ```

  (`ecbench band --help` lists the flag; the same flag exists on `compare`.) No
  re-pricing under the newer default enters the decision rule; a later one would be
  a separately labelled secondary table. Evidence: DRIFT.md E5. Not yet seen in a
  record: whether the spec's `mitm:negation_folded=1` emits the marker at run time.
- **D2 Build revision.** Local binaries were built at HEAD `0c2cc1b408`, not at the
  frozen commit `c6cd50bcc8`, because a frozen-commit worktree does not fit on this
  host (3.4 GB free at the start). Evidence of equivalence on the study's methods
  (DRIFT.md): exact replay at HEAD of committed v2 records written before the
  freeze, 604 of 604 identical (200 random from each of three sessions plus 4 named);
  `volcano-rho` regenerated v2 class l2-s1001 byte for byte; all 205 specs pass
  `ecbench plan` and, with a placeholder image receipt, `validate-request`.
  **Limit, stated plainly: no binary was built at c6cd50bcc8 and no two-binary
  comparison on the same spec was made; the check is old records reproduced by new
  code, plus reading of the diff on the call path.**
- **D3 mktemp path.** `run/01-classes.sh` passes `--out "$(mktemp -d)/s<seed>"`, which
  lands in the `description` of the 96 class-level `spec*.json`; two Stage 0 runs gave
  different full-manifest digests. Disclosed. The inputs digest of record is the
  manifest without those lines (section 4), plus the full manifest of the run that
  is launched.
- **D4 duplicate workload.** All 46 rejected seeds (22 at tier 24, 24 at tier 28)
  were rejected by `repsel specs` with `duplicate workload`, a rule of the frozen
  class generator that the written acceptance text (PROTOCOL.md section 3,
  `design.json` `class_search.acceptance`) does not state. Disclosed; no class was
  added or removed by hand. The accepted classes are conditioned on it; what in a
  rejected class produces the duplicate was not investigated.
- **D6 `protocol_sha256`.** Checked natively, no Python run. Definition read in
  `research/primefield-companion-schur-20261101/tools/experiment_protocol.py`
  (identical copy in `primefield-companion-followup-20261102`): lines 70-72 and 133-136,
  sha256 of `json.dumps(body, sort_keys=True, separators=(",", ":"), ensure_ascii=False)`
  where `body` is the protocol without the `freeze` key. Reproduced with
  `jq -S -c 'del(.freeze)' protocol.json | sha256sum` (the file has no non-ASCII
  character, no backslash escape and three plain floats, so the two serialisers
  should agree): **a8cddf42bab78401990b946b02bcf08d9d9d4c68c1784117bb14b0644986b115,
  the same for the file at c6cd50bcc8 and at HEAD, which is not the recorded
  `dcee3b0f731e2f8705b7ee3ce311140338dc1f4cf49bcc0842af4466a4da8188`.** Not
  reproduced. A jq/Python serialisation difference cannot be ruled out without
  running the Python tool, so this is neither a confirmed mismatch nor a
  confirmation. What was confirmed natively with `sha256sum -c`: the 16 file hashes
  in `extensions.crypto` (`native_sources_sha256`, `run_scripts_sha256`) and
  `design_sha256`, `comparisons_sha256` all match the files on disk, and
  `analysis_plan.script.sha256` (`analyse.rs`) matches. **Settled by the main session,
  2026-11-14:** the lab's protocol tool,
  `python3 -I research/primefield-companion-schur-20261101/tools/experiment_protocol.py hash protocol.json`,
  prints `dcee3b0f731e2f8705b7ee3ce311140338dc1f4cf49bcc0842af4466a4da8188`, equal to the
  recorded value, and `validate` reports 0 findings. The jq value differs only by
  serialisation (most likely float formatting). The frozen protocol is unchanged.

## 7. Statement on seen data

This Stage 0 session ran no `ecbench run`, no IC and no rho on any study curve or
any Stage 0 spec (only `ecbench plan` and `volcano-rho` class generation with
`--controls 0`). Replays used committed v2 records on v2 curves. The main session
adds its own statement when it files this amendment.

## 8. Still open

- Image receipt: filed 2026-11-14 in `inputs/image.json` from the `isolab-publication` artifact of
  `isolab-experiment.yml` run 43633983571 (publish_only, source revision c6cd50bcc8, the frozen commit):
  image `isolab/tasks@sha256:5f1febae…bf02`, ecbench sha256 `ee08c78e…d87c`. Mask nonce drawn the same day
  (section 5).
- Launch: blocked while every AWS CodeBuild concurrency quota in us-east-2 reads 0 (the cluster launcher is a
  CodeBuild project); raising it is an owner action.
- `protocol.json` is not edited for A1: the protocol tool treats any change to it as a
  new version (`validate`: "file an amendment under a new version"). A1 is this file,
  the one the protocol designates for the inputs freeze; its first commit, before any
  measurement, is its timestamp.
- `check-experiment-git` receipt for the pushed commit, executor AWS identity,
  budget check, first card, first job `repsel-t24-s2401-ab-0`.
