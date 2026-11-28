# Prime31 rational-doubling elimination: 21 -> 14 variables (frozen 2026-11-19)

Status: preregistered follow-up on existing PR #4812 before adding an executable. No unknown-index extraction or complete ECDLP gain is claimed.

## Frozen curve, input and control

Reuse original `prime31-algebraic-ic` Rust arithmetic at commit `1df714645aa683cf975bc76fcacb9842cb4780fd`; do not replace the field or generator. E(F_p): p=2147483647, a=847514846, b=2087444724, n=2147446831 (prime), cofactor one. The target is a *public point*, with eight signed leaves drawn from a public fixed window of eight among the first 31 rational-x factor-base points. Original four windows: [0..7], [8..15], [16..23], [23..30].

This is a polynomial-encoding experiment for the Horner powers-of-two weight schedule, not a public-target-only solver.

## Algebraic candidate and exactness

Let f(z)=z^3+a z+b, N(z)=z^4-2 a z²-8 b z+a² and D(z)=4 f(z).
Let F3(u,v,r)=A(u,r)v²+B(u,r)v+C(u,r), with A=(r-u)², B=-2((r+u)(ru+a)+2b), C=(ru-a)²-4b(r+u).
Substitute v=N(z)/D(z), clearing denominators:

    E(u,z,r)=A(u,r) N(z)^2 + B(u,r) N(z)D(z) + C(u,r)D(z)^2.

The x-only signed Horner chain can impose E(x_i,t_{i+1},t_i)=0 for i=0..6, with t_7=x_7 and t_0=x(Q), plus H_i(x_i)=0 for all eight leaves. All of d_0..d_6 are removed. After substituting the public x(Q) it has 14 variables (eight leaf x and six internal suffix x) and 15 equations (eight H + seven E), instead of the previous 21 variables / 22 equations. The E_i have total degree <=10, versus F3/quartic degrees <=4 before elimination, and denser support may offset all variable savings.

Because #E(F_p) is odd, E(F_p) has no 2-torsion. Thus f(z) has NO root in F_p: each root would give (z,0) of order two. Therefore D(z) !=0 for **every rational z in F_p**, not only valid planted suffixes. Consequently, for every (u,z,r) in F_p^3, E(u,z,r)=0 iff F3(u,N(z)/D(z),r)=0. This is an *exact finite-field rational-root equivalence*, not a claim of scheme isomorphism over the algebraic closure; when working over extensions, clearing denominators can create spurious components and saturation must be considered.

## Frozen validation before results

- Native Rust example will reuse `Arithmetic`, `PRIME31` and `check_curve`. Deterministic SplitMix64 seed `0x454c494d5f443134`, 256 planted controls per each of four B8 windows = 1024 fixtures; exceptions must be counted, not dropped.
- For each finite nonexceptional planted chain verify the rationally-eliminated E_i=0 and original doubled/F3 residuals for every i=0..6. Count and report checks, denominators zero, chart exceptions.
- On 8192 independent *uniformly sampled field triples*, test equality E=F3*D² modulo p and equality of the two zero tests; do not label these as natural public relation queries.
- A negative control mutates the target x-coordinate on each positive fixture; count same-witness rejection only. A mutated target might possess another witness.
- Two focused Rust tests; release JSON must include scientific booleans `public_only_unknown_index_extraction=false` and `complete_ecdlp_solved=false`.
- Independently use Python big integers to recompute the coefficient identity and random triple equality, plus the degree/variable counts.
- Run source-specific GitHub Actions with pinned original Rust. Any CI failures remain failures; no completed run claimed until verified.

## Prior-art and next decision

Source starting points: PR #4812 (21-variable x-only), #4692 (actual public-only RR9/F4 owner), #4809 (existing F3), #4810 (Kummer doubling), #4807/#4808 (weighted grading only), #4728 (whole solver), #4532 (optimized BSGS). See `research/prime31_ic_20261115/native/M8_UNIFORM_RANK_CEILING_AND_F4_PILOT.md`. Scholarly prior art: Semaev summation polynomials and Yokoyama et al. 2020 https://doi.org/10.1515/jmc-2019-0029. No novelty of the standard rational substitution is asserted. Shared KB tool unavailable on this runtime; these direct sources are the bounded fallback.

Pass means exact mathematical equivalence and original-Rust reproducibility, **not** a speedup. Then compare 14-variable degree-10 vs 21-variable degree-4 in the **same** existing Rust F4 engine, on the frozen 46 positive / 18 negative public B8 queries with signs/indices hidden. A valid efficient oracle must feed rank-31 + 1024-log whole solver and beat same-host optimized IC, BSGS and rho with complete costs. If F4 stalls, record as a failed lead and prioritize public-only extraction rather than more planted checks.
