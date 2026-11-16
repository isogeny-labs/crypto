# Read-only review of the native development prototype

Reviewer: `/root/theory_review`, 2026-10-29. No new execution, solver run, performance measurement, or code edit was performed for this review.

Files inspected in full:

- `research/prime_one_shot_20261029/prototype/norm_tower.cpp` (299 lines)
- `prototype/README.md`
- `independent-check/check_export.cpp` (100 lines)
- `independent-check/run.sh`
- The updated root report's construction, conditional complexity, native-results, limitations, and requirement-status sections.

## Verdict and scope

No blocking correctness defect was found for the explicitly fixed development contract: the curve over F17, arities 3/5/7, four membership-squaring layers, known-answer selected-chart witnesses, and assignment evaluation. This is a code review, not an independent execution or a proof of complete model-set correspondence. The code and documentation correctly avoid claiming an unknown-coefficient solver, DLP algorithm, degree bound, performance improvement, or complete exceptional-chart coverage.

## Exported coefficient systems

The odd-arity norm shape is correct: m=2h-1, monic A of degree h, C of degree at most h-2, monic U0 of degree m. The target equation vanishes at -R, and the norm factor is X-x_R with the correct sign.

In the remainder encoding, each R has degree below m, each retained quotient Q_j has degree at most m-2, and

    R_(j-1)^2 = Q_j U0 + R_j

has the required coefficient capacity because the squared remainder has degree at most 2m-2. The first remainder R0=X and the final constant c are handled correctly. The quotient/remainder intermediates are unique for monic U0, so these equations implement divisibility rather than adding arbitrary quotient branches. Retaining Q_j is what keeps the coefficient equations quadratic.

The Graeffe implementation uses the correct parity sign in general; the exported arities are odd. Its E/O split and monic terminal product are consistent with root squaring. The code separately checks the transform against a root-product computation in the fixed fixtures.

The variable counts can be reproduced symbolically. Base A,C,U0 variables total 2m. The chart variables total 3m+h-1. The Graeffe intermediates add (t-1)m. Thus the reported 25/42/59 variables for m=3/5/7, t=4 are consistent. The remainder arm adds (t-1)m+t(m-1), giving 33/58/83. The reported equation counts sum to 312 across all six systems. This review did not rerun a parser to recount the actual files; the retained independent evaluator's scope is described correctly.

## Chart witnesses

The gcd(C,U0) witness uses degree bounds at least as large as extended Euclidean witnesses require. Its U0 multiplier has one spare coefficient in the generic case; the highest coefficient equations force that spare coefficient to vanish, so it does not create an unsound coprimality condition.

The gcd(U0,U0') witness bounds are standard: multiplier of U0 has degree at most m-2 and multiplier of U0' degree at most m-1. Coefficient comparison enforces a Bezout identity equal to one. The explicit inverse witness for U0(x_R) enforces target separation. Both are quadratic in exported variables. Repeated roots and zero C are rejected before export, and these chart conditions also appear in the exported system; they are not just trusted preconditions.

These checks cover the selected chart, not all decompositions. The updated report and prototype README disclose that distinction.

## Geometric versus rational semantics

The implementation's F17^2 regression agrees with the hand derivation in the earlier theory review. It verifies the nonsquare extension parameter, the anti-Frobenius points and their group relation, the rational target, interpolation of monic A and linear C, the complete norm identity, nonzero C on the residual roots, nonrational coefficients, and a violated coefficient field equation.

The exported systems do not impose coefficient field equations. Their geometric solution sets can therefore contain extension-field coefficients. The README and root report explicitly state this, and the separate `require_rational` development check is not misrepresented as a constraint present in the `.ms` files. That disclosure is essential and is present.

The dual-number regressions correctly distinguish terminal Graeffe membership from divisibility at both even and odd arity. The odd perturbation is exactly the first-order polynomial obtained from roots 1+epsilon,-1+epsilon,4, and the first-order divisibility remainder is computed with the correct negative quotient-times-perturbation expression. These tests do not establish a global degree lower bound or a performance conclusion, and the documentation does not claim either.

## Independent textual evaluator

The separate evaluator does not reuse the exporter's AST or polynomial routines. It parses and evaluates the textual exported arithmetic, verifies the modulus is a supported prime, rejects incomplete/duplicate/out-of-range assignments, rejects unknown variables in expressions, and distinguishes a nonzero equation result (exit 1) from malformed input (exit 2). The shell harness exercises a changed coefficient, an incomplete assignment, and an empty equation list.

For its declared modulus range, addition cannot overflow uint64, and multiplication uses uint128 before reduction. The compiler command does not apply `-Wpedantic`, so its deliberate uint128 extension does not conflict with the stated build flags. This is acceptable for the fixed GNU C++ build contract; it is not a claim of portable ISO-only C++ arithmetic.

Assignment evaluation cannot detect omitted equations, an incorrect model specification that still accepts the supplied witness, or missing exceptional components. The prototype README and root report correctly identify it as independent implementation within this session, not independent scientific replay or model-set completeness.

## Minor nonblocking observations

1. A source comment says “All point pairs” before a loop that checks each nonidentity point's inverse and 19-fold multiple. It does not actually test every pair or associativity. No corresponding all-pairs claim was found in the README. Correcting the comment would avoid confusion.
2. The on-curve target-overlap guard exists, but current malformed-target tests use off-curve/infinity targets rather than a target whose x matches a U0 root. A small development regression can exercise that guard explicitly if expanding the fixed checks.
3. The exporter checks opening the `.ms` stream, but does not separately check write completion for the assignment and JSON streams. Existing retained files are subsequently parsed, so this is not an observed correctness failure; it is an output-robustness opportunity.
4. The independent parser drops empty comma-separated fields rather than treating every empty field as malformed. This does not affect the retained exports. Do not broaden its claim to a full strict msolve-format validator.
5. Fixture construction enumerates bounded point/sign choices to find known answers. This is ordinary development-fixture construction and is clearly labeled; it must not later be counted as a non-generic decomposition algorithm.

## Claim hygiene in the revised report

The earlier conditional-complexity correction is now implemented: exp(o(n)) assumptions imply the qualitative sub-exponential total only, while the sharper bound explicitly requires stronger overhead bounds. Polynomially many variables are allowed in the monomial-count implication, and rationality circuits are not hidden in the original O(n) count. The primary membership comparator is now radical quotient/remainder membership, with Graeffe retained separately. The evidence limitations, actual prototype role, and unimplemented unknown-coefficient solver are clear.

The present result is therefore a concrete, checked encoding-development artifact and a focused research audit. It is not the requested novel sub-exponential ECDLP algorithm, and the report says so directly. No major claim correction beyond the minor observations above is required by this read-only review.

## Parent follow-through after this review

The inaccurate all-pairs comment was corrected; the strict rebuilt exporter retains
162 passing checks and byte-identical six exports. The separate evaluator now uses
uint64 multiplication under its explicit p<=2^31-1 bound, so reduced products fit;
it builds with -Wpedantic -Werror as well. Its six positive checks and three negative
controls still pass. The optional target-overlap and output-stream hardening items
remain nonblocking follow-ups; they are not claimed completed.
