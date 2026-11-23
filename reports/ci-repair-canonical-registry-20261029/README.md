# Complete-input canonical registry repair

The unchanged full registry harvester on source `359af1050a756d20403f34e1b1397b4989778c92` reads all required historical inputs without unavailable objects. The complete local replay and the existing hosted metadata workflow produce byte-identical registry and aliases. Hosted run `40905094941`, artifact `12916848042`, ZIP SHA-256 `29303d07b93a23d4bc69a4fe3b101daef39d45727a595e4a1431364c1abb8e9e`; its source marker is verified.

Every old model geometry, identity, alias and representation is preserved. Two existing historical prime-field models are recovered; eight old provenance arrays change. Neither model has a recorded EC1 generator, so that remains unresolved. The earlier partial-clone attempt and older hosted source449 result remain diagnostic history, not the accepted current snapshot.

Native covers, links, Jacobian transfers and traits were regenerated with the original transfer budget200000 and trait budget262144. All20,809 old cover/transfer rows and20,985 old trait rows are exactly unchanged; each catalog adds only the two recovered models. The frozen formula cost model is unchanged. Native currentness checks all pass (covers6.62s, transfers13.58s, traits884.09s); traits generation took763.72s. No measurement, dated cost assessment, performance conclusion or promotion is recomputed.

Sources checked: the existing registry harvester and source inventory, docs/curves/{COVERS,JACOBIAN_TRANSFERS,TRAITS}.md, native generators, exact trait-provider manifest and existing metadata workflow. The original stale-registry check remains a failure in diagnostic runs; failure-only artifact retention does not bypass it. JSON receipts bind hosted/local equality, semantic acceptance, every old native row and actual check commands.

Integration into the existing CI-repair PR2818 and regeneration/checking against any subsequently merged main inputs remain pending. Results above are bound to source359 and must not be relabeled as validation of a newer integrated tree.
