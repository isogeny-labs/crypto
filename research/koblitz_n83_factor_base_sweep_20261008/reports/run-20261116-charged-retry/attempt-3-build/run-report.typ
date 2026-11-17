#set page(paper: "us-letter", margin: (x: 0.75in, y: 0.72in))
#set text(font: "Helvetica", size: 10pt)
#set par(leading: 0.7em)
= K600 charged build: compiled source path stop

*Source:* 6716be0e316c  ·  *Status:* setup stopped before stages  ·  *Scope:* local

The no-work admission passed in 422 seconds at clean source. The charged-build container then exited 1 with a 76-byte NotFound message. Its output directory and source-labelled image were absent. Docker inspection reported OOMKilled=false; outer cleanup passed.

#image("build-path.svg", width: 90%)

The bootstrap producer embeds its earlier checkout as CARGO_MANIFEST_DIR. Its first repository operation canonicalizes that path. This run mounted the current full checkout and omitted the earlier path; an independent image check confirmed it absent. The mismatch explains the NotFound before any stage began. The new producer must be compiled from the current committed checkout and its ELF/hash pinned before a fresh charged build.

The raw admission and build logs, terminal receipts, Docker inspection and source commits are beside the report. This attempt measured setup admission and an early stop only; it supplies no relation, rank, descent, matched rho or total cold IC result.
