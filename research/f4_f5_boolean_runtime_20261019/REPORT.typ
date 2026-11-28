#set page(paper: "a4", margin: 15mm, numbering: "1")
#set text(font: "DejaVu Sans", size: 9pt)
#set par(justify: false)
#heading(level: 1)[Native F4/F5, F6 and root solvers]
#text("Native F4/F5, F6 and both root-solving families; bounded public Koblitz m2 Boolean fixtures. Final same-host table: run 20261019-024352-boolean-native-policy-ty-fb16d2. Five admitted samples per arm, four primary fixtures, fresh paired planted holdouts231/232. Complete roots and first-witness decomposition have separate boundaries. Every output is checked against complete scalar roots or the original group equation.")


#text("Final host: Intel(R) Xeon(R) Platinum 8488C; rustc 1.99.1 (58cf8f9c3 2026-10-13); reserved CPUs 3,7; c7i.2xlarge on-demand, one Rayon worker, exclusive reserved physical core, isolation receipts and A/A. VM neighbours/frequency remain outside the reservation. Failed and contended attempts are retained, excluded, and counted in each JSON summary.")


#text("n17: icv1-f2m17-tm101-00378d4e,16 Boolean variables,205 base points, group order 131,174. n23: icv1-f2m23-tm5197-1f85e9e1,22 Boolean variables,2071 base points, group order 8,393,806. Exact constructor output is fixture-identities.json. n19 is a 36-variable implementation-limit refusal, not an intermediate-size timing.")


#text("Planted target uses seed230; generator control is the registered generator. The old driver flag random=true does not generate a random target for its single-target invocation. TARGET_LABEL_CORRECTION.md explicitly corrects that earlier label without rewriting frozen inputs or raw records. These cases do not establish an average-case ranking.")


#text("Complete-root runs start from the prepared ANF or JSON representation for that API; system construction and JSON encoding are charged when rebuilt in the decomposition wrapper. Cold decomposition charges fresh target-dependent ANF construction, solving, lifting and verification; Semaev arms additionally charge fresh oracle/table preparation once. Common curve/base/context construction is outside every solver interval. Independent complete-root and final group-audit checks run after elapsed capture for all arms. Instrumented query timings include common driver metadata/wrapper overhead and are not bare-kernel latency. No whole-IC/rho ratio is supplied.")


#text("F6 is compared through the repository's first-witness decomposition APIs; no complete-original-root F6 API is benchmarked here. The original n23 planted system has two Boolean roots and the generator system has none. Exact complete-root output is a separate obligation from returning one group witness.")


#text("The original single-write matrix route qualifies a small native improvement on the primary fixture. Extra SIMD table construction is neutral at both measured sizes and remains experimental. Semaev batch construction preserves every key, first witness, insertion order and abstract group charge; the n23 cold improvement is about 3.6x. Each before/after ratio is from its own same-host experiment; ratios across hosts are never multiplied or pooled.")


#text("Iteration1 generalized JSON improvements borrow input, simplify exact evaluation and integrate the existing SIMD Gray kernel. Same-candidate ablations found SIMD slower at 16 variables and faster at 22; auto now keeps scalar below 22. Fresh confirmation measures that policy and the typed F2BoolPoly entry, which avoids JSON/BigUint conversion while retaining exact counts, output caps, budgets, original coordinates and root verification. This is engineering integration, not a new enumeration algorithm.")


#text("Single-write, SIMD tables, serial row clearing and batched Semaev setup remain opt-in; extra SIMD tables were neutral. The first serial guard regressed one four-worker control; the refined guard requires explicit RAYON_NUM_THREADS=1 and a current one-worker pool. Generalized auto uses scalar below22 variables and wide Gray when available at22 or more. Typed input avoids JSON; exact counts and original-root verification remain enforced. The18-arm costs precede serial-clear experiments and do not claim their complete-root speedup.")


#text("Fresh confirmation retains all outcomes: all 16 typed cells qualify faster than iteration1; n17 prepared JSON roots qualify faster on all four primary/holdout cases. n23 prepared JSON controls are neutral. One n23 planted JSON decomposition control regresses about 3.5% (before/after 0.9654); it is not hidden or described as regression-free. The typed route qualifies about 6x against that frozen wrapper on the same fixture.")


#text("Native comparison/admission, statistics and ratios are computed by the Rust analyzers. Five paired ratios use all 3125 bootstrap resamplings with percentile indices78/3046; qualification additionally requires effect greater than max(2%, twice median absolute A/A noise). Charts show recorded medians and min/max; full CI, individual rounds and rejected attempts remain in JSON/raw archive. Python only renders these prepared values.")


#text("Validation:13 standalone generalized-solver tests, strict Clippy, example build/tests, exact batched-map and binary/prime backend tests. Before the merge, 4,398 nonignored library tests were covered by passing executions: the8-thread suite had one unchanged time-budget failure, which passed serially; a separate serial full run passed4397 before the new map test. Post-merge latest-candidate serial run:4,397 passed,10 failed,118 ignored; nine failures were sandbox-denied network binds and one was an unchanged wall-threshold test. The network suite then passes38/38 with network access and the exact wall test passes serially. All 4,407 nonignored tests are covered by passing executions. The refined explicit-one-worker guard then passes a complete network-enabled serial library run:4,407 passed,0 failed,118 ignored, exit0. Its four worker/switch kernel configurations and SDK/Clippy/example checks pass. Earlier failing and interrupted invocations remain retained with their actual outcomes.")


#text("Limits: no external Magma/msolve/FGb/Singular/PolyBoRi runtime comparison, Arm64/GPU measurement or whole-IC/rho promotion. The generalized native field/domain and full eight-coordinate S3 separator are functionally implemented; these m2 timings do not qualify m5 separator performance or the old20-case/8x hard all-roots research gate. The open problem record remains authoritative.")


#text("Next measured bottlenecks: repeated F4/F5 matrix construction and recursive complete-root solving, plus the remaining native search kernel in typed generalized decomposition. The fresh typed wrapper is close to its prepared-input root cost; conversion/normalization was the dominant avoidable wrapper cost. Preserve output/accounting contracts and qualify any further layout or construction change with the same paired controls. Leaderboard component panels and browser are regenerated; tables A-C, curve facts and the IC/rho progress series are unchanged because the unit/boundary differs.")


#text("Evidence: initial-summary.json and PROTOCOL.md (run 014036 baseline); optimization-summary.json and OPTIMIZATION_PROTOCOL.md (run 013858 first candidate); final-summary.json and FINAL_PROTOCOL.md (run 014050 Semaev); policy-summary.json, policy-comparison-summary.json and POLICY_PROTOCOL.md (run 024352 confirmation). Source/binary hashes, protocols, commands, every retry, preparation failures and corrected/failed auxiliary Callgrind profiles are retained. Callgrind Ir is not native runtime. See RAW_EVIDENCE.md for archive extraction and immutable job locations.")


#text("The initial serial-clear matrix trial qualifies all eight one-worker cells faster but regresses the n17 F4 four-worker control (ratio0.8436). The unrestricted registry-query guard is not regression-free. serial-clear-summary.json preserves all outcomes. The refined source requires explicit one-worker configuration before querying Rayon; its fresh confirmation and direct legacy ablation have a separate protocol, summary and report. The eighteen-arm solver costs above precede both experiments with serial clearing off; they are not silently repriced.")


#text("The fresh explicit-one-worker confirmation qualifies all eight one-worker incremental cells faster; all four four-worker controls are neutral. Primary n23 incremental ratios are1.0802 F4 and1.0730 F5. Direct legacy to candidate ratios on this same host are1.1141 F4 and1.1209 F5, rather than products of earlier experiments. All12cells have five eligible pairs; eleven rejected attempts remain. Activation stays opt-in. These matrix-call gains do not establish a complete-root or decomposition improvement.")


#image("native-single-write.svg", width: 100%)
#pagebreak(weak: true)
#image("native-vector-tables.svg", width: 100%)
#pagebreak(weak: true)
#image("native-generalized-changes.svg", width: 100%)
#pagebreak(weak: true)
#image("native-semaev-setup.svg", width: 100%)
#pagebreak(weak: true)
#image("native-typed-entry.svg", width: 100%)
#pagebreak(weak: true)
#image("native-complete-roots.svg", width: 100%)
#pagebreak(weak: true)
#image("native-cold-decomposition.svg", width: 100%)
#pagebreak(weak: true)
#image("initial-serial-clearing.svg", width: 100%)
#pagebreak(weak: true)
#image("initial-serial-controls.svg", width: 100%)
#pagebreak(weak: true)
#image("native-serial-clearing.svg", width: 100%)
#pagebreak(weak: true)
#image("native-serial-negative.svg", width: 100%)
#pagebreak(weak: true)
#image("native-legacy-to-candidate.svg", width: 100%)
#pagebreak(weak: true)
#heading(level: 2)[n23 complete_roots, planted; five samples, ratio relative to F4]
#table(columns: 4, inset: 4pt, text("Method"), text("Cold ms"), text("n"), text("F4 / method"), text("generalized-native"), text("2.7533"), text("5"), text("131.150"), text("generalized-typed"), text("1.4494"), text("5"), text("248.149"), text("generic-gb"), text("669.8408"), text("5"), text("0.538"), text("roots-gray"), text("2.5987"), text("5"), text("138.912"), text("roots-moebius"), text("34.4747"), text("5"), text("10.448"), text("roots-monica"), text("11.9093"), text("5"), text("30.193"), text("roots-wide"), text("1.4275"), text("5"), text("252.955"), text("solve-f4"), text("360.1913"), text("5"), text("1.000"), text("solve-f5"), text("363.9447"), text("5"), text("0.991"), text("solve-inherited"), text("40.4737"), text("5"), text("8.896"))
#pagebreak(weak: true)
#heading(level: 2)[n23 complete_roots, generator; five samples, ratio relative to F4]
#table(columns: 4, inset: 4pt, text("Method"), text("Cold ms"), text("n"), text("F4 / method"), text("generalized-native"), text("2.7096"), text("5"), text("135.873"), text("generalized-typed"), text("1.4774"), text("5"), text("249.159"), text("generic-gb"), text("449.6532"), text("5"), text("0.818"), text("roots-gray"), text("2.7031"), text("5"), text("136.175"), text("roots-moebius"), text("34.8805"), text("5"), text("10.553"), text("roots-monica"), text("12.0878"), text("5"), text("30.491"), text("roots-wide"), text("1.4900"), text("5"), text("247.049"), text("solve-f4"), text("368.1564"), text("5"), text("1.000"), text("solve-f5"), text("372.7797"), text("5"), text("0.988"), text("solve-inherited"), text("39.3968"), text("5"), text("9.342"))
#pagebreak(weak: true)
#heading(level: 2)[n23 decomposition, planted; five samples, ratio relative to F4]
#table(columns: 4, inset: 4pt, text("Method"), text("Cold ms"), text("n"), text("F4 / method"), text("decomp-f4"), text("26.3691"), text("5"), text("1.000"), text("decomp-f5"), text("26.2727"), text("5"), text("1.004"), text("decomp-f6"), text("7.3539"), text("5"), text("3.610"), text("decomp-f6-pairs"), text("7.3983"), text("5"), text("3.552"), text("decomp-generalized"), text("9.2244"), text("5"), text("2.859"), text("decomp-generalized-typed"), text("1.4781"), text("5"), text("17.840"), text("rootfind-mitm"), text("46.3986"), text("5"), text("0.564"), text("rootfind-table"), text("131.0207"), text("5"), text("0.201"))
#pagebreak(weak: true)
#heading(level: 2)[n23 decomposition, generator; five samples, ratio relative to F4]
#table(columns: 4, inset: 4pt, text("Method"), text("Cold ms"), text("n"), text("F4 / method"), text("decomp-f4"), text("370.4942"), text("5"), text("1.000"), text("decomp-f5"), text("375.1102"), text("5"), text("0.988"), text("decomp-f6"), text("18.6067"), text("5"), text("19.910"), text("decomp-f6-pairs"), text("18.3590"), text("5"), text("20.178"), text("decomp-generalized"), text("9.3417"), text("5"), text("39.691"), text("decomp-generalized-typed"), text("1.4931"), text("5"), text("248.331"), text("rootfind-mitm"), text("45.6419"), text("5"), text("8.117"), text("rootfind-table"), text("130.1315"), text("5"), text("2.853"))
#pagebreak(weak: true)
#heading(level: 2)[Same-host paired changes; different rows are not pooled]
#table(columns: 6, inset: 4pt, text("Change"), text("Target"), text("Before ms"), text("After ms"), text("Ratio"), text("Decision"), text("matrix-f4 single-write"), text("planted"), text("56.0625"), text("53.9084"), text("1.0400"), text("faster"), text("matrix-f5 single-write"), text("planted"), text("58.1545"), text("55.9272"), text("1.0363"), text("faster"), text("Semaev batched cold setup"), text("planted"), text("144.6895"), text("40.1599"), text("3.6050"), text("faster"), text("Semaev batched cold setup"), text("generator"), text("145.1419"), text("40.4301"), text("3.5899"), text("faster"), text("n17 generalized typed entry"), text("planted"), text("2.7787"), text("0.2124"), text("12.9877"), text("faster"), text("n17 generalized typed entry"), text("generator"), text("2.7740"), text("0.2178"), text("12.6796"), text("faster"), text("n23 generalized typed entry"), text("planted"), text("9.1164"), text("1.5370"), text("6.0022"), text("faster"), text("n23 generalized typed entry"), text("generator"), text("9.2590"), text("1.5568"), text("6.0379"), text("faster"), text("initial n17 F4 four-worker control"), text("planted"), text("8.3357"), text("9.8786"), text("0.8436"), text("slower"), text("matrix-f4 guarded serial clearing"), text("planted"), text("53.4171"), text("49.6467"), text("1.0802"), text("faster"), text("matrix-f4 legacy to guarded candidate"), text("planted"), text("54.9114"), text("49.6467"), text("1.1141"), text("faster"), text("matrix-f5 guarded serial clearing"), text("planted"), text("56.6003"), text("52.8025"), text("1.0730"), text("faster"), text("matrix-f5 legacy to guarded candidate"), text("planted"), text("59.2979"), text("52.8025"), text("1.1209"), text("faster"))
#pagebreak(weak: true)
#heading(level: 2)[Fresh cutoff and typed-entry confirmation, all holdouts]
#table(columns: 8, inset: 4pt, text("Arm"), text("n"), text("Seed"), text("Target"), text("Old/JSON"), text("Decision"), text("Old/typed"), text("Decision"), text("generalized-native"), text("17"), text("230"), text("planted"), text("1.2909"), text("faster"), text("4.2625"), text("faster"), text("decomp-generalized"), text("17"), text("230"), text("planted"), text("1.0847"), text("faster"), text("12.9877"), text("faster"), text("generalized-native"), text("17"), text("230"), text("generator"), text("1.2461"), text("faster"), text("4.1444"), text("faster"), text("decomp-generalized"), text("17"), text("230"), text("generator"), text("1.0613"), text("neutral"), text("12.6796"), text("faster"), text("generalized-native"), text("17"), text("231"), text("planted"), text("1.2162"), text("faster"), text("4.2937"), text("faster"), text("decomp-generalized"), text("17"), text("231"), text("planted"), text("1.0607"), text("faster"), text("11.5310"), text("faster"), text("generalized-native"), text("17"), text("232"), text("planted"), text("1.2148"), text("faster"), text("4.1460"), text("faster"), text("decomp-generalized"), text("17"), text("232"), text("planted"), text("1.0568"), text("neutral"), text("11.9363"), text("faster"), text("generalized-native"), text("23"), text("230"), text("planted"), text("0.9744"), text("neutral"), text("1.8596"), text("faster"), text("decomp-generalized"), text("23"), text("230"), text("planted"), text("0.9654"), text("slower"), text("6.0022"), text("faster"), text("generalized-native"), text("23"), text("230"), text("generator"), text("1.0099"), text("neutral"), text("1.8610"), text("faster"), text("decomp-generalized"), text("23"), text("230"), text("generator"), text("1.0151"), text("neutral"), text("6.0379"), text("faster"), text("generalized-native"), text("23"), text("231"), text("planted"), text("1.0243"), text("neutral"), text("1.9342"), text("faster"), text("decomp-generalized"), text("23"), text("231"), text("planted"), text("0.9830"), text("neutral"), text("6.0849"), text("faster"), text("generalized-native"), text("23"), text("232"), text("planted"), text("0.9985"), text("neutral"), text("1.9497"), text("faster"), text("decomp-generalized"), text("23"), text("232"), text("planted"), text("1.0136"), text("neutral"), text("6.0081"), text("faster"))