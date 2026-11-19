# Strong-rho source gate for the fresh n17 card

The [disclosed strong-rho readiness control](rho-readiness-v1/RESULT.md)
recovered one already exposed public point under L0 isolation. It is not a
pre-card source publication and cannot supply the rho arm of a fresh card.

`icprog rho-source-publish` builds `ecbench` offline from the same target-free
F5 source snapshot already sealed for the MatrixF5 arm
(`SOURCE_COMMIT` `bed4864cca41`, seal
`bd4c63599b107f97e13d26735bde87296f7800b04ac5df4120c981421df320ee`). It
archives that source together with the newly built macOS ARM64 `ecbench`
binary and build receipts, then issues a distinct external source seal.
`rho-source-replay` checks every archived member as data and executes no
archived program. The publication retains `target_config_archived=false`,
`target_card_created=false`, zero target worker calls and a null online
speedup.

The committed pre-card publication is under
[rho-source-v1/result-v1](rho-source-v1/result-v1/RESULT.md)
(external seal
`f74601e7be2ef1c7141fee5a997b38c090e0f19eadbe0d7e843577b59918cf59`;
6,958 archive members independently replayed). This gate alone does not
authorize a fresh rho run, mint an IC1/workload ID, or create a point card.
SAT and MatrixF5 source publications are already present; with the incumbent
publication also committed, the card generator remains required before any
four-arm dispatch. Old consumed controllers and the readiness session stay
closed.
