# Finite-precision Smart known-answer run

This is an exact computation on three fixed public inputs. The canonical
`protocol.json` must be validated, frozen and committed before the registered
calls. Small-control preflights are in-sample calibration; the 160-bit target
has not been executed when this design is written. [pcc:unverified]

The binary reads only p,a,b,Gx,Gy,Qx,Qy. It uses four fixed coefficient lifts,
`a+i*p,b+(2*i+1)*p` for i=0,1,2,3, Hensel square lifting modulo p² and
division-free Jacobian polynomial doubling/addition. Every lift is retained.
For p times each lifted point it computes the parameter `-X*Z/Y` modulo p²,
extracts its coefficient of p, and divides the Q coefficient by the nonzero
G coefficient modulo p. A zero G coefficient is recorded as degenerate,
with no division or hidden retry. [pcc:unverified]

The ordinary binary contains no expected scalar. It checks the initial field
equations, each Jacobian equation and primitive tuple, the Hensel equations,
and the final parameter's divisibility by p. Its check code is a producer
check, not independent arithmetic verification. Complete tuples, operation
prefixes, inverses and lifting witnesses are retained for Lean.
[pcc:unverified]

| Arm | Fixed input | Required output |
|:--|:--|:--|
| small | F5, y²=x³+3x+2, G=(1,1), Q=(2,1) | At least one nondegenerate lift; every nondegenerate result is 2 |
| offcurve | Same small input except Q=(2,2) | Exit 2 and explicit off-curve rejection |
| large | The corrected 160-bit curve and fresh certified G,Q in the parent study | At least one nondegenerate lift; every nondegenerate result equals the certified known scalar |

The positive and negative controls have actual Lean point/order/equation
proofs. The large point order and known answer are bound by the already
issued parent packet `PCC-20261110-90623b`. This does not assume the entire
curve point count equals p. [pcc:unverified]

Run exactly one process per arm, in the fixed order small, offcurve, large,
with a 15-second watchdog each, one thread, no warmups and no outcome-based
retry. Global watchdog budget is 45 seconds, zero paid spend. No accepted
timing metric or statistical inference. The four lifts are part of one fixed
algorithm, not four opportunities to hide failed outputs. All errors and
degenerate outcomes are retained. [pcc:unverified]

The frozen JavaScript analyzer checks known answers, statuses, trace-file
presence and the operation counts derived from the public scalar's bits:
6 steps per small lift, 420 per large lift across G and Q. It does not prove
the arithmetic or certify execution. A subsequent Lean certificate must
construct the concrete trace against `SmartArithmetic.lean`, check lifting
and inverse witnesses, and tie recovery to the actual point relation.
[pcc:unverified]

No A/A noise estimate is applicable to these exact known answers. Controls
are unblinded. Preflight clocks serve resource admission only; the large arm
is unmeasured before freeze. Security cost, generic p-adic/logarithm validity,
compiler refinement, other-machine independence, a matched rho comparison
and the SafeCurves constant are outside this run's decision. [pcc:unverified]

Sources: Smart's original HPL-97-128 report; Leprévost et al.'s original
preprint and explicit author erratum; the EFD `add-1998-cmo` and
`dbl-1998-cmo` formulas; actual Mathlib point proofs and ZMod inverse
uniqueness. Primary-source hashes/read scopes remain in the parent source
plan; EFD retrieval is recorded in this run's provenance. The corrected
paper and screened NIST recommendation are not refuted. [pcc:unverified]
