# Frozen rank-1 typed-rewiring dose screen

Frozen: 2026-11-01, after the unrestricted typed-null panel and before capped
held-out draws

Status: exact support-overlap and rank diagnostic

## Motivation

The unrestricted typed functional null preserved all registered graph and
predicate invariants but annihilated the left language: 127 of 128 draws had
support zero, one had support one, and no draw matched a natural support
stratum. That result is retained as inconclusive, not interpreted as a rank
effect.

This follow-up freezes a rewiring-dose grid to determine whether a minimally
changed topology can retain support and provide a meaningful rank comparison.

## Frozen panel

Use the same four held-out natural fixtures and source SHA-256
`045eb440e4a4348930b07f63418b2c98dfb8264e1771b7206bbfaa8c2bb4b0b6`.
For each cell, run exact definition-pair caps `1,2,4,8` and eight draws per cap.
Derive each seed from the first eight little-endian bytes of

```text
SHA256("rank1 typed rewire dose v1\0" ||
       field_bits_le64 || pair_cap_le64 || draw_le64).
```

The rewriter processes its seeded admissible-pair order and stops exactly at
the cap. A row is invalid if fewer than the requested number of pairs changes.
Every draw retains support, support stratum, exact quotient tensor, exact best
raw balanced rank, canonical hash, and rewiring receipt.

For each dose and cell, the conditioned row is the first of eight satisfying
the natural `floor(log2(1+support))` stratum. If none matches, report
`conditioned_unmatched`; do not substitute a nearby support.

## Controls

The unrestricted protocol's canonical predicate, degree, interface, pin, cut,
functional replay, natural reproduction, deterministic repeat, mutation and
rank-equivalence controls all remain mandatory. Draw zero at every dose and
cell is independently repeated. The mutation separator
`rank1 typed rewire dose mutation v1` must change the cap-one draw zero.

## Frozen reading

Cap one is primary; larger caps are a prespecified dose-response diagnostic.

- **advance the minimal typed topology mechanism** if cap one has a conditioned
  row in at least three cells and in at least three cells
  `natural_rank / conditioned_rank <= 0.8`;
- **reject the minimal typed topology mechanism** if cap one has a conditioned
  row in at least three cells and at least three rank ratios lie in
  `[0.9,1.1]`;
- otherwise report **inconclusive**.

This remains narrower than the full interior-rewired E1 null and makes no
scaling, runtime, relation-oracle, or ECDLP claim.
