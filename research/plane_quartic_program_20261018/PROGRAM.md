# Plane-quartic index calculus and the Smith walk: program scope

2026-10-18. This document is the scope for four workstreams. It is not a frozen
protocol: each workstream freezes its own `PROTOCOL.md` in its own study
directory before collecting any data. This document can be revised after those
protocols are frozen, but a revision never changes a frozen protocol.

## Question

For `E/F_{q^3}` with a genus-3 GHS descent to `F_q` (`q` an odd prime), what is
the measured end-to-end cost of solving the ECDLP through a smooth plane quartic
with Diem's index calculus, compared with the matched Pollard rho on `E(F_{q^3})`,
and at what `q` (if any) do the two cost curves cross?

The theory to compare against: plane-quartic index calculus is `Õ(q)` (Diem 2006,
ANTS VII; Diem–Thomé, J. Cryptology 21 (2008)). The genus-3 hyperelliptic index
calculus with double large primes is `Õ(q^{4/3})`. Rho on `E(F_{q^3})` is
`Õ(q^{3/2})`.

There are two routes to a quartic:

- **Route Q (direct).** The `m = 2` GHS type gives a plane quartic over `F_q`
  directly. It is built in `research/ghs_cubic_genus3_build_20261012/`, and toy
  index calculus runs in `research/ghs_cubic_quartic_ic_20261012/` (q ≤ 11).
- **Route S (Smith).** The `m = 3, r = 5` type gives a hyperelliptic `H`
  (`research/ghs_cubic_hyperelliptic_genus3_20261013/`,
  `research/odd_char_descent_genus3_20261014/`). Smith's `(2,2,2)`-isogeny then
  takes `J_H` to a plane-quartic Jacobian when Propositions 2 and 4 pass
  (`src/cryptanalysis/smith_prop24.rs`): 18.6% of random curves, with an
  isogeny-class ceiling near 50% (`problems/smith-kernel-isogeny-class-walk.md`).

## Claim hygiene (from AGENTS.md)

- Use only public synthetic or known-answer instances, and keep all evidence
  fail-closed.
- Make no key-recovery, asymptotic sub-rho or deployed-curve-security claims.
- None of this touches prime-field curves or the binary standard curves.
- A crossover statement is a measured fit over a stated `q` range, never an
  asymptotic theorem.
- Magma, Sage, PARI and msolve are not installed on the run host. Every speed
  result therefore says "not compared with an external implementation" and
  makes no fastest-implementation claim.

## Shared rules

- **Branches and directories.** Each workstream gets its own worktree, its own
  `claude/…-20261018` branch, its own PR and its own study directory under
  `research/`.
- **Conductor.** Run `conductor check` with the workstream's path scope before
  editing.
- **Protocol first.** Freeze `PROTOCOL.md` in a commit before any data is
  collected, and cite that commit hash in the report.
- **Shared host.** The host is shared with a frozen long run, so every
  workstream is pinned to its assigned cores: `taskset -c <cores> nice -n 10`
  for every build and run, and `cargo -j <ncores>`. Never touch
  `/home/ec2-user/crypto` (the running experiment's checkout) or other
  worktrees. Memory cap: 12 GB per workstream.
- **Cost unit.** Count `F_q` multiplications, with each inversion priced at
  its measured multiplication equivalent. Record wall time on the pinned cores
  as well. The normalised unit is `S = cost / √ℓ` as in the existing studies,
  with `ℓ` the large prime order of the target subgroup.
- **Verification.** Every recovered discrete log is verified by scalar
  multiplication on `E`. Every claimed class equality comes with an
  independent check, such as an annihilator, brute force on tiny `q`, or a
  second representation.
- **Language.** Computation is native Rust (AGENTS.md "Implementation
  language: no Python"). Python is used only to plot recorded values and lay
  out reports.
- **Reports.** Every run leaves a report, a graph and a PDF, following
  `.claude/skills/research-visuals/SKILL.md`. Performance results are also
  copied into `benchmarks/20261018-<slug>/`, with a row in
  `benchmarks/README.md`.
- **Pricing across arms (AGENTS.md §5a).** Every IC-versus-rho comparison
  lists the unpriced counters of both arms (hashing, distinguished-point
  tables, canonicalisation, sorting, root finding). It prices them at 0, 0.1
  and 1 GAE per operation, and ranks the arms only where the order holds
  across that whole band.
- **Validation before pushing.** Run `cargo test --release --lib` (pinned) and
  the touched examples' tests. If the lib tests are too slow on two cores, run
  the touched modules' tests and say so in the PR.

## Workstreams

### W1. Plane-quartic Picard arithmetic with canonical forms (prerequisite for W2b and W3b)

**Gap.** The current class keys use degree-6 Galois-orbit divisors, and some
classes collide after cubic reduction.

**Deliverable.** A library module `src/cryptanalysis/plane_quartic_pic.rs`
covering arithmetic in `Pic^0(C)(F_q)` for a smooth plane quartic `C/F_q`
(odd prime `q < 2^31`).

- **Representation.** Fix a rational degree-3 base divisor `D_∞`, for example
  `K − P0` for a rational point `P0`. A class `x` in `Pic^0` corresponds to the
  class `x + D_∞` in `Pic^3`.
  - Typical classes have `h^0 = 1` and are represented by their unique
    effective degree-3 divisor.
  - Exceptional classes have `h^0 = 2`, so the class equals `K − P` for a
    point `P`; they are represented by that point. Expect about `q` such
    classes out of about `q^3`.
- **Divisor encoding.** Store effective divisors as Galois-stable data over
  `F_q`: either a pair of polynomials, as in Flon–Oyono–Ritzenthaler, or a
  multiset of points over `F_{q^k}` (`k ≤ 3`) normalised to a canonical
  ordering. Whatever encoding is used, the canonical key must be a function of
  the class.
- **Operations.** Addition, negation and scalar multiplication. Reduction may
  use the adjoint-curve method: through `D1 + D2` (degree 6) pass a cubic, take
  its residual, then use a line or conic to get back to degree 3. Follow Flon,
  Oyono and Ritzenthaler, *Fast addition on non-hyperelliptic genus 3 curves*
  (2008), or Hess 2002, or derive and test your own. Handle the exceptional
  cases explicitly.
- **Validation.**
  - Brute force on small `q` (5, 7, 11, 13): enumerate every effective
    degree-3 divisor class and check that the count equals `#Jac` from the
    `L`-polynomial (point counts over `F_q`, `F_{q^2}`, `F_{q^3}`).
  - Random-triple associativity and commutativity, the homomorphism property,
    and `#Jac · x = 0`.
  - Agreement with the degree-6 keys of `research/ghs_cubic_quartic_ic_20261012/`
    on the classes they distinguish, including resolving the known collisions.
  - Scaling of operation cost up to `q ≈ 2^31`.

### W2a. Diem relation phase on GHS quartics at growing `q`, plus a priced rho arm

Runs in parallel with W1 and does not need class reduction.

- **Instances.** Port or extend the `m = 2` builder to prime `q` from about
  2^8 up to the budget limit. Keep only instances where `#E0(F_{q^3})` has a
  prime factor `ℓ` with `ℓ > q^{2.5}` and a small cofactor. Freeze the
  instance list (seeded) in the protocol.
- **Relations.**
  - Every line meets `C` in four points, and all line sections are linearly
    equivalent. A line through two factor-base points whose residual quadratic
    splits into factor-base points gives a relation among `[P − P0]`.
  - Implement the full factor base, Diem's reduced factor base with single
    large primes, and double large primes with graph-cycle recombination.
  - Measure the yield per line, the number of relations, the cost, and the
    `ℓ`-rank of the relation lattice. The `ℓ`-rank must show that the
    factor-base classes generate the `ℓ`-part; the toy study found that they
    did not at `q ≤ 11`.
- **Linear algebra mod `ℓ`.** Use the repository's sparse solver (structured
  Gaussian elimination plus Lanczos or Wiedemann), and price it.
- **Rho arm.** Run priced Pollard rho (r-adding walk, distinguished points,
  `F_q`-multiplication counts, matched to the same target subgroup) on
  `E0(F_{q^3})`. Reuse `src/cryptanalysis/pollard_rho.rs` or the `ecbench`
  path where they apply. The memory notes warn that ecbench silently exits
  with an explicit `--cpus` and adds about 0.3 s of overhead per run.
- **Output.** Per `q`: relation-phase cost and linear-algebra cost, both
  extrapolated, against the measured rho; and the fitted exponents. The target
  descent is left to W2b.

### W2b. End-to-end Diem pipeline and crossover fit

Starts after W1 and W2a land.

- **Descent.** For a target, take `φ(Q) + Σ r_i F_i` for random `r`, reduce it
  with W1, and accept it when the degree-3 representative splits over the
  factor base. Single and double large primes in the descent are allowed.
  Verify the recovered key on `E`.
- **Measurement.** End-to-end cold `S` per `q`, compared with matched rho. Fit
  the exponents and report the crossover estimate with a confidence interval,
  or "no crossover in range".

### W3a. Smith walk via the elliptic side, measured exhaustively at toy `p` and priced at larger `p`

The problem record frames the walk as hyperelliptic-to-hyperelliptic
`(2,2,2)`-isogenies, which generically do not exist (the quotient of a
hyperelliptic Jacobian by a maximal isotropic subgroup is generically a
quartic Jacobian). The walk that can be implemented now is on `E`:

- Rational 2-isogenies (Vélu) on `E/F_{q^3}`, already priced in
  `src/cryptanalysis/jv_isogeny_walk.rs`, move `E` through its
  `F_{q^3}`-isogeny class.
- Every member with a weak model (the `N(c) = 1` test, or the
  `m = 3, r = 5` form) descends to an `H'` whose `J_{H'}` lies in the same
  `F_q`-isogeny class as `J_H`.
- Different weak models and choices of `α` for the same `E` give further
  `H'`.

**Measure:**

1. Exhaustively at `p ∈ {7, 11, 13}` (bigger if cheap):
   - for each `E` with a descent of this type, the descended `H` and whether
     Smith's full test passes (`smith_prop24::analyze_curve`);
   - the elliptic 2-isogeny graph restricted to those curves;
   - the distribution of walk distance from a failing `E` to the nearest
     passing one.

   Compare with the isogeny-class ceiling of the problem record.
2. Sampled at `p` from about 2^10 to 2^20:
   - expected walk length and expected cost to the first passing curve;
   - the descent and Smith-test costs, priced in `F_q` multiplications;
   - whether the walk cost is `o(q)`, so that it does not dominate `Õ(q)`
     index calculus.
3. A literature note on genuinely hyperelliptic-to-hyperelliptic
   `(2,2,2)`/`(ℓ,ℓ,ℓ)` isogenies in genus 3 (Howe–Leprévost–Poonen, Smith's
   thesis, Couveignes–Ezome, Milio and others): do they exist on a
   positive-density set, and is any of them implementable here? Report this
   as text with citations. No implementation is expected.

### W3b. Explicit Smith isogeny on divisor classes

Today only the curve `X` and its point count exist; the map on divisors that
is needed to transfer a DLP does not. Implement Smith's correspondence
(Smith 2009, §5–6, arXiv:0806.2995) from `H` to `X` on Mumford divisors,
producing effective divisors on the quartic `X`.

**Verify:**

- `ψ(D1 + D2) ~ ψ(D1) + ψ(D2)`, `ψ` is non-zero on an order-`ℓ` element,
  `ℓ · ψ(G) ~ 0`, and the class-level structure is as expected;
- first with degree-6 keys or annihilators, then with W1's canonical forms
  once W1 lands (rebase onto W1's branch);
- end-to-end at toy `p`: a planted ECDLP on `E/F_{p^3}` through Route S, using
  the existing quartic toy index calculus or W2b once available.

## Order and dependencies

```
W1 ──┬──> W2b ──> crossover report
W2a ─┘
W3a (independent)
W3b ──(rebase on W1 for verification)──> Route S end-to-end
```

## Expected outcome (stated in advance)

At best, a measured crossover `q*` below which rho wins, for a family that is
a vanishing fraction (about `q^{-1}`) of the curves over `F_{q^3}`. It is
equally valid for the study to find that the constants put the crossover beyond
the feasible range, or that Route S's walk cost dominates. Both are recorded as
results. Neither is a security statement about any standardized curve.
