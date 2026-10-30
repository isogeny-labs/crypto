# Current-source N83 finalist bases in S3

The exact K992/root11, K1024/root11 and K3300/wide3 factor bases from the
current batched-root source are now stored under the requested
`s3://crypto-autoresearcher/factor-bases/icv1/etc/` prefix. All three pinned
pilot archives passed a fresh generic-field replay of every signed-Frobenius
base point and label, base cardinality and rank, four-point target relation,
subgroup scalar and producer phase sum. Their source point was already used in
prior research, so these pilot times do not enter a fresh-point selection.

| Base | Points | Canonical raw SHA-256 | S3 gzip SHA-256 | Object bytes |
| --- | ---: | --- | --- | ---: |
| K992/root11 | 164,672 | `0b700b7bbc6264d732d28c60b8e3ee139ecb826653d1063bc16c49b80822c146` | `115391d004768be1108d02176bceef2c9bad982decd74697cfa77a4f2055f3cd` | 3,930,670 |
| K1024/root11 | 169,984 | `6ae95097b960d2a0e5972c2f5a8931cbfd6bd5051f08f8206ead5d9abb0d79cd` | `fd7a79674ed74e12f70de75f702fe5371f692347d5042541a29f80683bdccce2` | 4,058,350 |
| K3300/wide3 | 547,800 | `5759bd2723e06bb53c88a8fd768cb3725404f79f66627817a430f9ea5e9eca66` | `a04f4559cc796ae147517bcc7f7483e4deaac189f62cdc939bc7a8bc6d6f668e` | 13,084,292 |

The [machine receipt](receipt.json) records the full S3 URIs, the exact
candidate manifest SHA-256 values, source commit
`0021730ef93baae79522b848877879e6284d2911`, IC source SHA-256
`4a94e4e570f0cc4e39ba5c0846e411a6c43820c28119f7b9802e549023596507`,
three pilot archive hashes, independent replay source, per-member hashes,
and fresh S3 HEAD and downloaded round-trip checks. The
[custody script](archive.py) performs all arithmetic checks before it makes
any S3 write; existing content-addressed objects are checked and never
overwritten. The three downloaded gzip objects matched their SHA-256 names,
and decompression matched the ordered raw dump hash for each base.

The pilots' one-point full-process observations were 81.741 s for K992,
66.920 s for K1024 and 636.799 s for K3300, as recorded in the pinned
[PILOTS.md on the source branch](https://github.com/aburan28/crypto/blob/0021730ef93baae79522b848877879e6284d2911/research/n83_final_extended_20261018/PILOTS.md).
Those values describe different candidates on one previously exposed point.
The separate 16-point holdout, its calibrations, same-point rho and independent
replay govern any current-source selection. These S3 objects are storage and
identity evidence, not a ranking or a reduction of the fully charged runtime.
