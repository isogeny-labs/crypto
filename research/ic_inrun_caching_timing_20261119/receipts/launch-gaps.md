# Launch gaps (disclosed before dispatch, 2026-11-19)

The launching session runs in a cloud container without an AWS CLI, kubectl or cluster
access. It dispatches through `isolab-experiment.yml` on `main` (GitHub OIDC
publisher), so these pre-launch steps from DISPATCH.md §0 could not run here:

- 0f `scripts/checkpoint.sh push/gate` (S3 resume checkpoint): not possible without AWS.
  The resume state is `RESUME.draft.md` in this directory, committed to Git.
- 0h `aws sts get-caller-identity` and `scripts/isolab_pool_check.sh` (capacity and budget
  headroom): not possible without cluster access. The workflow's own launcher checks
  capacity, the Job deadline (`seconds=4800`) bounds each run, and jobs are dispatched
  one at a time. The study ceiling stays USD 15 (DISPATCH.md §2), counted against the
  worst-case per-job cost.
- 0e pcc preregistration: optional for a T0 engineering study. Not filed.

Frozen revisions:
- OLD `104ca79ef997cb6be708559859b1a3bd342cea98` (= `ad2a014` + this directory only)
- NEW `ab42aff6d14572023fb8aca124e7b3d51774e97f` (PR #4756 head, code at `0f1a8cd8d`)

Spec SHA-256, identical at both revisions:
- `p24a-batch-sparse.json` `6def46f9df26d7688bd0acae132a65ac478ce61b998bf6ab048a290e78ecae41`
- `k0-13-descent-template.json` `f88f59566c33d4db8acf15591a9f6b9b54baaa84b7fbced5576a908e6d27f35a`
