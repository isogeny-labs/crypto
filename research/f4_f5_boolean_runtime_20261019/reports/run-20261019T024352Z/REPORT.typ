#set page(paper: "a4", margin: 15mm, numbering: "1")
#set text(font: "DejaVu Sans", size: 9pt)
#set par(justify: false)
#heading(level: 1)[Fresh native cutoff, typed-entry and solver comparison]
#text("Native F4/F5, F6 and both root-solving families; bounded public Koblitz m2 Boolean fixtures. Final same-host table: run 20261019-024352-boolean-native-policy-ty-fb16d2. Five admitted samples per arm, four primary fixtures, fresh paired planted holdouts231/232. Complete roots and first-witness decomposition have separate boundaries. Every output is checked against complete scalar roots or the original group equation.")


#text("Final host: Intel(R) Xeon(R) Platinum 8488C; rustc 1.99.1 (58cf8f9c3 2026-10-13); reserved CPUs 3,7; c7i.2xlarge on-demand, one Rayon worker, exclusive reserved physical core, isolation receipts and A/A. VM neighbours/frequency remain outside the reservation. Failed and contended attempts are retained, excluded, and counted in each JSON summary.")


#text("n17: icv1-f2m17-tm101-00378d4e,16 Boolean variables,205 base points, group order 131,174. n23: icv1-f2m23-tm5197-1f85e9e1,22 Boolean variables,2071 base points, group order 8,393,806. Exact constructor output is fixture-identities.json. n19 is a 36-variable implementation-limit refusal, not an intermediate-size timing.")


#text("Planted target uses seed230; generator control is the registered generator. The old driver flag random=true does not generate a random target for its single-target invocation. TARGET_LABEL_CORRECTION.md explicitly corrects that earlier label without rewriting frozen inputs or raw records. These cases do not establish an average-case ranking.")


#text("Complete-root runs start from the prepared ANF or JSON representation for that API; system construction and JSON encoding are charged when rebuilt in the decomposition wrapper. Cold decomposition charges fresh target-dependent ANF construction, solving, lifting and verification; Semaev arms additionally charge fresh oracle/table preparation once. Common curve/base/context construction is outside every solver interval. Independent complete-root and final group-audit checks run after elapsed capture for all arms. Instrumented query timings include common driver metadata/wrapper overhead and are not bare-kernel latency. No whole-IC/rho ratio is supplied.")


#text("F6 is compared through the repository's first-witness decomposition APIs; no complete-original-root F6 API is benchmarked here. The original n23 planted system has two Boolean roots and the generator system has none. Exact complete-root output is a separate obligation from returning one group witness.")


#text("Iteration1 generalized JSON improvements borrow input, simplify exact evaluation and integrate the existing SIMD Gray kernel. Same-candidate ablations found SIMD slower at 16 variables and faster at 22; auto now keeps scalar below 22. Fresh confirmation measures that policy and the typed F2BoolPoly entry, which avoids JSON/BigUint conversion while retaining exact counts, output caps, budgets, original coordinates and root verification. This is engineering integration, not a new enumeration algorithm.")


#text("Single-write, SIMD tables, serial row clearing and batched Semaev setup remain opt-in; extra SIMD tables were neutral. The first serial guard regressed one four-worker control; the refined guard requires explicit RAYON_NUM_THREADS=1 and a current one-worker pool. Generalized auto uses scalar below22 variables and wide Gray when available at22 or more. Typed input avoids JSON; exact counts and original-root verification remain enforced. The18-arm costs precede serial-clear experiments and do not claim their complete-root speedup.")


#text("Fresh confirmation retains all outcomes: all 16 typed cells qualify faster than iteration1; n17 prepared JSON roots qualify faster on all four primary/holdout cases. n23 prepared JSON controls are neutral. One n23 planted JSON decomposition control regresses about 3.5% (before/after 0.9654); it is not hidden or described as regression-free. The typed route qualifies about 6x against that frozen wrapper on the same fixture.")


#text("Native comparison/admission, statistics and ratios are computed by the Rust analyzers. Five paired ratios use all 3125 bootstrap resamplings with percentile indices78/3046; qualification additionally requires effect greater than max(2%, twice median absolute A/A noise). Charts show recorded medians and min/max; full CI, individual rounds and rejected attempts remain in JSON/raw archive. Python only renders these prepared values.")


#text("Evidence: initial-summary.json and PROTOCOL.md (run 014036 baseline); optimization-summary.json and OPTIMIZATION_PROTOCOL.md (run 013858 first candidate); final-summary.json and FINAL_PROTOCOL.md (run 014050 Semaev); policy-summary.json, policy-comparison-summary.json and POLICY_PROTOCOL.md (run 024352 confirmation). Source/binary hashes, protocols, commands, every retry, preparation failures and corrected/failed auxiliary Callgrind profiles are retained. Callgrind Ir is not native runtime. See RAW_EVIDENCE.md for archive extraction and immutable job locations.")


#image("../../native-typed-entry.svg", width: 100%)
#pagebreak(weak: true)
#image("../../native-complete-roots.svg", width: 100%)
#pagebreak(weak: true)
#image("../../native-cold-decomposition.svg", width: 100%)
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