# Preserve the completed lattice; repair only its comment header

rr-pid50-01 passes its actual coefficient roundtrips, integral matrix
roundtrip, complete original-module products, source containment and index376.
Its written header contains a backslash-n escape inside a Singular string.
Singular writes that as the literal letter n, so the first physical comment
line also contains the ring declaration. rr-mult50-01 rejects while loading
that file; no product is accepted from that attempt, regardless of its late
marker. Keep both original outputs unchanged.

The repaired consumer reads the saved string and finds the exact substring
`ring inverseBaseCertificate50=`. It drops only the preceding comment and
executes the existing declaration and ALL unchanged coefficient entries.
It then checks both full original-module matrix products again, plus the
complete source lift and every later product. This restores the saved data
without repeating the accepted lattice computation. Subsequent exporters
write a comment and declaration in separate write calls. This is a formatting
repair, not a polynomial, basis, expected-index or acceptance-limit change.
