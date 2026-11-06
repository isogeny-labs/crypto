#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#align(center)[#text(size: 15pt, weight: "bold")[Eigen-Séta speed-ups: results report]\
aburan28/eigenx\#5 (merged 580b633). Measured on a shared 4-vCPU Xeon @ 2.10 GHz VM; security unchanged by construction and re-measured.]

= Decapsulation wall time (medians, measured)
#table(columns: 4, stroke: 0.4pt, inset: 4pt,
  [set], [main fc02c6b], [PR \#5], [PR \#5 + target-cpu=native],
  [L1 (n=15)], [1.231 s], [1.070 s (1.15×)], [0.953 s (1.29×)],
  [L3 (n=6)], [5.925 s], [5.024 s (1.18×)], [3.459 s (1.71×)],
  [L5 (n=3)], [13.01 s], [11.58 s (1.12×)], [7.567 s (1.72×)],
)
Run run-20261017T2350Z-speedups (CARD-4). Ratios derived. Key generation is inconclusive (1–3 keys per set, dominated by the random attempt count); the per-attempt basis step measured 226 → 121 ms (2#super[1078]) and 147 → 30 ms (3#super[162]) at L1.
#figure(image("figures/decaps.svg", width: 90%), caption: [Bars median, dots every sample, log scale; measured.])

= Constant time and equivalence (measured)
Callgrind, all threads, identical inputs written by the main build: secret-phase instructions 18.18·10#super[9] → 12.36·10#super[9] (−32.0%); spread over two keys × valid/FO-rejected 286,937 → 2,027 Ir. Shared secrets byte-identical across builds; security_estimate output byte-identical.
#figure(image("figures/ct.svg", width: 95%), caption: [Ir(run) − Ir(prep); measured.])

= Not taken
Hand-written asm field multiply (needs `unsafe`; no faster than a native build). Half-length chains on a smaller prime: feasibility unproven, speed-up 1×–2.6×, torsion margin over log#sub[2] (d·f) would fall from about 540 to about 6 bits (ITEM7_HALF_CHAINS.md).

= Distance to practicality
L1 decapsulation is 0.95–1.07 s; a practical PKE needs tens of milliseconds. Next: the theta chain step (wall-time 618–670 F#sub[p²] mult.-equivalents; exact counts later showed 188.9 per theta step, already at formula cost: aburan28/eigenx\#7), native builds, an AVX-512 IFMA multiplier in safe Rust, Karatsuba and lazy reduction.
