# Joux-Vitse study review qualification

Review date: 2026-10-20 UTC. Comparative conclusions, K1/K2 decisions and exponent qualification are withheld. The historical report and its numerical observations remain available, with the limitations below.

## Retained evidence and replay

The original author commit is `e8d4b4228a5050bd28ec6e14dbb91841ae71143f`. The SHA-256 manifest binds all 856 study and benchmark artifacts (2,755,391 bytes), including the original protocol, raw records, summary, tables, six-page PDF and figures. All nine benchmark PDF/figure copies match their study sources. The 279 raw JSON files contain 444 rows: 80 dense IC, 80 sparse IC, 16 sparse A/A and 268 rho rows. All raw numeric values are finite; all rho rows record correct=true and capped=false. These are recorded producer outcomes, not independent re-execution of the walks.

The original native Rust analyzer reproduces the archived summary exactly, including floating-point values. The corrected native analyzer reads the same frozen rows into this directory's summary.json. It joins by seed identity, records missing/duplicate/invalid rows, reads the K2 observation from R3, applies the actual p>=1009 threshold, retains all 16 A/A seeds, and returns withheld status when control acceptance is unresolved. No scientific producer was run during this review. The original summary and all original raw/PDF/figure bytes remain unchanged.

## Why the comparison is withheld

The frozen positive control specifies R0 S approximately 1.3 without a numeric acceptance tolerance. The rho A/A requirement does not define an executable numeric decision rule. This review does not choose thresholds after seeing the data. All required scheduled seed inventories are present, but that completeness does not supply the missing control criteria. K1, K2 and the exponent qualification therefore remain withheld; historical estimates stay descriptive.

The original rho producer counted the walk hash but omitted the additional distinguished-point hash and two history hashes on negation walks. The source fix now charges those calls. The retained rows were produced before that correction: their S+ and native-price-band totals cannot be presented as complete accounting for the corrected implementation. Raw counts are not retroactively rewritten. The collision fix also distinguishes opposite Y signs when an x-only distinguished-point key matches; a small exact-group regression checks the formerly incorrect scalar calculation. Existing historical rows have not been rerun under either correction.

The protocol deviations record host reboots, killed attempts and runs restarted from the beginning. Completed rows alone do not account for the elapsed work of the lost attempts. The isogeny walk into this weak class was not remeasured in this packet, and the original harness does not emit all F4/linear-algebra additions. These limits also prevent a complete end-to-end cost claim. The class has full rational 2-torsion and a genus-3 cover; it does not include prime-order curves. Wall times compare different thread allocations and are descriptive.

## Validation and publication

The corrected library passed 4,405 release tests with 118 ignored; the log contains the new collision/counting tests. The standalone rho module passed 4 tests, the analyzer passed 7, and the touched jv_cover example compiled and ran its empty test suite. Original native-summary replay had zero structural differences. The current leaderboard consumes the qualified summary and displays the withheld status, while the original PDF, plots and tables remain an immutable historical record. The original prose and layout scripts are copied here before the current entrypoints are qualified. A future prospective protocol and complete accounting would be needed to establish a comparative result.

## Later author correction

The subsequent commit `f462a760ae2f7d8a4a90990dfe3fb3efe5f9588a` is retained in [a separate archive](author-correction-f462a760/README.md). Its native analyzer exactly reproduces its revised summary. The reconstructed omitted-hash accounting is derived from retained counts, while its newly chosen control thresholds do not resolve the frozen-protocol limitation. The current qualified analyzer, original artifact bytes, and withheld status are retained.
