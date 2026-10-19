# Round11 evidence

[Report](REPORT.md) · [PDF](report.pdf) · [proof map](PROOF_MAP.md) ·
[ledger](ledger.json) · [frozen protocol](PROTOCOL.md) · [editable diagram](routes.svg).

The460 diagonal multiplicity gap closes:400 geometric points have
contacts1,2,6,14 with counts260,100,20,20,weighted degree860.
This is complete under the written characteristic2 moduli bridge.
A five-quadric coupled graph has complete projective saturation and six
charts. The four whole proper quotient models and seven whole
noncuspidal divisors remain incomplete. Ranks are unchanged.

Extract and verify the immutable raw archive here:

    sha256sum -c runs.tar.xz.sha256
    tar -xJf runs.tar.xz
    sha256sum -c runs.sha256
    ./REPLAY.sh

The native runner performs all mathematics,record assembly and hashing.
The Python report renderer performs layout only. REPLAY.sh reconstructs
four new JSON outputs and compares them byte for byte. The previous
round10 is also replayed in this round's raw validation records.
ARCHIVE_VALIDATION.txt records fresh extraction and deterministic repack.
