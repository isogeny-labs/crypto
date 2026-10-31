# Hess c-parameter audit for overlooked GF(8) source orders

Frozen 26 October 2026 before the first calculator execution. The
question is whether the project’s direct-B order closure missed a low-genus
instance of Hess’s *published* generalized Artin-Schreier GHS descent.
Hess (2004), Sections 5-6, allows a nonzero parameter `c` in the model
`y^2+y=c/x+a2+(sqrt(1/j(E))/c)x`. The Magma `WeilDescentGenus(E,k,c)`
intrinsic computes the resulting genus; `WeilDescent(E,k,c)` constructs
the cover and map. Nonhyperelliptic outputs are explicitly within that
prior work, so finding one is not a new attack method.

Use Conway `K=GF(512)`, `k=GF(8)` embedded in `K`, and exact normalized
ordinary source curves `v^2+uv=u^3+a2*u^2+b`. Representatives of the
five outside-direct-B classes from the frozen quartic panel are:

| Source order | a2 | Conway integer b | Largest prime subgroup |
| ---: | ---: | ---: | ---: |
| 482 | 1 | 43 | 241 |
| 524 | 0 | 9 | 131 |
| 548 | 0 | 155 | 137 |
| 554 | 1 | 105 | 277 |
| 556 | 0 | 32 | 139 |

First run a deterministic pilot of `c=w^j` for `0<=j<16` on each source,
retaining every `(order,b,j,genus)` or error. If all calls finish within
the public Magma calculator window, extend to all `511` nonzero `c`
values for each representative; record interruptions without filling
unobserved rows. Every positive low-genus result must be reconstructed
with `WeilDescent` and have a checked exact source-to-divisor map before
being called a transfer. A speed claim additionally requires complete
same-target attack cost below rho. Genus alone is a screening result.

Prior literature and implementation:

- Hess, [Generalising the GHS Attack on the Elliptic Curve Discrete
  Logarithm Problem](https://www.cambridge.org/core/services/aop-cambridge-core/content/view/315278965D8D277A85812D9498236A88/S146115700000108Xa.pdf/generalising-the-ghs-attack-on-the-elliptic-curve-discrete-logarithm-problem.pdf),
  2004, especially Sections 5-6 and kernel discussion.
- [Magma Weil Descent in Characteristic Two](https://www.math.uzh.ch/sepp/magma-2.19.8-cr/html/text1383.htm),
  including `c`, `HyperellipticImage`, and `WeilDescentGenus`.
- Nart and Ritzenthaler,
  [Genus 3 curves with many involutions](https://arxiv.org/pdf/0905.0546),
  for the previously evaluated quartic quotient family.

This protocol bounds only the named five representatives and evaluated
`c` values. Isogeny-class coverage, subgroup preservation, and DLP cost
are separate gates.

## Population criterion added before the population computation

Hess Theorem 12 gives `g=2^m-2^(m-d_gamma)-2^(m-d_beta)+1` for
`f=gamma/x+alpha+beta*x`, where `m` is the degree of the least common
multiple of the Frobenius annihilators of `gamma`, `beta`, and, when
`Tr_(K/F2)(alpha)=1`, `t+1`. For the cubic extension in characteristic
two, an element with nonzero relative trace and nontrivial Frobenius
orbit can have annihilator degree three; a nonzero relative-trace-zero
element outside `k` has annihilator `t^2+t+1` of degree two.

For `a2=0`, the possible new genus-three case with both factors of
degree two requires nonzero `gamma,beta` of relative trace zero with
`gamma*beta=sqrt(b)`. The project’s direct-B family has one factor in
`k` and the other trace zero. Compute the product criterion for **all
1,022** frozen normalized ordinary source models, retaining the exact
`(a2,b,gamma,beta)` identities and group-order partition. Check any
candidate with Magma `WeilDescentGenus` before naming a cover. This
finite screen is not a new theorem or a speed claim; Hess’s construction
and criterion are published.
