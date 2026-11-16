# Native candidate instruction diagnosis

Qualification only: **false**. PCC-20261116-0e4eac. One whole-process guest-instruction profile; generation/preparation and parent capture excluded. No native time/cycle estimate, speed ratio, A/A admission, confidence interval, new target or degree-of-regularity conclusion. [pcc:unverified]

The complete checked canonical answer equals the frozen uninstrumented candidate. Executable/runtime hashes are unchanged. Total guest Ir: **108491681**, across708 attributed functions. Rust recomputation matches both raw summary/totals, every function row and the separately formatted Collected line from the same profiler. This is producer recomputation, not signed independent execution verification. [pcc:unverified]

Self costs are exclusive and add to total. Inclusive costs include callees, overlap and must not be added across functions. Inlined instructions belong to their containing compiled functions. Startup, libraries, allocation and exact checking remain included; instrumented dispatch can differ from native hardware dispatch. [pcc:unverified]

Function names longer than110 characters are abbreviated in tables; full names and object paths remain in functions.json.

## Largest self costs

| Rank | Function | Self Ir | Fraction of all Ir | Inclusive Ir (overlaps) |
|---:|---|---:|---:|---:|
| 1 | certified_relations::lead | 14074655 | 12.973% | 14074655 |
| 2 | certified_relations::axpy_in_place::&lt;alloc::vec::Vec&lt;usize&gt;&gt; | 12410313 | 11.439% | 19679028 |
| 3 | &lt;alloc::collections::btree::map::BTreeMap&lt;alloc::vec::Vec&lt;usize&gt;, u64&gt;&gt;::insert | 6000997 | 5.531% | 7669120 |
| 4 | certified_relations::normal_form | 4965928 | 4.577% | 13546634 |
| 5 | 0x00000000000a0fe0 | 4794585 | 4.419% | 5005603 |
| 6 | free | 4616315 | 4.255% | 8685979 |
| 7 | &lt;alloc::vec::Vec&lt;usize&gt; as alloc::vec::spec_from_iter_nested::SpecFromIterNested&lt;usize, core::iter::adapter... | 3997532 | 3.685% | 5784539 |
| 8 | relation_replay_checker::add_term | 3480336 | 3.208% | 8341688 |
| 9 | &lt;alloc::collections::btree::map::BTreeMap&lt;alloc::vec::Vec&lt;usize&gt;, u64&gt;&gt;::remove::&lt;alloc::vec::Vec&lt;usize&gt;&gt; | 3099851 | 2.857% | 4701407 |
| 10 | malloc | 2973192 | 2.740% | 5749820 |
| 11 | 0x00000000000a02e0 | 2686305 | 2.476% | 4461805 |
| 12 | certified_relations::quotient::change_order | 2175204 | 2.005% | 17849438 |
| 13 | core::slice::sort::shared::smallsort::insertion_sort_shift_left::&lt;(alloc::vec::Vec&lt;usize&gt;, u64), &lt;[(alloc::... | 1775811 | 1.637% | 1775811 |
| 14 | 0x00000000000a20c0 | 1753206 | 1.616% | 4724604 |
| 15 | &lt;core::iter::adapters::map::Map&lt;alloc::collections::btree::map::Keys&lt;alloc::vec::Vec&lt;usize&gt;, u64&gt;, certifie... | 1715001 | 1.581% | 1715001 |
| 16 | 0x0000000000162880 | 1699545 | 1.567% | 1699545 |
| 17 | core::iter::adapters::try_process::&lt;core::iter::adapters::map::Map&lt;core::iter::adapters::zip::Zip&lt;core::sli... | 1456390 | 1.342% | 7662354 |
| 18 | &lt;serde_json::de::Deserializer&lt;serde_json::read::SliceRead&gt;&gt;::parse_integer | 1358724 | 1.252% | 1505636 |
| 19 | realloc | 1346484 | 1.241% | 6071468 |
| 20 | &lt;serde_json::de::SeqAccess&lt;_&gt; as serde_core::de::SeqAccess&gt;::next_element_seed::has_next_element::&lt;serde_js... | 1327435 | 1.224% | 1327435 |
| 21 | &lt;alloc::collections::btree::node::NodeRef&lt;alloc::collections::btree::node::marker::Owned, alloc::vec::Vec&lt;u... | 1268096 | 1.169% | 2017196 |
| 22 | 0x000000000009ed80 | 1218008 | 1.123% | 1453729 |
| 23 | &lt;core::marker::PhantomData&lt;usize&gt; as serde_core::de::DeserializeSeed&gt;::deserialize::&lt;&mut serde_json::de::D... | 1086666 | 1.002% | 2076479 |
| 24 | certified_relations::quotient::rewrite | 1086562 | 1.002% | 17463383 |
| 25 | &lt;core::iter::adapters::map::Map&lt;alloc::collections::btree::map::Iter&lt;alloc::vec::Vec&lt;usize&gt;, u64&gt;, certifie... | 1052605 | 0.970% | 5390633 |

## Native constructor and separate checker: largest inclusive counts

| Function | Self Ir | Inclusive Ir (overlaps) |
|---|---:|---:|
| certified_relations::replay::produce_native_for_rust_with_kernels | 34 | 78323783 |
| certified_relations::replay::produce_native_for_rust | 135 | 78323744 |
| certified_relations::replay::native_product_impl | 250 | 75713777 |
| certified_relations::quotient::compact::solve_sparse_impl | 36 | 66589421 |
| certified_relations::quotient::compact::solve_impl | 17343 | 66589356 |
| certified_relations::monomials_ordered::emit'2 | 908621 | 28791134 |
| relation_replay_checker::verify_json | 35161 | 27109023 |
| certified_relations::axpy_in_place::&lt;alloc::vec::Vec&lt;usize&gt;&gt; | 12410313 | 19679028 |
| certified_relations::axpy::&lt;alloc::vec::Vec&lt;usize&gt;&gt; | 117720 | 17893600 |
| certified_relations::quotient::change_order | 2175204 | 17849438 |
| &lt;relation_replay_checker::Arithmetic&gt;::identity | 826870 | 17840185 |
| certified_relations::quotient::rewrite | 1086562 | 17463383 |
| certified_relations::quotient::compact::verify_impl | 138803 | 14316795 |
| certified_relations::lead | 14074655 | 14074655 |
| certified_relations::normal_form | 4965928 | 13546634 |
| certified_relations::verify | 417649 | 12265627 |
| &lt;alloc::collections::btree::map::BTreeMap&lt;alloc::vec::Vec&lt;usize&gt;, u64&gt; as core::iter::traits::collect::From... | 197057 | 9760081 |
| certified_relations::quotient::coordinate | 46545 | 9189310 |
| relation_replay_checker::add_term | 3480336 | 8341688 |
| certified_relations::check_reference_impl | 18434 | 8078843 |
| certified_relations::replay::complete_impl | 24462 | 8056140 |
| core::iter::adapters::try_process::&lt;core::iter::adapters::map::Map&lt;core::iter::adapters::zip::Zip&lt;core::sli... | 1456390 | 7662354 |
| certified_relations::replay::quotient_witness | 31596 | 6945208 |
| &lt;alloc::vec::into_iter::IntoIter&lt;certified_relations::Row&gt; as core::iter::traits::iterator::Iterator&gt;::try_... | 4451 | 6516488 |
| core::iter::adapters::try_process::&lt;core::iter::adapters::map::Map&lt;alloc::vec::into_iter::IntoIter&lt;alloc::c... | 37 | 6485783 |

All functions, object paths, counters, raw call graph and exact answer remain in the raw archive and functions.json. Counts describe this compiled binary/input under Callgrind. They can guide a subsequent explicitly qualified optimization; they do not predict its native gain. [pcc:unverified]

![Largest exclusive guest instruction counts](self.pdf){width=65%}

Numbers on the vertical axis map to the self-cost table.
