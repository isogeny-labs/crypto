# Strong-rho target-free source publication

`icprog rho-source-publish` offline-built `ecbench` from the accepted
target-free F5 source snapshot (`SOURCE_COMMIT` `bed4864cca41`, F5 source
seal `bd4c63599b107f97e13d26735bde87296f7800b04ac5df4120c981421df320ee`)
using publisher commit `45316f0c9ae2`, then archived that source with the
new macOS ARM64 `ecbench` binary and build receipts.

External source seal
`f74601e7be2ef1c7141fee5a997b38c090e0f19eadbe0d7e843577b59918cf59`,
source-manifest SHA-256
`a34f2e9e61c78ab5ae19c75a3b0649dac6f7ae9dd7adf1c497c05da4eb5bae9f`,
ecbench SHA-256
`60fa3f7a042de090faabffcc09af28a59970c6d112320564b0f0621ecd413d47`,
archive SHA-256
`b0f0983a01ad3d7fc113f4e9f7b394890dcf019647c423263c5b2e61639eb05a`.
The [independent replay](independent-replay.json) and
[repository-copy replay](repo-replay.json) each checked all **6,958** archive
members as data with zero archived binary executions, `target_card_created=false`,
and null online speedup.

This is a pre-card custody gate only. It does not authorize a fresh rho run,
mint an IC1/workload ID, or create a point card. The L0 readiness control
remains closed and unpaired.
