# Corrigendum to the independent blind algebra review

The frozen `report.md` has a sign error in the `a=2` row for `k=5,6,7`. This note corrects that row without editing the original report or attestation.

For `a=2`, `N=8`, `c=4`, and

`P(k)=(-1)^(7-k) k! (7-k)!`, so `P(c)=P(4)=-4!3!=-144`.

Direct substitution for `k=0,...,7` gives

`P(k)=[-5040,720,-240,144,-144,240,-720,5040]`,

and therefore the **correct** ratios are

`rho(k)=[35,-5,5/3,-1,1,-5/3,5,-35]`.

The original report printed `[35,-5,5/3,-1,1,5/3,-5,35]`. Only the final three signs differ. All eight corrected ratios remain nonzero 2-adic units, and `rho(4)=1`. The derivations of the node identity, center value, and valuation identity are unchanged. **All four original mathematical verdicts remain valid.** The `a=0`, `a=1`, binomial-row, and out-of-range checks are unaffected.

## New blind attestation

- `sources_read`: the original assignment prompt containing the definitions and targets; the parent's post-freeze arithmetic-control request identifying a possible sign discrepancy. I independently recomputed the product values above from those definitions.
- `joints_owned`: this independent arithmetic correction and its impact assessment.
- `repository_source_accessed`: no.
- `proof_or_proof_map_accessed`: no.
- `reports_receipts_or_git_history_accessed`: no; I did not reopen the frozen report, and used the values quoted in the correction request to identify the row under discussion.
- `parent_review_plan_accessed`: no.
- `sibling_report_accessed`: no.
- `builds_or_cloud_runs`: no.
