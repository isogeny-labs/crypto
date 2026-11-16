# A birthday-block gap test for four-sum energy

Status: proposed application of standard collision-testing facts. Exact
integer calibration and native development controls are separate evidence.
The probability argument is not yet independently reviewed or formalized.
It does not prove a decomposition-cost bound or a changed attack exponent.
[pcc:unverified]

## Objects and hypothesis

Fix one admissible torus base. Let Omega be its signed ordered four-tuples
of distinct raw indices, |Omega|=W0, with the same meaning as the frozen
energy draft. Let F:Omega->G send a tuple to its actual cofactor-cleared
signed sum in the prime subgroup. Uniform independent tuple samples induce
the target law p(Q)=r(Q)/W0. Zero sums are included. Put

    q=sum_Q p(Q)^2=C/W0^2,   t=sum_Q p(Q)^3.

The tester must compare exact full group elements, including sign and the
identity. A coordinate identifying Q and -Q changes q. Hash-only equality,
leaked generating tuples, correlated samples or reused blocks require a
different proof; the proposed guarantee does not certify its sampling law.
[pcc:unverified]

## One block

For n independent target samples let Z count equal unordered pairs, and
let H=1[Z>0]. Standard collision-count moments give

    mu=E[Z]=n(n-1)q/2,
    Var(Z)=n(n-1)(q-q^2)/2 + n(n-1)(n-2)(t-q^2).

Primary source read: Diakonikolas, Gouleakis, Peebles and Price,
[Collision-based Testers are Optimal for Uniformity and Closeness](https://arxiv.org/abs/1611.03579),
section 2.1, Lemmas 2–3 and the exact variance calculation on page 5.
This testing framework is established prior work. No novelty is claimed for
the moments, birthday method or binomial amplification. [pcc:unverified]

Because each p(Q)<=sqrt(q), t<=q^(3/2). Markov gives P(H=1)<=mu.
Cauchy–Schwarz on the support Z>0 gives

    P(H=1) >= mu^2/E[Z^2] = 1/(1+Var(Z)/mu^2),

when mu>0. (This is the zero-threshold second-moment inequality.)
If q>=1/[4(n-1)^2], then

    Var(Z)/mu^2
      <= 2/[n(n-1)q] + 4(n-2)/[n(n-1)sqrt(q)]
      <= 8(n-1)/n + 8(n-2)/n <= 16.

Thus P(H=1)>=1/17. If instead q<=1/(16n^2), then P(H=1)<=1/32.
No uniformity, maximum-atom or flatness assumption is used beyond the
declared independent draws. The heavy-atom case is covered by the t bound.
[pcc:unverified]

## Energy thresholds and finite calibration

Choose n=floor(sqrt(W0)/40). The native planner checks the exact inequalities

    n>=2,   1600 n^2<=W0,   W0<=2304(n-1)^2.

They hold for the actual instance; W0>=230400 is the declared generic planner
domain. Therefore C<=100W0 implies block collision probability <=1/32,
while C>=576W0 implies block collision probability >=1/17. There is no
classification promise in the gap 100W0<C<576W0. [pcc:unverified]

Use m=8192 independent blocks and accept if at most floor(m/24)=341 blocks
have a collision. Monotonicity of the binomial law reduces the two error
bounds to exact endpoint tails:

    false reject <= P[Bin(8192,1/32)>341],
    false accept <= P[Bin(8192,1/17)<=341].

The native calibration computes both rational numbers with integer binomial
weights, checks every recurrence division and normalization, and requires
each numerator*2^20<=denominator. These finite calculations do not establish
the probability law of any supplied sample stream. [pcc:unverified]

For a fixed bad base C>=576W0, acceptance is an event of probability at most
2^-20 under the hypotheses. This is a frequentist test, not a posterior
probability that an accepted base is good. Preregister at most R bases and
use the union bound R*2^-20 for false acceptance. Do not keep trying bases
without charging both multiplicity and work. Intermediate-energy bases have
no promised acceptance probability. [pcc:unverified]

## Conditional full-cost contribution

If a tuple can be sampled and mapped in polylog(p,M) work from the compact
torus parameters, this stage uses at most m*n=O(L^2) target samples, and
O(n) exact labels per block with O(n log n) balanced-tree comparisons.
From the proposed raw/cofactor fiber bound L<=8B+8, this is soft-O(B^2).
Field operations, point generation, cofactor clearing, sign-preserving
encoding, entropy and comparisons are part of that cost. None is priced at
zero. The constants and actual sample ceiling must be reported; this is not
an admitted group-equivalent work comparison. [pcc:unverified]

Conditional on the earlier proposed average C<25W0, at least three quarters
of bases have C<=100W0, so the new tester would accept those with its stated
error bound. This does not certify sigma=1, prove the earlier geometry,
establish the planted solver hit rate, or ensure relation rank.
[pcc:unverified]

The contribution would fit the existing sparse-linear-algebra exponent.
The central unresolved condition is still a decomposition budget
D=soft-O(B^gamma) with gamma<1, plus the growing-family and other full-phase
obligations. This proposed test is not a breakthrough on that solver bound.
[pcc:unverified]

### Cost review: diagnostic, not required base selection

The current native block API retains n labels, and its set may also grow
to n. Thus its worst-case RAM is quadratic in B, whereas the intended sparse
linear-algebra profile uses linear RAM in B. Its time exponent alone does
not satisfy that space target. External sorting would add a distinct disk/I/O
budget; no such implementation or price is included. This is an explicit
limitation of this implementation, not a lower bound for every energy test.
[pcc:unverified]

Moreover, the conditional attack need not identify a good base before trying
it. If the preceding energy and planted-success averages imply a constant
fraction of suitable bases, a uniformly chosen base can receive a fixed
complete attack budget, followed by bounded independent restarts. Good bases
then supply the success probability; unsuccessful bases consume only that
fixed budget. Every returned discrete log must be checked. This removes the
need to put the birthday diagnostic on the attack's critical path. The
diagnostic remains useful for an explicit finite distribution-testing claim.
The bounded-restart reduction and its assumptions require a separate account.
[pcc:unverified]

## Review and formalization map

- Existing CryptoLean.Group.PartiteCoverage.support_cauchy_schwarz supplies
  the finite support inequality; do not rederive that law in a study.
- The source paper supplies the collision-moment calculation. An actual
  finite-IID sample-space and collision statistic instance is still needed
  to connect it to the torus sampler. No formal theorem assumes that bridge.
- Generic new probability/counting laws, if needed for formalization, belong
  in crypto-lean. The current study can discharge concrete arithmetic only.
- Check the variance coefficient, heavy-atom moment bound, rounding margins,
  binomial threshold convention, exact labels and adaptive selection rule.
- Missing sampler integration and missing solver-cost proof remain explicit;
  the executable's calibration mode collects no base samples and runs no solver.
- The native algebra-only certificate checks the cleared variance-expression
  simplification. It does not certify the IID interpretation, heavy-atom bound,
  binomial amplification or actual torus energy.
