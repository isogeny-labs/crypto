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
- o24-series-01: the order helper reports the highest degrees of nonmonomial
  polynomials (for example 179 for a series beginning w^7), so its subsequent
  nu division and original-source residuals fail. The wrapper rejects the run.
  Those numbers are not valuations or a nu-pole result. Replace the helper
  by the degree of the last term in this univariate global ordering and add
  explicit unit/order-1/order-7 controls before evaluating any chart.
- The initial Round24 wildcard archive ignore also excluded the temporary
  checkpoint-evidence archive from source checkpoints. No migration occurred
  and the native raw files remain present. Add an explicit unignore before
  the next checkpoint; verify its inclusion. The final immutable run archive
  is separately sealed and published. Earlier checkpoints are not claimed to
  contain those ignored raw bytes.
- cf24-corrupt-01 is an expected rejection, not a failed search. Adding1 to
  the actual source factor product gives residual1 and the exact source
  product error. Singular still exits0 and prints the final marker; the
  wrapper rejects it with exit1. This records the detector's required behavior.
- The first downloaded-checkpoint member query omitted the leading ./ in
  ./untracked.tar.gz. The corrected exact-member query confirms that the
  checkpoint contains the temporary raw archive. No missing upload is inferred.

All R01-23 failures, capped normalizations, validation limitations and missing
first-host raw logs remain as previously recorded. No failed computation is
a nonexistence result. No root implementation or test expectation is changed.
