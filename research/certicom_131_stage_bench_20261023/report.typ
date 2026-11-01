#set page(paper: "us-letter", margin: 0.7in)
#set text(font: "Liberation Sans", size: 10pt)
#set heading(numbering: "1.")

= Certicom 131-bit rho and index-calculus stage benchmark

Run: 2026-10-23 UTC. Source base: `d7daaa6b42e7b5209e78c2407b2eaf35da25fbe4` plus `examples/certicom_131_stage_bench.rs` (SHA-256 `1e406f4cb146fa2739a5b74d93f151645414d46d5a7523bfac525f8c387f7acd`). Five samples per cell. CPU: Intel Xeon 6975P-C, sibling CPUs 14 and 30 reserved.

#image("stage-rates.svg", width: 100%)

== Observed CPU diagnostic

Rates are operations per second. Ranges are the minimum and maximum of five samples. Each rho sample used 200,000 state steps; each S3 sample used 50,000 quadratic-root queries in the exact 131-bit field.

#table(
  columns: (1.2fr, 1.7fr, 1.7fr),
  inset: 6pt,
  table.header([*Curve*], [*Rho state steps/s*], [*S3 root queries/s*]),
  [ECC2K-130], [207,822 (202,805–207,952)], [48,080 (47,517–48,218)],
  [ECC2-131], [207,602 (204,808–207,962)], [48,186 (46,639–48,266)],
  [ECCp-131], [87,055 (86,553–87,249)], [9,522 (9,068–9,531)],
)

The generic binary code has nearly equal stage rates on the two curves over the same field. The ECCp-131 generic CPU rho-step rate is 0.419 times ECC2K-130's; its S3 query rate is 0.198 times ECC2K-130's. These are stage ratios only.

== Measurement and limits

The run took 58.14 seconds, of which 58.12 were user CPU seconds. Other processes used 28.69 CPU seconds across the machine (0.493 CPUs on average). A first run failed the default admission gate: 0.95 CPU seconds during 2 seconds against a 0.20 limit. The reported run used a 0.5-CPU diagnostic threshold and stayed just inside it. *The rates are exploratory, below the repository's strict isolation tier.* All five checksums per cell agree.

The rho unit includes partition selection, one group addition and two scalar coefficient updates for a planted Q = [73]G. The S3 unit includes coefficient construction and quadratic root solving. Distinguished-point storage, cycle escape, collisions, factor-base build, relation yield, linear algebra, descent and logarithm recovery were not measured. Curve generator/subgroup checks and positive S3 controls passed before timing.

== GPU status and evidence

The planned ECC2K-130 GPU job never started: Spot launch returned `MaxSpotInstanceCountExceeded`, and on-demand launch was explicitly denied by the `SpotOnlyEC2Launch` policy. ECC2-131 and ECCp-131 have no matching GPU rho clients in this revision. No fresh GPU rate or full-size end-to-end IC result is reported.

Source records: `PROTOCOL.md`, `diagnostic-samples.csv`, `diagnostic-isolation.jsonl`, `admission.stderr`, `summary.tsv`, `summarize.rs`, and `plot.py` in `research/certicom_131_stage_bench_20261023/`. This report makes no fastest-implementation claim.
