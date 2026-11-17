#set page(paper: "us-letter", margin: (x: 0.8in, y: 0.65in))
#set text(font: "New Computer Modern", size: 10pt)
#set heading(numbering: none)
= N83 V5 Linux exporter build: attempt 4

*Observed result:* The disk watcher stopped the offline Linux ARM64
build after 18 min 27 s. SSD990-2 available space fell to 1,893,556 KiB,
below the frozen 2,097,152 KiB floor. The Docker child exited 137,
with `OOMKilled=false`; inspection and cleanup succeeded. Cargo had
compiled the library, but no exporter binary was produced.

The source was pinned to `cedc6dc8ea85bacf8b3b687177cd98cb0ba9ecff`.
The amendment and launcher were committed at `3c16b3dfe617`.
Resource gates: 12 GiB RAM, zero swap, four CPUs, offline and no network.

#figure(image("disk.svg", width: 100%), caption: [All recorded SSD990-2 available-disk samples in attempt 4; dashed line is the frozen 2 GiB stop floor.])

#figure(image("admission.svg", width: 100%), caption: [Build reached library compilation; exporter binary and IC measurement remained censored.])

== Source-linked records

`../AMENDMENT-4.md`; `../run-build-attempt-4.sh`;
`child.receipt`; `outer.receipt`; `disk.tsv`; `disk.stop`;
`container.inspect.json`; `build.log`; `post-stop-containers.txt`;
`input-output-size-kib.txt`; `sha256.txt`.
