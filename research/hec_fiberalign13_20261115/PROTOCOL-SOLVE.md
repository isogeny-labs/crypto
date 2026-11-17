# Exact inverse solve cohort after new-base alignment

Prep sourcecc24c005a260/checkpoint225944 ran ONCE, exit0/empty stderr. It
computed the first fiber base and fit rank63/matrixrank8 on first12multiples,
withheld13..16 and corruption check pass. No oldmatrix/originalD1image read.
Both original D1 and B use offset1; primary/adjacent vectors and TWO systems
were generated fresh. Generator positive/corruption controls pass for each.

This separate cohort runs the SAME exact verifier with a freshly rebuilt
private msolvev0.11.0 cc044551f8e202d00691990fd3fd0b1226a88ea2, pinned binary
hashes. Require71upstream tests pass before scientific launch. Fixed cells:
positive from generated/D1, then D1target, then Btarget; stop on firstfailure.
Two threads, genericity correction off, per-cell300second wall cap. No RSS
or peak-memory claim; the collector records exit/censored status only.

Every result must satisfy all12emitted equations by exact polynomial
substitution, squarefree eliminant/invertible denominator and unique sign
convention. Positive must recover the known control point. Target divisors
or DLP scalars are never supplied to equations or solver. Preserve each cell
stdout/stderr/res/status/verifieroutput and producer failures, no reruns or
primitive-element tuning without amendment. Source/binaries/inputs committed
and S3checkpoint/gate BEFORE cohort. Finite parametrization alone is not an
oriented class or a recovered logarithm; class/sign/descent gates remain next.
