#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#set par(justify: true)

= NIST binary curves: skipped candidates (2026-10-20)

*Status.* Measured census of how the FIPS 186-2 binary curves were chosen. No bearing on ECDLP hardness. Study directory: `research/nist_binary_skipped_20261020/` (`census.sh`, about 30 s; `cargo test --release --lib nist_seed_provenance`, 8 tests).

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

== Derived (not measured)
Prime runs: 21 accepted, 10 published. With each accepted candidate published independently with probability 10/21, four runs all publishing their first has probability $(10\/21)^4 approx 0.05$ — a contrast, not a test. Expected first acceptance index $approx (m-1) ln 2 \/ 2$: 80, 98, 141, 197 for $m$ = 233, 283, 409, 571; observed 155, 32, 60, 73.

== Open
The B-163 seed; binary base points; why $m = 283$ over $m = 277$; the prime-curve skip filter.
