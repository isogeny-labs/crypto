# Unified factor-base CLI: implementation and validation

Implementation began 2026-10-17; final CLI fixtures ran 2026-10-18. This is a
software validation record, not a funded research campaign, a performance
comparison or a claim of improved regularity. Native CLI source:
`b3ed888964100f1bce4780a4c0500a399c7aee50`. Later report-only commits retain
those native source bytes. See [launch and limitations](README.md),
[requirements](IMPLEMENTATION.md), [architecture](architecture.svg) and
[the rendered report](report.pdf).

## Standalone installation and release packaging (2026-10-18)

The native executable is installed on this workspace's existing user `PATH` as
`factor-base-sweep`. It has the same SHA256 as the previously tested executable:
`dd9c1e582effe751777081fc4d7fc6ba1acd90e1696797ef210d20da58c3c3f0`.
From `/tmp`, outside the source checkout, direct `--help` and `plan` succeed.
With `PATH` limited to the installed binary directory, `/usr/bin` and `/bin`
(no Cargo directory), direct `verify` replays all 16 retained attempts and 29
point witnesses. A locally staged/extracted Linux executable also passes
`--help` and writes a binary preset plan with that restricted `PATH`.

The release workflow now explicitly builds, smoke-tests and stages the CLI
for its three existing native targets. YAML parsing, shell syntax for the
build/smoke/stage steps and all nine CI-scope tests pass. The first CI-scope
invocation used the wrong test filename; its corrected sandboxed invocation
failed, and the networking-enabled retry passed. Both failures are retained.
Installing to the existing user PATH first hit a read-only sandbox path;
the authorized elevated installation succeeded. Cross-platform release
execution and published download availability remain pending CI/release.

This follow-up changes packaging and documentation only; Rust sources, Cargo
inputs, the tested executable and the existing sealed evidence are unchanged.
It does not claim a fresh full-library pass: the four documented upstream
library failures and the other integration blockers below remain. Installation
check logs are retained in `validation/install-20261018.tar.xz` with its SHA256
sidecar. This archive contains checks, not a new search or timing campaign.

## Requested behavior and evidence

| Requirement | Status and evidence |
| --- | --- |
| One native launch command, including a seed/dimension grid | Verified: five CLI unit tests and six actual executable integration tests. Grid test freezes all 52 requested binary seed/dimension combinations. |
| Search structured bases and check successful decomposition | Verified on bounded public fixtures: native S3/F4, original-equation checks, independent finite-inventory truth census and actual point addition. Successful, negative and exhausted outcomes are separate. |
| Preserve exact points, ordering and derivation | Verified: native FB1, FBM1, ordered signed inventory, explicit basis words, recipe, order Merkle root, signed lookup, exact systems and artifact hashes. Native `verify` reconstructs and replays these records. |
| CADO-style live feedback | Verified: flushed human stderr and JSONL include stages, solver degree/matrix activity, verified and independent rates, rolling rates, provisional ETAs and matched-prefix ratios. Heartbeat and deadline behavior are exercised by subprocess tests. |
| Resume safely | Verified: immutable completed bytes, retained interrupted attempts, active-run exclusion, configuration/executable/F4-environment binding, hash-corruption rejection and new numbered attempts. A torn last event line is preserved and requires explicit recovery. |
| Real-time predictions | Implemented operational estimates, not accepted speedup evidence. Rank can stall; old resume segments may have different host/load conditions. The rate interval includes probe construction, truth census, solving, checks and incremental logging. |
| Certified degree of regularity or algebraic refutation degree | Not implemented. Positive/negative solving degree is a procedural diagnostic over F4 and its substitution tree. Negative solves can enumerate finite-field branches; no algebraic certificate follows from that degree. |
| Wide curves, arbitrary binary nonlinear bases, other summand counts | Not implemented by this solver path. Current path is two summands, small prime fields and explicit binary linear subspaces; the shape builder caps field degree at 31 and dimension at eight. |
| Total-cost or fastest-speed verdict | Not established: native pricing is incomplete; no isolated timing qualification, matched fastest external comparison or end-to-end claim was attempted. |

## Actual final-source CLI checks

Final native commands used Rust 1.98.0, Linux x86-64, two build workers and
two Rayon workers in the build/test driver. A run plan independently fixes its
worker thread count; the retained functional run uses one. This is an
ungraded shared managed workspace. The functional run overlapped a Clippy
analysis, so its live timing stream is deliberately not presented as speed
evidence. No paid host was launched.

The exact environment/argv, inputs, logs, exits and failures are in
`validation/runs.tar.xz`. The entry points are `validate-feature.sh`,
`final-acceptance/acceptance-commands.txt` and `RECOVERY.md`.
All 567 member hashes passed after extraction; deterministic repacking is
byte-identical. Native replay of the extracted functional run passes too.
[Bundle checks and SHA256](validation/README.md) retain the packaging receipts.

| Check | Observed result |
| --- | --- |
| Native CLI type check and production build | Passed. |
| CLI unit tests | Five passed; zero failed/ignored. |
| Executable integration tests | Six passed; zero failed/ignored. Includes binary/prime replay, corruption, frozen resume, interrupted attempt, hard timeout, matrix cap, malformed plan and concurrent writers. |
| All-targets native compile | Passed. |
| Strict Clippy for `factor-base-sweep`, `ecbench` and `ic` | Passed after three new `.is_multiple_of()` lint repairs; original failed lint log retained. No allow annotation added. |
| Existing `ic_progress` integration tests | Four passed; unchanged assertions. |
| CI-scope helper | Nine passed with networking/process support; sandbox `EPERM` attempts retained. Local duplicate-declaration repair exactly matched the independently landed upstream #2110 repair. |
| Full release library, serial, one complete command | **4,380 passed, four failed, 117 existing ignores**, 4,501 total; exit 101. Every new library test passed. See precise failures below. |
| Historical trace unit tests | Two passed, two failed at the unchanged registration guard. |
| All 384 historical records/cache replay | **Blocked**, not passed: current main lacks the historical registry rows. |
| Broad / canonical maintained Clippy | Failed on unchanged research-driver `drop(emit)`; canonical maintained lint also flags the unchanged `n83_precomputed_rho` example. Targeted touched-bin Clippy passes. |
| Conductor | Unavailable, command exit 127; pinned `.conductor/WORKFLOW.md` was HTTP 404. Neither is a pass. |

All 731 actual library dependencies match the captured library source and
fixtures. The retained unit binary SHA256 is
`f6ec0b235135f0d41eee4f9163dcba33ce70a8042dfa4036f0107d3c427a9593`;
the exact current traits JSON bytes are embedded in it. Later CLI-only edits
are outside these library inputs. This binds the reported **failed** library
run; it does not convert it into a passing gate. An earlier environment restart
interrupted both a library run and a feature build; their partial logs and
unknown final exits remain preserved, without a pass claim.

## Retained functional run

Run `functional-20261018T0025Z` uses registered models
`icv1-f2m7-tm13-ac10a42c` and `icv1-fp16-t351-d433f24e`, seed one, four
training and two holdout probes plus one positive control per built candidate.

```sh
factor-base-sweep run --out functional-both-run --preset both \
  --train 4 --holdout 2 --candidate-seconds 30 --heartbeat-seconds 0.1
factor-base-sweep verify --run functional-both-run
```

Observed: 16 candidate configurations, 15 completed and one inapplicable
(a three-dimensional subfield of the degree-seven binary field was requested).
The completed candidates retain 105 trial records. Native verification passed
all 16 sealed attempts, reconstructing the 15 completed inventories/systems and
checking 29 actual point witnesses and training-row replay. Negative probes do
not count as successful witnesses; the inapplicable case is not zero yield.
The five unit and six executable integration tests are a separate validation
from this retained run.

Executable SHA256:
`dd9c1e582effe751777081fc4d7fc6ba1acd90e1696797ef210d20da58c3c3f0`.
This matches the run's immutable manifest. Raw stdout, human stderr, JSONL,
plans, attempts, exact point inventories, systems and verification output are
retained in the archive. [Recorded counts](functional-fixture-observations.json)
are copied into the [witness figure](functional-witnesses.svg) by a metadata
extractor; [the plotting source](render-functional-plot.py) only renders them.
There is no confidence interval or shape ranking from four/two probes. Sizes
and parameterizations differ; no quantitative speed or regularity conclusion
is drawn. The [architecture source](architecture.dot) stays editable.

## Precise pre-existing integration blockers

The pinned main comparison is
`50f56d6a2e5a3ce04d16a1d02f2dbc3faa0aa3d4`. None of the following source or
fixture files was changed by this feature:

- Three `curve_traits::query` tests deserialize committed `docs/curves/traits.json`
  and fail with `missing field levels`, line 66, column 7. The exact same
  fixture blob is on pinned main.
- `jacobian_transfer::tests::trace_of_a_decides_whether_the_descent_exists`
  gets `Boundary` rather than the expected `FormulaMarginal` for c2pnb304w1.
  The classification code, cost model and registry inputs are unchanged.
- Historical trace tests/replay stop with `register historical curve identities
  before exporting traces`. Read-only metadata comparison finds all 48 rows
  in the frozen historical source absent from main's 2,375-row registry. Their
  alias targets remain. The trace verifier source blob is unchanged.
- The naming check reports 4,042 problems, including unregistered historical
  slugs and registry/alias disagreement. The 35 diagnostic/checker inputs
  match main. Of 1,340 globally off-cone pin files, none is in an ancestor
  directory capable of exempting a diagnostic file. This is provenance
  analysis, not a second baseline execution.
- Broad Clippy reaches the frozen research target
  `research/full_volcano_graphwide_ic_20261017/execution/driver.rs:206` and
  rejects `drop(emit)` with `clippy::drop_non_drop`. That file is unchanged.
  The canonical `scripts/clippy-maintained.sh` reproduces it and additionally
  reports `clippy::manual_is_multiple_of` at unchanged
  `examples/n83_precomputed_rho.rs:249`. Its keep-going run checked the other
  maintained targets without new errors. No target was excluded to clear it.

These failures are reported for review; no gate, limit, assertion or coverage
was disabled. The base also had native compilation defects preventing the
initial build. A separate prerequisite commit restores missing exports/imports,
the public-fixture declaration, distinct quartic variants and their dispatch,
and the missing Gaussian helper flag/wrapper. Its regression tests pass.

All frozen research/backlog/registry/alias/store files are unchanged relative
to pinned main. This preserves this change's scope; it does **not** claim
that main's already-missing registry rows have been restored. No historical
CPython generation, live S3 verification or fabricated point logarithm/timing
was used. The #1963 card remains draft, unrun, zero-funded and unleased.

Canonical scoreboard, leaderboard, progress timeline, performance-gain and
figure paths were checked and left unchanged: this implementation contributes
no accepted performance or mathematical result to plot there. Current native
pricing bands and the matched fastest external-reference gate remain required
for future measured claims; engineering usefulness does not require a rho win.
