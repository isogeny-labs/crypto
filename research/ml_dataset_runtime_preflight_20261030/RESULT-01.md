# Prime n23 BSGS worker runtime preflight

The two-arm draft `protocol.json` and both exact configs were filed at Git
commit `66432f3185ea` before either command. The pinned official
`experiment.protocol/v1` checker returned zero findings on the draft. This
is a local pre-freeze feasibility observation, not final protocol execution.
The draft SHA-256 was
`e4ed4467875ddf8529cdac63bad606029098a3d97da048cc9ce0373e6062fdcd`;
the untwisted and twisted config SHA-256 values were
`201d9c295d8416a373dee4342fa0fbfb63e369944618ae47cc857a854ef86f0f`
and `6b7b77d6b88913ac66a79a73d457718a94ab2017ddb1b502585ccd4be29a7851`.

Both calls used retained clean-source binary commit
`c41bec150e16e4c4f9a00634002b633b821de262`, binary SHA-256
`b8e0881ec5e46f60c23e6bbb49ad7c3677b9168199f5787b68d89169bfaaa77e`,
the same curve/key seed and BSGS work limit20,000. They differed only in the
twist flag. The native runner's 90-second cap and one-thread environment were
fixed before launch. The `worker` path includes curve construction, known-answer
and factor-base replay, solver execution and artifact writing. It excludes the
`generate` parent, frozen protocol replay, process-sidecar setup and final
cell sealing; its elapsed times cannot estimate those omitted phases.

| Arm | Monotonic command duration | Terminal result | Registered artifacts | Restricted archive |
| --- | ---: | --- | ---: | --- |
| Prime n23, m2, untwisted BSGS | 7,582,437,679 ns | Exit0; native result `verified`, scalar recovery, known answer and factor base true | 16/16 hashes replayed | SHA-256 `7834d1a8b0840f1e0d033f64108a0fef6196edafa02786a25da0433c60bbc689` |
| Prime n23, m2, twisted BSGS | 916,209,436 ns | Exit0; native result `verified`, scalar recovery, known answer and factor base true | 16/16 hashes replayed | SHA-256 `08d61f39140baea7093058c4867a14febff7480c8c91a3b775c1a63f7da71ddd` |

Each runner receipt records stable binary and config hashes, exact argv,
stdout/stderr hashes, timeout classification and the scoped duration. Every
registered worker artifact passed a separate SHA-256 replay against
`worker-artifacts.json`. The two complete attempt archives, including
restricted witnesses and BSGS tables/probes, were uploaded under the
`restricted/ml-dataset-runtime-preflight-20261030/` store prefix and passed
fresh S3 GET byte comparisons. The archives are custody artifacts; no public
features or dataset export were produced. Verified cloud dataset cells: 0.

Next, obtain observations for every other declared method/family/twist arm
through a matching frozen bootstrap path and measure full `generate` overhead
separately. These two direct-worker times alone cannot fill the final campaign's
runtime estimates or certify broad population coverage.
