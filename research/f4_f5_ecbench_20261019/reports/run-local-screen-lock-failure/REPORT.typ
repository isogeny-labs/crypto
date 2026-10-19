#set document(date: none)
#set page(paper: "a4", margin: 15mm, numbering: "1")
#set text(font: "DejaVu Sans", size: 9pt)
#set par(justify: false)
#heading(level: 1)[Local screen preflight failure]
#text("The first L0 screen stopped before creating a session: /tmp/crypto-bench.lock was not writable, exit2. Source/spec/binary manifests and stderr remain in runs/adaptive-screen. No solver result or elapsed-time comparison was produced.")


#text("The separate retry uses a private workspace lock, --cpus none and --allow-busy for correctness/work only. No isolation gate is bypassed for a native claim; the dedicated final panels retain their original gates. Both attempts remain evidence.")


#image("../../local-screen-lock-failure.svg", width: 100%)
