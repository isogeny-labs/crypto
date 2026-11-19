# Pair-table incumbent source gate

The incumbent for the fresh n17 campaign is the accepted three-summand
`pairinv` partial pair-table pipeline, not a new approximation of it. Its
historical local macOS ARM64 build is retained in
[`paired-fresh-n17a1/results-20260929/evidence.tar.gz`](../paired-fresh-n17a1/results-20260929/evidence.tar.gz),
SHA-256 `039e535fa73e5c794aeff666bc02989076f04abbf751b5f609884a2785760678`.
The selected materials are exactly `pairinv-source/`,
`pairinv-preparation.json`, and the `incumbent/` worker, build record, build
log and dependency manifest. Its old point-bearing job, inventory and result
are deliberately excluded from this new publication. The prior disclosed
point remains exposed and cannot be a new card.

`icprog incumbent-source-publish` accepts a directory containing only the
454-file qualified source as `source/`, plus `worker`, `build-record.json`,
`build.log`, `dependency-manifest.json` and `source-preparation.json`. Before
copying anything it verifies each source file against the accepted manifest
`240b8daa0478aacb1f6fc248de9fd5ed9b0773c59d3ae869ced0bab8be438378`,
the local worker hash
`b3d8afacc746a34cf15562452dbec3d75deadbb9faf16051e97ad8fa07cbbd66`,
the pinned offline build policy, dependencies, preparation and build log. It
checks that the executable is macOS ARM64, then archives only these immutable
inputs and issues a distinct external source seal. `incumbent-source-replay`
checks all archive members as data and executes no archived program.

The target-free descriptor may enter the four-arm card only after this new
publication is committed and replayed from the repository copy. Source
custody does not authorize a target run: the native one-use incumbent controller,
post-card registration, original audit, same-point pairing and isolation gate
remain separate requirements. The incumbent's actual factor base and folded
columns must be re-inventoried from this exact worker before a new IC1 ID is
assigned; the historical n17 inventory had 272 usable points and eight
columns, while F5/SAT use a different base policy.

The committed pre-card publication is under
[incumbent-source-v1/result-v1](incumbent-source-v1/result-v1/RESULT.md)
(external seal
`36923bdb0110841fef8c481b941f134c87076244ed20440684bae3d385c6e6bf`;
459 archive members independently replayed). Earlier local
uncommitted-publisher validation also passed data-only replay of all 459
archived regular files with zero worker calls, including archive-only and
tamper negatives; those controls checked the gate and do not replace the
committed publication above.
