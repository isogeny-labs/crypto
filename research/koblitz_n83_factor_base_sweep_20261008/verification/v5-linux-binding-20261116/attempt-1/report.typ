#set page(paper: "us-letter", margin: (x: 0.8in, y: 0.65in))
#set text(font: "New Computer Modern", size: 10pt)
#set heading(numbering: none)
= N83 V5 Linux retained-K600 binding

*Observed result:* A fresh read-only S3 GET returned the exact
3,025,798-byte K600 object, independently verified by SHA-256.
The pinned Linux ARM64 exporter accepted V5 ordinal `26960859236`
and rejected both the adjacent ordinal and a changed manifest digest
with exit 1. Docker, inspection and cleanup exited 0;
`OOMKilled=false`. The three-second host interval is an
admission observation, not an algorithm timing.

#figure(image("admission.svg", width: 100%), caption: [Exact read and binding passed; two negative controls rejected. Relation, rank and target gates were not run.])

The source snapshot was
`cedc6dc8ea85bacf8b3b687177cd98cb0ba9ecff`; exporter
SHA-256 was
`87fdd7af02c140467b72225960c2c88be1af443b028d2ac99a8b25b611fa1236`.
No IC runtime or factor-base ranking follows from this admission.

== Source-linked records

`../PROTOCOL.md`; `../run-binding.sh`; `outer.receipt`;
`docker.receipt`; `aws-identity.json`; `s3-head.json`;
`s3-get.json`; `object.sha256`; `run/exact.json`;
`run/adjacent.exit`; `run/bad-manifest.exit`;
`container.inspect.json`; `OBJECT_POINTER.md`; `sha256.txt`.
