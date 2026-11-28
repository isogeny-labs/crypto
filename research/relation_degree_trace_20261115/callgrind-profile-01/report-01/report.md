# Native candidate instruction diagnosis

Qualification only: **false**. PCC-20261116-40f7a9. One whole-process guest-instruction profile; generation/preparation and parent capture excluded. No native time/cycle estimate, speed ratio, A/A admission, confidence interval, new target or degree-of-regularity conclusion. [pcc:unverified]

The complete checked canonical answer equals the frozen uninstrumented candidate. Executable/runtime hashes are unchanged. Total guest Ir: **160575923**, across702 attributed functions. Rust recomputation matches both raw summary/totals, every function row and the separately formatted Collected line from the same profiler. This is producer recomputation, not signed independent execution verification. [pcc:unverified]

Self costs are exclusive and add to total. Inclusive costs include callees, overlap and must not be added across functions. Inlined instructions belong to their containing compiled functions. Startup, libraries, allocation and exact checking remain included; instrumented dispatch can differ from native hardware dispatch. [pcc:unverified]

Function names longer than110 characters are abbreviated in tables; full names and object paths remain in functions.json.

## Largest self costs

| Rank | Function | Self Ir | Fraction of all Ir | Inclusive Ir (overlaps) |
|---:|---|---:|---:|---:|
| 1 | certified_relations::lead | 27079477 | 16.864% | 27079477 |
| 2 | certified_relations::axpy::&lt;alloc::vec::Vec&lt;usize&gt;&gt; | 13015229 | 8.105% | 31104566 |
| 3 | &lt;alloc::collections::btree::map::BTreeMap&lt;alloc::vec::Vec&lt;usize&gt;, u64&gt;&gt;::insert | 12058373 | 7.509% | 15629754 |
| 4 | free | 7794604 | 4.854% | 12721151 |
| 5 | 0x00000000000a0fe0 | 5983937 | 3.727% | 6240558 |
| 6 | &lt;alloc::vec::Vec&lt;usize&gt; as alloc::vec::spec_from_iter_nested::SpecFromIterNested&lt;usize, core::iter::adapter... | 5497338 | 3.424% | 7854739 |
| 7 | core::slice::sort::stable::quicksort::quicksort::&lt;alloc::vec::Vec&lt;usize&gt;, &lt;[alloc::vec::Vec&lt;usize&gt;]&gt;::sort_... | 5224280 | 3.253% | 32663999 |
| 8 | certified_relations::normal_form | 5024854 | 3.129% | 46883266 |
| 9 | malloc | 4990525 | 3.108% | 7605282 |
| 10 | core::slice::sort::shared::smallsort::small_sort_general_with_scratch::&lt;alloc::vec::Vec&lt;usize&gt;, &lt;[alloc::ve... | 4761301 | 2.965% | 4761301 |
| 11 | relation_replay_checker::add_term | 3906321 | 2.433% | 9879349 |
| 12 | 0x00000000000a02e0 | 3405683 | 2.121% | 5473801 |
| 13 | 0x0000000000162880 | 3045257 | 1.896% | 3045257 |
| 14 | 0x00000000000a20c0 | 2950128 | 1.837% | 7751761 |
| 15 | &lt;alloc::collections::btree::map::BTreeMap&lt;alloc::vec::Vec&lt;usize&gt;, u64&gt;&gt;::remove::&lt;alloc::vec::Vec&lt;usize&gt;&gt; | 2840216 | 1.769% | 5028135 |
| 16 | realloc | 2254903 | 1.404% | 10007064 |
| 17 | certified_relations::quotient::change_order | 2175309 | 1.355% | 31725948 |
| 18 | &lt;alloc::collections::btree::node::NodeRef&lt;alloc::collections::btree::node::marker::Owned, alloc::vec::Vec&lt;u... | 2089728 | 1.301% | 3081083 |
| 19 | core::iter::adapters::try_process::&lt;core::iter::adapters::map::Map&lt;core::iter::adapters::zip::Zip&lt;core::sli... | 2078505 | 1.294% | 10771191 |
| 20 | &lt;serde_json::de::Deserializer&lt;serde_json::read::SliceRead&gt;&gt;::parse_integer | 1811920 | 1.128% | 1958832 |
| 21 | &lt;core::iter::adapters::map::Map&lt;alloc::collections::btree::map::Iter&lt;alloc::vec::Vec&lt;usize&gt;, u64&gt;, certifie... | 1754488 | 1.093% | 8733653 |
| 22 | &lt;serde_json::de::SeqAccess&lt;_&gt; as serde_core::de::SeqAccess&gt;::next_element_seed::has_next_element::&lt;serde_js... | 1736736 | 1.082% | 1736736 |
| 23 | core::slice::sort::shared::smallsort::insertion_sort_shift_left::&lt;(alloc::vec::Vec&lt;usize&gt;, u64), &lt;[(alloc::... | 1651710 | 1.029% | 1651710 |
| 24 | &lt;core::iter::adapters::map::Map&lt;alloc::collections::btree::map::Keys&lt;alloc::vec::Vec&lt;usize&gt;, u64&gt;, certifie... | 1613874 | 1.005% | 1613874 |
| 25 | 0x000000000009ed80 | 1561666 | 0.973% | 1742313 |

## Native constructor and separate checker: largest inclusive counts

| Function | Self Ir | Inclusive Ir (overlaps) |
|---|---:|---:|
| certified_relations::replay::produce_native_for_rust | 135 | 122082559 |
| certified_relations::replay::native_product_impl | 250 | 118319379 |
| certified_relations::quotient::compact::solve_sparse_impl | 36 | 94063676 |
| certified_relations::quotient::compact::solve_impl | 17313 | 94063611 |
| certified_relations::monomials::recurse'2 | 1147802 | 53042565 |
| certified_relations::normal_form | 5024854 | 46883266 |
| relation_replay_checker::verify_json | 35161 | 35573404 |
| certified_relations::quotient::compact::verify_impl | 138803 | 32677708 |
| core::slice::sort::stable::quicksort::quicksort::&lt;alloc::vec::Vec&lt;usize&gt;, &lt;[alloc::vec::Vec&lt;usize&gt;]&gt;::sort_... | 5224280 | 32663999 |
| certified_relations::quotient::change_order | 2175309 | 31725948 |
| certified_relations::axpy::&lt;alloc::vec::Vec&lt;usize&gt;&gt; | 13015229 | 31104566 |
| certified_relations::lead | 27079477 | 27079477 |
| certified_relations::check_reference_impl | 18434 | 26644983 |
| &lt;alloc::vec::into_iter::IntoIter&lt;certified_relations::Row&gt; as core::iter::traits::iterator::Iterator&gt;::try_... | 4451 | 24653673 |
| &lt;relation_replay_checker::Arithmetic&gt;::identity | 1210836 | 23532453 |
| certified_relations::replay::complete_impl | 24462 | 23253520 |
| certified_relations::replay::quotient_witness | 54000 | 22232514 |
| core::iter::adapters::try_process::&lt;core::iter::adapters::map::Map&lt;alloc::vec::into_iter::IntoIter&lt;alloc::c... | 37 | 21512933 |
| alloc::vec::in_place_collect::from_iter_in_place::&lt;core::iter::adapters::GenericShunt&lt;core::iter::adapters:... | 966 | 21512896 |
| certified_relations::monomials | 316 | 15755066 |
| &lt;alloc::collections::btree::map::BTreeMap&lt;alloc::vec::Vec&lt;usize&gt;, u64&gt; as core::iter::traits::collect::From... | 348746 | 14667316 |
| certified_relations::quotient::rewrite | 993315 | 13807229 |
| certified_relations::row_shift | 280175 | 12195526 |
| core::slice::sort::stable::driftsort_main::&lt;alloc::vec::Vec&lt;usize&gt;, &lt;[alloc::vec::Vec&lt;usize&gt;]&gt;::sort_by&lt;cer... | 168 | 12006135 |
| core::slice::sort::stable::drift::sort::&lt;alloc::vec::Vec&lt;usize&gt;, &lt;[alloc::vec::Vec&lt;usize&gt;]&gt;::sort_by&lt;certif... | 60201 | 12005230 |

All functions, object paths, counters, raw call graph and exact answer remain in the raw archive and functions.json. Counts describe this compiled binary/input under Callgrind. They can guide a subsequent explicitly qualified optimization; they do not predict its native gain. [pcc:unverified]

![Largest exclusive guest instruction counts](self.svg){width=95%}

Numbers on the vertical axis map to the self-cost table.
