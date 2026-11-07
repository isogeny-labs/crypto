# Exact native value-only software checks

Class: software correctness and algebraic derivation, not a comparative run.
One standalone rustc build (120second cap), then one invocation (30second cap)
filtered to value_tests::. The preserved jet module is an unchanged reference;
its historical test methods are compiled but filtered out, not rerun.

Three selected methods: nine original saved division values and repeated-cache
requests; comparison with the checked jet engine at the three original public
P and -P, indices0..63 plus r,(r+1)/2,u64::MAX, with independent antipodal sign
parity; domain rejection and torsion zero. No hidden scalar labels enter engines.
No fresh elliptic sign source, recovery cohort, sampling or timing measurement.
Primality remains a caller precondition and p<=u64::MAX/2. Source hashes and
reference/fixture hashes are committed before the invocation. Test clock text
is diagnostic, not evidence of performance. No experiment.protocol/v1 claim.
