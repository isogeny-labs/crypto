# A selective capped oracle suffices; fast refutation is optional

2026-10-30 UTC. This corrects an overstrong requirement in the earlier
formulation. A complete fast decomposition/refutation oracle is sufficient
for the conditional reduction, but it is not necessary. This is a standard
randomized-reduction argument, not a new relation solver. PKM2016 Algorithm1
already collects whatever verified relations are found; the bound below makes
the relevant success/cost contract explicit.

## Contract

Fix an input P,Q=[k]P of odd prime order r, n=log2 r. A sampled base has
at most Bmax points, with bounded construction/storage cost F(n), for every
sample, including bad samples. Alternatively reject oversized bases under
that same bound. Suppose a fraction at least q>0 of sampled bases have this
property: for uniform R in <P> and independent oracle seed, a fixed
memoryless relation procedure, capped at known work T(n), returns a verified
relation over that base with probability at least a known theta(n)>0.

Both T and theta must be explicit computable bounds to define an algorithm.
The procedure sees R and independent randomness, never the mixed-target
labels a,b. Its choice of a relation, acceptance and stopping do not use
those labels. A returned object must have bounded length and its verification
is included in T. False negatives and bounded timeouts are allowed; neither
is called an UNSAT certificate. No correctness assumption is made on an
unverified return.

In the current constructions Bmax=2d globally. For sub-exponential expected
total cost, sufficient bounds are F,T,Bmax,theta^(-1)=exp(o(n)) and q a
known positive constant. A sub-exponential inverse q would also suffice
with the corresponding expected restart factor charged.

## Explicit finite block and recovery

Let K=Bmax+1 and N=ceil(4K/theta). For each attempt draw a,b independently
uniformly in F_r, form R=aP+bQ, and call the capped oracle. Retain only
verified rows and stop after B+1 successes, or after N attempts. A shortage
restarts with a fresh base. Actual B may be smaller than Bmax.

On a good fixed base, the success indicators are independent Bernoulli
variables with mean mu>=4K over the hypothetical full N calls. A Chernoff
bound gives

    Pr[number of successes<K]
      <=exp[-mu(1-K/mu)^2/2]
      <=exp(-9K/8).

The last inequality follows because (mu-K)^2/mu is increasing for mu>=K.
Stopping after the smaller actual threshold B+1 only helps.

Write u=a+kb, so R=[u]P. The transformation (a,b)->(u,b) is bijective;
conditional on any full label-free oracle transcript, the accepted b_i
remain independent uniform. This also holds at the bounded stopping time.
Choose any nonzero left-null vector w of the (B+1)-by-B coefficient matrix
C using C alone. Then beta=sum w_i b_i is uniform in F_r. Verified group
relations give alpha+k beta=0, where alpha=sum w_i a_i. With probability
1-1/r, beta is nonzero and k=-alpha/beta; verify [k]P=Q before returning.
A zero beta or failed block triggers a charged fresh restart.

Every block is bounded, including bad-base blocks. Each independently
sampled block succeeds with probability at least

    q [1-exp(-9K/8)] (1-1/r).

Thus the expected number of blocks is bounded when q is constant. The
complete expected work, including mixing and dense nullspace computation,
is at most a constant multiple of

    F(n)+N [T(n)+poly(n)]+Bmax^3 poly(n).

Storage is bounded similarly. Fresh independent blocks make previous
label-dependent beta failures harmless. No full factor-base rank, supplied
base logs, successful refutation or independent-relation heuristic is used.

## A positive-instance average is enough

For each good fixed base, suppose selected-chart coverage is rho_g>=rho0,
and on a uniform covered target the uncapped procedure's expected time to
its first verified relation, averaging independent seeds too, is at most
a known A(n). All construction/extraction/verification work counts in A.

Cap the procedure at2A(n). Conditional Markov gives at least1/2 success
on covered targets, hence unconditional theta>=rho0/2. On uncovered targets
the procedure can run indefinitely in its uncapped form: the actual cap
still bounds all their work. An averaged positive-instance bound is
therefore a sufficient alternative to a uniform complete solver bound.
The positive average must hold separately for each base in the good event;
an aggregate mean over bases could conceal bases on which collection stalls.

With the proved rho0=1/(192m16^m) for Kummer, or1/(320m16^m) for the
torus version, this change only alters constants in the established oracle
call exponent. It does not supply A, T or theta for an actual fast solver.

## Effect on the research obligation

The earlier complete-oracle contract remains one sufficient route. The
more permissive target is a fixed-base implicit procedure with a proved
sub-exponential capped cost and verified success probability, potentially
without efficient refutations. The current exhaustive coefficient solver
satisfies correctness but has exponential cost. No new procedure meeting
the cheaper contract has been constructed here.
