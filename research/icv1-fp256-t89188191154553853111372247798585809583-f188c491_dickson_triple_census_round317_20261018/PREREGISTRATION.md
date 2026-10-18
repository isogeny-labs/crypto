# Round 317 preregistration: triple-arity Dickson atom census on the P-256 base

Date: 2026-10-18.  Status: **planned, not executed.**  This document
freezes the hypothesis, inputs, and gates before any code runs; the
execution lands as a follow-on PR on this branch.

## Open request

Find a genuinely new relation source with certified nonzero rank for the
fixed P-256 factor base, after Rounds 312–314 closed the pair-arity
Dickson census (rank 0, complete at levels 1–7), the linear-character
family (Walsh, rank 0), and multi-target amortization (size-bound).

## Hypothesis

Round 312 censused only **pair** atoms: `P_i + P_j` and `P_i - P_j` for
4,185,775 unordered column pairs at minimal Dickson coalescence levels
1–7, finding zero identities.  The next arity was never measured: for
column triples `(i, j, k)` at a common minimal level, the four signed
images `±P_i ± P_j ± P_k` can collide with each other or with small
multiples `[1,2,3,4,8,16]P_l` of base columns, and each collision is a
homogeneous log relation with two independent sign parameters.

**H1 (yield):** the triple-atom family produces strictly more candidate
relations per examined triple than the pair family's measured zero, at the
toy control scale (order-101 base) where ground truth exists.
**H2 (rank):** on the actual base, a complete low-level triple census
certifies static rank strictly above zero.  **H3 (cost, only if H2
passes):** the per-relation collection cost of the surviving relation set
falls below Round 312's `2^148.080980` FME.

## Frozen inputs

- Curve `icv1-fp256-t89188191154553853111372247798585809583-f188c491`,
  base `FB1h2f8621cda105`, 131,458 columns, exactly as rebuilt from the
  Round 312 dependency set (`round2`, `round19`, `round24`, `round311`
  JSONs, hash-verified).
- Level structure: the verified Dickson chain grouping from Round 312,
  unchanged.
- Signs: all four sign patterns `(+,+,+)`, `(+,+,-)`, `(+,-,+)`,
  `(-,+,+)` (the rest are negations of these and produce the same
  collision structure up to row sign).
- Multiples: `[1,2,3,4,8,16]P_l` tables from Round 312, reused.

## Design

1. **Toy control first (order-101 base, ground truth).**  Extend the
   order-101 control to triples; the complete pair census there found 91
   relations.  The triple census must recover those and any new ones,
   with zero false positives/negatives.  If the toy triple yield per
   triple is not strictly above the pair yield per pair, H1 fails and the
   round closes negative without touching the full base.
2. **Complete low-level cells.**  For each level `L` with a tractable
   triple count (levels where the Round 312 pair count was complete,
   i.e. 1–7), enumerate **all** column triples within the level (and,
   where a level's triples exceed ~2^24, all triples containing at least
   one level-`L` column with the other two drawn from the same level's
   verified sibling domain), compute the four signed atoms, and collide.
   No sampling credit is claimed for any complete cell.
3. **Sampled deeper cells.**  Levels 8–18: 4,096 hash-selected triples
   per level per sign, mirroring Round 312's sampled protocol, labelled
   samples only.
4. **Rank.**  Exact rank of the resulting sparse rows modulo `2^61-1`
   and the P-256 subgroup order, dual-modulus as in Round 312.  A
   certified rank `rho_3 > 0` is H2.
5. **Cost, only if H2 passes.**  Charge every field operation of the
   census; project per-relation collection cost for the surviving
   relation set; compare to `2^148.080980` FME and the `394.425280x`
   free-oracle floor.

## Gates (conjunctive for a positive round)

| gate | condition |
|:--|:--|
| toy control exactness | 0 false positives, 0 false negatives, pair relations recovered |
| H1 yield | toy triple yield per triple > toy pair yield per pair, 95% interval excluding equality |
| H2 rank | certified static rank > 0 on the actual base from complete cells only |
| H3 cost | per-relation collection < `2^148.080980` FME |
| promotion | full-depth P-256 attempt only if H2 and H3 both pass |

Failure of H1 or H2 closes the round as a negative arity-extension result:
pair completeness at levels 1–7 then extends to triples, and the
Dickson-local additive family is exhausted through arity 3 at those
levels.

## What would falsify the direction

If H1 fails at the toy scale, no further P-256 triple work is justified
by this evidence; the next family must change the relation *source*
(e.g. target-coupled or nonlocal identities), not the arity.

## Implementation boundary

A new binary `p256_dickson_triple_rank.rs` extending the Round 312 runner
(reuse its dependency loading, atom records, dual-modulus rank, and
receipts); no change to the registered base or to any frozen round.
