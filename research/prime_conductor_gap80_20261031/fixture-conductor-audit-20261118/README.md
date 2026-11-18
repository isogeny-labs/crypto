# Actual conductor classification of retained maps

This fixed panel separates isogeny degree from the geometric endomorphism-conductor ratio. It certifies one small descending crossing and three horizontal maps, without altering any previous generic output or parameter. The original minimum80bit crossing remains open. [pcc:unverified]

| Fixture | Isogeny degree | Source conductor | Target conductor | Ratio |
|---|---:|---:|---:|---:|
| degree19 | 19 | 1 | 1 | 1 |
| degree10009 | 10009 | 3 | 3 | 1 |
| field202-degree2 | 2 | 1 | 2 | 2 |
| field96-degree5 | 5 | 1 | 1 | 1 |

Native PARI direct Vélu sums verify 112 retained map inputs and 98 matched known scalar pairs. All 6 negative controls reject. Both small-field endpoint cardinalities are recomputed; the large-field counts remain explicitly not recomputed. Large-field order certificates use actual CM endomorphisms/scalings and standard ordinary order theory. [pcc:unverified]

- [Full mathematical derivation](DERIVATION.md), [checked summary](summary.json), [draft transfer assessments](assessments.json).
- [Two-page report](reports/report.pdf), starting with the classification table on page one; editable SVG/Typst/render source is retained.
- [Exact native checker](check-v2.gp), [frozen qualifier](qualify-v2.sh), [input lineage](inputs.sha256), [primary sources](sources.json).

PCC-20261118-d00763 binds native source/input/qualifier receipts and this classification. PCC-20261118-a5aa69 retains the earlier successful version, which left the degree10009 endpoint conductors unknown but correctly classified its edge as horizontal. Both producer attestations verify as not_covered without a signed independent verdict. No formal Lean proof, timing result or official record promotion is asserted. [pcc:unverified]

The v2 refinement counts rational three-torsion using the division polynomial and rational ordinates, then cross-checks group invariant factors. This resolves both degree10009 endpoint conductors exactly. The v1 subject files and hashes are unchanged. No failed native check occurred in this task. [pcc:unverified]

## Reproduction and boundaries

From the repository root, use the pinned PARI executable and a fresh output directory:

```sh
timeout 120 bash research/prime_conductor_gap80_20261031/fixture-conductor-audit-20261118/qualify-v2.sh /workspace/tools/pari/usr/bin/gp FRESH_OUTPUT
```

prepare.py only serializes already recorded metadata. No Python curve arithmetic or logarithm harness is used. check-v2.gp performs all new algebra. The complete quotient uses every nonzero kernel point; infinity/kernel inputs retain their exact exceptional image. Nonzero prime-order subgroup generators, coprime kernel degrees and known scalar targets are checked. [pcc:unverified]

Construction, transfer, conversion, failed work, verification, destination solve, recovery and memory costs are unmeasured. No ECDLP solve, scalar timing, new IC arm or EKS job ran. Existing old recovery/timing records are not changed or promoted. The relevant full criterion still requires an explicit at-least80bit conductor crossing with endpoint conductor/subgroup evidence and matched measured scalar/ECDLP/IC trials. [pcc:unverified]

This audit is not an exhaustive survey of larger CM codomains or a world-record search. The largest ratio in this panel alone is two. The separate4774bit Frobenius-conductor prime is not one of these evaluated crossings. [pcc:unverified]
