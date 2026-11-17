# Verification records

- unit-tests.log: five native checks, including independent field arithmetic and
  positive/negative exact point-record lookup cases.
- container-controls.jsonl: eight completed exact controls in the pinned runtime.
- runtime-inspect.json: actual Docker image identity, mounted executable/base
  paths, read-only flags, container memory/CPU limits and start time. No secrets.

Production controls and input/base membership receipts are also retained in the
raw run. The final report validates96 unique cells/modes, ell27, exact factor-base
manifest identity, point-membership receipt counts, controls, binary/source and
all solver stdout hashes. Export validation and raw-manifest checks are added
when the producer terminates. This is producer verification, not independent
scientific review.
