# Preserve the narrow quartic constructor while excluding reducible candidates

Frozen 2026-10-25 before the first narrow-only profile or source edit. This
is an additional engineering experiment; the existing supported m14/15
queue keeps its original frozen binary and parameters.

Baseline: frozen `ic_quartic_core_probe-before`, SHA-256
81ab248653df3b5dbe3e9283becaba8e6a456432a799434f5289479248abcb06,
source 335d168aac85267e55b597d92e88efd92eae55b5. Its narrow library module
is byte-identical to the current clean source at
4d0bfe24276aac6ffc4b9c56afcd70429cd8e617. Retain the executable unchanged.

Profile `narrow 6` with tools/profile.sh cpu --mode callgrind on the recorded
shared AMD EPYC 9V74 Linux x86-64 host, one thread. Preserve the complete
profile, host/tool manifest, raw simulated Ir and stdout. Native constructor
controls use `both M` for M=4,5,6,7,8,9,10 with 2 GiB and 300 seconds per
case; retain any resource failure. The outputs assert field, coefficient
and 4,096-product parity against the already checked wide backend.

Then port only the existing wide exclusion to the narrow constructor:
skip c3=0 for even m, and c3=c1=0 for every m. The readable argument and
its partially formalized dependency map are in CONSTRUCTOR_EXCLUSION.md.
Keep the rank test, ordering of all remaining candidates, random seeds,
returned fields, model identities and reset arithmetic counters unchanged.
Add narrow regression controls for fixed returned fields and the odd-m
search case. Freeze the candidate binary before running it. Repeat exactly
the narrow6 profile and seven native controls; compare complete stdout.

The decisive engineering acceptance is identical supported output with the
excluded-family proof and meaningful regression controls, followed by the
full release library gate and affected example tests. Any instruction
difference is simulated whole-process attribution at L0. It is not native
time, a fastest-external-tool comparison, an end-to-end IC improvement or a
rho crossover. A/A noise, interleaved native A/B, complete price and external
performance qualification remain separate obligations.

No paid VM, artifact upload, publication or claim promotion is authorized by
the runner. Preserve timeouts and the original live quartet of protocols.
