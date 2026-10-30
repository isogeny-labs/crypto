# CPU receipt correction — TASK-20261026-dc4016

This is software accounting on owner base27fca7749299469bd93bad3a1eed8e7e6a3b6a75,
not a timing comparison, algorithm change or scientific observation. Published
DEC-20261026-45aec1 expectations define exactly17 fixtures; the producer copied
and hashed `cpu-accounting-expected.txt` before any fixture execution. Old check
artifacts, dependency locks, mathematical routines and admission rules stay frozen.

New receipts carry `cpu_semantics=native-smallroot-cpu-v2`. Authoritative CPU
fields are checked nonnegative signed64-bit integer microseconds: seconds times
1000000 plus microseconds. Negative seconds, microseconds outside0..999999 and
conversion overflow are explicitly unavailable with reasons. Failed counter
reads never turn zero-initialized or stale structs into measured zero. Retained
legacy decimal seconds have exactly six fractional digits; e.g.1us is0.000001s,
not0.1s. Previous published receipts are not relabeled as corrected observations.

`backend.hpp` and `cdd_ipc.hpp` reap their actual child with wait4, retrying EINTR.
They retain the old wait-status file and a new `.cpu-receipt` before checking
exit/signal status, returning output or allowing a codec exception. The receipt
records raw wait status, exit/signal classification and checked user/system us.
If wait4 itself fails, status and CPU are unavailable and errno is retained.
Per-child receipts are diagnostic records, never extra phase CPU addends.

`Archive::phase` independently snapshots RUSAGE_SELF and RUSAGE_CHILDREN before
and after the callable. It records their user/system deltas separately on both
success and exceptions, preserving exception checkpoints. RUSAGE_SELF covers
all threads of the worker. RUSAGE_CHILDREN credits terminated children that
were waited for, and descendants only when their usage propagated through
intermediate waits. A phase delta credits children reaped in the interval; it
does not time-slice each child's CPU to that interval. A child started earlier
can be credited at its later wait. Counter regression/unavailability is explicit.
The old clock-tick diagnostic remains labeled worker-only with CLOCKS_PER_SEC;
it is not substituted for the authoritative microsecond fields.

Full phase process-tree CPU remains unknown. Unwaited/live descendants are not
covered, individual wait4 records must not be added again to the already credited
RUSAGE_CHILDREN delta, and cgroup cpu.stat is a distinct call-level observation.
No complete tree CPU, exclusive complete phase cost or performance gain is claimed.

The supervisor's CPU formatting uses the same conversion and six-digit formatter,
with versioned integer `user_us`/`system_us` authoritative fields. It retains a
wait4 CPU receipt before later cgroup-artifact reads and on a cleanup reap. Its
admission/cgroup behavior is unchanged and is never launched by these fixtures.
Pure formatting fixtures exercise these fields without invoking supervised().

Only the dedicated native fake child runs. It chooses a fixed mode by executable
basename, emits literal codec strings and controlled exit0/exit17/SIGTERM/malformed
responses, and never opens/interprets a Singular or cdd script. The missing-exec
fixture invokes no child executable and retains exit127. Both backend and cdd IPC
paths are checked. The cdd malformed case is a fixed software consumer-codec
rejection after raw IPC return; native hull/facet construction is never invoked.
Actual dynamic CPU values need only be nonnegative when available; no minimum
runtime, positive CPU requirement or speed comparison is imposed.

Build all native source plus dedicated tests with a fresh explicit BUILD prefix:

```
make -C experiments/joint-smallroot-native \
  BUILD=/workspace/native-smallroot-cpu-build-dc4016-1 -j2 all cpu-accounting
/workspace/native-smallroot-cpu-build-dc4016-1/cpu-accounting \
  /workspace/native-smallroot-cpu-build-dc4016-1/fake-child \
  experiments/joint-smallroot-native/spec/checks/TASK-20261026-dc4016/fixtures-attempt1
```

Source semantics were checked against installed sys/resource.h/sys/wait.h and
Linux man-pages6.19:
https://man7.org/linux/man-pages/man2/getrusage.2.html and
https://man7.org/linux/man-pages/man2/wait4.2.html. They specify timeval seconds
plus microseconds, process/thread scope, waited-child/propagated-descendant
coverage, syscall failure and wait4 resource returns. No cryptanalytic literature
claim or new mathematical theorem is involved.

The full current Rust suite is not clean: parent attempt7 at27fca exits101 for an
unchanged upstream unclosed delimiter in binary_semaev_chain_sat.rs (blob
3803524139ef5382d85f835107339d66d5909a6d). Parent log SHA256:
c5c5cf1a7ee6b111b654532f5e94d28a09453219696205221acd2cdaf935be14.
The prior4862-pass run belongs to b02, not this source. No unrelated Rust source
is repaired and no full-library rerun or current pass is inferred here.
