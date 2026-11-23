# Publication and component review addendum

The original artifact set is frozen at commit
`371019835c0c03a50f29889fbce1be1b05aa9403`, tree
`65dee0dab7df199a781821e3c1265d323231ef64`. Its README, report, raw records
and original SHA256SUMS are unchanged. [pcc:unverified]

The separate reviewer bound its read-only checks to that commit in
[VALIDATOR-PUBLICATION-02.json](checks/VALIDATOR-PUBLICATION-02.json), with
[publication-evidence-02.sha256](checks/publication-evidence-02.sha256).
It reports no new discrepancies: 86 copied local artifacts agree with the
published subset, all 96 original checksum entries match Git blobs, and all
87 native source files match the tested snapshot. It reparsed the recorded
245 passing cases without rerunning tests. [pcc:unverified]

The disposition supports **published artifact consistency within scope**.
The broader execution claim remains insufficient for an independent execution
attestation or complete remote build provenance. Raw compiler JSON and native
binaries remain local-only, and the reviewer could not rehash the deleted
installer archives. Both PCC claims retain unsigned, unverified standing; this
same-team review is not a signed PCC verdict. [pcc:unverified]

The parent inspected both rendered pages of the unchanged two-page PDF;
[pdf-inspection.json](checks/pdf-inspection.json) pins the reviewed PDF and
local rendered images. The reviewer did not perform a second visual review.
Doctests, strict lint, Rust 1.78 compatibility, root regression, general
mathematical audit and the full 84-case comparison remain outside this
qualification. [pcc:unverified]

The complete local handoff includes the original compiler JSON, qualified
binaries, source payload and immutable reviews. `native-source.tar` is made
from the 87 hash-checked working files after the metadata-only publication;
it is not claimed to be a successful Git archive. An initial Git-archive
metadata command was stopped after 74 seconds with zero output; its empty
file and diagnostic are retained. Native tar readback checks the same 87-file
source manifest. No solver experiment was retried. [pcc:unverified]

The next work remains source-bound precision/cost auditing and selection of
the applicable optimized comparator for the larger solver study. S3 remains
excluded; access to it is not introduced as a blocker.
