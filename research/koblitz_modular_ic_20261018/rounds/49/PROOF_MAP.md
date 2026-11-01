# R49 proof map and concrete obligations

- `Degree49.relative_degree_one`: universal field tower K→L→M with compatible algebra structures; assumes both K-dimensions equal a positive natural d, concludes L-dimension of M is 1. Native ideals, generic degrees and existence of actual fraction fields are not supplied by Lean.
- `Degree49.equal_degree_map_bijective`: under the same hypotheses, concludes the actual intermediate algebra map L→M is bijective. This is the conditional field-theoretic step needed for birationality, not a curve-normalization theorem.
- Native source preparation: both ideal inclusions, eight units and generic degree32 checked by Singular in prep49-02. No Lean bridge to those arrays.
- Native normalization: require selected prime component, finite injective source map, normal target ring and generic degree32. Only then may the conditional tower lemma apply after constructing actual fraction fields and compatible maps. The installed normalP output alone is insufficient for this round's acceptance.
- Affine normalization still leaves the projective boundary, pole divisors, complete prescribed RR spaces, modular-factor map, nonzero order11 action, polarization and direct-relation rank open.

Machine compile/audit receipts and independent semantic review are separate. Review by another session is pending; the producer does not certify its own independence.
