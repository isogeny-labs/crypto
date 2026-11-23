#set page(paper: "us-letter", margin: (x: 0.8in, y: 0.65in))
#set text(font: "New Computer Modern", size: 10pt)
#set heading(numbering: none)
= N83 V5 Linux exporter build: attempt 2

*Observed result:* The frozen retry reached the pinned Linux ARM64
container and stopped in offline dependency resolution. Cargo returned
exit *101* because the image registry lacks `aws-credential-types`.
The run was 2026-11-16 14:13:19–14:13:21 UTC.

The capacity amendment was committed at `d1ecd2440873`, retaining
the code source `cedc6dc8ea85bacf8b3b687177cd98cb0ba9ecff`.
Host disk admission passed. Docker inspect reported 12 GiB memory,
zero swap, four CPUs, no network, and no OOM. Cleanup completed.

#figure(image("admission.svg", width: 100%), caption: [Reached stages and dependency stop.])

The source snapshot was checked against the clean checkout.
The compiler and solver were not reached; there is no Linux exporter
binary or IC timing. A future attempt can stage the existing host Cargo
registry into a new output path while retaining offline resolution.

== Source-linked records

`../PROTOCOL.md`; `../AMENDMENT-2.md`; `../run-build.sh`;
`prelaunch-df.txt`; `source-attestation.json`; `build.log`;
`child.receipt`; `outer.receipt`; `container.inspect.json`; `disk.tsv`.
