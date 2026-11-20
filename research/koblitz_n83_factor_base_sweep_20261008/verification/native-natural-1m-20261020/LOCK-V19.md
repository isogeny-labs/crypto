# Fresh attempts after protected existing-lock refusal

Preregistered2026-10-21UTC. Keep RESTORE-V18.md and every earlier attempt.
K2048-v18 exited1 after15.12057284commandseconds, no timeout, before CPU
planning/eviction/container/worker/query/SAT start. Error: cannot open lock
/tmp/crypto-bench.lock: Permission denied(os error13). Native card-check and
director verify actual relay/receipt/digests at unchangedc60158235. No search
or memory observation is admitted. K4096-v18 was never compiled/executed;
its planned attempt is superseded, not counted as an actual observation.

The existing lock is inode1088,uid1000,mode0600 in root-owned sticky/tmp;
fs.protected_regular=1. BenchLock previously requested O_CREAT on every
open; Linux can refuse that for a foreign-owned existing regular file.
Guard-v19 opens the existing inode without O_CREAT, uses create_new only
on NotFound, and reopens the same pathname if another creator wins.
flock exclusivity/wait/interruption semantics stay unchanged. Do not remove,
replace/chown/chmod the shared lock, bypass it, or change kernel protections.
The library change is in src/cryptanalysis/ecbench/isolation.rs, linked into
the guard. Retained SAT worker ELF and nine watched SAT sources stay unchanged.

28unit+7card tests pass. New tests cover held-lock exclusion, unchanged
inode/content and root opening a foreign-owned0600 lock in a sticky directory;
the last test was separately executed as root (not merely skipped as nonroot).
Native check-bench-lock successfully acquired/released the real inode1088,
preservinguid1000. That is instantaneous readiness, not a promise it stays free.
Archive exact setup outputs and source; later run fresh library/example checks
before publishing. Do not build/test during measured arms.

Authorize one K2048-v19 attempt, assess exact relay at unchanged frozen HEAD,
then separately compile/run/assess one K4096-v19 attempt. Keep the v18 live
60-second memory/swap gate and full-profile replay validation. Same original
public query, curve/group/field,seed,panels/fixtures,worker/image,CPU14/30/run14,
1CPUquota,8GiB/no extra swap,1000000conflicts/6000workerseconds/6600commandseconds,
strict quiet/thread/affinity/lock/Docker isolation gates. CPU PSIavg10<=5.00
before compile/dispatch plus original strict preflight. No automatic timed
retry or tracked edits/HEAD moves before assessment. New12000workerseconds;
conservative cumulative36000 across four actual plus two newly planned1M
commands. The superseded unexecuted K4096-v18 contributes no observation/cost.

Memory-limit drift or any new refusal remains a retained unadmitted outcome,
not permission to correct resource values during timing or silently replace it.
No claim of completion, memory cause, speed ranking, UNSAT/recovery or cold/rho/
identity promotion. All report/native summary/PDF/graph/benchmark/custody/PR
obligations from the earlier protocols remain. Original800K raw+sidecar are
now S3-verified. Original1M raw checkpointcbd453b3e37ff3cf97576ae40405ba9926eb70dfe9cd8dd90daa616b3868ecd9
is S3-verified with freshGET/payload/historical replay/resource rejection/
interrupted-prefix/repack checks. Final raw bundle must retain this lock refusal
and every fresh attempt along with all earlier failures.
