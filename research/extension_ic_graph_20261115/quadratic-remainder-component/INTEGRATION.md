# Integration with the concurrent stage40 publication

While the library checks ran, PR4288 advanced from ddefdb2b to1d6b19ae
through the separate checked prototype and Runpod stage40 publication.
That record now reports class-collision rows contributing to its actual
rank491 endpoint. This component adds a reusable library API and distinct
controls; it does not rerun, reprice or compose that prototype's counts.
Stage40 and its original source, protocol and evidence remain unchanged.
[pcc:unverified]

The incoming change modifies none of the root library's compiled source
or pinned package inputs checked here. The retained
`../validation/quadratic-remainder-incoming-compiled-inputs.diff` is empty.
All three final-source03 hashes still match the integrated library overlay.
The19-test and Clippy logs retain their original run and environment; no
new execution is asserted merely because the documentation was integrated.
[pcc:unverified]

## Preserve both journal histories

The unshared development commit56e327b2 appended two PCC entries to the
common ddefdb2b ledger. The incoming branch also appended entries there.
The integration starts from the incoming canonical ledger and uses the
official claim tool to issue PCC-20261116-dfda61, explicitly referencing
both original development claims. The original claim JSON and exact
two-entry fork are retained without alteration. No canonical entry or
issued proof was edited or rehashed. All11 appended daily-log blocks from
both branches were retained, including multiplicities, in timestamp order.
The canonical ledger's prefix/chain check succeeds locally. [pcc:unverified]

Reconstruct the complete original development ledger, byte for byte:

```sh
git show ddefdb2bf7a2ce9cdf2899e76962baae53ac485b:pcc/ledger.jsonl > /tmp/remainder-common-ledger.jsonl
cat /tmp/remainder-common-ledger.jsonl research/extension_ic_graph_20261115/quadratic-remainder-component/development-ledger-fork.jsonl > /tmp/remainder-development-ledger.jsonl
sha256sum /tmp/remainder-development-ledger.jsonl
```

Compare with the original development ledger digest in
`development-ledger-fork.txt`; the source common commit is reachable from
the incoming publication. The original development branch/commit is also
retained locally. PCC-20261116-dfda61 has unverified standing.

## Remaining scope

The separate [stage40 report](../runpod-extension-remainders-20261116-40/README.md)
supplies its own component evidence. NEXT.md was drafted during this
development work; use it as the library-integration qualification design,
not as a claim that no prototype has measured useful rank. A fresh
library/pipeline comparison still needs a frozen source/cohort, all
original checks and complete accounting for the particular cost claim.
