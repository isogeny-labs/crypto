# Minimal virtual-address amendment proposal

The completed first attempt remains failed and consumed. Its native error is Git pack mapping ENOMEM; the named pack is 4,680,295,288 bytes and the source installed RLIMIT_AS=4,294,967,296. This is consistent with virtual-address pressure, not proof of the exact failed mapping or a measurement of resident memory demand. Linux documents RLIMIT_AS as limiting virtual mappings (https://man7.org/linux/man-pages/man2/getrlimit.2.html).

This proposal changes one source line: remove the separate Python RLIMIT_AS assignment. It retains the existing mandatory live cgroup check for 4 GiB memory, 2 CPUs, zero swap and 512 tasks, plus all source/vendor/cache byte limits, deadlines, leases and cleanup controls. Git still runs inside that hard physical-memory cgroup. The proposal does not guarantee the next attempt fits RAM or completes; any OOM or further failure remains a recorded result.

Git offers packed mapping window/limit settings (https://git-scm.com/docs/git-config), but no configured override was observed. Rather than assert that an untested window setting is sufficient, this minimal proposal separates address space from the already enforced RAM bound. No diagnostic experiment or trial of either alternative was run.

A proposed single additional 900-second attempt would debit the existing 42,300-second balance to 41,400, leaving 39,600 after the separately reserved 1,800-second validation. The November 6 04:00 UTC deadline, one-payload concurrency and owner dollar waiver remain unchanged. No budget reset, new resource, role change or automatic retry is proposed.

Python 3.9 AST accepted the candidate. This is source only: independent review, explicit finite additional-attempt release, new unique runtime paths/lease, and publication of exact admission must precede any run. The old admission is not edited or reused for a retry.
