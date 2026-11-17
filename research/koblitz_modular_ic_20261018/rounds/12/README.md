# Round 12 evidence

[Report](REPORT.md) · [PDF](report.pdf) · [proof map](PROOF_MAP.md) ·
[ledger](ledger.json) · [protocol](PROTOCOL.md) · [editable diagram](routes.svg).

Complete supersingular closed supports and multiplicities are supplied for
all eight coupled functions. The antidiagonal splits into supersingular
degree 740 and remaining ordinary degree 500. All seven whole unfinished
divisors remain open. Full homogenized quotient circuits have explicit
counterpoints and a generated linear saturation step of membership depth
at most 5. Proper quotient embeddings remain open. Ranks are unchanged.

The raw tree is immutable and stored as one deterministic archive:

    sha256sum -c runs.tar.xz.sha256
    tar -xJf runs.tar.xz
    sha256sum -c runs.sha256
    ./REPLAY.sh

Replay reconstructs five JSON files and the actual-boundary Lean data
byte for byte. The previous round is separately replayed. Native Rust
performs all mathematics, record assembly and hashes. Python is layout
only. ARCHIVE_VALIDATION.txt records fresh extraction, replay and repack.
