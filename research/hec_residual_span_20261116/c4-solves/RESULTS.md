# Strong C4 solver comparison

The once-only bounded run at pushed checkpoint `4ed203d724b` exited0. All72calls completed, covering12freshsingle-targetworkloads on all3strongmodels, two source multiples, two targets and two repetitions of all3arms. Every returned scalar matches the held-out expected scalar and original Jacobian scalar multiplication; every repeat vector matches exactly. Folded and expanded walks reached the same first-success relation/trial/scalar. These are local execution/check observations, not signed PCC verdicts. [pcc:unverified]

| p | Folded walk / batched pencil base-field REDC | Folded walk / batched pencil subgroup REDC | Folded walk / batched pencil base-field inversions |
|---:|---:|---:|---:|
|101|2.262–2.902|8.535–18.017|4.080–5.043|
|149|3.623–4.871|15.962–22.072|6.808–8.728|
|197|2.877–4.810|16.178–31.244|6.121–9.928|

Ranges cover every registered source/target case, not selected endpoints. Ratios are computed independently from retained raw vectors by `summarize.rs`; all144modulus-separated records satisfy REDC=2*reduce128+pre+mul_pre and repeated-vector checks. All outcomes remain in CONTROL.tsv. [pcc:unverified]

Arm0isbatchedpencil;1foldedwalk;2expandedwalkwithorbitequations. Both walk arms have identical base-field counts: contraction affects subgroup linear algebra, not their shared relation stream. Expanded/pencil subgroup REDC ranges14.844–43.256. Thus some subgroup components exceed10x while every base-field REDC ratio remains below5x. The tenfold total-cost question is not established by these vectors. Adding p and ell counters with an arbitrary weight would not repair that gap. [pcc:unverified]

All counted factor-base construction, i/lambda derivation, partition, source normalization, collection, elimination and internal scalar check are charged afresh per target. Exact N/ell are supplied public input metadata, rather than recomputed group-order search inside the solve. Source and target construction are outside. Independent expected-scalar replay follows the counter snapshot. Input certification, native initialization internals, integer/bookkeeping/memory work and a full weightedCPU model are outside these vectors; this is a counted arithmetic comparison under the written metadata contract, not a complete cost or wall result. No matched-rho/external-fastest claim follows.

The C4setup uses actual source scalar action and exact N=2ell; no binomial Frobenius identity or hidden source-point index is imported. Classical sieve and automorphism contraction are prior art, and this experiment is their adaptation to the new strong genus4inputs. The new evidence removes weak-cofactor ambiguity and identifies relation-generation arithmetic as the remaining substantial workload. The broader mathematical tenfold genus2+ objective remains active.

Next discriminator: attribute base-field work to common setup, anchor scanning, residual construction and scalar verification, then screen a relation-generation mechanism against this charged baseline. Merely accelerating subgroup elimination cannot explain a tenfold reduction in base-field REDC, since both walk variants already have identical base-field vectors. Retain this as a component result in the research directory; no leaderboard promotion or independent admission.

Git custody: FREEZE.sha256 pins source/dependencies/inputreceipts/binary before execution; RUN.txt pins execution commit; OUTPUT.sha256 pins raw outputs. No S3 access, cloud spend, runtime retries, model substitution or bound extension. Build is warning-free on HomebrewRust1.93.1; no wall timing comparison across toolchains.
