# Prime31 public independent-slot mask parity protocol — preregistered 2026-11-18

Status: frozen before execution; preliminary generic group-algebra control only, [pcc:unverified].

## Fixed input
Original ordinary, cofactor-one E/F_(2^31-1): y²=x³+847514846*x+2087444724, n=2147446831, G=(625037715,2100627251). The unchanged native Rust `Arithmetic`, `Point`, `PRIME31` and generator certificate are imported from commit `1df714645aa683cf975bc76fcacb9842cb4780fd` (the same implementation as #4714); Rust 1.100.0. Do not use target logarithms in the oracle.

Use the first B rational x-coordinate points with canonical min(y,p-y), B=8,16,31. For each of 8 positive slots, independently select exactly ceil(B/2) of those B points. Public mask seed `0x4d41534b50333131`, selector: score j in slot i with a single SplitMix64 update from state `MASK_SEED ^ ((i+1)<<32) ^ (j+1)`, sort by (score,j), keep lowest ceil(B/2), return indices in ascending order. No scalar slot weights and no per-point secret data. Residuals are exactly 12 existing public points generated as `[1+splitmix64(state)%(n-1)]G`, state initially `0x57534c4f54503131`; scalar is fixture generation only and not passed to the candidate.

## Algorithm and exact controls
For each B, form two ordered four-slot XOR-parity dictionaries, toggling group sums of every ordered word. Query target R by looking up R-L for each surviving left key and XORing the corresponding right-key matches. Return a concrete eight-slot witness only if parity is odd; verify the original-curve group sum. A known planted eight-summand witness can have even parity and therefore is not an asserted odd-positive control.

Prove repeated *identical* half-sized base in all eight slots still collapses: A^8=Frob_8(A), so its odd-support size is ceil(B/2). Test B=4 with two masked points in each independently selected slot using a separate exhaustive 2^8 ordered-word enumeration compared against the XOR-halves. Record masked chosen indices, parity supports, all 12 target decisions, positive witnesses, setup and query field and group operations, and diagnostic wall clock. The candidate must not use a global 4-sum table from #4714 or any factor-base point discrete logarithms.

## Frozen acceptance/retirement gate
A stage may proceed to wider work only if the exact B4 control and independent Python original-curve replay pass, and if B31 yields at least 4 odd targets out of these *same* 12 and charges fewer than **12,918,455 EC additions** for candidate half-build plus 12 queries (the exact previous weighted B31 control: 1,852,467 setup + 11,065,988 queries). Report all results even if this gate fails; no seed tuning. This comparison is between generic enumeration diagnostics only. The 12 targets are not independent across B, and the gate is not a statistically significant relation-yield finding.

Independently verify original curve, exact mask indices and targets, odd witness group identities, B4 brute-force parity and B8 full half-convolution parity. Source verification and operation counts should be checked by pinned GitHub Actions. Run no more than this 12-target candidate panel in this protocol; do not manufacture 1,024 solved logs from archived witnesses.

## Scope and next distinct research gate
This experiment does **not** implement a nonenumerative extractor; any useful stage result must still be replaced by an RR9/F4 or tensor oracle operating on public residuals with no 4+4 enumeration, then supply rank-31 naturally discovered relations to the existing Rust 1,024-blinded-target complete solver. Benchmark fully charged cold index calculus, shared BSGS and matched optimized rho before any solver-level speed claim.

## Source/novelty review before design
- #4714: fixed-curve **weighted** distinct-slot parity, 7/12 at B31 but 12-target query 11,065,988 additions; current comparable diagnostic.
- #4678: prior mixed-slot / compact-weight study; avoid invention claim.
- #4692: nonenumerative RR9 shared-coefficient extractor remains a separate active lane.
- #4698 and [Bochum public project](https://github.com/bochum-crypto/subexponential-ecdl): independently hashed half-density slots are already part of the recent subexponential proposal. **This fixed finite B mask-control is not an original cryptanalytic method.**
- Semaev 2004/031, Gaudry 2009, Diem 2011: bounded-coordinate decomposition and finite-field index-calculus precedents. Fast prime-field complete solver evidence is not assumed.
- research-KB MCP and CLI are absent in the connector runtime; bounded read of linked primary repository sources and accessible paper summaries, **not a complete novelty survey**.
