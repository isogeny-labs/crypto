# First-information certificate packet

Start with REPORT.md, report.pdf, flow.svg and systems.csv. The 64 complete records are also in summary.json. PROOF.md states the boundary argument. This packet derives certificates from existing saved streams and contains no collector execution or new timing measurements.

Restore ignored artifacts using the repository artifact tool and the adjacent .artifacts.json, including ../signed-witness/certificates.tar.gz and ../signed-native-restarts/native-streams.tar. Then run `python3 check_only.py` from this directory for direct-sum readback. The checker reconstructs source rows, checks exact member closure and certificate hashes, validates all prefix and predecessor certificates and recomputes every table record. No Gaussian elimination runs in that readback.

`run.py` is the original producer; it overwrites derived outputs and should only be used in an isolated copy under the recorded validation limits. Original source hashes, archive bindings, initial readback, amendment and all exit receipts remain preserved. `remote-readback.json` records fresh official-store download checks and member verification.
