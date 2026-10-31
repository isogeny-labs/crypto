# Completed prime n23 BSGS 64-cell campaign

The separately frozen `experiment.protocol/v1` campaign completed its exact
64-cell fixed population on local XFS. The official protocol checker reported
zero findings before generation. The draft was filed at
`850055f73953d2adca7a403e237ce74dcc47147a`, frozen protocol and bound native
plan were committed at `cd7ee385d0f9`, and the final controller at
`74741a06973d6d317c19df48d4a55551d21a6f10`. Native preflight before launch
reported 64 planned cells, both twists, zero generated cells, and matching
source/binary/plan/protocol digests. A fresh S3 Git checkpoint readback and
bundle verification preceded the first attempt.

Every index 0..63 has one attempt marker and one verified native ECDLP cell.
The 64 distinct configurations cover eight curve seeds, two generator indices,
two key indices, and both quadratic-twist choices (32 untwisted and 32
twisted). Every cell has a native report and strict export, with one public
general feature row, one public cyclic-subgroup ECDLP feature row, and
separate restricted target/provenance rows. Each restricted full-run archive
and its custody pointer were uploaded to the content-addressed S3 store and
passed a fresh GET comparison before the controller advanced. The final
controller marker was `{"planned":64,"custody_receipts":64,"complete":true}`
and exit status0. The 64 restricted archives total 4,191,827 bytes; individual
sizes range from 34,168 to 223,794 bytes. `AUDIT-01.json` recomputes the
axis coverage, receipt counts, local archive hashes/sizes, strict export row
counts and S3 readback flags from the saved artifacts.

An independent read-only audit confirmed all 64 local manifests, archives,
checkpoint receipts, unique restricted target IDs, native verification outputs,
and strict export rows. It separately streamed both S3 objects for sample
indices 0, 31, and 63; all six downloads matched their recorded SHA-256
hashes. For the other 61 pairs, the S3 conclusion rests on the producer's
saved fresh-GET receipts.

The XFS admission floor caused three safe stops before new attempts. The
controller resumed from its retained manifests and S3 receipts without
repeating a shard; `STOP-01.md`, `STOP-02.md`, and `STOP-03.md` record the exact
boundaries and space observations. The tmpfs fallback remains provisional and
unlaunched, with no frozen protocol, plan or generated cell.

This completed component is one prime n23, m2, BSGS slice of the native
planning matrix. The 64 feature rows all share field size n23 and therefore
belong to the same whole-field-size split component; they do not supply
validation/test components by themselves. The next campaign dependency is
executable solver coverage and frozen budgets for the requested binary,
Koblitz and larger prime families, arities m2..15, and method arms. Exact
first/last fall-degree certificates and whole-worker retired instruction
counts are still null in the native reporting contract and are not inferred
from these BSGS cells.
