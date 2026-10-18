#set page(paper: "a4", margin: 15mm)
#set text(font: "New Computer Modern", size: 10pt)
= Result: N83 K = 16,627 selector-pinned control

*Result:* `PASS_SELECTOR_PINNED_WITNESS`, 2026-10-18 08:32:07–08:32:43 UTC.
The complete four-summand orbit-domain model had 201,598 variables,
5,915,238 clauses and 3,901 XOR rows before exact choice pins.

#image("selector-positive-stages.svg", width: 100%)

The worker imported the retained base in 35,356.152683 ms, constructed the
model in 187.394292 ms, reached a root fixpoint in 146.397875 ms and
returned one SAT model in an 8.861792 ms search call. The 35,821.806933 ms
worker wall is a cold stage diagnostic. The one-CPU Linux container had a
hard 4 GiB limit and zero swap; it exited 0 without OOM at a 2,662,318,080-byte
peak. The constructed relation for indices `[0, 166, 332, 498]` passed
original-field S3, exact x-block, stored-point group, rank-one and tamper
checks recorded in `positive/worker.json` and its phase stream.
The separate native `replay/receipt.json` recomputed the relation from the
stored object and rejected two tampered receipts. It did not replay the
SAT assignment bits, which the worker did not export.

The earlier unconditioned search reached its 1,800-second cap under a
different source revision and without the choice pins. These values are not
a paired speed ratio. Natural-query relation yield and useful rank, factor-base
selection across m = 2, 3 and 4, complete cold IC cost and matched-rho gap
remain open. The next frozen comparison separates selector and phase pins
under one source. See `RUN_REPORT.md` for exact provenance and raw paths.
