# Broad E26 native-worker instruction sweep, 2026-10-26

Continue the authorized experiments through every cell of the fixed training
grid. This is a solver-worker diagnostic, not complete cold IC accounting.
The earlier fixed-case probes and raw evidence are unchanged. Commit this
protocol, native source and generated compact plan before executing workers.

## Hypothesis and frozen population

Test whether the observed first-pair-invariant instruction reduction, or a
helper reduction after batching, persists beyond the first p=59 case.
Use every original E26 workload with arity 2 or 3: all ten ordinary fixtures,
B=8,12,16, original target label 0, exactly 60 configurations of ten target
pairs. These are already observed training cases, not heldout evidence.
The base and every equation are taken unchanged from the 2026-10-25 matched
job catalog. Record input/catalog hashes and each case's exact job IDs.
No final label 32--63 is generated or used.

For every configuration run all seven encodings, including all branches:
direct product membership, ordinary prefix order, Frobenius grouped order,
eight-bucket helper, first-pair invariants with diagonal, factored membership
and Boolean membership. A direct same-input A/A repeat precedes those arms
for each configuration. Freeze this order and the original workload order:
480 processes requested. No arm is omitted because of a previous timeout.

## Worker, interval and limits

Reuse the exact pinned E26 batch worker at source
497e54e7edf787a8743f216cd6967100f569ea61. Each process receives one candidate's
list of ideals and emits its list of bases. Native F4 uses the previous
grevlex/degree-32/GM=1 options. The new harness changes scheduling and records
only; it does not optimize the mathematical kernel between arms.

One process at a time; 4 GiB address space; one thread. Each candidate has
a 10,000 ms cooperative shared budget for every branch and a 12-second hard
outer timeout, with one-second TERM/KILL grace. Set a 32 MiB individual-file
limit. The 480-process worst-case timeout envelope is under two CPU-hours;
an explicit 7,200-second whole-sweep cap and 1 GiB aggregate raw-output cap
bound the run. Checkpoint after every requested cell. A cap leaves the rest
explicitly not started, not zero-cost outcomes. No paid host or lease.

Callgrind Ir counts worker startup, parsing, symbolic/F4 work and serialization.
It excludes encoder construction, parent orchestration, kernel/OS work,
certification, root/sign lifting, exception enumeration and complete IC.
Keep raw counters even for failed processes, but do not admit partial sums
as complete candidate costs. No wall time, GAE or fastest-tool claim.

## Correctness, reference and acceptance

Reopen the earlier per-branch input and certificate hashes. A new output
is accepted only if every branch has a complete old independent certificate,
the exact polynomial input matches, the new basis and row-operation counter
equal that certified result, no timeout/degree truncation occurs, and an Ir
total is present. A process is still attempted when an old certificate is
missing; retain its output as unverified until a separate certificate run.
Missing certificates must not silently select favorable comparisons.

The within-configuration direct A/A Ir relative spread must be <=5 percent
for that configuration's comparisons. Require complete candidate and control
outputs; preserve ties, losses and missing pairs. Compare each candidate with
direct and each of the other three ordinary controls separately. An observed
minimum of complete controls is descriptive, not a runnable portfolio or a
fully observed minimum when another control is incomplete.

An independent Rust auditor must reopen raw traces, capsules, result hashes,
parent completeness receipts and every branch before aggregating. Report
configuration counts and the ten distinct target pairs separately, with
paired ratios/ranges and all incomplete cells. Stock msolve remains the
recorded applicable external reference; this native-only sweep supplies no
new external-worker ranking. Broader external profiling, construction/root
intervals and the original E30/geometry cost experiments remain separate.

## Next decisions and stopping

If instruction reductions persist, test full construction and production
lifting on identical tasks before an E27 pipeline claim. If they disappear,
record a bounded null and retain the original full requirements. A mismatch
blocks cost interpretation and triggers a correctness investigation. Exhaustion
of this grid or its explicit caps is not exhaustion of all mathematical routes.
Deliver raw evidence, independent audit, source-linked visual report/PDF and
unadmitted benchmark copies in the existing PR. All full-campaign cards remain
blocked/unfunded and their original acceptance criteria stay unchanged.
