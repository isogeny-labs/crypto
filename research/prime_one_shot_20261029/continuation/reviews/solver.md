# Independent review: actual arity-three solver and direct IC plumbing

Reviewer: `/root/theory_review`, 2026-10-30 UTC. Read the candidate, independent point oracle, direct-IC implementation, frozen development protocol, check scripts, README, build receipts and retained records under `research/prime_one_shot_20261029/continuation/solver/`.

## Verdict

No blocking correctness defect was found for the fixed F17 development contract. The continuation now does more than evaluate supplied witnesses: it enumerates unknown norm coefficients from target/domain inputs and returns complete rational selected-chart models at arity three. It also uses those models in a functioning direct mixed-target IC pipeline recovering the 18 nonzero logarithms on the fixed order-19 curve.

The implementation remains an explicit exponential coefficient census. It does not supply the proposed growing-arity implicit solver, a sub-exponential algorithm, a runtime advantage, a unified complete field-operation price, or a scientific scaling measurement. The README makes those limits explicit.

## 1. Source separation and scalar-label hygiene

`coeff::solve` receives only field/curve parameters, tower parameters and R coordinates. It does not receive Q, the mixed-target labels a,b, source summands, a norm witness or a generation scalar. Its search is over every u in Fp and every nonzero w in Fp, with v eliminated from the target equation. No decomposition-table lookup, point-sum join, rho, BSGS, or discrete-log scan appears in that solver.

The direct `Solver` API receives Q as a point and an RNG seed. The fixture loop that constructs Q=[expected]P lies outside that class. The class does not receive `expected`; the separate `--solve Qx Qy Seed` executable interface strengthens the source-level check because there is no expected-scalar argument. The internal solver only multiplies P by the recovered scalar for final verification, which is not a logarithm scan.

For every relation attempt the direct class samples a,b, forms R=[a]P+[b]Q, and passes only R plus the fixed public domain into `coeff::solve`. It chooses the first canonical model using only the returned model set. It then binds the model to the canonical signed-base row C and independently readds the recovered points to R. The labels are retained separately.

`left_null` has only C and an arithmetic-counter object as inputs. The counter is incremented but its previous value never changes arithmetic results or pivot choices. No a,b,Q or expected scalar is used to select the kernel. Recovery subsequently computes alpha=w dot a and beta=w dot b, rejects/retries beta=0, and returns -alpha/beta when defined. The returned scalar is checked by an actual group multiplication against Q.

The deterministic MT19937 development seeds do not instantiate the ideal independent-uniform randomness theorem statistically. The README explicitly discloses this. The scalar mapping uses the standard rejection threshold for uniform 64-bit input words; that mapping is unbiased under an ideal uniform-word source, but the fixed unit runs are reproducibility fixtures, not a proof of random-label independence.

## 2. Arity-three selected-chart completeness

The candidate uses A=X^2+uX+v, C=w, with

    v=y_R*w-x_R^2-u*x_R.

This is the correct condition for A+C y to vanish at -R. The norm is monic degree four; dividing by X-x_R produces a monic cubic U and the code checks that the remainder vanishes. It requires squarefree U and target separation U(x_R)!=0, and w is nonzero by construction.

The repeated-remainder condition X^(2^t)=c mod U is implemented literally. The admitted d=2^t divides p-1 and c is a nonzero d-th power, so X^d-c splits separably. Thus a squarefree cubic divisor has three distinct rational roots. Recovery y=-A(x)/w has the correct sign and is rechecked against the curve, target separation and membership. Duplicate canonical models cause a failure rather than being silently discarded.

Conversely, a selected triple with distinct finite abscissas, target-abscissa separation and sum R has a principal divisor consisting of those three points and -R, with pole divisor 4O. Its function has the displayed monic quadratic-plus-constant-times-y shape. A zero C would yield paired zeros on the same abscissas and is incompatible with the selected divisor, so the nonzero-w census covers the selected chart. The candidate visits all p(p-1) choices, not a planted preferred pair.

The CLI admits only the stated bounded field/curve domain and checks prime characteristic, nonsingularity, whole prime-order non-anomalous curve, split nonzero coset and on-curve affine target. The fixed development cases use only p=17 and order19. This review does not promote the small CLI range to a general-curve theorem or a useful security-size implementation.

## 3. Independent reference and record verification

`point_oracle.cpp` has a separate affine group-law implementation and directly enumerates signed point triples. It contains no norm or polynomial-remainder solver. The direct IC implementation reuses that reference group law for point arithmetic, so its point verification is separate from the coefficient solver, but not a third independent group library. The documentation correctly describes independence as within-session implementation, not external scientific replay.

For this review I also wrote and strictly compiled a third fixed-record checker at:

    /workspace/audit/review_solver_records.cpp

It includes none of the candidate/oracle source. It uses extended Euclid for field inverses rather than the implementations' Fermat inverses, its own affine group law, and a simple fixed-unit scalar multiplication. It recomputes the complete tiny point-triple reference sets, then verifies retained relations, kernels, scalar answers and counter identities. It does not rerun the coefficient candidate, search logarithms, collect timings, or run a scaling experiment.

Commands:

    g++ -std=c++17 -O2 -Wall -Wextra -Wpedantic -Werror \
      /workspace/audit/review_solver_records.cpp \
      -o /workspace/audit/review_solver_records
    /workspace/audit/review_solver_records \
      /workspace/crypto/research/prime_one_shot_20261029/continuation/solver/checks

Compiler: g++ (Debian 14.2.0-19) 14.2.0. The final fixed-record verification exited zero:

    PASS fixed model census: cases=90 models=400 SAT=42 UNSAT=48
    PASS mixed-target relations=72 kernels=18 low_rank=12 internal_DLP=18 CLI_DLP=18
    PASS retained counters: attempts=230 identity=13 PDP_misses=145 PDP_calls=217 pairs=59024 accepted=72

The first strict compilation of this review-only checker stopped on a misleading-indentation warning in its own point-enumeration line. Splitting that line fixed the formatting without weakening compiler flags. No candidate solver failure or timing rerun is implied by that reviewer-code correction.

Artifacts are `/workspace/audit/review_solver_records.log` and `.sha256`, with hashes for this checker, binary, log, reviewed candidate sources and all recounted record files. This is independent fixed-unit arithmetic verification, not a formal certificate of the algorithms' general correctness.

## 4. Direct reduction and matrix logic

The transposed Gaussian elimination in `left_null` correctly computes a nonzero left-null vector for the (B+1)-by-B coefficient matrix. It verifies wC=0 before returning. The chosen free coordinate is set to one, so w is nonzero. The rank0/1/2/3 unit controls exercise the rank-free path. No full-column-rank gate is imposed.

The retained 72 relation rows satisfy both:

    R=[a]P+[b]Q,
    R=sum_j C_j F_j,

with canonical base points (9,1),(13,7),(16,4) and coefficients 1 or -1 mod19. All 18 retained kernels satisfy wC=0, the recorded ranks, and the recorded alpha,beta values. Recovered scalars satisfy [k]P=Q. Twelve of the 18 final matrices have rank below the three-column base dimension, demonstrating the intended reduction does not depend on full factor-base rank on these fixtures.

The chosen domain has only three canonical sign orbits (six signed points); four accepted rows suffice for a left dependency. This B=3 convention is clearly labeled and must remain distinct from the signed-point B used in the coverage theorem's counting formulas.

No natural zero-beta batch occurred in the retained seed panels. The synthetic denominator-zero control does reject it. Therefore natural retry behavior is implemented and unit-checked algebraically, but not observed in the retained full pipelines. The README states this rather than claiming empirical retry coverage.

## 5. Cold work and failure accounting

Each direct solve constructs its base afresh. The retained setup counters show 289 curve-pair admission checks and nine canonical membership tests per solve. No cross-target factor-base logs or decomposition setup are retained by `Solver`.

All 230 attempts partition into 13 identity misses, 145 unsuccessful nonidentity PDP calls and 72 accepted rows. All 217 nonidentity calls, including misses, contribute their full 272 coefficient pairs, totaling 59,024. The internal consistency check enforces these equalities before successful return, and the review checker recomputed them from retained records. Group-addition counters are divided by phase; matrix/recovery reductions are explicit diagnostics. They are not complete interchangeable field-operation units, and the report does not claim they are.

The development attempt/batch limits fail closed by throwing and returning an error rather than counting a cap hit as a successful or completed relation solve. No retained target hit a limit. A future cap failure would currently report an error without exporting a complete partial trace from inside the unfinished `Solver`; improving failed-run trace retention would be sensible before a scientific campaign. That is a future robustness gap, not an observed failure in these passing fixed cases.

## 6. Complexity qualification

The p(p-1) coefficient census is explicitly exponential in log p. The implemented root extraction also scans Fp per accepted polynomial. Neither mechanism is the proposed sub-exponential breakthrough.

One small asymptotic wording refinement was sent to the parent: for the literal code, each remainder `divrem` computes the inverse of its leading coefficient through Fermat exponentiation, even when that coefficient is one. If t is treated as a growing parameter, a safe field-operation upper bound is

    O(p^2*(t+1)*log p + p*M),

plus output/bit costs, rather than O(p^2*(t+log p)+p*M) unless the monic inversions are optimized away or t is held within the fixed cap. This does not change the exponential classification or any measured claim; there is no performance claim here.

## Final assessment

The continuation supplies actual unknown-coefficient selected-chart solving and complete fixed-fixture direct IC recovery, with independently recounted outputs and proper label isolation. Its development evidence is materially stronger than supplied-witness evaluation alone. It still establishes neither growing-arity completeness nor cheap implicit solving, no comparative speed result, and no sub-exponential ECDLP algorithm. The current documentation preserves those limits.
