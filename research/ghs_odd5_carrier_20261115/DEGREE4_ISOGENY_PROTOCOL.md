# Trace +26 class-wide degree-four cover screen

Frozen 15 November 2026 before execution. The fixed genus-five Y, field K and
nine alternate b-masks are as in `DEGREE5_ISOGENY_PROTOCOL.md`. The already
certified b-mask 3 is in `DEGREE4_THEOREM.md`. Adapt the frozen degree-four CRT
and direct-linear scripts independently for each target. The complete pole
list is `(x+1)Qi`, `(x+1)(x-c)^2` for c != -1, `(x+1)^3`, and `x+1`;
there are 249 denominators and 59,697 logical assignments per target.

For each of masks `27,30,103,139,167,182,194,230,233`, require source-bound
JSON, exit zero, empty stderr, all expected assignments and exact agreement of
the producer and direct-linear degree-four `(A,B)` candidate sets. Any
candidate must satisfy the full elliptic equation before it is called a map.
The frozen producer `.sage` and generated `.sage.py` SHA-256 values are
`a6f69c02728e06479ea69c71df839c6d6924d46f3f1ae07e09a03148b2be5f54`
and `36c08bb25238c110ac25e31fba65a1e585c8f53b0ee0983f5e7ec784a191c467`.
The independent direct-linear values are
`d335a97945fbb152fe530260ea9b38f1b31c8b504185088b76d68035aa626a53`
and `f6c502c67074dc529fd6a67506e394861a36b7e98edd445ccc87d206aff8c986`.
If every candidate set is empty, degree four is excluded for all ten exact
targets; the previous geometric certificate excludes degrees one to three.
Howe's upper degree twenty applies to each target because the common trace
discriminant -296 is fundamental. The resulting interval would be `[6,20]` for
each exact target over K, assuming the separate degree-five replays also pass.
This would not classify other genus-five carriers or build an F3 transfer.
