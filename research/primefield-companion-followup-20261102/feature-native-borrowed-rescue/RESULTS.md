# Borrowed interval ownership — T-1881

Producer observations; independent signed verdict pending. [pcc:unverified]

## Attempts

- validation01: compiler exit 1, E0106 (missing lifetime) in reciprocal-control.rs. Native checks did not run. Changed source and generator plus source hashes are retained.
- validation02: compiler and native exit 0 after the owned control field was annotated. All inherited checks pass.
- lifetime01: compiler exit 1 as required, diagnostic E0515 from the actual for_doubled API returning a borrow of a local owner. The validator accepts that diagnostic specifically; its check_exit is 0. The cfg fixture is compile-only.

## Ownership and layout

Three nested borrowing panels pass, sharing the same cell address across m,2m,4m wrappers and preserving the owner after inner scopes end. The production ownership constructors contain neither Rc nor a heap constructor; this is a source property, not a global allocation measurement. The generic engine vectors still allocate.

| Structure | Bytes on the checked compiler/target |
|---|---:|
| IntervalProducts engine metadata | 224 |
| Owned-or-borrowed storage enum | 240 |
| RecoveryProducts context | 264 |
| Prior Rc-based context layout | 32 |

The enum's borrowed variant still reserves space for the owned variant. Consequently this representation trades ownership allocation/reference-count work for larger inline contexts. These sizes do not measure peak stack, peak memory or elapsed time.

## Retained arithmetic

Twenty scalar-stream panels retain 760 coefficient equalities, 120 key/borrow rejections and 20 interleaved pairs. Shared polynomial/progression setups remain 16/16 versus 32/32 separately. Equal-width batches remain 10147; separate native-width batches remain 9953. The width tradeoff is unchanged.

All 1686 public target/cap comparisons retain predecessor scalar results, source calls and routes. Cap 8 retains ten unresolved targets per implementation; all other panels recover 281/281. Public shares remain cold. The source-call totals are 4053,3278,3228,3224,3068,2881 at caps 8,10,16,20,32,64 respectively.

```
borrowed_nested_panels=3 inline_engine_bytes=224 borrowed_storage_enum_bytes=240 recovery_products_bytes=264 prior_rc_products_bytes=32 ownership_heap_constructor=false reference_count_operations=false
shared_rescue_panels=20 coefficient_equalities=760 shared_polynomial_setups=16 fixed_width_polynomial_setups=32 native_width_polynomial_setups=32 shared_grid_setups=16 fixed_width_grid_setups=32 native_width_grid_setups=32 shared_evaluation_batches=10147 fixed_width_evaluation_batches=10147 native_width_evaluation_batches=9953 rejected_controls=120 interleaved_stream_pairs=20
shared_rescue_public_cases=281 cap=8 source_calls=4053 recovered=271 route_changes=0 shared_engines=0 warm_shares=0 cold_shares=0 retained_width_sum=0
shared_rescue_public_cases=281 cap=10 source_calls=3278 recovered=281 route_changes=0 shared_engines=260 warm_shares=0 cold_shares=260 retained_width_sum=780
shared_rescue_public_cases=281 cap=16 source_calls=3228 recovered=281 route_changes=0 shared_engines=260 warm_shares=0 cold_shares=260 retained_width_sum=780
shared_rescue_public_cases=281 cap=20 source_calls=3224 recovered=281 route_changes=0 shared_engines=260 warm_shares=0 cold_shares=260 retained_width_sum=780
shared_rescue_public_cases=281 cap=32 source_calls=3068 recovered=281 route_changes=0 shared_engines=205 warm_shares=0 cold_shares=205 retained_width_sum=615
shared_rescue_public_cases=281 cap=64 source_calls=2881 recovered=281 route_changes=0 shared_engines=0 warm_shares=0 cold_shares=0 retained_width_sum=0
```

Compiler: rustc 1.99.1 (58cf8f9c3 2026-10-13); dependency hashes: 100, checked again before the escape probe. PCC-20261107-2a715d pins the derivation; independent signed verdict pending. No cloud or timing work.
