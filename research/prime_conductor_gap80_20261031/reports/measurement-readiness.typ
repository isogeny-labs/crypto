#set page(paper: "a4", margin: 18mm)
#set text(font: "DejaVu Sans", size: 10pt)
= Requested gap80 measurements: execution prerequisites

The native metadata audit confirms why the existing constructor cannot yet
measure the requested complete 80-bit crossing. No performance experiment
was launched, and no missing time is presented as zero or extrapolated.

#image("measurement-readiness.svg",width:100%)

== Exact input and observed eligibility checks

The frozen degree is 729845429276318409687041 (80 bits). The earlier
`sqrt_velu_batched_v4.cpp` parses it into uint64; the audit reproduces that
extraction and observes rejection. Its scalar multiplication also uses that
width. A prior 96-bit field receipt is not an 80-bit degree receipt.

With a widened degree and the same window rule, the exact baby count is
427,154,957,034. Each fmpz slot occupies 8 bytes in this build; their array
alone needs 3,417,239,656,272 bytes. This excludes integer limbs, derivative
polynomials and product/remainder trees. It exceeds the 137,438,953,472-byte
lab cap. This is a method-specific accounting bound, not an observed OOM
or a lower bound for all isogeny algorithms.

The earlier constructor's source and CONSTRUCTOR.md explicitly limit it to
codomain construction. Arbitrary-point evaluation and a transported base
are absent. Consequently paired endpoint logarithm timings cannot be
formed from the current artifacts.

== Provenance, controls and next executable

`../native/measurement_preflight.cpp` implements exact GMP metadata checks;
`../measurement-readiness.json` preserves their output. The degree-10,009
control accepts the legacy parser and needs 50 root slots (400 bytes),
without claiming the metadata check constructs an isogeny. The build script
compiles this tool without launching a crypto performance experiment.

`../MEASUREMENT_PROTOCOL.md` preserves the requested complete construction,
32 non-kernel contexts, scalar multiplication and matched ECDLP trials,
including setup/online/verification boundaries, controls and the earlier
index-calculus request. Iso Lab EKS/native execution and checkpoints remain
required. No fallback fixture was dispatched in place of the target.

Next: implement an 80-bit-degree-compatible constructor with feasible
total memory/work and a verified full point evaluator; then freeze its
source/image/binary, scalar list and budget and run the paired protocol.
Source-only solves and partial tiles remain separate diagnostics. See
`problems/conductor-gap-upper-floor-and-cost.md` for the continuing obligation.
