# Independent replay checklist

This is the downstream procedure after a complete, sealed session exists. It
does not authorize a run or substitute a local replay for an independent host.

## Inputs frozen before delegation

- session directory and its immutable file manifest;
- exact source revision, executable SHA-256, OCI digest, spec and protocol;
- original `ecbench host` capsule and `env_class_id`;
- both selected run IDs (IC and strongest matched rho) and their workload ID;
- replay command, timeout, expected output path, and zero-retry rule; and
- public known-answer target or an independently checkable target constructor.

## Executor procedure

1. Build or retrieve the exact executable identified by the session. Record
   its SHA-256 before execution.
2. Record the validator host capsule. Its `env_class_id` must differ from the
   original measurement host; another pod with the same class is insufficient.
3. Run `ecbench verify --replay-all --exit-code` over the sealed session.
   Preserve stdout, stderr, exit status, and receipt even on failure.
4. Confirm every deterministic record replays identically. Recompute every
   derived count and current-rule grade rather than trusting producer fields.
5. Confirm the receipt reproduces both the selected IC and rho run IDs and
   checks every recovered scalar against the public point.
6. Hash the receipt and store it durably. Record its distinct host class and
   pointer in the validation bundle.

## Refusal conditions

- source, executable, session, workload, or run identity mismatch;
- same original and validator host class;
- fewer than all deterministic runs replayed;
- any nonidentical count, accepted invalid record, or wrong scalar;
- receipt covers only one arm;
- session bytes changed after sealing; or
- output exists already and would be overwritten.

Wall time is not reproduced by this procedure. A wall confirmation is a new
admitted session on a matched host class, not a reinterpretation of replay.

## Bundle fields discharged

Only after the procedure succeeds may the following be set:

```text
measurement.replay.audit_status = "OK"
measurement.replay.replay_all = true
measurement.replay.identical_replays = deterministic_runs
measurement.independent.original_host_class != validator_host_class
measurement.independent.both_arms_replayed = true
measurement.independent.every_recovered_scalar_checked = true
```

All receipt digests remain exact lowercase SHA-256 values. Producer-authored
booleans alone do not discharge these fields.

