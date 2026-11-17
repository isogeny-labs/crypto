# Native audit of the frozen F4 component

Run `cargo test --locked` and `cargo run --locked` in this directory. The tool
reads the checked-in before/after DLP rows and `summary.json` from the parent
study. It requires the four expected `p`/seed pairs, nonduplicated rows,
matching relations and residuals, successful recorded correctness flags, four
correct rho rows, zero reported mismatches, exact phase-counter sums, and the
reported normalized `S`, before/after ratio, rho mean, and `S`/rho ratios.

The archived rows do not contain recovered scalars or a replay certificate.
This audit checks their accounting and internal consistency; it does not
independently replay the DLP solutions. The Python leaderboard builder only
renders the frozen summary. CI runs this native audit on relevant PRs and
before the main-branch dashboard freshness check.
