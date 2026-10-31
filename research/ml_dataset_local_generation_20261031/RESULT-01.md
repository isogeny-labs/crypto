# Local-XFS two-shard generation pilot

The official `experiment.protocol/v1` draft passed the pinned checker with
zero findings and was filed at `84a2e82263e6eab64c201b75423178119dd062cb`
before either affected command. Official freeze at2026-10-31T00:15:08Z set
body SHA-256 `5e99d92dd6c735f036268a665bfec339d299c9837066dde2a5dc010626b50eb1`.
The frozen protocol file SHA-256 is
`a393e4f5910df6635c9d0347d0ac5bd1795764c40f85f9513d91c7a36ddcae3b`;
the native64-cell plan file SHA-256 is
`71653d882931d9bf1b102de3f498eda1155ddd4c7bad104332d16e35ea179680`.
Native preflight confirmed those joins against clean source commit
`c41bec150e16e4c4f9a00634002b633b821de262` and retained binary SHA-256
`b8e0881ec5e46f60c23e6bbb49ad7c3677b9168199f5787b68d89169bfaaa77e`.
Frozen inputs were committed at `e564c3721b92` before generation.

Output was on local XFS under `/var/tmp/ml-dataset-local-generation-20261031`,
with19,412,377,600 available bytes checked before launch. Each selected shard
had one new attempt, `--max-cases 1`, a30-second inner worker cap,90-second
outer cap,2GiB worker memory and64MiB trace cap. No cloud worker or publication
was invoked.

| Shard | Full command duration | Native outcome | Independent replay | Restricted S3 archive |
| --- | ---: | --- | --- | --- |
|62 untwisted |5,381,683,698 ns|Verified BSGS scalar recovery; 1,112 live events|Artifact hashes true; one independently solved ECDLP cell|`c2f7b2eb59f980c479c4e18da61c6240691e8fc8dd5260e7ea69faa0f9e35eba`|
|63 twisted |655,741,689 ns|Verified BSGS scalar recovery; 35 live events|Artifact hashes true; one independently solved ECDLP cell|`622e066a4c1693d8c8c23bbe820196c69e29afe3df3450dfca3f799460a3aaee`|

Each shard also produced a native partial coverage report and strict export:
one general allowlisted feature row, one cyclic-subgroup ECDLP feature row,
and separate restricted target/provenance rows. Both n23 rows were assigned to
the same whole-field-size train component, so these two rows alone do not
establish validation/test coverage. The full run directories, including
witnesses, captured events, solver artifacts, reports and exports, were uploaded
under a restricted S3 prefix and passed fresh GET byte comparisons before
local output was treated as durable. Verified cloud dataset cells:0.

The earlier NFS pilot of the same two plan indices had an untwisted timeout
and a verified twisted cell. These are unmatched operational observations
under different protocol/file-system/host-contention conditions; no storage
speedup or statistical comparison is claimed. The local-XFS path establishes
that two capture-heavy cells can be generated, independently replayed and
durably checked with the declared caps. The other62 plan cells and the
requested broader curve/method matrix remain unattempted by this protocol.
Whole-worker retired instruction counts are still null; phase PMU observations
have narrower scope and were not promoted as whole-worker counts.
