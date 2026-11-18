#set page(paper: "us-letter", margin: (x: 0.8in, y: 0.65in))
#set text(font: "New Computer Modern", size: 10pt)
#set heading(numbering: none)
= N83 V5 Linux exporter build: attempt 3

*Observed result:* The offline registry copy verifier stopped with
exit *103* before Docker or Cargo. The launcher emitted
`registry copy differs`. Its dry-run output says it skipped a
symlinked `src` tree. The copy step preserved that symlink, whose
target lies outside the Colima mounts.

The amendment and launcher were committed at `e200b4d5b1c7`.
The code source, image, resource limits, and offline policy did not
change. No compiler or solver work was reached.

#figure(image("admission.svg", width: 100%), caption: [Registry-copy verifier stop before Docker.])

The next attempt needs a new input path containing a dereferenced
extracted source tree, plus a comparison that checks those bytes.
Merely ignoring the verifier warning would leave a broken path
inside the container.

== Source-linked records

`../AMENDMENT-3.md`; `../run-build-attempt-3.sh`; `command.txt`;
`exit.txt`; `stderr.txt`; `verifier.stdout`; `symlinks.txt`;
`post-stop-df.txt`; `containers.txt`.
