# E26 follow-up: whole worker-process instructions on the fixed first case

The first p=59, B=8, m=2 target-label-0 workload was fixed before the E26
panel. Preliminary certified results record 458 row multiplications for the
eight-bucket helper and 7,920 for the direct chain. This follow-up asks
whether that counter difference survives work outside matrix reduction.
Freeze this protocol and source before measurement. Keep the parent panel.

Measure every branch of all seven ordinary/candidate encodings on exactly
that existing workload with native F4 and available stock msolve q0. Repeat
the direct native input for A/A. No new target, parameter search, candidate
selection or heldout label is used. Use the same serialized inputs from the
independently certified parent run. Retain incomplete/unavailable references.

Use Debian Valgrind 3.24.0-3 Callgrind, with cache/branch simulation disabled.
The package URL is
https://deb.debian.org/debian/pool/main/v/valgrind/valgrind_3.24.0-3_amd64.deb
and SHA-256 is
0359e30820ccee7ee5c8d4c951ca276e3978b18f157d71bd77981de05b344907.
This is installed into the workspace tool directory, not the system.
Use --error-exitcode=99 so profiler errors cannot appear successful.
The unit is simulated guest instructions (Callgrind event Ir), not field
operations, CPU cycles, wall time or GAE. Read the final totals counter,
falling back to the summary only when totals is absent; reject other events.

The interval includes worker-process user-space startup, serialized-input
parsing, symbolic preprocessing, matrix reduction and serialized-basis output.
Sum all helper/diagonal branch workers. It excludes encoder construction,
parent orchestration, OS/kernel work, independent certification, root/sign
lifting and DLP recovery. It is a prototype worker-stage diagnostic, not
complete IC or pipeline cost and not a fastest-tool claim. Include msolve
as the available reference; Magma/FGb/OpenF4 remain unavailable.

Require native output bases and row counters to match the parent's certified
values. Require msolve basis bytes to match its independently certified parent
output; a different valid representation remains unverified in this bounded
probe, not declared mathematically incorrect. Only complete matched workers
have instruction totals. Keep a 5% preregistered A/A relative-spread gate;
two repeats do not establish broad statistical stability.

One process at a time, 4 GiB, 300 seconds per process, a 900-second global
checkpoint cap; native cooperative budget 250 seconds, GM=1 and one thread.
Run only after the parent solver/checker phase is quiescent. Preserve raw
Callgrind files, outputs, hashes and signals. Labels 32--63 remain unopened.
An advantage disappearing with these extra costs supports counter work-shifting
in this prototype. A surviving difference motivates full E27 accounting;
neither outcome closes E27 or proves a general twist-assisted advantage.
