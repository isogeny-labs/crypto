# Original package preservation

The original `PACKAGE.sha256` binds29 payloads. All are retained byte for
byte; the61856-byte executable is packaged as timestamp-free gzip
`coded-kernel.gz`. The verbatim compiler-version log, including its trailing
blank line, is also gzip packaged so whitespace checks preserve the evidence.
Their original digests remain in the untouched manifest.
`verify-original.sh` checks all29 payloads, decompressing these two as needed.
The replay helper rebuilds from source and checks
all seven frozen outputs; it does not execute the archived binary.

The post-run copy `candidate-frozen.md` is explicitly labeled as such by the
original receipt. The prospective protocol, code and precompile candidate
hash were frozen before execution; concurrent changes to the candidate
Markdown are preserved and do not redefine the fixed a3 experiment.
