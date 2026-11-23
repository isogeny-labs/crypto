#set page(paper: "us-letter", margin: (x: 0.75in, y: 0.72in))
#set text(font: "Helvetica", size: 10pt)
#set par(leading: 0.7em)
= N83 Colima bootstrap compilation

*Run:* `run-20261116T0036Z`  ·  *Source:* `0f558a0f7998`  ·  *Status:* producer bootstrap passed

The source-bound Linux ARM64 bootstrap ran from 2026-11-16 00:36:41 to 01:00:47 UTC. The measured outer interval was *1,446 seconds (24 minutes 6 seconds)*; Cargo reported 23 minutes 19 seconds. The committed protocol and launcher are in `verification/k600-colima-bootstrap-20261116/` at the source commit above. The command was `cargo build --release --locked --no-default-features -j 1 --example n83_k600_m6_linux_build`.

#image("bootstrap-flow.svg", width: 100%)

The child and Docker exit codes were 0. Docker reported `OOMKilled=false`. The hard cgroup checks required 12 GiB memory, zero swap, four CPU quota and 512 PIDs. The output was a Linux ARM64 ELF with SHA-256 `5d7dea0ae6afea06528b5680652f83e1a7f3309a9ce3241c607976a20213edb7`. The source checkout was clean after execution, and the named container was removed.

The raw evidence is `bootstrap.log` (16,367 bytes, SHA-256 `9c49b9b8c78ff976d3112f93d4082e0fb71358240fd4f267282fa8e1a798089c`), `child.receipt`, `outer.receipt`, `container.inspect.json`, and `source.commit`, all beside this PDF. The local 1.3 GiB Cargo target directory is a build cache, not durable custody; its producer can be rebuilt from the committed source, lockfile and pinned image.

This is one setup measurement, so the diagram marks subsequent work as pending rather than plotting a comparison. The charged five-binary/solver build, K600 worker, relation yield, rank, descent, matched rho and fully charged cold one-target IC cost are unmeasured. The 1,446-second setup interval remains in the cost ledger until a frozen comparison protocol states how reusable setup is priced. No canonical IC graph changes from this result.
