#set page(paper: "us-letter", margin: (x: 0.75in, y: 0.72in))
#set text(font: "Helvetica", size: 10pt)
#set par(leading: 0.7em)
= Current-checkout N83 K600 producer bootstrap

*Source:* ea47352a1d0b  ·  *Status:* bootstrap pass  ·  *Scope:* charged setup

The source-bound Linux ARM64 run completed 2026-11-16 04:52:17–05:18:36 UTC, a derived whole interval of *1,579 seconds*. Cargo reported *18m56s* for its fresh output-local release build. The pinned container had 12 GiB hard memory, zero swap, four CPUs and 512 PIDs.

#image("bootstrap.svg", width: 91%)

The installed 846,392-byte ARM aarch64 ELF has SHA-256 b3dcec9005ce87b26a5a7d5b613781d8dbbc2d5534368d7524c4e36ee3941980. The current full-checkout path was found in the binary. Child, Docker-inspect and cleanup exits were zero; OOMKilled=false, source clean, and no named container remained.

The raw log, terminal receipts, Docker inspection, source commit, binary, machine summary and diagram source are beside this PDF. This run charges setup only. A new versioned admission must pin this binary before the five-binary/solver/image build. It does not measure relation yield, rank, descent, matched rho or total cold IC cost.
