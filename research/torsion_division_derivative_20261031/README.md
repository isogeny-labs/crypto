# H109 — torsion-local division-polynomial derivative oracle

Status: **preregistered; no outcome data generated** (2026-10-31).

This directory is the canonical experiment record for H109. It freezes one
mechanism-distinct question: whether the invariant derivative of the
order-`r` division polynomial at a public target exposes scalar parity through
a constant-size two-power character formula.

- [Frozen protocol](PROTOCOL.md)
- [Transfer/cost assessment](assessment.json)
- [Editable evidence diagram](figures/h109-torsion-derivative-flow.svg)
- [Rendered preregistration report](h109-torsion-derivative-preregistration.pdf)

The paired reference-harness work is proposed in
`isogeny-labs/cryptanalysis` on branch
`research/h109-torsion-derivative`. The lead-atlas control is
[crypto-autoresearcher ECDLP-IDEA-006](https://github.com/isogeny-labs/crypto-autoresearcher/blob/main/ideas/ECDLP-IDEA-006_elliptic_net_short_annihilator_hypothesis.md).

## Frozen workload

Seven prime-order toy curves × four generator relabelings × every nonzero
target = 9,096 target rows, plus 32 symmetry-matched random-label controls.
The formula family has at most `4^5 = 1,024` members per row.

## Claims

No correctness, novelty, or subexponential-speed claim is made by this
preregistration. A positive toy formula still owes generic replication,
end-to-end scalar reconstruction, independent verification, a Lean proof, and
complete setup/advice/online/memory accounting.
