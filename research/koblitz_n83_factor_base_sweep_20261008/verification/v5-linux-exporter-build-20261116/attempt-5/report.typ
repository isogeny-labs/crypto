#set page(paper: "us-letter", margin: (x: 0.8in, y: 0.65in))
#set text(font: "New Computer Modern", size: 10pt)
#set heading(numbering: none)
= N83 V5 Linux exporter build: attempt 5

*Observed result:* The offline Linux ARM64 release exporter build
completed with child, inspection and cleanup exits all 0. The child
receipt spans 17 min 27 s. Docker reports `OOMKilled=false`; the
disk watcher did not stop the run. The 9,884,664-byte ELF has SHA-256
`87fdd7af02c140467b72225960c2c88be1af443b028d2ac99a8b25b611fa1236`.

The source was pinned to `cedc6dc8ea85bacf8b3b687177cd98cb0ba9ecff`.
The attempt-5 protocol and cache-preservation amendment were committed
before execution. The source and dereferenced registry were read-only;
the fresh Cargo target was on writable boot-volume virtiofs.

#figure(image("disk.svg", width: 100%), caption: [All 105 boot-volume availability samples; the 2 GiB stop floor was never approached. SSD990-2 minimum was 3,579,248 KiB.])

#figure(image("admission.svg", width: 100%), caption: [Exporter build passed; binding, relations, rank, target verification and full IC timing remain separate.])

== Source-linked records

`../AMENDMENT-5.md`; `../AMENDMENT-5A.md`;
`../run-build-attempt-5.sh`; `child.receipt`; `outer.receipt`;
`disk.tsv`; `container.inspect.json`; `build.log`;
`binary.file`; `binary.sha256`; `binary.bytes`;
`setup/guest-mount.txt`; `sha256.txt`.
