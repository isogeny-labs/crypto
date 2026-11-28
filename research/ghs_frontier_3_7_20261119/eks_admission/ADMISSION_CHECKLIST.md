# Fail-closed admission checklist

Every box is conjunctive. A failed or unknown item stops before build or
submission; it does not authorize a smaller workload, another provider, a
different pool, or relaxed isolation.

## A. Freeze the scientific object

- [ ] Workstreams 3–7 have one reviewed final source revision; the checkout is
  clean, committed, and pushed to `isogeny-labs/crypto`.
- [ ] The exact protocol, specs, seeds, arm identities, ordering, target law,
  resources, deadlines, success/falsification rules, and raw-result URI are at
  that revision.
- [ ] Baseline, candidate, strongest matched reference, and byte-identical A/A
  control are explicit. Wall sessions have one warmup and at least five rounds.
- [ ] The execution contract is selected prospectively:
  - matched M3 counted work may use the reviewed standalone command package,
    with all wall fields labelled diagnostic; or
  - isolated wall work uses an implemented audited L2 benchmark contract, not
    the standard command path.
- [ ] The R67–R83 `cpu-spot` versus L2 `cpu-l2-spot` conflict is amended before
  any data exist.
- [ ] `check-experiment-git` accepts every source, protocol, request generator,
  request, image recipe, validator, and analysis input at the pushed revision.
- [ ] `scripts/checkpoint.sh gate` succeeds and its S3 receipt is bound into the
  launch manifest.

## B. Freeze finite resources and spending

- [ ] Account is exactly `104186811093`, region is `us-east-2`, and identity is
  recorded without credential material.
- [ ] Both USD 200 daily aliases and the USD 1,000 monthly budget are read in
  the same fresh window. Actual, reserved, and remaining values plus telemetry
  timestamps are retained; the daily aliases are not summed.
- [ ] Existing lane reservations, builds, queue time, storage, transfer, failed
  attempts, and an infrastructure retry are charged before deciding headroom.
- [ ] M3 retains its existing USD 5 all-attempt ceiling and one infrastructure
  retry within that same ceiling.
- [ ] R67–R83 worker scaling has its own finite all-attempt dollar ceiling.
  It is currently unset, so that lane is not launch-admitted.
- [ ] Maximum concurrency, vCPU, RAM, ephemeral storage, original absolute
  deadline, and retry count are frozen. Automatic measurement retries remain
  zero.
- [ ] Stop new launches when remaining headroom is unknown or insufficient.

## C. Prove live capacity and identity

- [ ] Run `scripts/isolab_pool_check.sh` immediately before each wave against
  the pool selected by the implemented contract. The receipt includes every
  Node and every active or pending Pod in all namespaces, NodePool status,
  namespace ResourceQuota, and named blocking Jobs.
- [ ] The exact requested wave fits both NodePool and namespace limits without
  changing quotas, providers, regions, or instance purchasing mode.
- [ ] Existing request, Job, and receipt identities have been reconciled; an
  ambiguous timeout is not resubmitted under a new name.
- [ ] The image is an immutable `isolab/tasks@sha256:...` digest built from the
  final source revision. Source SHA, workflow SHA, binary SHA-256, and OCI
  digest remain distinct and all match their manifests.
- [ ] The selected workflow/bridge revision has passed its contracts. Current
  main does not: run 44374607134 failed before build.

## D. Execute and admit

- [ ] Measurement workers receive Guaranteed requests/limits, whole-core NUMA
  placement, required affinity readback, and zero automatic retries.
- [ ] Every wall execution earns L2, every A/A pair passes the frozen gate, and
  no failed record is discarded.
- [ ] Request hash, GitHub/CodeBuild run, Job UID, Pod UID, actual image digest,
  native binary hash, input spec, and resource envelope agree.
- [ ] S3 manifests and every payload hash verify; paginated listings are
  complete.
- [ ] Native audit and exact replay pass. A second distinct host class performs
  the independent replay required for a frontier claim.
- [ ] Failures, timeouts, OOMs, Spot losses, censored intervals, and costs are
  retained under their actual status.

## E. Publish only after validation

- [ ] Complete cold one-target counts, all phases, the 0/0.1/1 pricing band,
  calibrated points where available, correctness, uncertainty, memory, and
  wall practicality are derived from sealed records.
- [ ] Producer output, independent validator, replay receipt, `VALIDATION.json`,
  and PCC standing all agree on scope.
- [ ] Only then regenerate the affected scoreboard, leaderboard, progress
  timeline, lab browser, benchmark record, visual report, and PDF.
- [ ] A restricted-family GHS result remains explicitly restricted to
  `E_0/F_{q^3}` and is not generalized to generic or standard curves.

