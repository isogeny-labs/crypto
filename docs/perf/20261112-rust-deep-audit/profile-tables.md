#### rho-neg32

Total Ir (all parts) 57079114.

| exclusive Ir | % | function |
|---:|---:|---|
| 27049059 | 47.4 | `<TunedWalker<PrimeCurve, NegationClasses>>::walk` |
| 23499347 | 41.2 | `<PrimeCurve>::add_raw` |
| 3647721 | 6.4 | `<TunedWalker<PrimeCurve, NegationClasses>>::remember` |
| 287924 | 0.5 | `_dl_relocate_object` |
| 268239 | 0.5 | `__memcpy_avx_unaligned_erms` |
| 212878 | 0.4 | `_int_malloc` |
| 134072 | 0.2 | `<PrimeCurve>::double_raw` |
| 128671 | 0.2 | `_int_free` |
| 124093 | 0.2 | `__GI___tunables_init` |
| 107588 | 0.2 | `<core::str::iter::SplitWhitespace as core::iter::traits::iterator::Iterator>::next` |
| 95976 | 0.2 | `num_bigint::biguint::monty::montgomery` |
| 94656 | 0.2 | `do_lookup_x` |

| inclusive Ir | % | function |
|---:|---:|---|
| 54444607 | 95.4 | `ecbench::methods::solve_generic::<PrimeCurve>` |
| 54427299 | 95.4 | `rho_walk_with_policy::<PrimeCurve, NegationClasses>` |
| 54302719 | 95.1 | `rho_search_from::<PrimeCurve, NegationClasses>` |
| 54113884 | 94.8 | `<TunedWalker<PrimeCurve, NegationClasses>>::walk` |
| 23499701 | 41.2 | `<PrimeCurve>::add_raw` |
| 583844 | 1.0 | `<ecbench::workload::Workload>::build` |
| 554264 | 1.0 | `<ecbench::Cli as clap_builder::derive::CommandFactory>::command` |
| 553229 | 1.0 | `<ecbench::Cmd as clap_builder::derive::Subcommand>::augment_subcommands` |
| 527726 | 0.9 | `dl_main` |
| 390508 | 0.7 | `<ecbench::workload::Workload>::on` |
| 355451 | 0.6 | `prime_curve_id` |
| 355349 | 0.6 | `curve_id::prime` |

#### kangaroo32

Total Ir (all parts) 58871588.

| exclusive Ir | % | function |
|---:|---:|---|
| 39859052 | 67.7 | `<PrimeCurve>::add_raw` |
| 15994000 | 27.2 | `ecbench::generic::kangaroo_with::<PrimeCurve>` |
| 287924 | 0.5 | `_dl_relocate_object` |
| 266761 | 0.5 | `__memcpy_avx_unaligned_erms` |
| 230324 | 0.4 | `__memcmp_avx2_movbe` |
| 209413 | 0.4 | `_int_malloc` |
| 125727 | 0.2 | `_int_free` |
| 124093 | 0.2 | `__GI___tunables_init` |
| 107588 | 0.2 | `<core::str::iter::SplitWhitespace as core::iter::traits::iterator::Iterator>::next` |
| 95976 | 0.2 | `num_bigint::biguint::monty::montgomery` |
| 94656 | 0.2 | `do_lookup_x` |
| 87732 | 0.1 | `<hashbrown::map::HashMap<u64, (u8, u64), core::hash::BuildHasherDefault<MixHasher>>>::insert` |

| inclusive Ir | % | function |
|---:|---:|---|
| 56257337 | 95.6 | `ecbench::methods::solve_generic::<PrimeCurve>` |
| 56255556 | 95.6 | `ecbench::generic::kangaroo_with::<PrimeCurve>` |
| 583844 | 1.0 | `<ecbench::workload::Workload>::build` |
| 554264 | 0.9 | `<ecbench::Cli as clap_builder::derive::CommandFactory>::command` |
| 553229 | 0.9 | `<ecbench::Cmd as clap_builder::derive::Subcommand>::augment_subcommands` |
| 527726 | 0.9 | `dl_main` |
| 390508 | 0.7 | `<ecbench::workload::Workload>::on` |
| 355451 | 0.6 | `prime_curve_id` |
| 355349 | 0.6 | `curve_id::prime` |
| 297767 | 0.5 | `<ecbench::workload::Instance>::facts` |
| 267733 | 0.5 | `<num_bigint::biguint::BigUint>::modpow` |
| 267727 | 0.5 | `num_bigint::biguint::power::modpow` |

#### bsgs32

Total Ir (all parts) 73910441.

| exclusive Ir | % | function |
|---:|---:|---|
| 32292005 | 43.7 | `<PrimeCurve>::add_raw` |
| 28323658 | 38.3 | `ecbench::generic::bsgs_textbook::<PrimeCurve>` |
| 10556606 | 14.3 | `__memcmp_avx2_movbe` |
| 287924 | 0.4 | `_dl_relocate_object` |
| 266493 | 0.4 | `__memcpy_avx_unaligned_erms` |
| 207337 | 0.3 | `_int_malloc` |
| 131298 | 0.2 | `__memset_avx2_unaligned_erms` |
| 124584 | 0.2 | `_int_free` |
| 124093 | 0.2 | `__GI___tunables_init` |
| 107588 | 0.1 | `<core::str::iter::SplitWhitespace as core::iter::traits::iterator::Iterator>::next` |
| 95976 | 0.1 | `num_bigint::biguint::monty::montgomery` |
| 94656 | 0.1 | `do_lookup_x` |

| inclusive Ir | % | function |
|---:|---:|---|
| 71294655 | 96.5 | `ecbench::methods::solve_generic::<PrimeCurve>` |
| 71292893 | 96.5 | `ecbench::generic::bsgs_textbook::<PrimeCurve>` |
| 32292327 | 43.7 | `<PrimeCurve>::add_raw` |
| 583625 | 0.8 | `<ecbench::workload::Workload>::build` |
| 554264 | 0.7 | `<ecbench::Cli as clap_builder::derive::CommandFactory>::command` |
| 553229 | 0.7 | `<ecbench::Cmd as clap_builder::derive::Subcommand>::augment_subcommands` |
| 527726 | 0.7 | `dl_main` |
| 390580 | 0.5 | `<ecbench::workload::Workload>::on` |
| 355205 | 0.5 | `prime_curve_id` |
| 355103 | 0.5 | `curve_id::prime` |
| 298123 | 0.4 | `<ecbench::workload::Instance>::facts` |
| 268760 | 0.4 | `<ecbench::record::SolveProbe>::read` |

#### ic-igauss

Total Ir (all parts) 165787495.

| exclusive Ir | % | function |
|---:|---:|---|
| 46283071 | 27.9 | `<PrimeCurve>::add_raw` |
| 44013596 | 26.5 | `collect_and_solve_relations_scaled_touching::<PrimeCurve, dyn RelationSolver, std::collections::hash::map::Has…` |
| 13648821 | 8.2 | `<hashbrown::map::HashMap<u64, (), core::hash::BuildHasherDefault<MixHasher>>>::insert` |
| 13385796 | 8.1 | `<ic_measurement::Trace>::boundary` |
| 9275065 | 5.6 | `<PairTable>::get` |
| 9237926 | 5.6 | `<hashbrown::raw::RawTable<(u64, ())>>::reserve_rehash::<hashbrown::map::make_hasher<u64, (), core::hash::Build…` |
| 7065485 | 4.3 | `<PairTable>::probe::<PrimeCurve>` |
| 6692766 | 4.0 | `<plugins::MitmOracle as stages::DecompositionOracle<PrimeCurve>>::decompose_relations` |
| 6320941 | 3.8 | `decompose_mitm::<PrimeCurve>` |
| 4090801 | 2.5 | `clock_gettime@@GLIBC_2.17` |
| 519608 | 0.3 | `__memset_avx2_unaligned_erms` |
| 469602 | 0.3 | `<PrimeCurve>::double_raw` |

| inclusive Ir | % | function |
|---:|---:|---|
| 163053784 | 98.4 | `ecbench::methods::solve_ic_prime` |
| 162245159 | 97.9 | `run_pipeline_with_base::<PrimeCurve>` |
| 46293021 | 27.9 | `<PrimeCurve>::add_raw` |
| 29360494 | 17.7 | `<plugins::MitmOracle as stages::DecompositionOracle<PrimeCurve>>::decompose_relations` |
| 23398086 | 14.1 | `<hashbrown::map::HashMap<u64, (), core::hash::BuildHasherDefault<MixHasher>>>::insert` |
| 22666552 | 13.7 | `decompose_mitm::<PrimeCurve>` |
| 17847732 | 10.8 | `<ic_measurement::Trace>::boundary` |
| 16344118 | 9.9 | `<PairTable>::probe::<PrimeCurve>` |
| 9749265 | 5.9 | `<hashbrown::raw::RawTable<(u64, ())>>::reserve_rehash::<hashbrown::map::make_hasher<u64, (), core::hash::Build…` |
| 1128025 | 0.7 | `<serde_json::value::Value as serde_core::de::Deserialize>::deserialize::<&mut serde_json::de::Deserializer<ser…` |
| 802652 | 0.5 | `<<serde_json::value::Value as serde_core::de::Deserialize>::deserialize::ValueVisitor as serde_core::de::Visit…` |
| 648253 | 0.4 | `<alloc::vec::Vec<(PrimePoint, u64, u64)> as alloc::vec::spec_from_iter::SpecFromIter<(PrimePoint, u64, u64), m…` |

#### ic-igauss32

Total Ir (all parts) 4191154130.

| exclusive Ir | % | function |
|---:|---:|---|
| 1295113893 | 30.9 | `<PrimeCurve>::add_raw` |
| 1067068549 | 25.5 | `collect_and_solve_relations_scaled_touching::<PrimeCurve, dyn RelationSolver, std::collections::hash::map::Has…` |
| 335201828 | 8.0 | `<hashbrown::map::HashMap<u64, (), core::hash::BuildHasherDefault<MixHasher>>>::insert` |
| 327720840 | 7.8 | `<ic_measurement::Trace>::boundary` |
| 304848436 | 7.3 | `<hashbrown::raw::RawTable<(u64, ())>>::reserve_rehash::<hashbrown::map::make_hasher<u64, (), core::hash::Build…` |
| 226771462 | 5.4 | `<PairTable>::get` |
| 172970986 | 4.1 | `<PairTable>::probe::<PrimeCurve>` |
| 163859660 | 3.9 | `<plugins::MitmOracle as stages::DecompositionOracle<PrimeCurve>>::decompose_relations` |
| 154756306 | 3.7 | `decompose_mitm::<PrimeCurve>` |
| 100141074 | 2.4 | `clock_gettime@@GLIBC_2.17` |
| 17272337 | 0.4 | `__memset_avx2_unaligned_erms` |
| 9103359 | 0.2 | `ic_measurement::dump` |

| inclusive Ir | % | function |
|---:|---:|---|
| 4188403214 | 99.9 | `ecbench::methods::solve_ic_prime` |
| 4187317217 | 99.9 | `run_pipeline_with_base::<PrimeCurve>` |
| 1295200333 | 30.9 | `<PrimeCurve>::add_raw` |
| 718409413 | 17.1 | `<plugins::MitmOracle as stages::DecompositionOracle<PrimeCurve>>::decompose_relations` |
| 656817544 | 15.7 | `<hashbrown::map::HashMap<u64, (), core::hash::BuildHasherDefault<MixHasher>>>::insert` |
| 554539785 | 13.2 | `decompose_mitm::<PrimeCurve>` |
| 436961124 | 10.4 | `<ic_measurement::Trace>::boundary` |
| 399773264 | 9.5 | `<PairTable>::probe::<PrimeCurve>` |
| 321615716 | 7.7 | `<hashbrown::raw::RawTable<(u64, ())>>::reserve_rehash::<hashbrown::map::make_hasher<u64, (), core::hash::Build…` |
| 18625672 | 0.4 | `<plugins::MitmOracle as stages::DecompositionOracle<PrimeCurve>>::prepare` |
| 18625379 | 0.4 | `<PairTable>::build_negation_folded::<PrimeCurve>` |
| 16946126 | 0.4 | `<hashbrown::raw::RawTableInner>::fallible_with_capacity::<alloc::alloc::Global>` |

#### rf-mitm-prime

Total Ir (all parts) 311913284.

| exclusive Ir | % | function |
|---:|---:|---|
| 171117503 | 54.9 | `<PrimeCurve>::add_raw` |
| 35899840 | 11.5 | `rootfind::row_points::<PrimeCurve>` |
| 34066450 | 10.9 | `<PairTable>::get` |
| 25954723 | 8.3 | `<PairTable>::probe::<PrimeCurve>` |
| 19220984 | 6.2 | `<rootfind::RootfindMitmOracle<PrimePoint> as stages::DecompositionOracle<PrimeCurve>>::decompose` |
| 16361856 | 5.2 | `<alloc::vec::Vec<PrimePoint> as alloc::vec::spec_extend::SpecExtend<PrimePoint, map::Map<filter::Filter<core::…` |
| 2534529 | 0.8 | `collect_and_solve_relations_scaled_touching::<PrimeCurve, dyn RelationSolver, std::collections::hash::map::Has…` |
| 771180 | 0.2 | `<ic_measurement::Trace>::boundary` |
| 768851 | 0.2 | `<hashbrown::map::HashMap<u64, (), core::hash::BuildHasherDefault<MixHasher>>>::insert` |
| 336462 | 0.1 | `_int_malloc` |
| 298054 | 0.1 | `<hashbrown::raw::RawTable<(u64, ())>>::reserve_rehash::<hashbrown::map::make_hasher<u64, (), core::hash::Build…` |
| 287924 | 0.1 | `_dl_relocate_object` |

| inclusive Ir | % | function |
|---:|---:|---|
| 309193568 | 99.1 | `ecbench::methods::solve_ic_prime` |
| 308381541 | 98.9 | `run_pipeline_with_base::<PrimeCurve>` |
| 299983559 | 96.2 | `<rootfind::RootfindMitmOracle<PrimePoint> as stages::DecompositionOracle<PrimeCurve>>::decompose_relations` |
| 299735489 | 96.1 | `<rootfind::RootfindMitmOracle<PrimePoint> as stages::DecompositionOracle<PrimeCurve>>::decompose` |
| 220486360 | 70.7 | `rootfind::row_points::<PrimeCurve>` |
| 171127485 | 54.9 | `<PrimeCurve>::add_raw` |
| 60026409 | 19.2 | `<PairTable>::probe::<PrimeCurve>` |
| 16363980 | 5.2 | `<alloc::vec::Vec<PrimePoint> as alloc::vec::spec_extend::SpecExtend<PrimePoint, map::Map<filter::Filter<core::…` |
| 1128025 | 0.4 | `<serde_json::value::Value as serde_core::de::Deserialize>::deserialize::<&mut serde_json::de::Deserializer<ser…` |
| 1084359 | 0.3 | `<hashbrown::map::HashMap<u64, (), core::hash::BuildHasherDefault<MixHasher>>>::insert` |
| 1028244 | 0.3 | `<ic_measurement::Trace>::boundary` |
| 802652 | 0.3 | `<<serde_json::value::Value as serde_core::de::Deserialize>::deserialize::ValueVisitor as serde_core::de::Visit…` |

#### rf-mitm-k19

Total Ir (all parts) 18519673.

| exclusive Ir | % | function |
|---:|---:|---|
| 2711061 | 14.6 | `<koblitz_fast::FastCurve>::add_many_clmul` |
| 1693011 | 9.1 | `<PairTable>::get` |
| 1633829 | 8.8 | `<semaev_decomp::Gf2>::batch_inv_clmul` |
| 1590765 | 8.6 | `<semaev_decomp::Gf2>::inv_clmul` |
| 1221404 | 6.6 | `<PairTable>::probe::<BinaryGroup>` |
| 1149370 | 6.2 | `rootfind::row_points::<BinaryGroup>` |
| 901101 | 4.9 | `<rootfind::RootfindMitmOracle<koblitz_fast::FastPoint> as stages::DecompositionOracle<BinaryGroup>>::decompose` |
| 850148 | 4.6 | `<alloc::vec::Vec<PrimePoint> as alloc::vec::spec_extend::SpecExtend<PrimePoint, map::Map<filter::Filter<core::…` |
| 792114 | 4.3 | `crypto_lib::binary_ecc::f2m::reduce_words` |
| 343464 | 1.9 | `__memcpy_avx_unaligned_erms` |
| 314305 | 1.7 | `_int_malloc` |
| 287924 | 1.6 | `_dl_relocate_object` |

| inclusive Ir | % | function |
|---:|---:|---|
| 14207933 | 76.7 | `ecbench::methods::solve_ic_binary` |
| 13216147 | 71.4 | `run_pipeline_with_base::<BinaryGroup>` |
| 12093837 | 65.3 | `collect_and_solve_relations_touching::<BinaryGroup, dyn RelationSolver, run_pipeline_with_base<BinaryGroup>::{…` |
| 11014805 | 59.5 | `<rootfind::RootfindMitmOracle<koblitz_fast::FastPoint> as stages::DecompositionOracle<BinaryGroup>>::decompose…` |
| 11000117 | 59.4 | `<rootfind::RootfindMitmOracle<koblitz_fast::FastPoint> as stages::DecompositionOracle<BinaryGroup>>::decompose` |
| 7167493 | 38.7 | `rootfind::row_points::<BinaryGroup>` |
| 5163914 | 27.9 | `<koblitz_fast::FastCurve>::add_many` |
| 5123786 | 27.7 | `<koblitz_fast::FastCurve>::add_many_clmul` |
| 2929731 | 15.8 | `<PairTable>::probe::<BinaryGroup>` |
| 2411843 | 13.0 | `<semaev_decomp::Gf2>::batch_inv` |
| 2407619 | 13.0 | `<semaev_decomp::Gf2>::batch_inv_clmul` |
| 2171367 | 11.7 | `<ecbench::workload::Workload>::build` |

#### da-f4-k13

Total Ir (all parts) 191166060.

| exclusive Ir | % | function |
|---:|---:|---|
| 21817320 | 11.4 | `pq_f4_f2::echelon_block_tables` |
| 15328763 | 8.0 | `_int_malloc` |
| 12424408 | 6.5 | `_int_free` |
| 11529396 | 6.0 | `pq_groebner_f2::sort_masks_descending` |
| 9418833 | 4.9 | `pq_f4_f2::groebner_basis_f4` |
| 9078543 | 4.7 | `__memset_avx2_unaligned_erms` |
| 7282547 | 3.8 | `<pq_groebner_f2::F2BoolPoly>::mul_mono` |
| 6989534 | 3.7 | `<map::Map<core::slice::iter::Iter<u64>, pq_f4_f2::groebner_basis_f4::{closure#26}> as core::iter::traits::iter…` |
| 6837177 | 3.6 | `<pq_f4_f2::Columns>::pack` |
| 6538707 | 3.4 | `malloc` |
| 5575199 | 2.9 | `free` |
| 5420299 | 2.8 | `<alloc::vec::Vec<usize>>::retain::<pq_f4_f2::echelon_block_tables::{closure#2}::{closure#2}>::{closure#0}` |

| inclusive Ir | % | function |
|---:|---:|---|
| 187743669 | 98.2 | `ecbench::methods::solve_ic_binary` |
| 186748716 | 97.7 | `run_pipeline_with_base::<BinaryGroup>` |
| 186401568 | 97.5 | `collect_and_solve_relations_touching::<BinaryGroup, dyn RelationSolver, run_pipeline_with_base<BinaryGroup>::{…` |
| 186036642 | 97.3 | `<plugins::DescentAlgebraicOracle as stages::DecompositionOracle<BinaryGroup>>::decompose_relations` |
| 186034312 | 97.3 | `<plugins::DescentAlgebraicOracle as stages::DecompositionOracle<BinaryGroup>>::decompose` |
| 175811181 | 92.0 | `<plugins::SolverHarness>::solve` |
| 174103609 | 91.1 | `<solvers::F4F2 as stages::SystemSolver>::solve` |
| 168423884 | 88.1 | `pq_f4_f2::groebner_basis_f4` |
| 35553199 | 18.6 | `<pq_groebner_f2::F2BoolPoly>::mul_mono` |
| 29231326 | 15.3 | `<alloc::vec::Vec<core::option::Option<pq_groebner_f2::F2BoolPoly>> as alloc::vec::spec_from_iter_nested::SpecF…` |
| 29076003 | 15.2 | `<map::Map<core::slice::iter::Iter<u64>, pq_f4_f2::groebner_basis_f4::{closure#26}> as core::iter::traits::iter…` |
| 28524566 | 14.9 | `pq_f4_f2::echelon` |

