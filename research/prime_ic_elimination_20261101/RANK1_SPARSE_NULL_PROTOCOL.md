# Frozen rank-1 support-matched sparse-null screen

Frozen: 2026-11-01, after the natural held-out panel and before generating any
null support

Status: exact quotient-level mechanism falsifier; not the topology-rewired E1 null

## Question

Are the low best balanced ranks in the four held-out natural equality-quotient
tensors lower than expected from support sparsity alone?

This screen preserves each cell's quotient width, raw equality-class
multiplicities, support count, and unit nonzero completion values. It replaces
only the locations of nonzero quotient entries. It does not preserve local
predicates or graph topology, so a positive result merely justifies the harder
topology-rewired null. A negative result rejects only the explanation that the
observed natural rank is special beyond matched sparsity.

## Frozen null generator

For each of the four held-out rows in
`rank1-equality-quotient-heldout.json`, generate 32 null tensors. For every
assignment in `[0,2^q)`, hash the following canonical byte sequence:

```text
"rank1 sparse null v1\0" || field_bits_le64 || draw_le64 || assignment_le64
```

Sort by the 32-byte SHA-256 digest, breaking ties by assignment, and retain the
first `support` assignments. Set each retained value to one and every other
value to zero. This is deterministic sampling without replacement; no retry or
result-conditioned seed is allowed.

Compute the exact minimum raw balanced rank with the same equality-class
symmetry algorithm as the natural panel. The runner must first reconstruct the
natural tensor from its retained nonzero entries and reproduce its recorded
best rank exactly.

Controls:

- all 32 supports in every cell have the exact registered cardinality, no
  duplicates, and only unit values;
- repeated generation of every support is byte-identical;
- replacing the domain separator by `rank1 sparse null mutation v1` changes
  draw zero in every cell;
- the 48 exhaustive raw-rank equivalence controls and direct functional/raw
  fanout control remain green.

## Frozen reading

For each cell sort the 32 null ranks. Use order statistics 8, 16, and 24
(one-indexed) as `Q1`, lower median, and `Q3`. Record

```text
p_lower = (1 + number(null_rank <= natural_rank)) / 33
ratio = natural_rank / lower_null_median.
```

- **advance to topology-rewired null implementation** if at least three cells
  have `number(null_rank <= natural_rank)=0` and `ratio<=0.8`;
- **reject low rank beyond support sparsity** if the natural rank lies in the
  inclusive null interquartile interval in at least three cells;
- otherwise report **inconclusive**.

No multiplicity-adjusted theorem claim is made: this is a fixed four-cell
mechanism gate, and the full topology-preserving natural/null experiment
remains mandatory before any E1 or performance conclusion.
