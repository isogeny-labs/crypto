# Amendment 1 (2026-10-23, after the ABBA sessions and the `fix-notrim` sessions; before any `fix-notrim2` data)

What the data said before this amendment (sessions already on disk, unedited):
H4 was run as preregistered and the trim-free build (`fix-notrim`: the PR's source with
`malloc_trim(0)` replaced by `Some(0)`) reads **as fast as the PR binary** on `smoke`
(pooled medians of the non-IC arms 0.12-0.28 ms against 5.3-6.8 ms for the merge-base
build), i.e. the preregistered reading is "the wall drop comes from another change in the
PR". `src` has no other `malloc_trim`/`mallopt` call.

A candidate explanation visible in the PR diff (not a measurement): the PR also adds
`SolveProbe::read()` before the solve window, which reads `/proc/self/task`,
`/proc/self/task/*/schedstat`, `/proc/stat`-type files (`irq_counts`, `cpu_schedstat`) and
calls `getrusage`. Those reads allocate buffers; a request above the fastbin range makes
glibc run `malloc_consolidate`, so the probe could consolidate the fastbins incidentally
before the clock opens, with or without the trim.

New exploratory arm, labelled as such, to test that and nothing else: `fix-notrim2` =
the PR's source with both the trim removed (`analysis/notrim.patch`) and `SolveProbe::read`
made to read nothing (`analysis/notrim2.patch`, cumulative). Same specs, flags and one
session per spec (`fix-notrim2-smoke`, `fix-notrim2-claim`), after the sessions above.
Prediction written now: if the probe is the incidental consolidator, `fix-notrim2` pooled
smoke medians for the non-IC arms return to the `base` level (base / fix-notrim2 between
0.7 and 1.4 for at least 6 of 8 arms). If they stay at the `fix` level, the cause is
neither the trim nor the probe and is reported as unexplained. Nothing in the preregistered
H0-H3 changes. The `fix-notrim2` binary reports `solve_obs` as empty by construction.

## Addendum (same day, after `fix-notrim2` data, before `fix-noprobe` data)

`fix-notrim2` (no trim, probe reads nothing) read at the `base` level (smoke: base /
notrim2 = 0.75-1.05 across the 9 arms; claim `ic` online 1.06, solve 0.93), so with the
trim already shown unnecessary in the PR build (`fix-notrim` as fast as `fix`), the probe
is what removes the cost in the PR build. The cell of the 2x2 still missing is "trim kept,
probe removed" (`fix-noprobe`: `analysis/noprobe.patch`, the PR's source with only
`SolveProbe::read` reading nothing). Prediction written now: `fix-noprobe` reads at the
`fix` level (base / fix-noprobe >= 2 for at least 7 of 8 non-IC smoke arms); if it reads at
the `base` level the trim does not remove the confound by itself in this build. One session
per spec, same flags, run after this paragraph is committed.
