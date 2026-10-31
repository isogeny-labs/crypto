# Round 13 evidence

[Report](REPORT.md) · [PDF](report.pdf) · [proof map](PROOF_MAP.md) ·
[ledger](ledger.json) · [protocol](PROTOCOL.md) · [diagram](routes.svg).

The ordinary antidiagonal500 has two simple degree250 points. The ordinary
product-plus-one580 has four simple degree145 points. Both retained
projected images vanish for all six points, under the written modular,
pairing and descent bridges. Saturation advances through inverse,
shared-factor, four-leaf and characteristic-two square relations on all
four circuits. Complete h-saturation and proper polarized embeddings
remain open. No IC efficiency verdict or rank promotion is supplied.

The raw tree is immutable once sealed and stored deterministically:

    sha256sum -c runs.tar.xz.sha256
    tar -xJf runs.tar.xz
    sha256sum -c runs.sha256
    ./REPLAY.sh

Replay reconstructs15 JSON/checkpoint outputs and three generated Lean
modules byte for byte. Native Rust performs all arithmetic, searches,
record assembly and hashing. Python is layout only. ARCHIVE_VALIDATION.txt
records fresh extraction, replay and byte-identical repack. Failed and
partial attempts are preserved in the archive and described in the report.
