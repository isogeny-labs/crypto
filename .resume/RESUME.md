# Resume: crypto @ claude/cool-davinci-szm3om

## Goal
Owner (2026-11-15): establish a rigorous solving degree and growth formula alongside d_ff and d_lf for the ECDLP index-calculus systems, prime (two- and three-summand chains) and binary.

## Spec
Frozen protocol: research/solving_degree_20261115/PROTOCOL.md (predictions P1–P6). Claim: for a unit prime chain instance the static Macaulay solving degree is sd = δ + maxwt(supp g^{-1}), g^{-1} the inverse in F_p[(Z/2^k)^m]; hence d_lf ≤ sd ≤ δ + mk with generic equality.

## State
2026-11-15 02:10Z: four lanes running locally (4 vCPU, no AWS identity): A prime m=2 (brute force k=3,4; inverse scan k=3..12), B prime m=3, C binary static degrees from the ic-bounds-binary records, D proof note SD1 plus red-team. Outputs under research/solving_degree_20261115/lane*/.

## Next
Collect lane RESULTS.md, assemble RESULTS.md and REPORT (research-visuals), copy SD1-solving-degree.md to ml-cryptanalysis docs/proofs on claude/lfd-proofs-20261108, session log, PR with auto-merge.
