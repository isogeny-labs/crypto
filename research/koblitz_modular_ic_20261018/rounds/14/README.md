# Round 14 evidence

[Report](REPORT.md) · [PDF](report.pdf) · [proof map](PROOF_MAP.md) ·
[ledger](ledger.json) · [protocol](PROTOCOL.md) · [geometry](GEOMETRY.md) ·
[diagram](routes.svg) · [validation](VALIDATION.md).

The five remaining ordinary components have57 simple closed points with
complete even/odd images under the written global bridges. Forty fresh
split columns give35 even and30 odd nonzero images. The joined principal
rows have nonzero-image ranks15/15; adding eight separate deck-fiber rows
gives18/17 on1200/1190 columns. All eight F2 closed signed divisors are complete: O28 enumerates44
closed cusps and refines the earlier O27 stage; see [the cusp bridge](CUSPS.md).
Full h-saturation and actual proper polarized equations remain open.
No IC efficiency verdict is supplied.

The write-once raw tree is sealed deterministically:

    sha256sum -c runs.tar.xz.sha256
    tar -xJf runs.tar.xz
    sha256sum -c runs.sha256
    ./REPLAY.sh

Native Rust performs all arithmetic, record assembly and hashing. The
final replay reconstructs102 mathematical JSON/checkpoint outputs and
four generated Lean modules. The proof map records the65 named Lean
additions and their exact limits; the final audit count is in the ledger.
Failed runs, caps and the corrected character action remain archived.
