#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#align(center)[#text(size: 14pt, weight: "bold")[Run report: run-20261018T0320Z-ifma]]
Card CARD-eigen_seta_ifma-20261018-1 (director-compiled, executor-run). eigenx main 580b633 (base) vs 0a8da40 (ifma; native = same with target-cpu=native). Xeon @ 2.10 GHz, 4 vCPU shared KVM with AVX-512 IFMA/VBMI, L0, unpinned; load 0.07/0.77/1.20 at start, 1.18/1.11/1.18 at end. bench_kem L1 3×5, L3 2×3, L5 1×3, one job; 9/9 runs exit 0, all decapsulations agreed.
#table(columns: 4, stroke: 0.4pt, inset: 4pt,
  [decaps], [base], [ifma], [native],
  [L1 (n=15)], [1.050 \[0.987–1.245\] s], [0.635 \[0.598–0.752\] s], [0.656 \[0.581–0.836\] s],
  [L3 (n=6)], [4.989 \[4.799–5.406\] s], [1.605 \[1.582–1.855\] s], [1.413 \[1.344–1.479\] s],
  [L5 (n=3)], [11.069 \[11.051–11.077\] s], [3.345 \[3.325–3.455\] s], [2.954 \[2.819–2.962\] s],
)
All values measured. Keygen per key: L1 base 3.4/7.1/7.9 s, ifma 3.1/6.9/2.7 s; L3 base 73.0/98.0 s, ifma 32.7/24.1 s; L5 base 41.1 s, ifma 28.2 s.
#figure(image("../../figures/decaps.svg", width: 90%), caption: [run-20261018T0320Z-ifma; bars median, dots every sample; measured.])
