# Cold IC with discovered chart: verified integration

The separately frozen correctness execution passed all stages on its first scientific run. Source freeze c10ec95e2917, checkpoint 20261116T002448Z. The actual input is PUBLIC.txt; runtime setup, point logs, aligned coordinates and inverse results were constructed in run/. All private solver dependencies passed hashes. No previous chart or anchor table was used by candidate section construction.

The runtime quartic selects O=(0,4), tangent y=29x+4, irreducible residual x²+34x+69, and canonical line X=0 with points (0,4),(0,27),(0,74),(0,98). This differs from the earlier configured origin (18,52). The chart is rediscovered deterministically by each consuming process from mandatory runtime setup.

## Validation

- Fresh source recurrence/Weil recovery, kernels, normalization and quotient construction passed; expected missing-setup control exits101 at the mandatory loader.
- Degree18 calibration fiber passed. Alignment rank63/matrix rank8, twelve training and four withheld pairs passed; corrupted matrix rejected.
- Sections pass exact coefficient identity, multiplication rank18, diagonal rank112, 480 determinant and 480 split tensor controls, plus120 irreducible quadratic base-divisor cancellation controls.
- Positive, D1 and B parametrizations each have dimension0/degree12 and satisfy all12equations exactly. Both actual inputs have one recovered class with two divisor representations; corrupted public adjacency rejects.
- Fresh point relations and point logs pass their residual checks. Four D1 conic descents and two B descents agree separately and pass independent divisor identities with the new canonical divisor and origin.
- k=183049 passes source and destination public scalar multiplication. k+1 fails the source control.
- All scientific positive stages exit0; all exclusions and outputs retained. No scientific rerun, dropped input or unexpected scientific failure. Two module-dependency build failures are retained as BUILD-PRODUCER-1/2.txt; preflight path corrections preceded execution.

## Mathematical change

Write Kc=K−4O and Dc=Dinf−3O. Define 2beta=2Dc−Kc. For a centered effective divisor E representing X−beta relative to Dinf, and a conic with E+A+B+R~2K, one obtains

    log X = (3/2)log Kc − log(A−O) − log(B−O) − sum log(Ri−O).

The old tangent-residual two-point constant is replaced by the four-point canonical constant. Alignment, inverse sections, recovered-class centering and conic verification use this same choice. This is a conditional divisor-class identity, checked here with exact arithmetic; complete geometry is not formalized in Lean. The construction follows the preceding registered chart gate and Tian2020 section4.2.3.

## Scope

Engineering integration at F127 on one source/quotient instance, not a mathematical novelty or timing result. Target chart discovery is now integrated in the full executed path. Source map/kernel code retains its registered model restrictions; calibration construction remains specific to this family. Bounded rational chart search, field portability, scalable final linear algebra, genus2/4 and the mathematical tenfold objective remain open. The old failed cold performance comparison is unchanged. Correctness completion does not establish an end-to-end speedup.
