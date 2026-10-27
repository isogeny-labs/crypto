#set page(paper: "a4", margin: 1.8cm)
#set text(size: 9.5pt)
#set par(justify: true)
= gpu/nist: best of $P + M$ and $P - M$ (`RHO_BEST2`)

Study `research/nist_best2_small_batch_20261027`, branch `research/nist-best2-small-batch-20261027`. Iteration run `iterations-v1` on Modal CPU containers (16 vCPU, shared), 2026-10-27; GPU A/B `gpu-ab-v2` on one RTX PRO 6000 on Modal, 2026-10-27 (commit 5ef52df7d; native driver `gpu/nist/best2_ab.sh`, parsed by `rho_walk_leads gpuab`). It reproduces the earlier Python-parsed `gpu-ab-v1` within 0.003 in every rate ratio. The iteration ratio is a diagnostic from the standalone harness, not an ecbench session.

The kernel change computes both images of an ADD step from the one shared inverse (one extra multiplication and squaring) and keeps the one whose $x$ limb 1 scores higher. CPU suites pass with and without it, and the toy-curve end-to-end solve recovers the planted logarithm through the new subtract path.

#figure(image("iterations.svg", width: 100%), caption: [Steps to a useful collision, paired by trial (1,000 per arm), 95% bootstrap; red ticks are the derived predictions ($sqrt(2)$ for the negation map, $sqrt(4\/3)$ for the selection). Rows against "no negation" sit below their prediction by the distinguished-point overhead (64 walks × 256). Source: `iterations-v1/summary*.jsonl`.])

Measured: best of $P plus.minus M$ needs *1.143×* fewer steps than the kernel's negation-map walk (95% CI 1.094–1.194; predicted 1.155). Fruitless 2-cycle escapes rise from 1,931 to 2,570 per $10^6$ steps.

#figure(image("gpu.svg", width: 100%), caption: [RTX PRO 6000 A/B (`gpu-ab-v2.summary.jsonl`): left, rate of best of $P plus.minus M$ over the plain kernel (median and min–max of 3 alternating repetitions; dotted: break-even $1\/1.143$); right, derived GPU time per solve $1 \/ (1.143 times "rate ratio")$, the interval combining the iteration CI with the rate min–max.])

*Verdict.* The selection costs 8–12% of the rate. Net GPU time per solve is 0.947–0.994 of the plain kernel. The clearest saving is P-256 at W = 64, the kernel’s fastest cell: 0.947 (0.905–0.990). Other cells save 1–3% with intervals crossing 1. This is an engineering gain, not a change of exponent.

*Open observations.*
- With the selection, P-256 distinguished points ran 2–8% below the $2^(-24)$ expectation (0.923, 0.946, 0.974 at W = 16/32/64, about 2–3σ). P-384 and the plain build sit at the expectation. No cause is identified.
- The kernel's cycle-seek iterations add about 0.4% more GPU time for the selection (derived).
- The result stays in this research directory: the validation gate admits a `benchmarks/README.md` row only with a replay certificate and a validator report, and this run has neither yet.
