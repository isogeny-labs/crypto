| function | bytes | div/idiv | 128-bit div calls | panic/bounds calls | alloc/free call sites | BTreeMap<String> calls | SipHash calls | mem* calls |
|---|---:|---:|---:|---:|---:|---:|---:|---|
| `<PrimeCurve>::add_raw` | 958 | 13 | 0 | 0 | 0/0 | 0 | 0 | 0 |
| `<TunedWalker<PrimeCurve, NegationClasses>>::walk` | 7030 | 14 | 0 | 0 | 0/0 | 0 | 0 | 0 |
| `ecbench::generic::bsgs_textbook::<PrimeCurve>` | 4004 | 1 | 0 | 0 | 0/0 | 5 | 0 | 2 |
| `ecbench::generic::kangaroo_with::<PrimeCurve>` | 6212 | 7 | 0 | 0 | 0/0 | 9 | 0 | 1 |
| `collect_and_solve_relations_scaled_touching::<PrimeCurve, dyn RelationSolver, std::collect` | 53688 | 10 | 0 | 0 | 0/0 | 65 | 0 | 1 |
| `collect_and_solve_relations_scaled_touching::<PrimeCurve, dyn RelationSolver, GuardLineSet` | 54888 | 10 | 0 | 0 | 0/0 | 63 | 0 | 1 |
| `rootfind::row_points::<PrimeCurve>` | 1133 | 0 | 0 | 0 | 0/0 | 0 | 0 | 0 |
| `<fiber::FiberPairTable>::probe::<PrimeCurve>` | 811 | 0 | 0 | 0 | 0/0 | 0 | 1 | 0 |
| `<PairTable>::probe::<PrimeCurve>` | 501 | 0 | 0 | 0 | 0/0 | 0 | 0 | 0 |
| `<PairTable>::get` | 1218 | 0 | 0 | 0 | 0/0 | 0 | 0 | 0 |
| `<ic_measurement::Trace>::boundary` | 156 | 0 | 0 | 0 | 0/0 | 0 | 0 | 0 |
| `<IncrementalGauss>::add_row` | 2994 | 17 | 0 | 0 | 0/0 | 0 | 0 | 0 |
| `pq_f4_f2::echelon_block_tables` | 10629 | 9 | 0 | 0 | 0/0 | 0 | 0 | 3 |
| `<stages::SolverTotals>::absorb` | 2499 | 0 | 0 | 0 | 0/0 | 1 | 0 | 1 |
