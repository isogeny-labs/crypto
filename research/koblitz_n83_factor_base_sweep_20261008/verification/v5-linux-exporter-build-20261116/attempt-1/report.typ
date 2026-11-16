#set page(paper: "us-letter", margin: (x: 0.8in, y: 0.65in))
#set text(font: "New Computer Modern", size: 10pt)
#set heading(numbering: none)
= N83 V5 Linux exporter build: attempt 1

*Observed result:* Host admission stopped with exit *93* at approximately
2026-11-16 14:03 UTC. The launcher emitted `boot below 2 GiB` before
creating a source snapshot, Docker container, or Cargo build output.

The frozen protocol and launcher were committed at `5ae32d781cbd`.
The intended code source was `cedc6dc8ea85bacf8b3b687177cd98cb0ba9ecff`.
The post-stop disk record shows 1,150,912 KiB boot space available;
that later observation is not an exact launch-time measurement.

#figure(image("admission.svg", width: 100%), caption: [Frozen admission path and observed stop.])

The custom Colima ARM64 profile was running, but the boot-space gate
failed. The empty post-stop container inventory and absent source/output
paths are consistent with the launcher's gate order. This attempt
establishes no compiler, relation, rank, or complete-runtime result.
A new versioned attempt needs adequate measured headroom.

== Source-linked records

`../PROTOCOL.md`; `../run-build.sh`; `command.txt`; `exit.txt`;
`stderr.txt`; `post-stop-df.txt`; `colima-status.txt`; `containers.txt`.
