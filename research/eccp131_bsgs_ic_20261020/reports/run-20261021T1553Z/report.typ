#set page(paper: "a4", margin: (x: 16mm, y: 15mm))
#set text(font: "Libertinus Serif", size: 9pt)
#set par(justify: true)

= Exact-curve bounded BSGS measurement

#text(size: 7.5pt)[21 October 2026 · source `4259adc16b` · Apple M4 Pro, macOS 26.6 · L0 wall observations]

The frozen protocol ran 24 fresh `ecdlp-nist solve` processes: Certicom ECCp-131 and ECC2K-130, interval widths $2^16$ and $2^20$, three rounds, and an identical A/A repeat per cell. Each invocation planted $Q = [12345]G$, used BSGS and one thread, and verified the returned scalar. All 24 completed with exit 0 and correct verification. The exact command, binary hash, raw JSON, resource stream and exit code are retained under `bsgs_bench_20261021/`.

#figure(image("bsgs_process.svg", width: 100%), caption: [All fresh-process wall observations. Circles and squares distinguish identical A/A arms; black segments are medians. Source: `raw/`, recomputed by `plot_bsgs.py`.])

#table(
  columns: (auto, auto, auto, auto, auto, auto),
  align: (left, right, right, right, right, right),
  inset: 3pt,
  [Curve], [bits], [runs], [group ops], [table], [median process ms],
  [ECCp-131], [16], [6], [291], [183], [24.29],
  [ECCp-131], [20], [6], [813], [726], [37.85],
  [ECC2K-130], [16], [6], [291], [183], [26.11],
  [ECC2K-130], [20], [6], [813], [726], [28.12],
)

Median native solver intervals were 7.0, 20.5, 4.0 and 7.0 ms in table order. A/A process differences reached 10.63, 15.64, 5.94 and 9.73 ms respectively; the 61–72 ms first prime/16 pair remains in the raw data. Peak resident memory stayed between 7.47 and 7.72 MiB across all arms. Concurrent host activity and lack of affinity make wall results descriptive. The two curves have different arithmetic and no cross-curve speed ratio is asserted.

These bounded intervals establish correctness on both exact groups at the frozen widths. They do not estimate whole-group challenge cost: no matched full-group rho reference, full-parameter relation corpus, target descent, or published-point scalar is supplied. The full run report is `REPORT.md`; raw values and all A/A pairs are in `summary.json`.
