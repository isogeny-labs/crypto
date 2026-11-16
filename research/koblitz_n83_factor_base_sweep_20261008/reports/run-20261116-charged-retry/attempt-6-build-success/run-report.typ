#set page(paper: "us-letter", margin: (x: 0.72in, y: 0.66in))
#set text(font: "Helvetica", size: 9.5pt)
#set par(leading: 0.65em)
= N83 K600 source-matched charged build passed

*Source:* ba620214684e  ·  *Status:* three build stages PASS  ·  *Scope:* setup component, one attempt

The fresh Linux ARM64 build completed at 06:27:54–07:17:32 UTC, 2,978 seconds by second-resolution outer endpoints. The native manifest records `PASS_SOURCE_MATCHED_BUILD`, six output-local executables, and runtime image `sha256:b84bd73914a2c76a7c8635ee4d40564d974f0fc7188b9e71f315726050bae2b3` labelled with the source commit. Child, inspect, image-inspect and cleanup exits were zero; Docker reported no OOM and source-clean checks passed.

#image("build-cost.svg", width: 100%)

The Rust, solver and runtime-image wall-to-receipt-start intervals were 1,540.905, 523.518 and 199.725 seconds respectively, each PASS and uncapped. These do not sum to the complete outer interval because source checks, receipt writes, launch and cleanup are also charged. One attempt was observed; no spread was estimated. The separate no-work admission lasted 184 seconds by UTC endpoints. The disk watcher recorded 64 samples above the 5 GiB boot and 20 GiB SSD stop lines.

The solver repair extracted and compiled the pinned archives on Linux-local storage, checked both symlinks, then copied the finished ELF to output. Raw stage stdout/stderr, JSON receipts, Docker and image inspections, the outer receipt, binary hashes and the 45-second disk log are beside this report. This is a completed build setup component; relation collection, rank, descent, matched rho and full cold one-target cost remain for later frozen runs.
