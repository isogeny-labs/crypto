# Prime-field continuation: checked baselines, reductions and recent tensor prior art

2026-10-30 UTC. The requested **original** sub-exponential algorithm has **not** been achieved. This continuation moves beyond supplied witnesses: it implements a complete small-curve direct index-calculus solver and derives supporting rationality, coverage, recovery and auxiliary-curve construction bounds. The current norm-coefficient PDP remains exponential. No new speedup, growing-size experiment, formal proof-assistant certificate, or certified novelty claim is made.

## Current prior art changes the assessment

The initial focused audit missed two recent public manuscripts. Version0.2 of A Subexponential Time Algorithm for Elliptic Curve Discrete Logarithm claims expected classical bit time

    2^{O(b/(log b)^(1/4))}, b=ceil(log2 p),

through compact Boolean tensor encodings and independent affine-hash factor bases. Its reduction is conditional on a contraction theorem stated and proved in AppendixD of The Exponential Time Hypothesis is False. All setup, failed queries, seed restarts and linear algebra are charged. This is materially stronger prior art than an unproved factor-base proposal. It is not an algorithm invented in this session.

[tensor-review/](tensor-review/README.md) pins the manuscripts and records the checks. Two independent within-session readings of the full decisive contraction proof found no concrete gap in its recurrence or local precision algebra. The parent also checked the compact multiplication construction, point cases, hash coverage, recovery and cost balance in the ECDLP reduction. These are scoped proof readings; general implementation correctness and the full manuscript claims are not certified by them. Public source labels alone are not endorsement.

The tensor theorem chain is now the best-supported existing claimed sub-exponential route identified in this review. The original norm/divisor choice remains an unproved research candidate. Transcribing or renaming the tensor algorithm would not meet the user's strict novelty requirement. An original algorithm remains unachieved, and the prior-art correction is explicit rather than presenting the earlier partial audit as exhaustive.

New bounded development evidence includes an independent native GMP reconstruction unit:72 cases,308 Cauchy charts,8832 node errors,1152 final errors and four negative controls pass, with exact replay. The existing native tensor crate also builds under an official unprivileged Rust toolchain and passes all34 release correctness tests. These are fixed implementation/lemma checks; they do not certify the complete theorem or general recursive implementation, and no challenge logarithm or performance claim follows.

## What now runs without a witness

[solver/norm_solver.cpp](solver/norm_solver.cpp) takes only the curve, target and base parameters. For arity three it searches the two remaining norm-function coefficients, enforces divisor membership through quotient remainders and extracts every rational selected-chart solution. No source summands, generation scalar, norm witness or decomposition table is supplied.

The fixed native development suite compares its **complete model sets** with a separate signed-point oracle for all 18 affine targets on E/F17: y^2=x^3+2x+2 across five domains. All 90 cases agree: **42 SAT, 48 UNSAT, 400 models**. In the t=3,c=1 domain there are six natural successes, six nonempty-chart natural failures and six target-exclusion empty-chart failures. This avoids using empty charts as the only negative controls.

[solver/direct_ic.cpp](solver/direct_ic.cpp) then recovers the unknown logarithm of every one of the curve's 18 nonzero targets through actual mixed-target IC. It receives Q as a point and a seed, not its logarithm. It constructs a cold canonical base, draws a,b, forms R=aP+bQ, calls the coefficient solver with R only, collects four relations, chooses a left-null vector from the factor-base coefficient matrix only and verifies the recovered scalar by group multiplication. There is no rho, BSGS, target-log scan, supplied factor-base log or cross-target setup reuse in that path.

Both an internal point-only interface and a separate CLI pass all 18 targets. Twelve of the internal recoveries have a rank-deficient factor-base matrix: rank one on two targets, rank two on ten, rank three on six. No full-rank gate is imposed.

The internal panel retains 230 attempts: 13 identity misses, 145 unsuccessful PDP calls and 72 accepted rows. All 217 nonidentity PDP calls are charged, totaling 59,024 coefficient pairs, plus separately retained root scans, cold setup, group phases and matrix arithmetic. These counters are diagnostics, not a calibrated cost or performance comparison.

An independently implemented native verifier, which includes no candidate source, rechecks all 90 complete model sets, all 72 point relations, all 18 kernels, the scalar answers from both panels and the counter partitions. Exact records and reproduction commands live under [solver/](solver/README.md) and [reviews/](reviews/solver.md).

## Why this still is exponential

Each current PDP call visits p(p-1) coefficient pairs. Root extraction is additional work. A safe literal field-operation bound includes the repeated Fermat inversions in polynomial division:

    O(p^2 (t+1) log p + p M),

where M counts candidate root-recovery work in the implementation's stated convention. In bit length n=log2 p, even the coefficient loop alone is Theta(2^(2n)). The verified tiny DLPs establish complete plumbing and correctness references; they do not satisfy the requested complexity threshold. The toy curve also uses dense bases and is not an asymptotic sample.

## Supporting mathematics: three gaps can be bounded in one restricted family

The detailed derivations are [rationality-and-reduction.md](theory/rationality-and-reduction.md) and [coverage.md](theory/coverage.md), with separate [rationality](reviews/rationality.md) and [coverage](reviews/coverage.md) reviews. These are written proofs independently re-derived within this session, not formally certified new theorems or a complete novelty search.

Assume a nonsingular E/Fp with odd whole group order r, a power of two d=2^t dividing p-1, a uniform random coset g mu_d, and odd arity m. The direct DLP recovery additionally assumes r prime. The signed rational base includes both y signs; its size B is not the abscissa count. The selected chart excludes repeated abscissas, infinity and target-abscissa overlap. Arbitrary primes without a sufficiently large compatible d are outside this Kummer family; the auxiliary extension below has different setup hypotheses.

**Rationality.** Odd group order makes the cubic f irreducible. Its roots cannot have a nontrivial power-of-two ratio in Fp, so the branch sets of f(gw), w in mu_d, are disjoint. Independent square classes and a one-square-root Galois flip show that a substituted summation polynomial is not identically zero in g. A polynomial root count and union bound give

    Pr[short zero sum] <= Delta,
    Delta = (p-1)^(-1) sum_(s=3)^(m-2) binom(d,s) s 2^(s-2).

Outside this event, every selected-chart geometric norm solution for every nonzero rational target has rational core coefficients. A nonrational subset would consist of anti-Frobenius points whose sum is zero; distinctness and target separation force its length into the excluded interval 3 through m-2. The old F17 arity-five counterexample remains valid: at its dense parameters Delta is vacuous. No efficient recognition of a realized good coset is inferred.

**Coverage.** This is stronger than substituting mean tuple count for success probability. For M0=binom(d,m), let V_g(R) count unordered signed distinct-abscissa decompositions, and T_g their total. On the multiquadratic cover belonging to k distinct abscissas, a weighted sum of projection maps has degree

    deg(sum_i c_i P_i)=2^(k-1) sum_i c_i^2.

The elliptic theorem of square and individual sign-flip automorphisms establish this identity; it is not a genericity heuristic. Counting the zero fibres for pairs of supports, including all relative signs and the identical-tuple diagonal, yields

    sum_(g,R) V_g(R)^2 <= sum_g T_g + m 4^m M0^2.

A second weighted-map calculation controls the zero-target and target-overlap exclusions. Together with a proved usable-base density bound and a second-moment inequality, this gives a constant fraction of admissible bases selected-chart coverage

    rho_g >= rho0 = 1/(192 m 16^m)

under the explicit finite conditions in the note (including p>=17, d>=8m^2 and d^m/(m!r)>=2^m). The result bounds coverage, not just average representations. It uses covers for the proof; constructing their Jacobians is not an algorithm step.

**Recovery without full base rank.** Draw fresh uniform a,b modulo r and let R=aP+bQ. Conditional on R, b is still independent uniform. The oracle must see only R and independent randomness. After B+1 accepted rows, choose any nonzero left-null vector w using the base coefficient matrix C alone. Then beta=w.b is uniform, so it is nonzero with probability 1-1/r; alpha+k beta=0 recovers k. This standard direct-IC argument does not require full rank(C), independent base logs, or an independent-relation heuristic. A fixed memoryless oracle, or uniform conditional acceptance bounds, is essential; a marginal acceptance mean cannot justify arbitrary adaptive stopping.

## A sufficient oracle contract and a weaker route

Set n=log2 r, take odd m near sqrt(n)/2, and let d be the least compatible power of two at least 2(m!r)^(1/m). Such a d must actually divide p-1; its existence is a field-family condition. The preceding bounds apply eventually within that family, including the short-relation bound. Actual rational base construction costs O(d poly(n)), and B<=2d.

One sufficient missing procedure is a **complete fixed-base decomposition oracle**: given E, g, d, m and R, return a verified rational selected-chart decomposition, or correctly refute its existence, with uniform cost C(n) on the good input family. All symbolic construction, rational root extraction, exceptions and failed calls count. No supplied solution, generic-completion table or free preprocessing is permitted.

The supporting reduction needs

    2^(4 sqrt(n)+O(log n)) oracle calls

and dense linear algebra at most

    2^(6 sqrt(n)+O(log n)) bit-polynomial factors.

With finite per-base call caps, per-call operation caps, verification and restarts, a proved uniform oracle bound C(n)=exp(o(n)) would therefore give a complete sub-exponential algorithm in this family. The inability to recognize a good sampled base need not itself cause an unbounded run. But **no computable sub-exponential cap with a proved sufficient verified yield is supplied for this norm-system procedure**. Call count is not runtime; the implemented oracle instead has an explicit p^2 loop. These supporting bounds do not prove sub-exponential ECDLP for all prime fields, P-256 or secp256k1.

The earlier wording made complete fast refutation sound necessary. It is only sufficient. [selective-oracle-contract.md](theory/selective-oracle-contract.md) gives a weaker, independently checked route: a capped, label-free procedure may return no relation found, provided its verified success probability is bounded below by a known exp(-o(n)) on a constant fraction of sampled bases. In particular, a known exp(o(n)) average time to the first verified relation on covered targets suffices, using a fixed cap and the proved coverage bound. Uncovered targets need no fast refutation. This correction broadens the solver search without weakening correctness or omitting failed-call cost; the needed cost/success bound remains unproved.

## Strategies examined rather than assumed fast

[code-and-guessing-boundaries.md](theory/code-and-guessing-boundaries.md) reformulates the problem as finding a singular m-column submatrix of a genus-one evaluation-code matrix. The matrix itself is sub-exponential to build, but choosing its support is the hard step.

- Interpolating after guessing m-1 roots gives the ordinary completion R-sum(P_i); it does not create a new non-generic oracle.
- Uniformly guessing a constant fraction of root blocks still takes exp(Omega(n)) trials in the stated small-representation regime, even if the residual solver is free.
- Enumerating all divisors or all minors has roughly binom(d,m) states, exponential at balanced parameters.
- Standard unique/list-decoding guarantees do not find the desired extremal nonzero codeword merely by receiving the zero word.

These exclusions are scoped to the listed strategies and distributions; they are not impossibility claims about all implicit solvers.

## The p+1 extension and what it changes

[torus-extension.md](theory/torus-extension.md) supplies the separately reviewed norm-one version. For d dividing p+1, take H in the norm-one torus in Fp2 and abscissas x_w=gw+(gw)^(-1). Every abscissa is in Fp. Reject g^2 in H, an event of probability at most 2d/(p+1), to obtain distinct abscissas. Membership is the split separable Dickson polynomial D_d(X)-[g^d+g^(-d)], implemented by a repeated-square-minus-two circuit. Its terminal value may be zero; only +/-2 are excluded.

The branch independence proof, projection degrees, density character sum and trace exclusions have been checked separately. Projection and collision degrees double. A conservative selected-chart coverage bound is 1/(320 m 16^m), with the density, chart, trace and short-relation errors all stated in the note. The conditional asymptotic call and linear-algebra exponents above are unchanged. The full selected norm/remainder scheme is finite and geometrically reduced at fixed input; that property does not give a solving-degree bound.

P-256 has a 2-power divisor of p+1 with exponent96, so the tower-divisor condition has concrete relevance there. The Dickson base itself has already appeared in the reviewed repository work and is not claimed new. No efficient relation solver, P-256 attack or all-prime-field construction follows from the torus construction alone. Using auxiliary Fp2 arithmetic to construct rational abscissas does not change the original problem to ECDLP over an extension field.

## Removing the congruence restriction through an auxiliary curve

The separately reviewed [auxiliary-curve extension](theory/auxiliary-curve-extension.md) removes that modulus restriction from the supporting reduction, for sufficiently large prime fields and main curves of odd prime whole order. The auxiliary elliptic coset and isogeny tower already appear in PKM2016 sections3.4-3.5; they are not a new factor-base invention.

The setup bound here uses classical modular-curve point counting rather than the paper's heuristic near1/d auxiliary-curve probability. Primary-source checks establish at least1/(24d^2) probability that a random short model over Fp has a rational exact-order-d point when d=2^t>=8 and p>=256d^4. Schoof's unconditional polynomial-time cardinality algorithm and an actual point-order test then give expected O(d^2 poly(log p)) charged construction work. Divisibility of a group order is not substituted for existence of a cyclic subgroup.

For a rational subgroup H on the auxiliary curve A, use x_A(G+h), h in H. Reject2G in H with the exact quotient-isogeny test. A separate six-branch-point proof establishes independent squareclasses; projection degrees, chart losses and the short-relation bound transfer. The same conservative coverage bound1/(320m16^m) follows, with all four error terms stated in the note. The radical membership circuit uses bounded remainders and denominator inverses along the rational2-isogeny tower.

At balanced m near sqrt(n)/2, the condition p>=256d^4 holds eventually without assuming d divides p-1 or p+1. Setup is at most2^(4sqrt(n)+O(log n)) expected cold work; the conditional query and linear-algebra exponents remain as above. The main odd-prime-whole-order hypothesis remains. This is a derived and reviewed setup/reduction, not an implemented auxiliary pipeline or the missing efficient relation solver.

There is also a proved [positive-target degree bound](theory/small-degree-positive-targets.md). On each simultaneous good base, at least rho0/2 of all targets have a nonempty selected quotient of geometric degree at most128m32^m. The tuple count equals that degree because the normalized scheme is finite etale with unique witnesses. At balanced parameters this makes subsequent polynomial-in-degree conversion/root extraction sub-exponential on those targets. It does **not** bound construction of the initial basis or imply a small solving degree. That distinction now locates the main algebraic gap more precisely.

[initial-elimination-obligation.md](theory/initial-elimination-obligation.md) makes one sufficient next question concrete: prove a Macaulay certificate-degree bound O(m+t) on enough of those positive inputs. With O(mt) variables this would bound the initial matrix work by exp(O(sqrt(n)log n)), and the selective reduction would charge its failures. The bound is a proposed proof obligation, not an established property of this circuit. A small final printed basis degree does not establish it.

## External solver interface and the execution route

[canonical/](canonical/README.md) now provides a native polynomial normalizer. Its 21 format checks pass, all six prototype exports preserve their coefficient maps, and an installed native msolve 0.7.5 parses the canonical systems with all variables bound to the known assignments. An initial unit-ideal failure was traced to unnormalized x+0 input and retained; normalization fixes the imports. These six checks validate the importer, not unknown-solving performance or the fastest external backend.

A separate [unbound fixed-fixture library check](canonical/unbound-check/README.md) passed all33 variables as unknowns for the F17 m3 remainder system. It exited0 with243 printed basis elements. An independent native leading-monomial checker counts28 standard monomials, matching the28 expected point models. It does not certify equality with the source ideal or extract the complete CAS model set. The printed maximum basis degree3 is not an intermediate solving-degree bound. Prospective unit protocol, resource caps, raw basis and failures remain separate from scientific timing evidence.

The compute-route audit corrected the old blanket blocker. The main-branch ecbench route and ARC research runners exist. A direct arbitrary-command endpoint is still unavailable here; registered-runner and Actions-variable reads return403. The existing read-only EKS smoke workflow was actually dispatched as [run40979213857](https://github.com/isogeny-labs/crypto/actions/runs/40979213857), pool linux. It executed zero steps and was cancelled after the 600-second queue-wait limit; terminal state is retained in `readiness-run.json`. The bounded probe did not establish runner readiness. It was not a solver experiment or timing admission. There is no remaining job from this dispatch.

Several parallel tooling/research turns were interrupted by automatic content filtering. They are retained as execution limits, not negative mathematical evidence; incomplete follow-up derivations are not presented as finished. No permissions, identities, routing variables or cluster settings were changed.

## Recheck the retained evidence

From the repository root, run bash research/prime_one_shot_20261029/continuation/check.sh. It checks the source/artifact hashes, the solver's frozen manifest, independent native record replay, the retained CAS staircase and all regenerated Hermite-unit table hashes. Large raw tables use the normal artifact store; the replay does not require downloading them. The solver and canonical directories contain their separate generation/reproduction commands. No compiler, backend binary or credential is committed. The plot/PDF renderer performs layout only. The source register and reviews identify exactly which literature and derivations were checked; none is an exhaustive novelty certificate.

## Status against the original request

The objective of producing an original sub-exponential algorithm remains open. This continuation identifies an existing explicit claimed sub-exponential theorem chain, records scoped proof checks and bounded native tests, and corrects the initial prior-art assessment. It also supplies the complete tiny-curve norm/IC baseline and supporting probability/setup proofs. It does **not** establish novelty, certify the general tensor implementation, prove sufficient cost/yield for the norm-system candidate, implement its auxiliary pipeline, or exhaust every historical experiment. The precise unresolved requirement and discriminating next step are tracked in [the problem record](../../../problems/prime-divisor-uniform-solver.md). There is no claim that work continues unattended after this turn unless a named running job remains active.
