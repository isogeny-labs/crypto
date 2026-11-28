# S3 custody for the replayed N83 cold finalists

The exact K1104/root11 and K1120/root11 factor bases from the sealed
16-point Tier 1 [precision panel](../n83_e2e_precision_20261016/RESULT.md)
are stored under the requested `s3://crypto-autoresearcher/factor-bases/icv1/etc/`
prefix. Each base is one content-addressed gzip object. A fresh download of
both objects matched their compressed SHA-256 digests, and decompression
matched the canonical ordered dump SHA-256 digests. The independent precision
replay admitted all 64 IC arms and 16 same-point rho arms before these objects
were exported. This storage operation is outside every timed IC interval and
does not change the panel's selection status.

| Base | Point count | Ordered raw SHA-256 | Gzip SHA-256 | S3 object |
| --- | ---: | --- | --- | --- |
| K1104/root11 | 183,264 | `aa2821e9c27222a94a0ba03bf1fcd7377ccabad3fdd4cff4046ccd1ca77f593f` | `d81ed95a23c8554d65562bc6801ee48176a6e5f11506b9072786ddd030135676` | `s3://crypto-autoresearcher/factor-bases/icv1/etc/n83_e2e_cold_precision_20261019/a1/objects/d81ed95a23c8554d65562bc6801ee48176a6e5f11506b9072786ddd030135676.jsonl.gz` |
| K1120/root11 | 185,920 | `a1264aef4163f14f11738459fdd6620ca557c55c59e5f1e133fd3aa918b0ff34` | `5b5b08e32eae726cdd25d4c1e71bff841ac82996509010ad5516164c9421d5fa` | `s3://crypto-autoresearcher/factor-bases/icv1/etc/n83_e2e_cold_precision_20261019/a1/objects/5b5b08e32eae726cdd25d4c1e71bff841ac82996509010ad5516164c9421d5fa.jsonl.gz` |

[The machine receipt](receipt.json) binds the provider ZIP SHA-256
`559578cd386693ce3681827ba3a9f192bd99be7ca72920fc35734cf515fd844d`,
source run `38970960376`, source commit
`d13e28e76d3709c19f15cf2c352d4ab912812335`, both exact IC1 candidate
manifests and source base hashes. It enumerates all 64 source dump members:
K1104 and K1120 each occur twice on each of 16 points, and every occurrence
has the same canonical raw digest for its K. The [custody script](archive.py)
checks the independent-replay receipt, provider ZIP bytes, ZIP CRC, candidate
identities, all source dumps, selected object bodies, S3 HEAD metadata, and a
fresh downloaded round-trip. It refuses to overwrite an existing object
whose metadata or downloaded bytes differ.

The 16-point cold comparison measured K1120 145.454 ms faster on average,
with a 95% paired interval of [-92.776, 383.684] ms for K1104 minus K1120.
The interval crosses zero, so the selection remains `BOUNDED_UNKNOWN` and no
minimum across the wider factor-base families is inferred from these objects.
