# Many positive targets have sub-exponential quotient degree

2026-10-30 UTC. This is a consequence of the reviewed second-moment and
coverage bounds. The [independent check](../reviews/small-degree-targets.md)
also verifies the tuple-to-scheme-point bijection. It is not a bound on
intermediate elimination, solving degree or the cost of obtaining a basis.

On a simultaneous good auxiliary base write

    nu_g=(1/r) sum_R V_g(R)^2 <=4K,
    K<=5m4^m kappa^2,
    kappa=d^m/(m!r).

For the least power of two d>=2(m!r)^(1/m), one has kappa<4^m. Hence
nu_g<20m64^m. Selected-chart coverage is at least
rho0=1/(320m16^m), with 0<=V'_g(R)<=V_g(R).

Put L=128m32^m. Markov on the squared tuple count gives

    Pr_R[V'_g(R)>L]
      <=nu_g/L^2
      <20/(16384m16^m)
      =(25/64)rho0.

Subtracting this tail from coverage leaves

    Pr_R[1<=V'_g(R)<=L] >=(39/64)rho0>rho0/2.

No pointwise representation-uniformity assumption is used. O and other
uncovered targets remain in the uniform-target denominator r with count0.

On the rationality-good event, every geometric selected norm solution is
rational. The monic norm function is uniquely determined by the signed
unordered tuple: its zero divisor is sum(P_i)+(-R), and its pole divisor
is(m+1)O. For m=2h-1, even leading pole order gives A monic of degree h
and deg C<=h-2. Conversely roots of U recover the signed residuals through
y=-A/C. There is no extra factorial or global-sign multiplicity. The
selected radical scheme is finite etale, and the declared bounded quotient
and inverse witnesses are unique. Its geometric quotient degree therefore
equals V'_g(R).

At least rho0/2 of all targets on each good base consequently have positive
quotient degree at most L. For m=O(sqrt(n)), log2 L=5m+O(log m), so
this is sub-exponential. Polynomial-in-degree FGLM and finite-field root
extraction would also be sub-exponential on these targets after obtaining
the initial basis or multiplication representation. The library's tiny-field
separating-linear-form issue must still be handled correctly.

The result removes large final output dimension as a necessary obstruction
on a sufficiently large set of positive targets. It does not identify those
targets in advance, construct their initial basis, or establish cheap
verified relation yield. A proof of low final degree is not a proof of low
solving degree. The cost of the initial implicit elimination remains open.
