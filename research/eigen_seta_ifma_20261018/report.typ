#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#align(center)[#text(size: 15pt, weight: "bold")[Eigen-Séta: AVX-512 IFMA field multiplication]\
aburan28/eigenx\#6. Measured on a shared 4-vCPU Xeon @ 2.10 GHz VM; parameters, outputs and security estimate unchanged.]

= Decapsulation wall time (medians, measured)
#table(columns: 4, stroke: 0.4pt, inset: 4pt,
  [set], [main 580b633], [IFMA, default build], [IFMA + native],
  [L1 (n=15)], [1.050 s], [0.635 s (1.65×)], [0.656 s (1.60×)],
  [L3 (n=6)], [4.989 s], [1.605 s (3.11×)], [1.413 s (3.53×)],
  [L5 (n=3)], [11.07 s], [3.345 s (3.31×)], [2.954 s (3.75×)],
)
Run run-20261018T0320Z-ifma (director-compiled card, executor-run). Encapsulation main → IFMA: 0.165 → 0.109 s (L1), 1.024 → 0.310 s (L3), 2.498 → 0.880 s (L5). Versus the white-paper run (d89d790): L1 1.9×, L5 3.8× (derived).
#figure(image("figures/decaps.svg", width: 90%), caption: [Bars median, dots every sample, log scale; measured.])

= Constant time (measured)
dudect on L1 decapsulation, 1,000 samples per class: IFMA max |t| 0.87, portable 2.14 (threshold 4.5). Every branch of the IFMA kernels compares counters or lengths only. Results byte-identical to the portable multiplier on all five sets. A field-level fixed-vs-random dudect flags both paths (including the instruction-count-checked portable one) and is not used as evidence.
#figure(image("figures/dudect.svg", width: 75%), caption: [Welch t per crop; measured.])

= Distance to practicality
L1 decapsulation 0.635 s; a practical PKE needs tens of milliseconds. Next: the work around the theta chain (the step itself is at formula cost, aburan28/eigenx\#7), F#sub[p] add/sub (about 16%), the public 3#super[F3] basis (about 9%), multi-core chains.
