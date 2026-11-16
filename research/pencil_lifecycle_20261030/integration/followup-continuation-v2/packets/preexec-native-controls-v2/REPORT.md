# Fixed native pre-exec gate controls

## Proposal

Separate fixed software test translation unit includes the exact reviewed gate source, renames main, and redirects only the mountinfo open to a fixed fixture. Pristine production gate.c is compiled separately. No native compilation or test execution has occurred.

## Fixed controls

Sixteen immutable0444 fixture files in a0555 directory cover correct read-only mountinfo, writable/wrong-type/wrong-root/duplicate/missing/overlong/truncated variants, exact1-newline, empty/missing-newline/wrong-limit/max/64-byte/65-byte/embedded-NUL data. Nonce controls include length, punctuation, character and case rejection. The helper expects45 assertions and five group markers. It refuses PID1 and exercises the original main early-failure paths for Q1/Q2/Q3, invalid case/nonce and missing argument.

## Compiler bindings

364 existing files bind the qualified private musl1.2.6 wrapper, specs, include/lib trees, GCC11 driver/cc1/collect2/header/CRT/libgcc and binutils. This is input pinning based on the retained qualified toolchain, not a new toolchain qualification. The dynamic compiler process host libraries are not independently rebuilt. Both commands use static AMD64 baseline flags and strict warnings; production and test sources have distinct outputs/maps.

## Runner

Root admission must bind this manifest and fixed300s sequence,120s per compile,20s control caps. Fresh private output is required; no retry or overwriting. Each child receives only PATH=/usr/bin:/bin, LC_ALL=C, REALGCC=/usr/bin/gcc and fresh TMPDIR. Timeouts kill/reap the child process group and retain output. Deferred signal recording cannot raise between Popen and child assignment; persisted launch uncertainty precedes Popen, owned PID follows assignment, and checks after ownership/communicate fail closed. Unknown launch remains explicitly unresolved without retries; repeated signals do not interrupt cleanup. Use outer timeout -k5s300s supervision because cooperative hashing/filesystem IO is not a hard realtime bound.

## Readback and interpretation

Runner captures ELF and disassembly for both binaries, checks AMD64/noINTERP/noNEEDED, requires exact helper markers/stderr and pristine host-negative exit126 with no stdout. It rehashes source/toolchain inputs afterward. Independent review must inspect linked call paths; symbol presence alone does not establish invocation. Passing fixed fixtures does not observe container cgroups or qualify an image.

## Scope

Producer of these tests previously reviewed the gate source; later results require independent review. No Docker, cloud, tested ecbench/owner payload, scientific trial, campaign extension or image modification is included. Existing case and qualification budgets remain separate; this plan covers only deterministic native software checks.
