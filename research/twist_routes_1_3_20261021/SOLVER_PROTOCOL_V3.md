# Solver resource amendment, version 3

Preserve the entire V1 and ASCII V2 attempts. V2's Singular comparisons and
native records have their recorded scope; V2 does not provide the intended
msolve comparison. A tiny valid ASCII input segfaults before any output with
the 768 MiB virtual-address limit, also with msolve's hash-table parameter
reduced to 12. The identical input succeeds with a 4 GiB address-space limit.
The diagnostic outputs are retained in `execution-v1/logs/`.

Repeat the exact V1/V2 1,930 cells, target seeds, orderings and four encodings in
a fresh directory, with the V2 serializer and a **4 GiB virtual-address limit
for every engine**. Keep the one-second hard child limit, native 500 ms
cooperative limit, one thread per child, at most three concurrent cells and
90-minute outer limit. This is a resource-amended panel, not an interchangeable
replicate of the earlier attempts. No timing or speed ratio crosses attempts.

Keep every failed attempt, raw input, basis, dimension/equality result, exit
code and incomplete cell. Interpret a resource failure as unresolved. All V1
correctness checks and original broader experimental obligations still apply.
