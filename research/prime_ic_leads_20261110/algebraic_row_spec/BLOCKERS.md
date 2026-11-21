# What prevents running this row from this container (2026-11-11)

Everything below was checked in this container; nothing was fixed. Each item names what unblocks it.

## Hard blockers (the row cannot be submitted from here)

1. **No AWS identity.** `aws` is not installed (`command -v aws` empty); `~/.aws/` holds only a
   `config` with an empty `[default]` profile and no credentials; no isolab EKS MCP connection is
   present in this session. So `aws sts get-caller-identity`, `launch-remote-command`,
   `scripts/isolab_pool_check.sh` and `oci-layer-publish.py` (ECR token) cannot run.
   *Unblock*: a session whose executor has the account-104186811093 identity (isolab-eks skill,
   `taskq/deploy/eks-karpenter/ACCESS.md`), or the GitHub route for the build step.
2. **The deployed image cannot run these arms.** `taskq/deploy/eks-karpenter/image.json` is ecbench
   `cb2ec6873f30` built 2026-10-19; `HEAD` (`2729b9056b`) is 7,061 commits later and the isogeny
   membership of `pkm-semaev` (`Membership::Isogeny`, `prime-isogeny` in the oracle) is in `HEAD`'s
   history but not in that build (the problem record dates it 2026-11-02; the file's first
   appearance in `git log -S` is the merge `52f6bc680d`, 2026-11-08). A fresh image at a pushed commit
   is required: `build-bundle` then `publish-image` (GitHub route, `main`-only OIDC, or the AWS
   route) and a layer carrying `spec.json`, `spec-m3.json` and `job-algebraic-row.sh`
   (`oci-layer-publish.py`, as `research/arity_completion_20261104/launch/` did).
   *Unblock*: item 1 plus a frozen, pushed commit.
3. **`build-bundle` cannot run here either.** It needs `cargo +1.99.1` and the
   `x86_64-unknown-linux-musl` target and a clean tracked tree
   (`git status --porcelain --untracked-files=no` empty). This container has rustc 1.97.0, one
   toolchain (`stable-x86_64-unknown-linux-gnu`), no musl target, and the worktree has a modified
   tracked file, `examples/pkm_tower_pilot.rs` (another agent's edit, which I was told not to touch).
   Build from a clean checkout at the frozen commit elsewhere. Also: cargo was out of bounds for this
   task, and `target/release/` holds no `ecbench`.
4. **The repository's spec linter was not run.** `ecbench plan --spec spec.json` is the real check
   (every curve `registered=true`, every method resolves, the `prime-isogeny` tower builds at each
   `(p, t)`, the base `columns` counts appear). There is no `ecbench` binary here, and no standalone
   spec validator exists (searched `tools/`, `scripts/`, `docs/ecbench/`). `spec_check.out` is only an
   ad hoc structural check. Risks it cannot cover: a refusal by `isogeny_tower` for one `(p, t)` (it
   needs a point of order `2^t` on a seeded auxiliary curve and `2^t <= p/4`); `prime-abscissa:size=N`
   meaning columns rather than base points (assumed from the program README, "51 abscissae at 20
   bits"); `max_trials` as a string param (as in the smoke).
5. **Conductor is not installed** (`conductor` not on PATH, no `.conductor/`), so
   `conductor check --summary ... --scope path:...` was skipped; the hooks warn and let edits through
   (CLAUDE.md). Run it before the frozen commit.
6. **A resume checkpoint and a benchmark record are required before launch and were not made.**
   `.resume/RESUME.md` exists but belongs to the leads session (null-support ladder, prefetch patch);
   it must be rewritten for this run, `scripts/checkpoint.sh push` and `gate` must pass
   (`CHECKPOINT_UPLOAD_TOKEN` is set in this environment; not tested), and `tools/benchrec.py init ...
   close` records the run in the same PR. I did neither because the write scope was this directory.
   The session log (`docs/session-logs/YYYY-MM-DD.md`, CLAUDE.md) was likewise not written for the same
   reason: the main session should log this work.

## Not blockers, but gaps that affect the row

7. **No local validator for command requests.** `remote/validate-request` accepts only
   `isolab.eks.request/v1`; `remote/run-command` validates `isolab.eks.command-request/v1` on the
   server (`cpu` an even number >= 2, `seconds` <= 604,800, `wait_seconds` <= 3,000, `retries` 0).
   `make-request.sh` was dry-run with `jq` on dummy inputs only (output checked by eye against those
   rules; not submitted).
8. **Route decision pending.** The audited benchmark route needs L2, >= 5 rounds, warmup and A/A and caps
   a Job at 3,600 s; this row is a count and is written for a command Job (L0, wall time
   descriptive). A command Job has no native L2/A/A admission (the arity study recorded the same
   deviation). The owner or main session must accept that route, or accept a larger spec for L2
   (README section 11). CLAUDE.md says L2 benchmarks require native admission; this row claims no
   wall-clock result and is not an L2 benchmark.
9. **D-EKS wording.** `DECISIONS.md` D-EKS approves "compute for the published reference arm
   (`reference_arm/`, about 49 CPU-hours, range 15-150)", whose first arm is msolve on a naive
   interval-style system. The lead note (section 7, lead 4) treats it as covering this
   pkm-semaev row; the PKM tower is the reference arm's "phase 2" (`pub-pkm`). The main session
   should confirm that reading. The row's derived cost (25 pod vCPU-hours, ceiling 46) is inside 49
   under either counting of CPU-hours.
10. **msolve is not needed**: `pkm_semaev.rs` has no msolve reference (0 matches); the engine is native
    `f4_fp_tower`. The pinned-msolve work in `reference_arm/` is a different arm and unaffected.
11. **Memory**: measured peak RSS 25 MB at k = 20, isogeny t = 6 (anchor); the pod request is 4Gi
    (command-request default and minimum useful), 8Gi for the m = 3 cell, whose memory is unmeasured.
    The ladder's bits form at t = 9 reached 4.5 GB; not used here.
12. **Container resources.** 4 CPUs, 15 GiB RAM, about 7 GB free on `/` (the volume reached 0 MB free for
    a moment during this session while a ladder runs from this worktree, which cost one command's
    output). Nothing here needs more, but a build in this container would compete with that
    ladder.
13. **Non-resumable Job.** A command Job is an ordinary fresh Job (no CRIU); a Spot loss during the
    4-8 h run restarts from scratch (one infrastructure retry in the protocol). If the owner wants
    resumable work, it needs the separate `isolab.eks.resumable/v1` contract and is not specified here.
14. **No positive control for the oracle** (ecbench cannot plant a decomposition into an
    `ic.pipeline` trial; no `oracle-check` for `pkm-semaev`). Recorded in the protocol; an
    implementation task (a planted-decomposition mode) would remove it.
15. **Model risk.** Columns, relations needed and F4 work per trial on fp16/fp18/fp20 are modelled
    from one curve and two targets; fp20 t = 6 is the one consistency point (README section 6). A
    cheap preflight on the first fp16 targets (seconds each) tests the model before the fp20
    cells commit hours; `ecbench plan` supplies the real column counts. Neither was possible here.
16. **Knowledge base not queried.** The `research-kb` MCP server is not exposed in this session, so the
    "knowledge base first" and lead-atlas steps were not done; no novelty or "not tried" claim is made.
