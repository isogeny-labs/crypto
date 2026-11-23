# Local pilot evidence

The measured result is in [report-02/report.md](report-02/report.md), with
standalone PDF/SVG/PNG artifacts. PCC-20261116-1075bb is an unsigned producer
measurement; PCC-20261116-52edbc records a conditional accounting observation.
No signed independent replay, L2 or fresh-family result is asserted.

The archived raw data contains every prepared input and complete original
result directory, including all failed A/A controls and missing remainders.
The executable/library archive retains the actual copied native/CAS/adapter
and renderer binaries and runtime libraries. archive-envelope.json pins their
bytes and the final report artifacts. Trusted artifact migration may replace
large data files with content-addressed .artifacts.json pointers.

draft-render-01.tar.xz preserves the initial draft report before its standing
phrase was corrected in the separately built renderer. It is archival draft
prose, not the maintained report; immutable original renderer/post-call records
remain. Two attempted Chromium conversions timed out; Inkscape and Pandoc/
XeLaTeX subsequently produced the final standalone artifacts. These rendering
attempts occurred after measurement and do not supply performance samples.

The benchmark record is
benchmarks/records/20261116T0144Z-local-compact-pilot-20261116.json. It is closed
and must not be rewritten. The global metadata checker lacks historical raw
payloads in this sparse worktree; this run's closure and progress regeneration
completed after restoring the existing pinned integrity-exclusion registry.
No new historical exclusion or source history rewrite was introduced.
