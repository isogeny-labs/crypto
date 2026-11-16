# Matched native monic runner

This source-only construction joins the preserved counted baseline and a mechanically
instrumented monic candidate in one Rust binary. A/A calls the same baseline code
with a separately prepared decoder. One shared thread-local meter and System
allocator count all arms. No panel has been executed.

`generate.py` routes seven production dependencies and adds the original hooks plus
three monic expressions: leaf negation, Horner addition and low-remainder subtraction.
`verify_instrumentation.py` verifies hashes and reverses every change to recover the
original source bytes. The candidate's test-only baseline path stays uninstrumented.
Original accounting files and previous evidence remain unchanged.

Build with `rustc --edition=2021 -O main.rs -o runner`. `runner --describe` executes
no crypto. The opt-in `--run-frozen-panel PROTOCOL_SHA256` mode requires external
protocol freeze and admission; the argument records identity and does not itself
validate authorization. Production code receives public points only. Expected
scalar labels appear only in checker and predeclared scheduling code.

The fixed eight target scalars are 0,1,2,25,173,292,440,478. Even targets run baseline,
candidate,A/A; odd targets run A/A,candidate,baseline. Setup and preparation generate
six separate records, followed by 24 query records. Successful recovery already
includes independent point multiplication. A mismatch or recovery error writes its
row and makes the process fail; constructor failures abort and preserve partial output.
Every snapshot is taken before output formatting. Arrays retain all five stages,
eight components and seventeen operations, with names in the run header.

Next implement strict complete-panel parsing and exact A/A analysis; bind its hash,
the binary and preflight to the existing protocol draft; reconcile EKS admission;
freeze and publish before data. The original 4563-case cohort is unchanged.

Prior work: preserved accounting and monic components; FLINT's documented Newton
inversion and subproduct trees, https://flintlib.org/doc/nmod_poly/index.html.
No novelty claim is made for these polynomial methods.
