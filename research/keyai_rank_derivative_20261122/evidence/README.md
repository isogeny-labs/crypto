# Native development evidence

The frozen scoped qualification is under [qualifier/attempt2](qualifier/attempt2/).
Its [pre-call manifest](qualifier/attempt2/pre-call.json) binds copied executables
before execution; the [post-call record](qualifier/attempt2/post-call.json)
records exit status, raw-output hashes, source checks and fixture fields.
The [source manifest](qualifier/attempt2/source-manifest.json) identifies whole
production files and exact, unmodified dependency spans. The complete
[fixture output](qualifier/attempt2/controls.stdout.json) includes replayable
group/rank receipts before and after the explicit generator anchor.

The logs record four actual example tests, five independently authored tests,
and six unchanged production-module support tests passing. The independent
support oracle covers 2,830 small-field cases using powered divisibility;
Jacobian and small matrix-rank checks use separate dual-number and minor-based
calculations. This is same-host, unsigned development evidence, recorded as
`PCC-20261123-08e731`, whose standing is unverified. [pcc:unverified]

The retained [first attempt](qualifier/attempt1/) failed at compilation because
the new qualification wrapper omitted an existing `Fp64` namespace import and
the actual `prefetch_cache_line` helper span. No test executed in that attempt.
The second attempt corrected the wrapper and added the unchanged module-test
target; no production arithmetic was replaced. Its original binary/source
copies and Cargo event streams are listed in the
[artifact inventory](qualifier/artifact-inventory.json).

The full root-library release gate exited 101 before tests because of delimiter
errors in the untouched binary-hyperelliptic module. Its source is byte-equal
to the pinned base, and a parser-only check of that exact base file also fails.
The [baseline receipt](root-library-baseline.json), [compiler output](root-library.stderr.log)
and [terminal record](root-library.post-call.txt) retain the evidence. This is
recorded as unsigned statement `PCC-20261123-db257a`; it is not a full-library
pass. No timing or performance comparison is inferred from test-runner
durations. [pcc:unverified]
