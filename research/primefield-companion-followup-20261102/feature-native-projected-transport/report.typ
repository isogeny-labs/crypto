#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= Paired projected adaptive transport
2026-11-07 | Development integration | PCC-20261107-8ba257

#image("flow.svg", width: 100%)

*Construction.* Replace each monomial exponent e by m e modulo p-1, where m is the odd part of p-1. Project the geometry residual and calibration by the same exponent. The paired wrapper checks every evaluated sign and homogeneous projected ratio against the unchanged full-value source. Order-two and order-four images are supported. [pcc:unverified]

*Observed tests.* Two selected native invocations pass: 499 complete-subgroup rows, 100 split windows, 1375 advances and 450 skipped-sign steps, plus limits/rejections. Fifty-one other methods are filtered in each invocation. Fixtures are F5/a2/b1/P(0,1)/r7; F7/a0/b3/P(1,2)/r13; F509/a2/b10/P(3,68)/r479. Evidence: receipt.json, test.log, limits-receipt.json and source-lock.json. Producer evidence remains unsigned. [pcc:unverified]

*Scope.* The full-value reference is mandatory. All geometry lanes and zero checks remain, including factors with zero projected exponent. No scalar recovery, cloud job or comparative panel ran. The original cohort is unchanged. Shared geometry and residual dependencies mean this is not independent verification.

*Next construction.* Establish which domain checks are implied by subgroup assumptions before deleting zero-exponent factors. Investigate projected geometry residuals without full-value construction. Count projection, initialization, residual and validation work before making a speed claim.

Sources: DERIVATION.md; preceding sign-projection derivation; inherited adaptive monomial source. Cyclic-group background: Sutherland, MIT18.782 Lecture3, https://math.mit.edu/classes/18.782/2013fa/LectureNotes3.pdf. KB unavailable in this runtime. No novelty-priority claim.
