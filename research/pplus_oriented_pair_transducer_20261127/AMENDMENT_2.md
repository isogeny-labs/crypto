# Amendment 2: independent level-metric replay

Date frozen: 2026-11-27 UTC, before any admitted depth census.

## Reason

A proof-coverage review after source-manifest v1 found that the independent
verifier reconstructed the complete root pair support, target classifications,
recovered witnesses, and misses, but did not independently recompute the
per-level incidence ranks or semantic circuit totals. Those quantities are
promotion inputs, so source inspection alone is weaker evidence than the
available deterministic replay.

No depth-census candidate output, verifier output, analysis row, or promotion
classification existed when this amendment was written. The checkpoint gate
had stopped the run before release build or execution because checkpoint
transport was unavailable.

## Prospective verifier strengthening

The separately coded Rust verifier now reconstructs every node-pair state at
every tree level by **direct enumeration of the leaves below the two nodes**.
It does not use the candidate's bottom-up union routine. From those direct
states it independently recomputes and compares:

- node-pair rows, nonempty rows, support columns, support entries, and exact
  `GF(2)` incidence rank at every level;
- actual and semantic state counts and entry totals;
- actual and first-representation semantic union-edge counts;
- leaf rank, maximum rank, root-support size, and peak live entries; and
- the previously required complete root digest, 60 fixed queries, canonical
  witnesses, replay, and exact misses.

The audit uses a separately expressed pivot map and a sorted global column
enumeration for rank, rather than importing the candidate rank implementation.
The root state is also cross-checked against the verifier's separate flat
unordered-pair enumeration.

## Unchanged contract and limitation

The candidate algorithm, curve, targets, absence of randomness, operation-cost
panels, subquadratic inequalities, 1.6-million limits, no-cross-target rule,
and stop-before-m=7 rule are unchanged. A second source manifest binds the
strengthened verifier before launch.

This remains transparent deterministic replay, not a zero-knowledge proof,
trusted host attestation, or proof of execution. Implementation-specific field
operation counters are source-instrumented and disclosed but are not
independently reconstructed instruction by instruction. No PCC claim is minted
by this amendment. [pcc:unverified]
