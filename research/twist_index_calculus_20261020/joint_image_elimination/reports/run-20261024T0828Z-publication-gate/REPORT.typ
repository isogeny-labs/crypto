#set page(paper: "a4", margin: 17mm)
#set text(size: 9pt)
#set par(justify: true)
#show raw: set text(size: 7.6pt)
= #text("Post-merge software and evidence gates all pass")

#text("The managed serial gate finishes every declared phase with exit zero. This is repository integration validation, not a scientific timing result.")


#image("../../figures/publication-software-gates.png", width: 100%)

== #text("Actual results")

#text(size: 6.8pt)[#table(columns: 2, inset: 4pt, text("Gate"), text("Observed outcome"), text("Release library"), text("4,488 passed, zero failed, 119 existing ignores"), text("quartic_twist_s4_integer example"), text("One test passed"), text("quartic_twist_s4_structure example"), text("Three tests passed"), text("Curve identity self-test and naming lint"), text("Both exit zero"), text("Validation-gate tests and base comparison"), text("Both exit zero"), text("Official large-file check"), text("Exit zero; no non-exempt added/modified blob over 10 MiB"), text("Separate public-point native gate"), text("81 passed, zero failed or ignored"))]

#text("The library log reports 1,058.80 seconds inside the unchanged 1,200-second watchdog. That is an unadmitted test duration, not a solver or IC performance comparison. Library and example tests run serially under 8 GiB after separate compilation without the address-space cap. All heavy work uses the existing shared busy reservation.")


#text("The root gate records source label 8f4612602e5e47365f4813a46fe3ea980f6ae458, exact root source hashes, an empty touched-source diff, Rust 1.99.1 and host/commands/stdout/stderr under runs/publication-gate-v1/metadata/. Later study-module and report commits do not alter those root inputs. The separate 81-test native gate belongs to five-point-replay-v1, with its own source and frozen runner; it is not silently folded into the earlier root-gate source label.")


== #text("Merge and scientific boundaries")

#text("The main merge uses ordinary three-way reconciliation of the generated registry and the repository alias serializer. All 15,797 curve identities and every parent alias are retained. The 325 shared records with differing metadata are one-sided main changes; canonical model and identity fields agree. One changed modulus-provenance descriptor is explicitly retained in the semantics addendum. Frozen experiment records and ledgers are not resolved by selecting a side. Unrelated dirty files remain outside commits.")


#text("Passing these gates does not certify a generic S5 ideal, complete prices, strongest-rho comparison, scientific promotion, GitHub CI or merge. The earlier failed and interrupted checks remain intact. No tests or evidence gates were weakened. The existing ready PR requires review and applicable checks on its eventual published head; no merge is inferred here.")
