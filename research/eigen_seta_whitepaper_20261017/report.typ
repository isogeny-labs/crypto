#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#align(center)[#text(size: 15pt, weight: "bold")[Eigen-Séta white paper: study report]\
Evidence for `crypto:papers/eigen-seta-whitepaper/`. Measured on a shared 4-vCPU Xeon @ 2.10 GHz VM (L0); security levels derived.]

= Timings (all cores, medians)
#table(columns: 6, stroke: 0.4pt, inset: 4pt,
  [set], [keygen], [encaps], [decaps], [pk = ct], [sk],
  [L1], [4.6 s], [0.20 s], [1.20 s], [1344 B], [1855 B],
  [L3], [84 s], [1.1 s], [5.8 s], [1992 B], [2730 B],
  [L5], [51 s], [2.3 s], [12.8 s], [2632 B], [3594 B],
  [Q3], [14 min (4 jobs)], [5.9 s], [37 s], [3904 B], [5311 B],
  [Q5], [33 min (4 jobs, 1 key)], [13.7 s], [89 s], [5192 B], [7050 B],
)
Run run-20261017T2022Z-kem-bench, eigenx `d89d790`; every result checked. The attacker's chain test costs 618–670 F#sub[p²] multiplication-equivalents per step by wall time, which includes setup and additions; exact counts give 188.9 F#sub[p²] multiplications per theta step (aburan28/eigenx\#7), below the model's 256, which therefore overcharged the attacker (corrected to 2#super[7.5]).

#figure(image("figures/perf.svg", width: 90%), caption: [Every sample, log scale; measured.])

= Same-host comparison
Decapsulation at level 1: ML-KEM-512 14 µs, McEliece-348864 158 µs, FrodoKEM-640 441 µs, BIKE-L1 458 µs, HQC-128 (portable) 5.1 ms, Eigen-Séta L1 1.2 s, about 89,000 times ML-KEM. liboqs 0.13.0, one pinned core, two repetitions.
#figure(image("figures/compare.svg", width: 100%), caption: [Decapsulation by category, log scale; measured.])

= Security (derived)
L1, L3 and L5 meet categories 1, 3 and 5 only in gate count under MAXDEPTH. At least 2^λ quantum queries holds only for L5 (λ = 128), Q3 and Q5. Largest risk: the hidden orientation becoming effective, which would break every set quantumly.
#figure(image("figures/attack_margins.svg", width: 80%), caption: [Margins over λ by tier; derived by the eigenx cost model.])
