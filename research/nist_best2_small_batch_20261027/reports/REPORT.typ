#set page(paper: "a4", margin: 1.8cm)
#set text(size: 9.5pt)
#set par(justify: true)
= gpu/nist: best of $P + M$ and $P - M$ (`RHO_BEST2`)

Study `research/nist_best2_small_batch_20261027`, branch `research/nist-best2-small-batch-20261027`. Iteration run `iterations-v1` on Modal CPU containers (16 vCPU, shared), 2026-10-27; GPU rate run on an RTX PRO 6000 in progress.

The kernel change computes both images of an ADD step from the one shared inverse (one extra multiplication and squaring) and keeps the one whose $x$ limb 1 scores higher. CPU suites pass with and without it, and the toy-curve end-to-end solve recovers the planted logarithm through the new subtract path.

#figure(image("iterations.svg", width: 100%), caption: [Steps to a useful collision, paired by trial (1,000 per arm), 95% bootstrap; red ticks are the derived predictions ($sqrt(2)$ for the negation map, $sqrt(4\/3)$ for the selection). Rows against "no negation" sit below their prediction by the distinguished-point overhead (64 walks × 256). Source: `iterations-v1/summary*.jsonl`.])

Measured: best of $P plus.minus M$ needs *1.143×* fewer steps than the kernel's negation-map walk (95% CI 1.094–1.194; predicted 1.155). Fruitless 2-cycle escapes rise from 1,931 to 2,570 per $10^6$ steps.

The GPU rate cost decides whether this pays: the change wins at a given batch width W only if it slows the kernel by less than 1.143×. That measurement (W = 16, 32, 64; P-256 and P-384; three alternating repetitions) is running and will be added to this report.
