# Standalone GHS L2 bridge: fail-closed design and contract

Status: **design and offline contract validation only; not runnable and not
measurement-admitting**.

No Job was submitted, no image was built or published, no AWS resource was
changed, and no spend was incurred.  The new verifier exits `3` by default even
for a structurally valid package because no independent native isolation grade
exists for the standalone driver.

## Requested scope and actual execution

| Requirement | Status | Evidence or remaining gap |
|---|---|---|
| Digest-pinned image, source, driver, runner, and validator | Implemented offline | `ghs-l2-request.schema.json` plus cross-field validator; image is restricted to `104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:…` |
| Explicit argv/spec and no arbitrary shell | Implemented offline | Runner argv is one fixed five-element vector; each driver argv is an exact eight-element `ghs-strong-rho run q seed i0 i1 policy arm` vector; unknown fields fail |
| L2 observations | Blocked | Pod-authored `l2-gate.json` is checked structurally but is not trusted as an independent grade; default verification fails closed |
| Warmup and at least five rounds | Implemented offline | `warmup >= 1`, `rounds >= 5`; every order row must contain every named arm exactly once |
| Matched reference/A-A control | Implemented offline | Exactly one reference and one control are required and their driver argv must be byte-identical; result CI must be contained in `[0.9, 1.1]` and include `1.0` |
| Native validation and replay-all | Contract implemented; producer blocked | Validator path/hash and replay-all are bound; artifact counts and hashes are checked; no native GHS validator binary or producer exists yet |
| Zero retries/restarts | Implemented offline | Request retries are fixed at zero; artifact verifier requires Pod restart count zero and job receipt retries zero |
| S3 readback hashes | Implemented offline | Exact artifact set and strict `SHA256SUMS` verification; paths containing `/` roots or `..` fail |
| Source-commit image build/publish | Not implemented | Existing build and publish paths contain only `ecbench` and `isolab-fresh-owner` |
| Trusted launch and admission | Not implemented | No MCP tool, launcher, workflow, runner, or native isolation-grader integration was added |

This preserves the original measurement requirement as open.  Passing the
offline tests does not establish an L2 measurement or any performance result.

## Why generic command-on-L2 is insufficient

The scheduler layer can place a no-shell argv vector on the `cpu-l2-spot` pool,
request four exclusive CPUs, disable retries, apply anti-affinity, and enforce a
restricted container (`submit-job`, lines 48–84).  That is placement, not an L2
measurement grade.

The generic remote command route accepts a broadly user-selected command array
and records Kubernetes completion (`remote/run-command`, lines 21–44 and
79–109).  It does not download an exact Pod attempt and run the native replay,
isolation, A/A, and S3 admission gates.  Selecting its `l2` profile therefore
does not make a result admissible.

By contrast, `run-ecbench` binds the build before warmup (lines 46–52), requires
L2/warmup/five rounds/reference/control (lines 67–70), invokes native replay-all
(lines 115–125), regrades every record and rejects sub-L2 observations (lines
126–140), then requires a reference/control A/A result (lines 141–146).
`remote/run-request` independently downloads the exact Pod-UID S3 prefix and
rechecks its hashes and gates (lines 69–84).

The standalone driver cannot currently supply equivalent evidence:

- Its `run` output includes a driver-local `wall_s`, while `GHS_ISOLATION` is an
  environment string printed into calibration output; neither is a native
  isolation grade (`main.rs`, lines 512–542 and 587–621).
- It has a deterministic `check-parallel` control, but explicitly refuses
  `run-parallel` because worker affinity/NUMA readback and admitted timing are
  not implemented (`main.rs`, lines 940–976).
- The current image build compiles only `ecbench` and `isolab-fresh-owner`
  (`build-bundle`, lines 17–39); the Dockerfile copies only those binaries
  (`Dockerfile.ecbench`, lines 5–14); the publisher validates only that bundle
  (`publish-image`, lines 10–27).
- `isolab-fresh-owner` is coupled to ecbench session/preparation/publication
  schemas and `run`/`verify` semantics.  Treating an arbitrary GHS process as an
  equivalent child would weaken the existing replay and epoch-custody boundary.

## Smallest safe production route

The smallest **trusted-computing-base delta** is a native ECBench integration,
not a new standalone launcher.  It requires adding the restricted GHS curve
family/workload and strong-rho arms to ECBench, plus a canonical replayable GHS
record and mathematical validator.  It then reuses the already present:

1. source/digest-pinned build and image publication;
2. native isolation observation and record grading;
3. fresh-owner custody and exact execution epoch;
4. replay-all audit and reference/control comparison;
5. exact Pod-UID S3 readback and independent launcher verification.

A runnable custom bridge would instead need all five of those trusted
components generalized or duplicated, in addition to the GHS method adapter.
The contract in this directory is useful as a boundary test, but it intentionally
does not expose submission.

## Offline contract added

The following files are the complete code slice:

- `taskq/deploy/eks-karpenter/contracts/ghs-l2-request.schema.json`
- `taskq/deploy/eks-karpenter/remote/validate-ghs-l2-request`
- `taskq/deploy/eks-karpenter/remote/verify-ghs-l2-artifacts`
- `taskq/deploy/eks-karpenter/remote/test-ghs-l2-contract.sh`

The request validator additionally enforces relationships JSON Schema cannot:
source/protocol equality, exact runner argv, driver argv matching the workload,
`i1 = target_index + 1`, unique arm names, reference/control argv equality, and
complete per-round arm permutations.  The accepted driver target policies are
the implemented `sequence` and `ic-repeat` values (`main.rs`, lines 512–514 and
933–977).

The artifact verifier checks a test package but refuses admission unless invoked
with `--contract-test-only`; even then its output says `NOT L2-admitted`.  In
normal mode it performs every structural check and exits `3` with:

```text
NOT ADMITTED: external native isolation grade is not implemented for the standalone GHS bridge
```

This is deliberate.  A JSON file written by the measured Pod cannot establish
its own isolation.

## Reproducible validation

From the repository root:

```sh
jq -e . taskq/deploy/eks-karpenter/contracts/ghs-l2-request.schema.json >/dev/null
bash -n taskq/deploy/eks-karpenter/remote/validate-ghs-l2-request \
  taskq/deploy/eks-karpenter/remote/verify-ghs-l2-artifacts \
  taskq/deploy/eks-karpenter/remote/test-ghs-l2-contract.sh
taskq/deploy/eks-karpenter/remote/test-ghs-l2-contract.sh
```

Expected test result:

```text
ghs-l2 contract tests: PASS (2 positive requests, 9 negative requests, 1 structural package, 1 tampered package, default non-admission)
```

The positive fixtures cover both implemented target policies.  The negative
fixtures cover retry, tag image, wrong AWS account, shell argv, fewer than five
rounds, mismatched A/A control, a wrong `i0/i1` interval, disabled replay, and an
unknown field.  The artifact tests cover one hash-bound structural package, a
tampered package, and the mandatory default non-admission exit.

## Exact unblocker

Implement the GHS method natively inside ECBench and demonstrate, at one frozen
source commit, that `[pcc:unverified]`:

1. the build workflow emits an image digest and hashes the GHS code into the
   same build identity;
2. the native GHS validator re-derives the target relation and replay-all
   reproduces every canonical record;
3. reference/control are the same method and input, independently labeled;
4. the existing native grader produces L2/L3 for every measured record after at
   least one warmup and five measured rounds;
5. `run-request` downloads the exact Pod-UID prefix, verifies `SHA256SUMS`, image
   digest, source/binary identity, replay, A/A, and L2 gate;
6. tests demonstrate rejection of wrong answers, a replay mismatch, a sub-L2
   record, a mismatched image, a restarted Pod, and a changed source commit
   `[pcc:unverified]`.

Only after those checks pass should a finite-budget request be generated and
submitted through the existing ECBench MCP route.

## Scope limit: WS5 high/top jobs

This bridge schema is intentionally narrow: one standalone strong-rho workload,
one fixed driver path, four CPUs, and a maximum 4,800-second Job.  It cannot
represent or finish the WS5 high/top IC jobs, their different driver/protocol,
or their longer budget.  Expanding the deadline or substituting a different
binary would be a new contract and budget admission, not a compatible use of
this design.  Those jobs remain open and are not discharged by these tests.
