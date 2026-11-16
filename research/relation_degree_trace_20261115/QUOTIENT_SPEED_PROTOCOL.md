# Conditional runtime evaluation of the quotient lead

Status: frozen before any quotient timing. Do not execute unless the required
S3 checkpoint and local isolation gates pass. No new runtime or practical
advantage is established `[pcc:unverified]`.

The output is the complete reduced grevlex basis of the original paired-f3
ideal. Compare the existing best measured native `projection-criteria` arm
with `quotient`, in one newly frozen uninstrumented worker binary. A/A uses
`projection-criteria`. Do not use the old textbook raw baseline as the
denominator for a claim about this lead. The direct-quadratic construction
remains a separate incremental ablation, not a free preprocessing step.

The quotient arm includes original-input projections/certificates, Euclidean
unit checks, coordinate propagation, all rewrite/proof expansion, change of
order and final membership/input/S-pair checks. In the worker, all those stages
are in `solve_and_internal_certificate_checks_ns`, rather than the separate
preprocessing field. The total charged interval includes all of them. Both
arms exclude parsing/primality, witness-based fixture generation, process
launch and printing. Add the driver-timed basis equality replay to both.
The existing arm uses a chain of certificates through augmented generators;
the quotient arm expands final original-input certificates. Both must establish
the same full ideal/basis correctness; neither may drop checks to win.

Exploratory roster: both original primes, variables 3/4/6, seeds 101/307/911.
These 18 cases have already informed development; they are not fresh. Use
three bounded panels: variables 3/4 on both fields (12 cases), six-variable
31-bit cases (3), and six-variable 53-bit cases (3). This preserves the per-panel
20-minute cap without knowingly scheduling every slow baseline in one panel.
No deterministic failure or nonunit is replaced with a new seed.

21 alternating AB/BA paired samples, 7 A/A pairs, 10000 fixed paired-bootstrap
draws with seed 20261115. Admission requires all samples and an A/A interval
including 1. A drastic native result requires lower 95% ratio endpoint >10.
Report smaller gains, losses, unsupported cases, failures and capped samples
as such; no pooled ratio or censored-baseline division can supply a win.

For each variable count with an admitted exploratory tenfold cell, confirm
that entire variable-count family on both primes with seeds
**11003, 13001, 17011**. These are frozen here before generation or timing.
Keep all six cases, with no replacement after failed controls or losses.
Do not use the direct-quadratic lead's separate 5003/7001/9001 holdouts.
Keep per-case conclusions separate from exploratory results and other sizes.

One thread on an allowed isolated CPU, `tools/isolated_bench.py reserve`,
2 GiB, 30 seconds per child, 20 minutes per panel. Retain the exact source,
compiler, executable hashes, host, protocol label, inputs and all raw samples.
No build/CAS jobs overlap timed native samples. Stop a case after its first
failed pair, and retain later missing samples as incomplete. Instrumented
counter builds cannot provide timing results. Modular-product and inversion
counters are diagnostics; sparse coefficient-update counters omit dense
quotient/linear-algebra updates and are not total operations or GAE.

External references: supported Singular std/slimgb and msolve on identical
equations with exact output agreement. Before a practical or fastest claim,
compare full construction plus solver and matched checking against the best
supported mature route in one common clock scope. Previous solver-only CPU
or CLI startup-inclusive diagnostics cannot provide that ratio. Unsupported
coefficient rings stay explicit; no scaling, whole-ECDLP, relation-yield,
P/Q-dependency or regularity claim follows from a local native result.

Stop after this bounded roster/conditional confirmation and report the actual
result, including failure to meet tenfold. If execution remains gated, retain
the tested prototype and exact blocker; do not substitute unit-test elapsed
times for measurements. No cloud launch or spending is authorized here.
