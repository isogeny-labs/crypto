# Explicit corrected-span representation over F479

2026-11-03. Fixed exact identity certificate and reusable reference factor.

## Constructed identity

The preceding feature-fiber-repair certificate identified two old signature
fibers (25,173) and (292,440), and the target correction405*chi(X+15).
This component constructs the previously missing coefficients. The published
program has476 nonzero terms using only offsets0..14 in degree at most3,
plus the single term405*chi(X+15). It satisfies

    X = sum_{S subset {0,...,14}, |S|<=3} c_S product_{d in S} chi(X+d)
        + 405*chi(X+15)

at every one of the479 field elements. `certificate/program.json` contains
all coefficients, with canonical residues and no duplicate labels. The
independent checker rebuilds characters from quadratic residues, imports no
solver code, and evaluates all terms at all points. Thus this explicit identity
no longer depends on assuming the historical rank477 computation. No full
independent rank replay is claimed; the factor's observed rank is a diagnostic.
The coefficient field remains F479 for the retained elliptic source over F509.

## Consistent rectangular solve and exact row omission

Remove only the two already-certified duplicate-signature rows173 and440.
The477 retained representative rows and576 old cubic features form a dense
rectangular matrix A. Evaluate the corrected RHS X-405*chi(X+15) on these rows.
Ordinary exact row elimination produces a factor with477 pivots. Its saved
row swaps, pivot inverses, lower elimination multipliers and upper rows permit
subsequent RHS solves without repeating matrix elimination. Free column
coefficients are set to zero, then pivot coefficients are recovered backwards.

This is a reference rectangular backend. It uses ordinary Gaussian elimination,
not a new general linear-system algorithm. It does not assume that a weighted
Gram has the same image as the rectangular feature matrix. It stores and
processes dense coefficient-field data; it does not claim a compressed or
sublinear construction. No elliptic-point search or unknown scalar labels enter.

Deleting rows alone is not sound for X. The old signature is constant within
each pair, so every old-feature program assigns the same value to the two pair
members. The correction makes the RHS constant there as well. Independently
checking the final polynomial at all479 points is the implementation gate that
ensures no omitted row escaped validation.

## Fixed exact checks and controls

The one bounded construction invocation completed with exit0. It performed:

- Independent full-field verification of the repaired program on479 points.
- A negative control solving uncorrected X on the477 representatives: it fits
  every retained row but fails exactly at173 and440 globally, as required.
- Reuse of the same factor for constant-one and singleton-character RHSs,
  each independently evaluated at all479 points.
- Rejection of an altered old coefficient and an altered new coefficient.
- Small exact checks for a consistent singular system, inconsistent singular
  RHS, a required row swap, and a malformed RHS dimension.

There were four RHS solves using the fixture factor: corrected target,
uncorrected negative control, constant and singleton. The stored factor is
reusable in memory; serialization is preserved by hash, but a reloaded-factor
solve has not yet been separately tested. Returned solutions are verified
through polynomial evaluation, not through the factor's own arithmetic.

The coefficient program is a public function identity; no unknown-scalar
recovery targets were executed in this round. Integration with the original
point-feature evaluator and decoder remains a separate next check.

## Resource and cost scope

Sources and the exact scope were committed at c311b8a before invocation.
One existing-host CPU,512MiB address-space ceiling,30second deadline,8MiB
aggregate artifact ceiling and one attempt were declared. The retained outputs
occupy1,243,148 bytes; the receipt records their limits, source and exit status.
No cloud job, build, package installation or capacity change occurred.
No timing, throughput or complete compiler-cost comparison is reported.

NumPy was absent in both the report virtualenv and system Python, so the
implementation uses Python arbitrary-precision integer arithmetic. The old
rank-check source was reviewed as the reference elimination pattern; the new
module retains transformations and accepts rectangular consistent systems.
It uses no floating-point inverse or solve.

For n representative rows, m old columns and rank t, worst-case field work
for the reference factorization is O(n*m*t), and for one new RHS is
O(n*t + m*t), with t inversions at factor construction. This is a derived
loop bound, not a measured operation count. The original feature generation
and final full-field checking remain additional work. Upper rows hold t*m
field entries; elimination factors hold sum_{i=0}^{t-1}(n-i-1) entries, plus
inverses, row/pivot indices and temporaries. Persisted JSON bytes are recorded
separately from Python heap use. This exact reference construction provides
coefficients and a correctness baseline for later structural improvements.

## Artifacts and replay

- `certificate/program.json`: explicit repaired polynomial, SHA256 recorded
  in certificate/summary.json and construction receipt chain.
- `certificate/factor.json`: representative rows, labels, pivots, upper rows,
  swaps/inverses and lower multipliers, for the exact reference factor.
- `verify_program.py`: bounded independent square-set full-field checker.
- `check_certificate.py`: read-only replay entrypoint; run
  `python3 -B feature-consistent-repair/check_certificate.py` after hydrating
  any artifact-managed program file.
- `construction.log`, `construction-receipt.json`, `check-lock.json`:
  one-attempt custody, exact inputs and source identities.

The checker explicitly scopes this certificate to prime479, exactly one new
shift term with coefficient405, and degree at most3. It rejects out-of-range,
duplicate, noncanonical or altered terms. Its reference implementation bounds
primality/square-set evaluation to primes<=10000; it is not a large-field oracle.

## Sources and next integration

Sources: preserved feature-span-dual/check.py (exact modular row elimination),
feature-fiber-repair/DERIVATION.md and its repair certificate, and the quadratic
character definition in [NIST DLMF27.9](https://dlmf.nist.gov/27.9).
A preliminary NumPy datatype lookup was not used by the final backend.
The construction makes no novelty claim for Gaussian elimination.

Next adapt the program to the existing public point-feature evaluator and
independently verify the resulting target extraction on the original fixtures.
Keep all original cohorts and controls. Before claiming any compiler or online
cost gain, admit a frozen comparison that includes factor construction, reuse,
character evaluation, all failed attempts and final verification. Separate
cold setup from subsequent RHS solves. Do not infer a speedup from fewer
new-shift terms or successful exact checks.
