# Source correction before component execution

The initial frozen membership_certificate.cpp failed its first native build
with exit1. Retain that file and the original compiler log. The successor
membership_certificate_v2.cpp adds the missing functional header, copies a
map entry into a mutable pair, and separates a misleadingly indented loop.
It also hashes the actual newline-terminated certificate bytes written by
the inherited write helper. No initial executable or scientific output was
produced. The successor compiles with the same warnings-as-errors command.

The protocol's mathematical objects, fixed depths, comparisons, negative
controls and full-goal acceptance are unchanged. Run the successor only.
These are local exact proof-development checks, without timing claims.
