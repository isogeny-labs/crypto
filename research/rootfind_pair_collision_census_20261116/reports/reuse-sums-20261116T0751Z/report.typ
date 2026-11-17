#set page(paper: "a4", margin: 19mm)
#set text(font: "New Computer Modern", size: 10pt)

= Reusing verified pair sums in collision rows

*Current source:* `6ab9a48d4f729b3178d2f273d8501e837ba3f1a8`  \
*Prior source:* `a9fa59faf5e11b223efdbff47eb55408a8f150fb`  \
*Receipts:* integration-check-20261116T0734Z and reuse-sums-20261116T0751Z  \
*Claim:* `PCC-20261116-bcb6f9` unverified

The opt-in harvest verifier independently recomputes both pair sums. It now
reuses them for the full four-point group identity check, spending one final
addition rather than rebuilding the four-point sum from its base points.
The sign check and rejection of a modified witness remain in place.

#figure(
  image("reuse-sums.svg", width: 100%),
  caption: [Recorded K13 setup additions and modeled total GAE before and after the verifier edit. One fixed correctness fixture, no arm timing.],
)

Both harvest revisions collected 27 collision candidates, 5 independent
seed rows and 13 target-bearing relations in 23 trials on the same public
K13/d5 target. Both verified scalar 641. The new source-pinned release
`ecbench` suite passed 21 tests with one ignored. Its fixed census test
matched K13/K19/K23 candidate counts and ranks and asserted three charged
additions per candidate. The recorded setup change is 440 to 386 additions;
modeled total GAE is 2,370 to 2,316.

These are producer operation diagnostics. The raw commands, environments,
stdout, stderr and exits are beside this PDF and its predecessor's report.
No per-arm wall or CPU time was measured. Signed independent verification
and the frozen L2 F4/F5/F6 panel remain pending; the canonical IC graphs
are unchanged.

This intermediate source trusted the factor base's negative-index map; the
subsequent guarded revision needs its own test receipt. The broader release
library suite was interrupted after eight unrelated `ecbench` test failures.
Its partial stdout, stderr and exit 101 are retained beside this PDF.
The 282,709-byte library stdout is selected for PR artifact-store migration;
the local upload was refused with S3 403.
