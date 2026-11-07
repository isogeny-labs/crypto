# Shared geometry for projected character windows

T-1790. Producer construction and software checks; independent review pending.

## Mechanism

For the fixed multiplier labels a_j of the source grid, prepare S_j=[a_j]P. Initialize lanes T_j=[a_j]Q. Advance T_j <- T_j+S_j simultaneously with Q <- Q+P. Group distributivity gives T_j=[a_j](Q+iP) at every offset i. A single batch inverse converts the collected projective prefix and finite lane coordinates. The existing projected resultant and division-factor expression then runs on each supplied affine grid unchanged. This is the existing joint-window construction applied to the standalone source, not a new group identity.

The source stores its original public base. PreparedWindow borrows that source and its curve, preventing accidental cross-base preparation through the public API. Before building target lanes, ValueDivision validates canonical curve coordinates and r-torsion. The extracted raw_on_grid repeats the torsion and factor checks for each finite shift. Identity offsets yield sign0 and contribute no finite slots. Supplied slot offsets, labels and lane order are checked before evaluating the residual. No old full-value source is used in normal builds.

## Recovery integration

recovery.rs reuses the prior frozen Program module and candidate verification. The initial width14 sign window uses PreparedWindow. Conditional disambiguating signs still use standalone single-point queries and advance only to the requested offset. Every candidate still passes the internal [candidate]P==Q check before return. Program setup still enumerates the public scalar field. The wrapper prepares public strides per recovery call; callers of the lower-level window API can reuse preparation across calls.

## Resource and cost scope

For L offsets and m grid lanes, the old per-sign route constructs L separate grids. This route constructs one target grid, prepares one public stride grid and advances m lanes per subsequent offset. The query combines finite prefix/lane normalization in one batch (zero for an identity-only singleton). Preparation has its own normalization and the Source constructor still evaluates its calibration. Per-point division caches and resultants remain; the initial torsion precheck adds work. Thus a reduced normalization count is not an end-to-end speed measurement.

The materialized snapshots, slots and affine conversion buffers require O(L*m) storage, versus sequential per-sign geometry's O(m) storage. prepare_window checks the maximum flattened-point count L*(m+1) and a nonzero maximum length before construction; these are element-count bounds, not byte/RSS limits. Single-point native word/prime/projection domain restrictions remain. This round introduces no wider domain.

## Evidence and replay

validate.py runs bounded optimized Rust software tests and a normal metadata/dep-info build, with source hashes before and after. Use a new output directory. validation-01 preserves two compile errors and the exact failed source; validation-02 preserves the passing window-only stage; validation-03 records the integrated recovery stage. The original4563-case cohort is untouched. No cloud or comparative timing panel is run.

## Sources checked

KB wrapper unavailable (no executable). Fallback: feature-native-joint-window/lib.rs Prepared::joint_window, feature-native-standalone-source/lib.rs and DERIVATION.md, feature-native-standalone-recovery/lib.rs and program.rs. Rust lifetime semantics: https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html . This reuses the existing algebraic and implementation constructions; no novelty claim.

## Next construction

Share the prepared public strides across recovery calls and retain the final target lanes for conditional extra offsets, eliminating reconstruction there. The larger scaling dependencies remain per-shift resultant work and the compiler's enumerated public setup.
