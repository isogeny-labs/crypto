# Run 20261115T0551Z

Frozen q=8 direct-product falsifier on local arm64 macOS, one deterministic
run per cubic. Protocol: [`AMENDMENT_2.md`](../../AMENDMENT_2.md). Commands:
`cargo test --release` and `cargo run --release -- --q8-direct` against the
study manifest. Two tests passed; they cover q=2,4 controls. The q=8
direct run reports 182938 members per cubic, union 237839, and complement
24304 among the 262143 nonzero trace-zero elements. The frozen candidate
formula predicted 3577 and failed. There is no statistical interval.

Source SHA-256:
`7dcf817b3b87a78de36693f6121529e4b4a5aca9149314a20f20d799b94deb01`.
Executable SHA-256:
`c1986b7006ea3daa9b6bd04d5f468ef64841dfa66ed52fb25939d7fcfc416f7d`.
Host: Darwin arm64 `Mac.attlocal.net`, kernel 27.2.0. `time -l` recorded
26.57 seconds wall, 4.88 seconds user, 0.21 seconds system, and
5,750,784 bytes maximum resident size. Raw output:
[`result.txt`](result.txt), [`timing.txt`](timing.txt),
[`cargo-test.log`](cargo-test.log). Pre-run S3 checkpoint:
`20261115T055032Z-eacae612c8f8`. See [study report](../../REPORT.pdf).
The study PDF and quantitative SVG are copied into this run directory.
