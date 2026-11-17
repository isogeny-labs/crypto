# Syndrome-coded kernel native development unit

Run `bash native/check.sh /tmp/coded-kernel-check`; use `SANITIZE=1` for
the same exact controls under ASan/UBSan. The output directory is separate
from the frozen package. `native/PROTOCOL.md` is the original prospective
protocol, copied unchanged. It predates all compilation attempts.

The final unit checks dimensions1through6:30,796 projected operator pairs,
126 orientation fibers,21 exact rational selector valuations. It is not an
ECDLP solver, benchmark, scaling study or proof of the full coded-kernel
analysis. Input codes/syndromes and selectors are explicit in the source.

The first strict compilation failed on misleading indentation. The second
compiled but its first run exited134 inside GMP; ASan reproduced and
localized the misuse of noncanonical rational factors. Final source
canonicalizes every factor before arithmetic. The failed source snapshots,
compile logs, empty buffered first output, actual sanitizer diagnostic and
failure explanation are retained. Final strict and ASan/UBSan outputs are
identical. This was a unit implementation defect, not a counterexample to
the mathematical identity.

Original audit filenames are retained in `receipts/` for provenance;
`native/exact-coded.cpp` copies the final `coded-operator-native.cpp` source.
`native/expected.tsv` copies successful run002. No binary is included.
