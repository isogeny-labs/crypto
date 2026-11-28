#set page(paper: "a4", margin: 19mm)
#set text(font: "New Computer Modern", size: 10pt)

= Guarded pair-sum reuse

*Source:* `d85f6a9107d2fcef09be530f6fd327c184335969`  \
*Receipt:* guarded-reuse-sums-20261116T0805Z  \
*Claim:* `PCC-20261116-28659f` unverified

The opt-in verifier recomputes both colliding pair sums, checks their sign,
validates the negative-index map for an equal-sum row, and reuses those sums
in the final full group identity check. A modified witness and a corrupted
negative index are rejected in the fixed K13/K19/K23 census test.

#figure(
  image("guarded-reuse.svg", width: 100%),
  caption: [Source-pinned K13 setup additions and modeled total GAE before and after guarded reuse. One fixed correctness fixture; no arm timing.],
)

Candidate counts and ranks remained 27/5 at K13/d5, 99/11 at K19/d7,
and 201/23 at K23/d8. Both harvest revisions recovered and group-verified
public scalar 641 at K13, with 23 target trials and 13 target-bearing rows.
The setup count changed from 440 to 386 additions; modeled total GAE from
2,370 to 2,316. Native index checks are outside that GAE total. The pinned
release `ecbench` binary suite passed 21 tests with one ignored.

Exact commands, environment, test-binary hash, dirty state, stdout, stderr
and exit are beside this PDF. The separate full library suite was interrupted
after eight unrelated `ecbench` test failures, with partial output retained
in the preceding intermediate receipt. Signed independent verification and
the frozen L2 F4/F5/F6 panel remain pending.
