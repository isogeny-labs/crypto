# Prime-degree transfer floor and exact n=7 trace screen

Frozen before execution, 2026-11-15 UTC. This is a necessary-condition screen, not an attack benchmark.

## Sources and prior-work boundary

- [Diem, *Weil-restrictions of abelian varieties*, Theorem 3](https://www.esaga.uni-due.de/f/ulrich.goertz/iem-archiv/dissertation_diem.pdf): the Weil restriction of a new ordinary elliptic curve over a finite extension of degree not divisible by four is simple. The genus floor below is a direct consequence.
- [Hess, *Generalising the GHS attack*, Theorems 14–15 and Section 8.3](https://www.cambridge.org/core/services/aop-cambridge-core/content/view/315278965D8D277A85812D9498236A88/S146115700000108Xa.pdf/generalising-the-ghs-attack-on-the-elliptic-curve-discrete-logarithm-problem.pdf): transfer kernel, Frobenius polynomial, and zero norm through intermediate fields of the x-line.
- [Menezes–Teske, *Cryptographic implications of Hess' generalized GHS attack*, Section 5.2](https://cacr.uwaterloo.ca/techreports/2004/corr2004-25.pdf): published n=7 W1 family and classical genus-seven slice.
- [Achter–Casalaina-Martin, *Putting the P Back in Prym*, Theorem A](https://www.math.colostate.edu/~achter/math/prym1224.pdf): characteristic-two Prym polarization for a degree-two cover with ramification divisor of degree two.
- Internal KB `LEAD-TRF-weil-restriction-jacobian-class` and `IDEA-20261024-a29e53` already propose genus-n Jacobians in this isogeny class. A matching trace class is not a new attack method.

## Mathematical claim under review

For k=F_q, K=F_(q^7), and an ordinary E/K with trace t, let A=Res_(K/k)E. Its Frobenius polynomial is `X^14-tX^7+q^7`. If t is not `u^7-7qu^5+14q^2u^3-7q^3u` for an integer u with `|u|<=2sqrt(q)`, A is simple. Any nonzero algebraic homomorphism A to J_C over k then forces genus(C)>=7. This follows from Diem; the trace test is a direct prime-degree specialization. For q=2 and ordinary E, only t=+13 or -13 can be exceptional. This is **not** a bound on all set-theoretic homomorphisms of finite groups.

For a hypothetical genus-seven curve Y with `J_Y` isogenous to A, its L-polynomial is `1-tT^7+128T^14`; hence `#Y(F_(2^m))=2^m+1` for 1<=m<7, and `#Y(F_128)=129-7t`. The count of degree-seven places is `(126-7t)/7=18-t`, so t<=18 is necessary. This does not establish Y's existence.

## Frozen census

- Field: `F_128=F_2[z]/(z^7+z+1)`, bit encoding, modulus `0x83`.
- Both irreducible cubic arms `p1=T^3+T+1` and `p2=T^3+T^2+1`; `Vp=ker p(sigma)` with `sigma(x)=x^2`.
- W1 coefficients: `b=(r(v+1))^2` with nonzero r,v in Vp; union by b. Expected 42 coefficients per arm and 56 in union from the previous independent membership census. The classical set is nonzero b in either Vp, expected 14.
- For each b and both binary twists a=0,1, count `#E_a,b(F_128)` by the character equation for x!=0; calculate t=129-#E, `18-t`, and mark the ±13 exceptional traces. Report a CSV row for each of 112 exact source curves, and aggregate by trace and classical/nonclassical membership.
- Positive controls: the coefficient and classical set counts above; `#E` even and in Hasse interval `[107,151]`; odd t; identical trace for the same b may occur but is not assumed.
- Success for this screen: all 112 rows, count checks, and a transparent trace-class table. Stop on any failed assertion. A nonclassical coefficient with a trace class absent from the classical slice is a candidate *isogeny class for further geometry*, not a vulnerability.
- No timing claim is made. The next decision requires a genus-seven curve model and explicit algebraic map on a chosen subgroup, then full GHS-vs-rho costs on the same subgroup.

Raw output: `reports/run-20261115T*/curves.csv` and `summary.txt`; source hash, command, toolchain, and checkpoint receipt in adjacent `RUN.md`.
