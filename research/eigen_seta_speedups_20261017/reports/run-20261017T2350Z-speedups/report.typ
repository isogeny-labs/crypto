#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#align(center)[#text(size: 14pt, weight: "bold")[Run report: run-20261017T2350Z-speedups]]
Card CARD-eigen_seta_speedups-20261017-4 (director-compiled, executor-run). eigenx main fc02c6b (base) vs 1e8b361 (new; native = same with target-cpu=native). Xeon @ 2.10 GHz, 4 vCPU shared KVM, L0, unpinned; load at start 3.00/5.00/5.82, at end 1.32/1.46/2.73. bench_kem L1 3×5, L3 2×3, L5 1×3, one job; 9/9 runs exit 0, all decapsulations agreed.
#table(columns: 4, stroke: 0.4pt, inset: 4pt,
  [decaps], [base], [new], [native],
  [L1 (n=15)], [1.231 \[1.117–1.296\] s], [1.070 \[1.010–1.176\] s], [0.953 \[0.865–1.036\] s],
  [L3 (n=6)], [5.925 \[5.428–6.137\] s], [5.024 \[4.879–5.330\] s], [3.459 \[3.334–3.504\] s],
  [L5 (n=3)], [13.01 \[12.66–13.98\] s], [11.58 \[10.91–11.80\] s], [7.567 \[7.524–8.232\] s],
)
Keygen medians (1–3 keys per set): L1 5.07 / 7.23 / 3.39 s; L3 79.0 / 85.9 / 51.6 s; L5 50.8 / 71.6 / 40.8 s. F#sub[p] mul L1 537 / 568 / 467 ns. All values measured.
#figure(image("../../figures/decaps.svg", width: 90%), caption: [run-20261017T2350Z-speedups; bars median, dots every sample; measured.])
