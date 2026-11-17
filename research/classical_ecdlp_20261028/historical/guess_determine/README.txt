Guess-and-determine ECDLP experiment — classical only
Date: 2026-10-27

Algorithm: Ayan Mahalanobis, "A Guess and Determine Attack on the Elliptic
Curve Discrete Logarithm Problem", arXiv:2607.09814v2, 6 August 2026.
Primary source: https://arxiv.org/html/2607.09814v2

IMPLEMENTATION

This is an independent implementation of the published algorithm, not the
author's source or a reproduction of the author's timing tables. It builds
the homogeneous evaluation matrix from random known multiples of P and -Q,
computes its left kernel, makes a half-column guess, and uses normalized
small-nullspace collisions to find a zero minor. Explicit column maps lift
the result to a group point-sum relation, whose known labels recover the log.

Only public instance data (curve, P, Q, prime order) reach the attack routine.
Fixture generation knows the scalar only to check the final answer. There
is no hidden exhaustive-log fallback. All 60 successes used the signature-
collision path. Full normalized vectors are dictionary keys, so accidental
hash collisions cannot produce a false mathematical collision. Kernels in
dimensions two and three use exact elementary formulas. Rank exceptions
are counted and skipped. Input zero entries are not exploited as shortcuts.

RESULTS

Ten independently seeded instances on each of six curves: 60/60 solved.
Every successful result passes zero-minor, point-sum, and w*P=Q checks.

Field prime   Group order   Guess-determine ms    Pollard-rho ms
251           271          1.874                 0.063
1021          991          7.495                 0.150
4093          4049         14.476                0.261
16381         16567        47.843                0.592
65521         65761        71.442                1.005
1048573       1048193      1115.716              4.146

Numbers are median runtimes across ten trials. These are tiny reference
experiments in Python, not evidence about cryptographic-size keys or an
optimized implementation comparison. Timings include random point sampling,
matrix construction, failed guesses and verification. They exclude fixture
generation. The simple rho baseline includes its random-start setup and
verification, uses a three-part walk and Floyd cycle detection, and runs on
the same public instances. Degree/defect choices are included in results.json.

All curves have prime group order and are non-anomalous. Orders and one
complete successful witness from each curve were independently verified
using Magma 2.30-1. The independent checker rebuilds the evaluation matrix,
checks its rank, K*M=0 and rank(K), the selected determinant, the elliptic
point sum, and the recovered logarithm. All six checks passed. The saved
Magma response has success=true; printed PASS strings alone are not accepted.

REPRODUCE

Python 3.9+; no external packages required:
    python guess_determine.py --trials 10 --output results.json
    python probability_audit.py

The main run limits each method to 15 seconds per instance. Guess-determine
allows up to four point-set matrices and 32 outer guesses per matrix.
No timeout or search limit was reached in the recorded run.

Optional independent CAS verification (network access required):
    python verify_magma.py

This submits public mathematical witness data to the public calculator
endpoint linked at https://magma-maths.org/calc/. The generated self-contained
verify_witnesses.magma file can instead be run with a local Magma installation.

PROBABILITY DIAGNOSTIC

The separate probability_audit.py script is not used by the ECDLP attack.
It enumerates the candidate index sets for one outer guess. Let h=ell/2.
Choosing b of size h-d in the dense half and extending by d columns produces
    N = binomial(h,d) * binomial(h+d,d)
occurrences. Many occurrences describe the same final h-column set. The
number of distinct sets is exactly
    U = sum(binomial(h,j)^2, j=0..d).
To see this, let j be the number chosen from the sparse half. Then h-j must
come from the dense half, and existence of b requires j <= d. There are
binomial(h,j)^2 possibilities for each j. A set with j sparse columns is
counted binomial(h-j,d-j) times among the N occurrences.

For h=6,d=3, there are 1680 occurrences but only 662 distinct sets. In 3000
independent draws of globally distinct nonzero group exponents modulo 991,
the candidate family produced a zero sum in 48.27% of draws. Applying the
paper's independence expression to all occurrences predicts 81.57%.
For h=6,d=2, the observed rate was 24.83%, versus a model prediction of 34.50%.

This is a small scalar-model calibration, not another matrix benchmark or
a general formula for the true success probability. Even distinct candidate
sets can have dependent zero-sum events. The exact counting result explains
why the work-count product cannot be interpreted as a count of independent
chances of success without further justification.

FILES

guess_determine.py: complete attack and rho baseline.
results.json / run.log: all 60 instances, parameters, timings, counters,
and six saved complete witnesses.
verify_magma.py / verify_witnesses.magma / witness-magma.json: independent
Magma verification and recorded successful output.
fixture-magma.json: Magma generation record for the two larger fixtures.
probability_audit.py / probability-audit.json: separate counting diagnostic.

The workspace briefly disconnected after the benchmarks. It recovered with
all files intact; independent Magma verification completed after recovery.
