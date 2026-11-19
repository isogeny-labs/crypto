# Pair-table incumbent target-free source publication

`icprog incumbent-source-publish` archived the accepted three-summand
`pairinv` materials extracted from
[`paired-fresh-n17a1/results-20260929/evidence.tar.gz`](../../../paired-fresh-n17a1/results-20260929/evidence.tar.gz)
(SHA-256 `039e535fa73e5c794aeff666bc02989076f04abbf751b5f609884a2785760678`)
using publisher commit `45316f0c9ae2`. The publication retains only the
454-file qualified source, macOS ARM64 worker, build record, build log,
dependency manifest and source preparation. No point config, inventory or
result from the old job was archived.

External source seal
`36923bdb0110841fef8c481b941f134c87076244ed20440684bae3d385c6e6bf`,
source-manifest SHA-256
`240b8daa0478aacb1f6fc248de9fd5ed9b0773c59d3ae869ced0bab8be438378`,
worker SHA-256
`b3d8afacc746a34cf15562452dbec3d75deadbb9faf16051e97ad8fa07cbbd66`,
archive SHA-256
`3855b216a927fb4ee43ab408187ee190f11048e171e99fbf8ce0cdb7a49b8bf3`.
The [independent replay](independent-replay.json) and
[repository-copy replay](repo-replay.json) each checked all **459** archive
members as data with zero archived binary executions, `target_card_created=false`,
and null online speedup.

The matching [source descriptor](source-descriptor.json) asserts the same
seal, source-manifest and archive digests under
`fresh-paired-n17-source-publication-v1` for the public-point card generator.
This remains a pre-card custody gate only: it does not mint an IC1 identity
or admit a four-arm dispatch.
