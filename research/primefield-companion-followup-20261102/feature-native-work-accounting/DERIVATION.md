# Interval instrumentation boundary and count identities

The arithmetic expressions are unchanged by generation; reverse application of each recorded edit reconstructs the pinned source byte for byte. The only arithmetic hooks are entry to the existing scalar mul, polynomial fm, polynomial inverse and pow with initial exponent r-2. The interval-products region surrounds validation, direct tails, lazy product-tree setup, progression setup/evaluation and flush. Calls outside that region are excluded even while capture is armed.

For direct length L with L<32, the existing fold calls mul exactly L times, including L=0 and factors equal to zero. A nonzero dense naive product of coefficient lengths a,b calls fm exactly ab times when min(a,b)<=12; the fixture lengths 2,3 therefore require six events. For p=97, binary pow with exponent95 has seven loop iterations and six set bits: its unchanged implementation calls mul thirteen times, including the final square. Constant-series inversion of [2] modulo97 invokes the EEA inverse once and returns49.

Nested regions increase depth but event recording tests depth>0, so nesting does not multiply counts. A rejected nested capture leaves the outer session untouched. Guards disable the session and restore depth on ordinary errors and unwinding. The instrument is single-thread only; global atomic counters are not per-thread attribution.

Allocator accounting records requests, not object lifetime. The direct allocator-trait control allocates16, reallocates to24, zero-allocates16, and pairs two deallocations through the same trait API. Its expected requested bytes are56, with alloc/zeroed/realloc/dealloc counts1/1/1/2. Failure bookkeeping is injected without causing a real OOM. Overflow saturates and sets a rejection flag. This accounting remains meaningful when returned vectors and caches survive capture, unlike a net-live-byte calculation reset between operations.

Qualification and provenance are recorded in validation02/receipt.json. validation01 preserves the failed relocation of gcd.rs and its exact source snapshot. Assertions are producer checks pending independent signed verification. [pcc:unverified]

No field-addition, copy, index or EEA-step count is supplied. A future total-work model must include these dimensions and all source/coefficient/order/group work outside this interval boundary. The next width comparison must bind a frozen protocol, source closure, exact compiler/binary, controls, exporter and analysis before new comparison rows are collected.
