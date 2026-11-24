# Development checks for the refined oracle

2026-11-24. Local evidence; [pcc:unverified].

- Isolated crate: six release tests passed. The recorded
  [test output](evidence/refinement_v2/unit_tests.txt) includes full tuple-partition
  checks, an expired deadline and failure after a completed branch.
- Five version-1 invocations and two version-2 invocations exited zero; all
  intended query and recovery records are retained in their versioned directories.
- Root library: `cargo +1.99.1 test --release --lib --locked`, two build jobs,
  exited 124 under an initial 180-second and a subsequent 600-second timeout.
  Both stopped while compiling dependencies, before any library-test result.
  [First log](evidence/refinement_v2/root_build_180s.txt) ·
  [Second log](evidence/refinement_v2/root_build_600s.txt).
- Benchmark-record gate: after materializing the tracked records and progress
  files, `python3 tools/benchrec.py gate` stops on
  `benchmarks/records/20261105T0828Z-icb-large-prime-n53-s1-archive.json`:
  command 1 lacks a successful integer exit code and needs historical handling.
  The recorder, all record files and progress files are unchanged from
  `origin/main` at `a6223dac`. The flagged record has Git blob
  `49346ce39eb6ec7d77f3c507eac89ac73cce3bd3` in both trees. No benchmark metadata
  is changed by this lane.
- PCC statements have unverified standing. The ledger has 1,862 entries and
  its hash chain and `origin/main` prefix pass the local check. Report and
  protocol prose lint passes.
- The three-page report PDF was rendered and visually inspected, including
  the graph, tables, source links and command lines. Its initial build used
  the wrong Pandoc math-reader setting; enabling `tex_math_single_backslash`
  corrected it without changing the mathematical text.

The complete root library suite, admitted measurement and independent replay
are not represented as passed. The public PR reports those remaining checks.

## Production comparator continuation

The [new report](PRODUCTION_RESULT.md) and [reproduction instructions](PRODUCTION_REPRODUCE.md)
retain the optional source integration and all thirteen full-base resource
refusals. The final adapter suite passes ten tests; the production closure's
32 library tests, eight public-example tests, and six unchanged default-feature
tests also pass locally. A separate planted four-leaf unit control exercises
successful native output and adapter conversion. [pcc:unverified]

The frozen panel source is 8be878c873; the later source adds that unit test
without changing the panel executable. Evidence and before-call identities
live in `evidence/production_v1`. The earlier root compilation limitation and
unchanged historical benchmark-record gate error remain open. No broad root
test rerun or cost measurement is represented as successful.
