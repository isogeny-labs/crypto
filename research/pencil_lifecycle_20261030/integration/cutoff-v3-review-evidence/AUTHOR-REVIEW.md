# Computation cutoff V3: independent findings corrected

Private unapplied proposal. V2 and every earlier source/log/receipt are preserved. No root edits, cloud actions, scientific execution, image publication, namespace-wide signaling or deadline renewal occurred. The new complete patch is incremental over frozen fresh-closure cloud3e7516046a07d65df3ca136c80da8af691c03088716172f588730fe06edbffbe and supervisorf8bae891. A separate exact V2-to-V3 correction patch is supplied. Both hashes and source hashes are in source-manifest.json.

## Monotonic bound

V2 checked only SystemTime, permitting a backward UTC correction to extend the remaining allocation. V3 captures a single Linux CLOCK_MONOTONIC deadline at supervisor admission. It samples monotonic before UTC and computes remaining time in nanoseconds, avoiding whole-second rounding or sampling-delay extension. The Deadline object is copied unchanged to watchdog, command supervision, checkpoint, launch, continue and operation admission; it is not reconstructed per call. All checks still reject UTC expiry, and also reject missing/invalid monotonic samples or reaching the fixed monotonic deadline. UTC moving backward after admission cannot extend that bound. Legacy requests retain UTC-only behavior. UTC/monotonic clock-read errors fail closed.

The fake-time fixtures exercise rollback while the original monotonic allowance remains, cutoff at that same original bound despite rollback, a forward UTC expiry, an invalid backward monotonic sample, missing sample, expired admission, arithmetic overflow and fractional-second allowance. Copies representing command and watchdog have the same boundary. These tests do not alter host clocks or run the namespace watchdog.

## Kubernetes default and owner identity

ObjectFieldSelector.apiVersion defaults to v1 according to the official Pod API reference: https://kubernetes.io/docs/reference/kubernetes-api/core/pod-v1/#ObjectFieldSelector . The computation matcher now normalizes only an OMITTED apiVersion on an actual env.valueFrom.fieldRef object to v1. It preserves fieldPath, all other keys, array ordering, owner env, image, command, mounts and namespace settings. Explicit null, empty string, v2, changed selector or extra env do not gain equivalence. Production matcher fixture accepts the genuine API default and rejects changed selector. The generated reservation transport itself remains exact.

The matcher also rejects nonempty or malformed initContainers/ephemeralContainers, closing the reviewer's additional same-namespace owner gap. Empty lists are equivalent to omission. Future privileged external injection after reconciliation remains a controller/RBAC trust boundary, not something this software receipt certifies.

## Validation and limits

The actual final private copied controller corpus passed seven named software fixtures: three deadline fixtures, delayed absolute reservation binding, legacy transport rejection, owner/namespace reconciliation and fieldRef default-only normalization. Strict all-target Clippy passed. validation/FINAL-VALIDATION.json binds exact source, dependency, command, log and receipt hashes. No full core/ecbench tests or scientific solver calls were run. Existing fake-process fixture invokes only /bin/sh/sleep/nonzero/signal cases; pure-time fixtures execute no child.

The namespace watchdog remains source-only and requires explicit root review and actual reviewed private-Pod/pinned-runtime admission, as documented in V2. It was NEVER executed. Controller assertion plus root PID1 is a trusted deployment boundary, not standalone attestation. No physical infrastructure deallocation or measured allocation-cost claim follows. Original25200seconds and2026-10-30T22:08:40Z remain unchanged.
