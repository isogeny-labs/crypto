# Replay and pivot-directory controlled CPU comparison

This panel preserves the original baseline and all three complete public-point
fixtures. It isolates the two candidates added after round20 without changing
round20's frozen inputs. No cloud execution is authorized by these files.

| Arm | Integrated source | Binary | Batch | Contrast |
| --- | --- | --- | --- | --- |
| u | 2bd4588fba603ba1787e1d8ad82f084ef461d7cf | baseline | 64 | Original baseline |
| x | Same executable as u | baseline | 64 | A/A control |
| v | 5ad00c65dc4f06d66c8f09abb9ca4a265783c20d | candidate | 64 | Accumulated candidate |
| w | Same executable as v | candidate | 1 | Batch scheduling control |
| y | 2f08282ec3f5ee9b16f61795421e4871a1282800 | previous | 64 | Before compact pivot directory |
| z | 5b37f957e00d2347210f9bcadaeeac09a895ce8f | inverse | 64 | Before projective replay |

The basename inverse is a legacy packaging name for integrated source z; it
does not indicate a patched inverse-cache control. y/v isolates compact pivot
traversal and z/y isolates projective replay. Both contrasts include test-source
changes excluded from release example execution. Source-delta files and full
materialization manifests document these boundaries. u/v compares accumulated
implementation changes and cannot attribute a gain to one constituent.

## Preserved workload and decisions

Keep cases 0/1/2, seeds 111/222/333, one warmup and nine measured rounds per
arm/fixture, yielding 180 executions. Keep max_trials=16, max_work=1,000,000,
one software thread, deterministic interleaving and reversal. The complete
interval includes curve and target construction, all trial setup and discarded
trials, factor bases, relation collection, elimination, both independent full
replays and serialization. Process startup and artifact I/O remain excluded.
Both positive recovery and corrupted-scalar negative controls remain required.
No historical timing rows are pooled with this panel.

Retain all rows and compare complete certificate bytes. The existing native
analyzer describes L0 intervals; these do not establish the 10x acceptance
claim. In particular, replacing the pivot tree with a vector incurs insertion
shifts and may regress. A result is useful for candidate selection even if it
rejects one or both changes. The original baseline, all fixtures and complete
cost boundary remain requirements for the larger goal. An isolated acceptance
run and independent evidence review remain separate requirements.

The relevant prior work is projective-replay/README.md, pivot-directory/README.md,
the EFD Jacobian definitions and official Rust collection contracts linked in
the protocol. This panel makes no novelty claim.

## Reproduction and custody

prepare.sh creates a fresh output directory, archives the three named sources,
runs standalone release library tests, builds each complete example and hashes
each source tree and executable. It copies only the verified original baseline
and unchanged analyzer from round20. It refuses an existing output directory.
draft.sh creates metadata from the source manifests and refuses an existing
draft. These are thin shell build and metadata wrappers, not experiment runners.
The panel runner and native analyzer retain the previously reviewed fixed
workload. No root crypto or CUDA build is needed.

The updated native exporter explicitly supports round21; unknown study names
remain rejected. Synthetic exporter fixtures are software tests, not measured
recovery records. Preparation artifacts are outside the repository at
/home/ec2-user/hashed-ecdl-cpu-round21; only source manifests, protocol metadata
and software validation logs belong in this directory.

## Session handoff, 2026-10-30

### What was done

T-1042 prepares the comparison artifacts and T-1044 adds native exporter support.
The round20 worker-capacity dependency was rechecked; the existing occupying
job remained Running. No round20 or round21 execution was submitted.

### Decisions made

Measure the accumulated candidates next rather than continuing to stack
unmeasured code changes. Keep the single existing execution authorization
attached to round20. Preparation and protocol freeze do not authorize round21.

### Open threads

See qualification.json for the actual software readiness state. Any future
round21 run needs separate bounded admission, eligible capacity and raw custody.
The 10x goal remains active and is not established by software checks.

### Errors

The shared dated session log remains reserved by T-828; preserve this scoped
handoff rather than editing another owner's file. Record any actual build or
validation failures in the corresponding logs.
