# Round24 failures and controls

- o24-charts-01: all displayed base and tower/inverse residuals are zero, but
  Singular's map application requires a named object and rejects p(det(J)).
  This interrupts both residue loops. Despite its later success marker and
  native exit 0, the wrapper rejects the error diagnostics with exit 1. No
  Jacobian or complete branch claim uses this attempt. The correction binds
  each determinant to a polynomial before mapping; inputs/caps are unchanged.
- A private uploader setup was interrupted before receiving credentials or
  uploading anything; the succeeding uploader is a single interactive owner.
- A documentation patch did not match its expected context and changed no file.
- lean-01: the 1+1 versus numeral-2 normalization and an unimported
  interval_cases tactic fail elaboration. The compiler prints sorryAx for
  the failed declarations; exit 1 is retained and no proof/audit pass is
  claimed. Preserve the exact failed source. Replace the numeral conversion
  explicitly and use the already available omega tactic with d nonzero.

All R01-23 failures, capped normalizations, validation limitations and missing
first-host raw logs remain as previously recorded. No failed computation is
a nonexistence result. No root implementation or test expectation is changed.
