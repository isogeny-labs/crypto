# Design A prefetch patch on today's main: analysis (read-only, 2026-11-11, HEAD 2729b9056b)

## Headline
The patch is **obsolete, not stale**. `main` already contains an independent implementation of the
same idea, with `layout=lines` as opt-in on `mitm-prime-batch` and on `mitm:negation_folded=1`,
a cache-line pair index, a cache-line relation guard, a prefetching lane round, and equality tests.
It landed inside merge 52f6bc680d (PR #3738, via commit 891c8946dd, 2026-11-04) and fbf24eac9d (PR #3704,
test commit ee862bf9cc "Test that IC mitm layout=lines matches the default layout"). Its source comment
cites "#3116". The measurement was also already run: the line-layout tournament
(`research/ic_line_layout_tournament_20261031/`, v6 at fp28) and the lead assessment
(`research/prime_field_lead_assessment_20261107/README.md` lines 53, 146) record
`layout=lines` v6 at fp28 as `invalid_measurement`, "candidate 12 % slower in valid sessions, not an
improvement" (tournament DECISION §16 is provisional pending validator; figures `[pcc:unverified]`).
The README of the prefetch directory and lead 5 of `prime_ic_leads_20261110` §7 do not mention this.

Recommendation for the main session: do NOT schedule a port build pass. Instead reconcile lead 5 with
the tournament result, and treat "software prefetch" as already tried (see section 5 for what, if
anything, differs from Design A).

## 1. Are #2783 and #2806 in main?
- `git log --oneline --all --grep '#2783' --grep '#2806'`: no output.
- `git log --merges --oneline | grep -E '#2783|#2806'`: no output.
- `git branch -a | grep -i wide`: nothing. `lead/ic-wide-ext-20261028` is not a local or remote-tracking ref.
- `git ls-tree -d HEAD research/ic_wide_batching_20261028 research/ic_wide_ext_20261028 benchmarks/20261028-ic-wide-ext`: empty.
  So neither PR's directories are in main. (A `git log -S` over `--all` was still running in the
  background when I wrote this; it does not affect the conclusion.)
- Code paths the patch modifies, checked with `git show HEAD:<path> | grep -n` (ic_boundary.rs is 20408 lines at HEAD):

| patch symbol | at HEAD |
|:--|:--|
| `collect_and_solve_relations_touching` (ic_boundary.rs) | exists, line 12631, already has `touch` and `line_guard: bool` params (patch adds them) |
| `collect_and_solve_relations_scaled` | exists line 12695, but now a thin wrapper; real body is `collect_and_solve_relations_scaled_touching<G, L, R: RelationGuard>` line 12732 (new at HEAD; patch's base had `_scaled` with the `touch` arg) |
| round touch loop (`seen.contains_key` + `touch`) | replaced at ~13020 by `if R::PREFETCH { seen.prefetch(g.key(&point)); touch(&point); }` |
| `seen: FastMap<()>` guard | now generic `R: RelationGuard` (trait line 7760; impls for `FastMap<()>` with `PREFETCH=false`, and `GuardLineSet`) |
| `Line` / `lines_for` / `prefetch_line` | exist as `GuardLine` (7656), `cache_lines_for` (7665), `prefetch_cache_line` (7671; same `_mm_prefetch::<_MM_HINT_T0>`) |
| `GuardSet` | exists as `GuardLineSet` (7686) |
| `FlatPairIndex` | exists as `PairLineIndex` (~7800-7830) |
| `PairTable.flat` | exists as `PairTable.lines: Option<PairLineIndex>` |
| `PairTable::build_negation_folded_prime_batch_lines` | exists line 8069 (also `build_negation_folded_lines` line 7993 for the generic `mitm` oracle) |
| `PairTable::touch` | renamed `PairTable::prefetch` (line 8256), advisory no-op for non-line layouts |
| `PairTable::get` dispatch | exists line ~8240 (`if let Some(lines) = &self.lines`) |
| `DecompositionOracle::line_guard` (stages.rs) | exists, stages.rs:271; `touch` at stages.rs:266 |
| `PrimeBatchMitmOracle.lines`, `layout` param (plugins.rs) | exists, plugins.rs:2970-3122 |
| `mod.rs` reads `oracle.line_guard()` | exists, ic_framework/mod.rs:621, passed at :642 |
| patch tests `guard_set_answers_as_the_std_map` etc. | equivalents: `cache_line_guard_matches_map_across_growth_and_repeats` (ic_boundary.rs:17542), `mitm-prime-batch` line-vs-map table test around 17584 (`build_negation_folded_prime_batch_lines`), and plugins.rs `layout_lines_equality_tests` |

## 2. `git apply --3way --check` output
```
error: repository lacks the necessary blob to perform 3-way merge.
Falling back to direct application...
error: patch failed: src/cryptanalysis/ic_boundary.rs:6133
error: src/cryptanalysis/ic_boundary.rs: patch does not apply
(same three lines for ic_framework/mod.rs:606, plugins.rs:2134, stages.rs:235)
```
Plain `git apply --check` gives the same four failures. Nothing was applied. The 3-way fallback is
unavailable because the patch's index blobs (branch lead/ic-wide-ext-20261028) are not in the repo.
All four files fail on the first hunk; per file:

- `ic_boundary.rs` (first failing hunk @6133, "Line tables" block, which inserts after `fn mix`-adjacent
  hash code): reason = **moved code and already-implemented**. The insertion context (end of the `mix`
  helper at old line 6133) is at line ~1404 in HEAD (`fn mix` 1404); line tables now sit at 7656-7830.
  Every later hunk (struct `PairTable` field `flat`, builders, `get`, `touch`, the `line_guard` arg on
  `collect_and_solve_relations_touching`/`_scaled`, the `Guard` enum, tests) targets code that either
  exists in a different shape or has been replaced by the `RelationGuard` trait. Current targets:
  - `PairTable` struct: ic_boundary.rs:7865 (field `lines`, line ~7874+).
  - `get`: ~8240; prefetch: 8256-8260:
    `pub fn prefetch<G: CountedGroup>(&self, g: &G, target: &G::Elt) { if let Some(lines) = &self.lines { lines.prefetch(g.key(target) >> 1); } }`
  - Collector: 12631-12690 (`line_guard` selects `GuardLineSet::with_capacity(capacity)` vs `fast_map(capacity)`).
  - Round loop: ~13018-13024 (`if R::PREFETCH { seen.prefetch(...); touch(&point); }`).
- `ic_framework/mod.rs:606`: context drift plus already present. Current: mod.rs:621 `let line_guard = oracle.line_guard();` and :642 `line_guard,`.
- `plugins.rs:2134`: code moved (`PrimeBatchMitmOracle` is at 2970, not 2134) and already present:
  `lines: bool` (2972), `parameters()` lists `layout` (3038-3045), `prepare` parses it (3056-3081), `touch` (3114) and `line_guard` (3120).
- `stages.rs:235`: moved (trait method now at stages.rs:266-271) and already present (`fn line_guard(&self) -> bool { false }`).

Differences that matter if one still wanted the patch's variants:
- Patch's `layout=map` guard performs a demand `contains_key` touch each round; HEAD's `FastMap` guard has
  `PREFETCH=false` and does no touch (incumbent got *cheaper* than the patch's base; the paired baseline differs).
- Patch makes `layout=lines` silently allowed only for `mitm-prime-batch`; HEAD also allows `mitm:negation_folded=1`
  (plugins.rs:2240-2259, error "layout=lines requires negation_folded=1") and rejects it on `mitm-prime-blocked` (3079).
- Patch's flat index falls back to the map for p >= 2^32 or base > 2^16; check HEAD's `PairLineIndex` (ic_boundary.rs ~7800)
  for its own limits before assuming equal coverage (not read in full).

## 3. Opt-in surface
- Spec string parser: `Params::parse_spec` at `src/cryptanalysis/ic_framework/stages.rs:102-122`
  (`name:k=v,k=v`, `,` ends a parameter, `;` separates list items).
- `layout` option **already exists**:
  - `mitm-prime-batch`: `PrimeBatchMitmOracle::parameters()` plugins.rs:3038-3045 (help text "map (default) or lines (cache-line pair index and relation guard)"),
    accept-list and parse in `prepare` plugins.rs:3056-3081 (`None|Some("map") => false`, `Some("lines") if !self.blocked => true`, else error).
  - `mitm:negation_folded=1`: plugins.rs:2240-2259.
  - Where `layout=lines` would plug in (already there): `prepare` selects `PairTable::build_negation_folded_prime_batch_lines` (3084ish); `touch` (3114) calls `PairTable::prefetch`; `line_guard()` (3120) returns `self.lines`; mod.rs:621 forwards it to the collector.
- CLI/ecbench method dispatch: `src/cryptanalysis/ecbench/methods.rs:6650-6674` (`PrimeBatchMitmOracle::new()` bound to `"mitm-prime-batch"`);
  help string at methods.rs:221 does not mention `layout=` (documentation gap only). `src/bin/ic/bench.rs:772` also maps the name.

## 4. Invariant tests (records byte-identical between layouts)
Already present at HEAD:
- `plugins.rs` `mod layout_lines_equality_tests` (doc at ~6303): over 6 small prime curves (fp14..fp20 plus two `find_prime_order_curve` draws) and both specs in `SPECS`
  (`mitm:negation_folded=1`, `mitm-prime-batch:m=2`):
  `layout_map_is_the_default_and_lines_selects_the_line_guard` (6371),
  `layout_lines_gives_identical_decompositions_and_counted_operations` (6391; build ops, `setup_work_native`, per-target decompositions and counters),
  `layout_lines_solves_the_same_logarithm_at_the_same_counted_cost` (full `run_pipeline`, same relation stream and logarithm; `assert_same_run`).
- `ic_boundary.rs`: `cache_line_guard_matches_map_across_growth_and_repeats` (17542); a prime-batch map-vs-lines table check near 17584; a `collect_and_solve_relations_touching` stream test near 17619.
- Replay: ecbench `verify --replay-all` over the tournament sessions (research/ic_line_layout_tournament_20261031/eks-results-v6); not re-run here.
What they do not check: the **record-level** byte identity of two ecbench sessions (they check the `RunReport`/outcome fields). Smallest added test, if wanted (proposal only, nothing written to the repo):
```rust
// in plugins.rs layout_lines_equality_tests; reuses curves(), arms(), ctx(), oracle_for()
#[test]
fn layout_lines_gives_equal_hit_sets_on_a_toy_curve() {
    let (inst, size) = curves().remove(0);           // fp14, base 16
    let fb = prime_factor_base(&inst, size);
    let c = ctx(&inst, inst.generator_point());
    let (name, map_p, line_p) = arms("mitm-prime-batch:m=2");
    let (mut a, mut b) = (oracle_for(&name), oracle_for(&name));
    a.prepare(&c, &fb, &map_p, &mut GroupOps::default()).unwrap();
    b.prepare(&c, &fb, &line_p, &mut GroupOps::default()).unwrap();
    let hits = |o: &mut Box<dyn DecompositionOracle<PrimeCurve>>| {
        let (mut ops, mut ctr) = (GroupOps::default(), OracleCounters::default());
        let mut v = Vec::new();
        let mut p = inst.generator_point();
        for _ in 0..inst.r {                       // every multiple of G: exhaustive on a toy curve
            v.push(o.decompose(&c, &fb, &mut ops, &mut ctr, p));
            p = inst.curve.add(&mut GroupOps::default(), p, inst.generator_point());
        }
        (v, ops, ctr.lookups)
    };
    assert_eq!(hits(&mut a), hits(&mut b));
}
```
(Exact helper names/fields (`OracleCounters`, `ctr.lookups`) must be checked at build time; this is a sketch.)
This is largely subsumed by `layout_lines_gives_identical_decompositions_and_counted_operations`.

## 5. Effort and commands
- Port effort for the patch as written: moot. All 4 files fail at the first hunk; the equivalent code is in main
  (about 460 added lines in the patch, all with a counterpart at HEAD). A "port" would mean redoing nothing, or
  deliberately adding the patch's *differences* (a demand-touching `layout=map` baseline; `FlatPairIndex`
  packed 32+16+16 words vs HEAD's `PairLineIndex`), which is a new design decision, not a port.
- Build/test commands for HEAD's existing implementation (after the other agent's build finishes; do not run now):
  - Binary hosting the oracle: the oracle is library code in `src/cryptanalysis/ic_framework/plugins.rs`; it is exposed by
    `src/bin/ecbench.rs` via `ecbench/methods.rs:6650` (primary route for sessions) and by `src/bin/ic/bench.rs:772` (`ic` binary, `[[bin]] name = "ic"`, `src/bin/ic.rs`).
    `cargo build --release --bin ecbench` (and `--bin ic` if the `ic bench` route is wanted).
  - Unit tests (lib): `cargo test --release --lib layout_lines` (the three equality tests), `cargo test --release --lib cache_line_guard`, `cargo test --release --lib prime_batch`.
    Test targets need a lib build of the whole crate (large; several GB disk, per the program notes).
  - Gate before any session: `cargo fmt --check`; `cargo clippy --all-targets -- -D warnings`.
- Any measurement already exists (tournament v6; `invalid_measurement`, 12 % slower in valid sessions,
  provisional). A new session would need a new design variant and the isolab-eks route, not this patch.

## Files read
research/prime_ic_program_20261029/hashtable_prefetch/{README.md,patch.diff}; research/prime_ic_program_20261029/{README.md §7 row 5 via leads README,DECISIONS.md D-PREFETCH};
research/prime_ic_leads_20261110/README.md §7 lead 5 (line 319); src/cryptanalysis/{ic_boundary.rs,ic_framework/{plugins,stages,mod}.rs,ecbench/methods.rs} at HEAD;
research/ic_line_layout_tournament_20261031/DECISION-V6-REDESIGN-20261106.md; research/prime_field_lead_assessment_20261107/README.md.
