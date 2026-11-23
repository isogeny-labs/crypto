# Checked-map Boolean equations and sparse row certificates

The library now expands an independently checked Semaev auxiliary-wire map into ordered Boolean generators, preserving zero generators and original ordinals. A finite multiplier window produces sparse rows by exact GF(2) elimination while retaining generator-combination provenance. Canonical BQI1 encoding is verified by the unchanged independent arithmetic checker. Dynamic variable indices remove the 64/128-bit mask ceiling.

The focused release-library run passed 18 tests with unchanged source: the genuine n=2,m=4 checked-map control derives a nontrivial row and checks its certificate on both twists; the remaining tests cover source expansion and the independent arithmetic checker. The prior standalone kernel suite also passed 23 tests, including variable129 and retained refusal controls; it remains separately archived. These are software correctness controls, not measured full-cohort dataset results.

The wrapper retains its checked source, rows, encoded bytes and checker audit on failure. Original-worker budget identity remains a private caller integration requirement. This finite row construction is not full Groebner completion, F5 signature processing, or a fall-degree measurement. The requested n23–31/n47–61,m2–15 dataset scope remains tracked in ../coverage-audit-20261103/REQUIREMENTS.md.

Source contracts and independent review were checked before integration. Exact command, source manifest, counts and terminal log digest are in compile.receipt.json. Full logs and source have verified S3 readback in s3.receipt.json.
