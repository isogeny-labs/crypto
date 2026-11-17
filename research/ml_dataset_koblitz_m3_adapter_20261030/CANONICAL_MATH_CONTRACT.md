# Canonical mathematical file verification

This component runs on a sealed, non-mutating cell after original-run and
manifest admission. It requires the four frozen canonical artifact identities
and unique manifest entries for the live committed head and event stream. It
verifies manifest files and checks the exact bytes used by each mathematical
parser against their canonical length and SHA-256.

It independently verifies original equations, verifies the strict group envelope
against the actual input-file digest, matches the acknowledged arithmetic prefix
to the authenticated capture stream, and verifies the canonical witness's
path/role/encoding/digest/length against its unique capture publication. A live
published witness without its canonical identity fails this canonical path;
censored/raw-prefix admission remains separate.

Return values certify only the checked mathematics and witness-publication
sequence. The native result and canonical boundary-event artifacts are required
by identity but are not semantically validated by this component. Their parsers,
aggregate resource reconciliation and supervisor outcome policy remain required
before any complete cell verdict. The control intentionally supplies empty
placeholder contents for those two artifacts to expose this scope explicitly.

The positive control uses actual bounded S4 construction, original-input
serialization, signed enumeration, typed events and dedicated witness publication.
It rejects missing capture-root and canonical witness identities, and a
mathematically identical but byte-distinct rehashed canonical witness. Fixtures
are software controls, not newly generated scientific dataset cells.
