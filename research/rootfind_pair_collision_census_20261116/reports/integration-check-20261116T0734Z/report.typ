#set page(paper: "a4", margin: 19mm)
#set text(font: "New Computer Modern", size: 10pt)

= Opt-in collision harvest: integration check

*Receipt:* integration-check-20261116T0734Z  \
*Source commit:* `a9fa59faf5e11b223efdbff47eb55408a8f150fb`  \
*Claim standing:* `PCC-20261116-fdce53` unverified

The source-pinned release `ecbench` binary suite completed with 21 passed,
1 ignored and 0 failed on local macOS ARM64. The fixed K13/d5 fixture uses
standard-subspace dimension 5, target scalar 641, walk targets, seed 811,
and a 20,000-trial ceiling. Existing and opt-in harvest engines both
recovered 641 and passed full group verification. The companion test matched
K13/K19/K23 collision counts and ranks to the census and rejected a modified
witness. These are producer checks in one session.

#figure(
  image("integration-diagnostics.svg", width: 100%),
  caption: [Recorded operation and relation diagnostics for one K13 fixture; integration-check-20261116T0734Z. Bars are counts or modeled GAE, not time.],
)

The raw diagnostic line in `stderr.txt` records 36 versus 23 target trials,
17 versus 13 target-bearing relations, 305 versus 440 setup additions, and
2,857 versus 2,370 total modeled GAE. The harvest contributes 5 independent
homogeneous seed rows. Its additional 135 setup additions equal the recorded
27 witnesses times 5 extra additions each. Row construction and matrix
seeding are charged by the implementation.

No per-arm wall or CPU measurement was made; the test-suite duration is not
an arm timing. Signed independent verification and the frozen EKS
performance panel are pending. The complete command, stdout, stderr, exit,
environment and dirty state sit beside this PDF. The canonical IC graphs
are unchanged because this fixture has no comparable performance entry.
