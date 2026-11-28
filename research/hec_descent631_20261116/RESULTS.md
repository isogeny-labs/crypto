# Verified F631 staged transfer solve

The registered F631 source/target computation recovers k=15648086. Public scalar multiplication verifies B=kD1 on the original hyperelliptic source and on the recovered plane-quartic destination classes. The source check rejects k+1. No expected scalar was supplied.

For each original input, two rational representatives times ten fixed anchor pairs give20conic attempts. Four pass and sixteen have nonsplit residuals; all successful descents agree within each input. Derived projected logs are53659370forD1and40380864forB, relative to the certified point-log normalization. Their ratio modulo59296481is15648086. The calibration projected log is7720582.

Each candidate conic resultant is checked against an independent Sylvester determinant at all631field elements. Accepted residuals are reduced/disjoint/split, and actual target group arithmetic verifies the conic divisor identity. The complete four-point canonical divisor replaces the older two-point special case. Point logs describe4(P-O); the derived class logs likewise describe fourfold projections. The calibration image already includes factor4, so its projected log uses another factor4on the point-value difference. The common projection factor cancels in the final scalar ratio. Mixed-primary rational centering was checked during input recovery.

Freezeaa1dce88ec63/checkpoint20261116T025045Z precedes one execution. Exit zero, stderr empty. Every exclusion remains in RUN.txt. Recovered classes, verified parametrizations, point logs and quotient setup are reused from their separate frozen stages. This is a staged correctness result, not a fresh cold-path timing measurement or proof of the global isogeny.

The earlier failed charged F127 comparison remains unchanged. No measured speedup, tenfold gain, novelty or exponent improvement is claimed here. Full cold integration and charged same-input comparisons remain, with genus2/4mechanisms and broader applicability also unresolved. The full objective stays open.
