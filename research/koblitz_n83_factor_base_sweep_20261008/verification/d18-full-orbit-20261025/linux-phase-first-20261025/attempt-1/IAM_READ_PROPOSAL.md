# Exact D18 read grant proposed for the hosted run

The GitHub workflow currently authenticates as IAM user
`rho-gh-snapshot-readonly-20261024`. Its existing policies remain intact.
[`exact-read-policy-proposal.json`](exact-read-policy-proposal.json) would add
an inline policy named `N83D18ExactInputRead20261025` with one Allow
statement: `s3:GetObject` for only the D18 `manifest.json` and its exact
`c295da88…jsonl.gz` object. It grants no listing, write, or wildcard
resource. The policy has **not been applied**.

This change would persist beyond the experiment until removed and would let
GitHub Actions fetch the two private objects from the user's AWS account.
Apply it only after the owner explicitly approves this exact cross-service
read and egress. After authorization, use `aws iam put-user-policy` with the
reviewed JSON, confirm the effective policy and fresh GETs, then rerun the
failed job of [run 40220890684](https://github.com/aburan28/crypto/actions/runs/40220890684)
on its frozen source commit. Retain the IAM and provider receipts. If the
read is needed only for this run, remove the inline policy after its terminal
artifact is saved and verify that the grant is gone.
