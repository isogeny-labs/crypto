#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#align(center)[#text(size: 14pt, weight: "bold")[Run report: run-20261017T2022Z-kem-bench]\
Observations only. eigenx `bench_kem` at commit `d89d790`; shared 4-vCPU Xeon @ 2.10 GHz VM, rustc 1.97.0, isolation L0.]

*Command.* `bench_kem SET keys trials jobs`: L1 5 10 1, L3 5 5 1, L5 5 5 1; single core (`taskset -c 0`) L1 2 5 1, L3 1 3 1, L5 1 3 1; Q3 2 3 4; Q5 1 3 4. 20:22–22:10 UTC. All operations completed and every result checked. A LaTeX build, a plotting script, a short ML-KEM run and a cargo download overlapped Q5 key generation only.

#table(columns: 7, stroke: 0.4pt, inset: 4pt,
  [set], [keys], [keygen wall s], [keygen CPU s], [encaps s], [decaps s], [ops],
  [L1], [5], [4.6 [3.6–6.5]], [5.4], [0.204 [0.165–0.304]], [1.20 [1.08–1.80]], [50],
  [L3], [5], [84.1 [76.5–94.0]], [87.8], [1.14 [0.80–1.57]], [5.83 [5.40–6.30]], [25],
  [L5], [5], [50.6 [39.2–80.3]], [60.2], [2.30 [1.74–3.06]], [12.8 [11.8–14.9]], [25],
  [Q3 (jobs 4)], [2], [815 [690–941]], [2592], [5.88 [5.38–6.56]], [37.3 [36.1–38.5]], [6],
  [Q5 (jobs 4)], [1], [2005], [6909], [13.7 [13.5–14.1]], [88.7 [87.8–89.7]], [3],
)
Single core, median: L1 0.240 / 1.88 s, L3 1.06 / 9.61 s, L5 2.94 / 22.2 s (encaps / decaps).

#figure(image("../../figures/perf.svg", width: 95%), caption: [Every sample, log scale; measured.])
