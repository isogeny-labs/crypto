# Supported-characteristic trial: crash and timeout

Frozen protocol hash `e0d22db89b467cb39d7382a624091eb550a04b5f8138ff66776cab2c04305772`;
source/protocol commit `608f40db` precedes both affected calls. Ordinary F65537
exits 139 with a segmentation fault. Saturation reaches the 60-second timeout
and exits 124. Both output files are empty; both frozen analyzer rows are
`not_validated`. No retry or postdata parser change. [pcc:unverified]

`export.json` includes both outcomes and passes canonical file/schema/protocol
checks. Flow: planned 2, attempted 2, completed execution 1, not completed 1,
no exclusions. A completed execution with status `failed` is not a successful
fixture. No native conformance or independent-host execution is established.
The cause of the crash/hang remains open. [pcc:unverified]

This predata amendment preserves the [first version](../f4-saturation-v1/README.md)
and its failures. It corrects cell counts, uses the supported characteristic
dispatch, widens coefficient products, and replaces exhaustive point counting
with four Boolean candidates. No full F65537-square enumeration is claimed.
See the [formal audit](../F4_MODEL.md) and
[PDF report](../reports/proof-20261110-f4-model/report.pdf). [pcc:unverified]
