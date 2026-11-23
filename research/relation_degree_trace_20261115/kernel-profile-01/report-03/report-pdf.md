# Native candidate instruction diagnosis

Qualification only: **false**. PCC-20261116-84b178. One whole-process guest-instruction profile; generation/preparation and parent capture excluded. No native time/cycle estimate, speed ratio, A/A admission, confidence interval, new target or degree-of-regularity conclusion. [pcc:unverified]

The complete checked canonical answer equals the frozen uninstrumented candidate. Executable/runtime hashes are unchanged. Total guest Ir: **136731306**, across700 attributed functions. Rust recomputation matches both raw summary/totals, every function row and the separately formatted Collected line from the same profiler. This is producer recomputation, not signed independent execution verification. [pcc:unverified]

Self costs are exclusive and add to total. Inclusive costs include callees, overlap and must not be added across functions. Inlined instructions belong to their containing compiled functions. Startup, libraries, allocation and exact checking remain included; instrumented dispatch can differ from native hardware dispatch. [pcc:unverified]

Function names longer than110 characters are abbreviated in tables; full names and object paths remain in ../report-01/functions.json.

## Largest self costs

| Rank | Function | Self Ir | Fraction of all Ir | Inclusive Ir (overlaps) |
|---:|---|---:|---:|---:|
| 1 | certified_relations::lead | 15864276 | 11.603% | 15864276 |
| 2 | certified_relations::axpy::&lt;alloc::vec::... | 13015229 | 9.519% | 31100449 |
| 3 | &lt;alloc::collections::btree::map::BTreeMa... | 12058373 | 8.819% | 15643193 |
| 4 | free | 7800626 | 5.705% | 12776093 |
| 5 | 0x00000000000a0fe0 | 5960910 | 4.360% | 6216140 |
| 6 | &lt;alloc::vec::Vec&lt;usize&gt; as alloc::... | 5497338 | 4.021% | 7856920 |
| 7 | certified_relations::normal_form | 5060212 | 3.701% | 36675637 |
| 8 | malloc | 5003799 | 3.660% | 8278418 |
| 9 | relation_replay_checker::add_term | 3906321 | 2.857% | 9895887 |
| 10 | 0x00000000000a02e0 | 3436624 | 2.513% | 5523251 |
| 11 | &lt;alloc::collections::btree::map::BTreeMa... | 2840216 | 2.077% | 5027210 |
| 12 | 0x0000000000162880 | 2769251 | 2.025% | 2769251 |
| 13 | 0x00000000000a20c0 | 2445318 | 1.788% | 6461026 |
| 14 | certified_relations::quotient::change_order | 2175309 | 1.591% | 18064266 |
| 15 | &lt;alloc::collections::btree::node::NodeRe... | 2089728 | 1.528% | 3081083 |
| 16 | core::iter::adapters::try_process::&lt;core... | 2078505 | 1.520% | 10789925 |
| 17 | realloc | 1876213 | 1.372% | 8337639 |
| 18 | &lt;serde_json::de::Deserializer&lt;serde_j... | 1811920 | 1.325% | 1958832 |
| 19 | &lt;core::iter::adapters::map::Map&lt;alloc... | 1754488 | 1.283% | 8735127 |
| 20 | &lt;serde_json::de::SeqAccess&lt;_&gt; as s... | 1736736 | 1.270% | 1736736 |
| 21 | core::slice::sort::shared::smallsort::inser... | 1651710 | 1.208% | 1651710 |
| 22 | &lt;core::iter::adapters::map::Map&lt;alloc... | 1613874 | 1.180% | 1613874 |
| 23 | 0x000000000009ed80 | 1561170 | 1.142% | 1832004 |
| 24 | &lt;core::marker::PhantomData&lt;usize&gt; ... | 1488648 | 1.089% | 2844266 |
| 25 | &lt;relation_replay_checker::Arithmetic&gt;... | 1210836 | 0.886% | 23572626 |

## Native constructor and separate checker: largest inclusive counts

| Function | Self Ir | Inclusive Ir (overlaps) |
|---|---:|---:|
| certified_relations::replay::produce_native... | 32 | 98196540 |
| certified_relations::replay::produce_native... | 135 | 98196503 |
| certified_relations::replay::native_product... | 250 | 94433444 |
| certified_relations::quotient::compact::sol... | 36 | 74708459 |
| certified_relations::quotient::compact::sol... | 17313 | 74708394 |
| certified_relations::normal_form | 5060212 | 36675637 |
| relation_replay_checker::verify_json | 35161 | 35609700 |
| certified_relations::axpy::&lt;alloc::vec::... | 13015229 | 31100449 |
| certified_relations::monomials_ordered::emit'2 | 908621 | 28768679 |
| certified_relations::quotient::compact::ver... | 138803 | 26987366 |
| &lt;relation_replay_checker::Arithmetic&gt;... | 1210836 | 23572626 |
| certified_relations::check_reference_impl | 18434 | 20954072 |
| &lt;alloc::vec::into_iter::IntoIter&lt;cert... | 4451 | 19000012 |
| certified_relations::replay::complete_impl | 24462 | 18726715 |
| certified_relations::quotient::change_order | 2175309 | 18064266 |
| certified_relations::replay::quotient_witness | 54000 | 17706950 |
| core::iter::adapters::try_process::&lt;core... | 37 | 17018062 |
| alloc::vec::in_place_collect::from_iter_in_... | 966 | 17018025 |
| certified_relations::lead | 15864276 | 15864276 |
| &lt;alloc::collections::btree::map::BTreeMa... | 348746 | 14668812 |
| certified_relations::quotient::rewrite | 993315 | 13804267 |
| certified_relations::row_shift | 280175 | 12195977 |
| certified_relations::verify | 427590 | 11944898 |
| core::iter::adapters::try_process::&lt;core... | 2078505 | 10789925 |
| relation_replay_checker::add_term | 3906321 | 9895887 |

All functions, object paths, counters, raw call graph and exact answer remain in the raw archive and ../report-01/functions.json. Counts describe this compiled binary/input under Callgrind. They can guide a subsequent explicitly qualified optimization; they do not predict its native gain. [pcc:unverified]

![Largest exclusive guest instruction counts](self.pdf){width=65%}

Numbers on the vertical axis map to the self-cost table.

Presentation correction: report-01 PDF contains overlapping long function names. Its Markdown/JSON counts remain unchanged. Report-02 retains its oversized-figure warning; this final sibling preserves the full evidence and uses shorter labels in its PDF only; no profile or calculator was rerun. [pcc:unverified]
