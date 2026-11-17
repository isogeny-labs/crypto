#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#set par(justify: true)

= NIST binary curves: skipped candidates (2026-10-20)

*Status.* Measured census of how the FIPS 186-2 binary curves were chosen. No bearing on ECDLP hardness. Study directory: `research/nist_binary_skipped_20261020/` (`census.sh`, about 30 s; `cargo test --release --lib nist_seed_provenance`, 9 tests; `prime_skipped.sh`, about 2 min).

#figure(image("census.svg", width: 100%), caption: [(a) Accepted candidates of each seed run up to its last published seed: prime runs from the 2026-10-16 census, binary runs from this study (`census/summary.json`). (b) Almost-prime Koblitz curves for prime $m in [100, 600]$ with the FIPS thresholds $n >= 2^(2s)$ (`census/koblitz_sweep.txt`).])

== Method
- Candidates (Rust, `examples/nist_binary_seed_candidates.rs`): candidate $i$ is five `lrand48()` words after `srand48(run)`; its X9.62 A.3.3.2 value is $b_0 || "SHA-1"("seed"+1) || dots$, with $b_0$ the rightmost $m - 160 s$ bits of SHA-1(seed).
- Basis (GP, `census.gp`): the value is $b$ in the Gaussian normal basis. The root $beta$ of the Gauss period's minimal polynomial in $"GF"(2)[z]\/(f)$ that maps each published seed's value to the published polynomial-basis $b$ was unique for all five FIPS curves (B-163 included), all with the same bit order. Rotating the basis is a Frobenius, so the orders do not depend on the root.
- Point counting: PARI/GP 2.15.4 `ellcard`. Acceptance: $\#E = 2n$, $n$ prime, MOV degree $>= 2^10$. Checks on every row: $\#E equiv 2 space (mod 4)$, Hasse bound, $[\#E]P = O$ at three random points.
- Koblitz sweep (Rust, `examples/koblitz_degree_sweep.rs`): Lucas recurrence, Miller–Rabin with 20 bases.

== Results (measured)
#table(columns: 6, align: (left, left, right, right, left, right),
  [*run*], [*curve*], [*candidates*], [*checks pass*], [*accepted*], [*published*],
  [446413329], [B-233], [155], [155], [155], [155],
  [446413432], [B-283], [32], [32], [32], [32],
  [446413509], [B-409], [60], [60], [60], [60],
  [446413564], [B-571], [73], [73], [73], [73])
In every binary run the published seed is the *first* accepted candidate; no B-curve candidate was skipped. Positive control: at the published index the pipeline reproduces the published polynomial-basis $b$ and $\#E = 2n$ for all four.

#table(columns: 4, align: (right, right, right, right),
  [*level $s$*], [*need bits(n)*], [*smallest qualifying m*], [*FIPS m*],
  [80], [160], [163], [163], [112], [224], [233], [233],
  [*128*], [*256*], [*277*], [*283*], [192], [384], [409], [409], [256], [512], [571], [571])
The one unexplained Koblitz skip is $y^2 + x y = x^3 + 1$ over $"GF"(2^277)$ ($a = 0$): $\#E = 4n$, $n$ a 275-bit prime, least GNB type 4 (K-283: type 6), no trinomial (as for 163, 283, 571), $a = 0$ like the four large K-curves. ICV1: `icv1-f2m277-t412886371405096939848969796546990392459541-c0810384` (least sparse irreducible modulus; no published polynomial). At $m = 283$, $K_1$ also qualifies with a larger $n$ (283 vs 281 bits); FIPS took $K_0$.

== Prime runs: the 11 skipped candidates (measured)
With $b$ from the 1997 root rule (native `x962_b_root`, which reproduces all ten published $b$), each of the 21 accepted prime-run candidates of the 2026-10-16 census was recounted in GP. In every case the recount equals the census order, $N$ is prime, and $[N]P = O$. The 10 published ones give ICV1 identities already in the registry. The 11 skipped ones, registered by this study:
#table(columns: 3, align: (left, right, left),
  [*run*], [*cand.*], [*ICV1*],
  [p256 871696354], [876], [`icv1-fp256-t105867235723906477692553525193676385009-a67f0bc0`],
  [p224 891607927], [777], [`icv1-fp224-t3901553279810317031433354100437823-678d288d`],
  [p224 891607927], [1485], [`icv1-fp224-t160144979725177297209579645503161-eb89557e`],
  [p224 891607927], [5736], [`icv1-fp224-t3224555599954095973222272472665529-02fd4a93`],
  [p192 863966734], [2198], [`icv1-fp192-t129206357909530666949855705783-7b13329f`],
  [p192 863966734], [5405], [`icv1-fp192-t74833800813696691018643670623-451ee7e6`],
  [p239 863711343], [36], [`icv1-fp239-t1372818672269960337649506790623857933-33e03c6d`],
  [p239 863711343], [1562], [`icv1-fp239-tm212979043306305046992073578916190239-283a4bfa`],
  [p239 863711343], [2491], [`icv1-fp239-tm282982696996081900270513229575094413-9c88d2da`],
  [p239 863711343], [2682], [`icv1-fp239-tm115978852368569857142798214995901191-c8c51d61`],
  [p239 863711343], [2863], [`icv1-fp239-t839727583499860327191657852262033049-6b52f54b`])
The p239 rows are X9.62 `prime239` candidates, not NIST P-curves.

== Derived (not measured)
Prime runs: 21 accepted, 10 published. With each accepted candidate published independently with probability 10/21, four runs all publishing their first has probability $(10\/21)^4 approx 0.05$ — a contrast, not a test. Expected first acceptance index $approx (m-1) ln 2 \/ 2$: 80, 98, 141, 197 for $m$ = 233, 283, 409, 571; observed 155, 32, 60, 73.

== Open
The B-163 seed; binary base points; why $m = 283$ over $m = 277$; the prime-curve skip filter.
