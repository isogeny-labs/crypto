#set page(paper: "a4", margin: 2cm, numbering: "1")
#set text(size: 9.5pt)
#set par(justify: true)
#show heading: set text(size: 11pt)

= R5.3: √élu with a three-prime NTT
Round 5, 2026-10-18. Code `examples/r5_3_ntt_sqrt_velu.rs` and `src/cryptanalysis/isogeny_eval/ntt.rs` (native Rust). Raw records: `threshold.json`, `rational.json`, `generic_karatsuba.json`, `generic_ntt.json`; fits and derived table: `analysis.json` / `analysis.txt`. Full write-up: `RESULTS.md`. Toy, synthetic instances only; no sub-rho or deployed-curve claim.

== Question and hypothesis
Round 4 measured √élu at exponent 0.82 [0.805, 0.838] with Karatsuba. Hypothesis: with NTT products the exponent drops toward 0.5–0.6; the rational-kernel conclusion only strengthens; the generic crossover moves only if kernel finding speeds up.

== Method
Same $j = -3375$ vertical instances (generator ported unchanged; 87/87 rational and 16/16 generic rows reproduce round-4 primes, codomains and find/ladder counts). Unit: M = $FF_p$ multiplication; W = word product mod a 62-bit NTT prime, *stated* at 1 M; I = 91 M. Under `Auto` the library takes the NTT only where its counted cost is below Karatsuba's for that exact shape.

== Results (measured)
#table(columns: 3,
  [series], [range], [exponent [95% CI]],
  [√élu Karatsuba], [$2^16..2^24$], [0.822 [0.805, 0.838]],
  [√élu NTT], [$2^16..2^24$], [0.798 [0.775, 0.820]],
  [√élu NTT], [$2^20..2^24$], [0.745 [0.686, 0.804]],
  [generic find, Karatsuba], [31..863], [2.587 [2.550, 2.625]],
  [generic find, NTT], [31..863], [2.582 [2.544, 2.620]],
)
At $ell = 2^24$ the NTT arm counts 2.21e7 Meq vs 2.45e7 (ratio 1.11).

#figure(image("fig1_sqrtvelu_cost.svg", width: 90%), caption: [√élu counted cost against $ell$, both arms (measured).])
#figure(image("fig2_local_exponent.svg", width: 80%), caption: [Local exponents over 4-octave windows (measured fits).])
#figure(image("fig3_generic.svg", width: 80%), caption: [Generic kernel finding, both arms (measured).])
#figure(image("fig4_threshold.svg", width: 80%), caption: [NTT vs Karatsuba counted cost and contended wall time per product shape.])
#figure(image("fig5_crossover.svg", width: 80%), caption: [Derived (extrapolated) crossovers with rho.])

== Derived crossovers ($g = 6$, log2 $ell^*$)
#table(columns: 4,
  [model], [$r=2^64$], [$r=2^128$], [$r=2^256$],
  [$ell_max = 2 sqrt(p)$], [33], [65], [129],
  [rational, √élu NTT fit ≥ $2^16$], [36.4], [76.5], [156.8],
  [rational, √élu NTT fit ≥ $2^20$], [37.4], [80.4], [166.3],
  [generic, round-4 fit], [9.6], [21.5], [45.7],
  [generic, NTT fit], [9.6], [21.7], [46.4],
)

== Verdict
Exponent toward 0.5–0.6: *refuted at the measured range* (trend downward, log factors dominate up to $2^24$). Rational-kernel conclusion: *confirmed and stronger* (no crossover within $ell <= 2 sqrt(p)$). Generic crossover: *unchanged* ($2^(21.5)$–$2^25$ at $r = 2^128$), since kernel finding did not speed up.

== Limits
W = 1 M is stated, not measured; contended wall shows this NTT costlier per unit. Generic arm capped at $ell = 863$. Crossovers are extrapolations.
