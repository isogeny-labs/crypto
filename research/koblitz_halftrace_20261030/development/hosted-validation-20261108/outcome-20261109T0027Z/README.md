# Hosted native full-library terminal outcome

Run [42529489383](https://github.com/isogeny-labs/crypto/actions/runs/42529489383),
attempt1, requested and checked out immutable source
`cda1e5810f41550a2b929cbdb6fe4ff242865617`. The workflow and manual job
128880307287 finished with **failure**. The complete unfiltered parent suite
is **incomplete**: its test step reached the declared55-minute limit, and
`parent-exit.json` and a parent summary were never produced. Cargo and log-writer
exit codes remain null. The job log records runner cleanup of Cargo, tee and the
test process; that cleanup does not supply their terminal statuses.

The log records a failure in
`cryptanalysis::ic_wide::tests::run_f2m4_is_the_one_word_pipeline`.
Its assertion diagnostic was not emitted before timeout. The long-running
`jv_isogeny_walk::tests::end_to_end_from_a_non_weak_curve_recovers_and_verifies`
has no completion line. This observation does not establish why it failed to
finish. Two successful one-test summaries each have5,936filtered tests; they
are nested children, never the5,937-test parent result.

Host admission, immutable checkout/toolchain binding, release binary build,
post-run source checks, tracked cleanliness and artifact upload passed.
All four diagnostic resource-publication tests report ok on this host,
including the two that failed on the unsupported local host. Those old local
failures and incomplete parent remain unchanged. Separate job128880359995
passed workspace/all-targets and wide-vars compilation plus core-crate tests.

## Receipts and limits

- `outcome.json` records the terminal scope; `api-snapshot.json` retains
  the one run/jobs/artifacts observation.
- `receipts/` retains original host/request/compiler-version/source/binary/log
  identities, build exit0, time-interface reads and post-run checks.
- `verification.json` records checks actually performed locally. The compiled
  binary itself was not uploaded; its SHA is a runner-recorded identity, not an
  independently rehashed binary.
- `selected-log-lines.txt` and `timeout-lines.txt` are exact selected lines,
  not substitutes for the complete partial log.
- `FILES.sha256` identifies every raw file in `runs/`: all14 original
  artifact entries and the decoded complete job-log text.

The original Actions ZIP is123,058bytes, SHA-256
`a158b153347ae6fd01e0caae377997f83656c79b1c161f40837caa71cd59f3e8`;
the downloaded bytes match the API/upload digest. Artifact13638049715 is
temporarily available as
[native-library-42529489383-1](https://github.com/isogeny-labs/crypto/actions/runs/42529489383/artifacts/13638049715)
until2026-11-22T23:28:48Z. This is development retention, not canonical S3 storage.

The deterministic full raw archive is155,880bytes, above the64KiB Git data
limit. It and the original ZIP are ignored locally pending an approved uploader;
their hashes are committed in `LOCAL_ARCHIVE.sha256` and
`ACTIONS_ARTIFACT.sha256`. No fabricated store manifest or durable-upload claim.
The partial library log is531,906bytes, SHA-256
`cfbdc1e118cbde671f8e25bea5dd8345a471a7b4a83663fea8a7cbb674c93835`.

With the retained archive present, extract and verify from this directory:

```sh
sha256sum -c LOCAL_ARCHIVE.sha256
tar -xJf runs.tar.xz
sha256sum -c FILES.sha256
```

Repacking the extracted tree must reproduce the same archive:

```sh
tar --sort=name --mtime='2026-11-08 00:00:00Z' --owner=0 --group=0 \
    --numeric-owner --mode='u+rwX,go+rX,go-w' --format=gnu -cf - runs \
  | xz -6 -T1 > /tmp/native-library-repacked.tar.xz
sha256sum /tmp/native-library-repacked.tar.xz
```

No additional attempt, timeout increase, local suite restart, test suppression,
clock fallback or native source edit occurred. A main-branch test change at
`de17b7b6897387c71e41e26c3bd913a1bdabdf40` separates feasible wide parity
from two-large-prime cycle coverage; it is a triage lead, not a recovered
assertion diagnostic or an integrated fix in this branch.

This job is development correctness validation. It provides no performance
comparison, checkpoint, scientific execution or speedup. P1 stays unresolved
and auto-merge paused. The original dispatch/progress receipts, frozen study
inputs and dated November6 report remain unchanged.
