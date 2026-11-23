# Interval-engine work instrument

The interval product engine now has a scoped, single-thread instrumentation variant. `generate.py` copies the exact planned-width source and inserts enumerated entry counters. `generation.json` pins every source, output and reversible edit. The polynomial module actually used by modes.rs is copied explicitly; an unused same-named polynomial file is not mistaken for the dependency.

`meter::capture` arms a session; `IntervalProducts::products` opens the counted region. Nested regions count each event once. The callback can retain engine caches and output vectors. Source-grid/group work, coefficient-stream arithmetic outside the interval engine, order checks, input construction, formatting and reference calculations remain outside this component boundary and must be charged separately for a recovery comparison.

Ten dimensions stay separate: scalar field multiplications (including squares), polynomial field multiplications, extended-Euclid inverse entries, exponent-r-minus-two entries, alloc/zeroed/realloc calls, requested bytes, allocation failures and dealloc calls. Fermat inverse products are already included in multiplication counts. Inverse events are not multiplication equivalents. Modular additions, subtractions, independent reductions, EEA integer steps, copies and index arithmetic are not counted by this instrument. Do not label the vector total field work or sum its entries.

Requested bytes sum each alloc/zeroed size and each realloc NEW size, including failed requests. Dealloc calls do not subtract bytes. Objects may originate before or survive after capture: requested bytes are neither live bytes nor peak memory. Compiler optimizations may elide allocations; only the pinned build's observed allocator requests are reported. Calls delegate unchanged pointer/layout operations to System. Instrumentation itself changes machine work and cannot support timings.

This is a single-thread executable contract: atomics avoid allocator recursion, not concurrent capture support. The session rejects nested captures; region/session guards restore state on returns and unwinding. Overflow saturates and flags the snapshot; consumers must reject flagged snapshots or allocation failures. Nothing here depends on allocation counts for program safety.

## Qualification

Run `python3 generate.py`, then `python3 validate.py /fresh/output/path`. The validator reverses every edit, compiles with optimized overflow checking, executes small hand-counted software controls, and runs the inherited correctness suite with instrumentation disabled. It compares inherited stdout byte-for-byte with the preserved planned-width output. Bounds: build90s, self-test30s, inherited60s; one attempt per output directory. Failed attempts remain immutable. These are software tests, not width selection or new comparative measurements.

Controls cover direct lengths 0/1/2/7/12, a 2-by-3 naive polynomial product, constant-series inversion, binary Fermat inversion, exact allocator trait calls, disabled regions, nested regions, nested-capture refusal, invalid-domain returns, synthetic allocation failure, overflow, unwind restoration and two identical blocked engines. All arithmetic is Rust. No original cohort change, timing or cloud execution.

## Next comparison

Before collecting costs, freeze and commit an experiment.protocol/v1 and analysis/export adapter. Reuse the declared 20 scalar-stream panels with first-stream and doubled phases separate; keep the public warm m35 panel separate. Include identical-arm repeats, a planted additional multiplication and invalid-snapshot rejection. Preserve every per-panel vector and regression. Add outside-engine work and copy/addition counters before making total-work conclusions. No such comparison was run in this instrument round.

Sources checked: planned-width interval.rs/progression.rs/modes.rs, hgcd-roots/polynomial.rs, resultant-accounting/meter.rs and its GlobalAlloc contract reference (https://doc.rust-lang.org/std/alloc/trait.GlobalAlloc.html). KB unavailable in this environment. This adapts existing accounting mechanics to caches that cross operation boundaries.
