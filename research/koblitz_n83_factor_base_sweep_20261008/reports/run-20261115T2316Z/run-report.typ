#set page(paper: "us-letter", margin: 0.65in)
#set text(font: "New Computer Modern", size: 9pt)
#set heading(numbering: "1.")

= Current-head N83 root library validation

#text(size: 9pt)[Run 2026-11-15T23:16Z · Source `acbda5793b6b19f752bc6ea5e3024565dca72b8b` · Shared macOS arm64 · Rust 1.99.1]

*Observed result: required validation failed.* The frozen full release-library suite ran to terminal exit 101. Its receipt records a 23:16:44Z start, 23:52:00Z end, child exit 101, wrapper exit 101, and no stop signal. The disk guard did not fire. The suite reported 6,302 passed, 63 failed, 138 ignored, and 0 filtered in 2,116.12 test seconds. The exact N83 checkpoint test passed within the suite.

#figure(image("failure-counts.svg", width: 100%), caption: [Recorded failed-test counts in two full-suite attempts. The earlier source and launch conditions differ; the graph is a validation comparison, not an IC runtime comparison.])

== Failure comparison

#table(
  columns: (2.2fr, 1fr, 1fr, 1fr),
  inset: 5pt,
  [Attempt], [Passed], [Failed], [Ignored],
  [Earlier `c01b1b6d`], [6,305], [60], [138],
  [Current `acbda579`], [6,302], [63], [138],
)

Both attempts had 6,503 tests. Their failing-name sets have 59 entries in common. Four newly failing `diagnostic_child_v2_tests` report “diagnostic root has unprotected shared writable ancestor”; one earlier deadline failure is absent here. The current temporary root was under `/Volumes/SSD990-2`, whose ancestor is group-writable without a sticky bit. The root validator rejects that condition. The code for those tests is byte-identical to current `origin/main`, so temp-root placement is the supported explanation for these four additional failures. Other failures are not all independently reproduced on the base branch.

== Source and custody

The frozen protocol is `verification/k600-m6-root-library-warm-20261115/PROTOCOL.md`. The command was `cargo test --release --lib --locked --offline -j 1 -- --test-threads=2`, wrapped with a 7,200-second timeout, pinned Rust 1.99.1, the warm SSD Cargo target, and the SSD scratch directory. The complete raw log is `lib-test.log.gz`, 118,230 compressed bytes, SHA-256 `1880c40ad4c23f61e15ad33d88d0dbb53459c58bb4697de466a8edec1f11016f`; it decompresses to 638,332 bytes, SHA-256 `443137637bf74676e09315faf76fd342652150ebb4b2433c59a3b2757b78a638`. `receipt.txt`, `failures.txt`, `source-commit.txt`, and `plot-data.csv` are beside this report.

This shared-host warm validation interval is excluded from cold IC cost. The K600 solver arms, matrix rank, target descent, total IC runtime, and matched rho remain unmeasured. Canonical IC graphs remain unchanged because this run supplies no IC performance row. A qualified Linux release-suite validation with a private temporary root remains required, followed by the distinct source-bound build and hard-cgroup preflight before any charged K600 relation search.
