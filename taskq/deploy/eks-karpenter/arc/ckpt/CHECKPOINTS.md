# Checkpoints in S3 for the extra-high-memory runners

A closure that runs for days on Spot capacity cannot restart from nothing each
time the node is reclaimed. `src/cryptanalysis/closure_checkpoint.rs` saves the
mathematical state of the dense MutantXL scan, `scripts/ckpt-s3-sync.sh` copies
each committed generation to S3, and `.github/workflows/dreg-ckpt-job.yml` runs a
job as a chain of segments that each restore the newest generation. This file is
what the cluster owner has to provide; nothing here has been applied.

## What a loss costs

State is saved at every word boundary of a round's elimination (the unit of
progress inside the long step), between rounds, and after the first elimination of
the Macaulay matrix. It is written when `DREG_CKPT_SECONDS` (3600 in the workflow)
have passed since the last save, and when the process gets SIGTERM or SIGINT.

- A Spot reclaim gives two minutes. The pod gets SIGTERM at once
  (`terminationGracePeriodSeconds: 110`), so the closure saves if a save fits in the
  time: several minutes per 200 GB (measured here: 2.5 GB in 6–9 s, about 0.3–0.4 GB/s, so 8–11 minutes for 200 GB on that disk; the cluster's gp3 volume is not measured), which it does not. Plan on
  losing up to one interval (an hour) plus the time from the last copy to S3.
- **A save is only in S3 once the copy finished.** If the node dies before the copy
  does, the previous generation in S3 is what the next segment restores. The copy
  of a 200 GB generation takes about 10 minutes at 3 Gbit/s inside one region.
- **Disk:** a save commits the new generation after deleting the older of two, so the peak on the node is two states, plus the build. A failed periodic save (a full disk) is logged and retried after an interval; the run goes on.
- Not covered: the Macaulay matrix build and the first elimination before the
  first save (hours at degree 5; it is repeated), and the XL basis, whose answer
  is kept once it is done. This is stated in the module documentation.

## What the owner provides

1. **A bucket in the cluster's region** (`us-east-2`). The lab bucket
   `s3://crypto-autoresearcher` is in `us-west-2`; copying hundreds of GB across
   regions costs $0.02/GB each way. A lifecycle rule that expires `dreg/` objects
   after 14 days keeps the cost to what is in flight. Set the repository variable
   `CKPT_BUCKET` to its name.
2. **An IAM role for the runner pods**, trusted for the service account
   `arc-runners/arc-xmem-runner` through the cluster's OIDC provider
   (`aws eks describe-cluster --name isolab --query cluster.identity.oidc.issuer`;
   create the provider if it does not exist), with `s3-policy.json` attached.
   `trust-policy.json.tmpl` is the trust policy. The node role stays unreachable
   from pods (IMDS hop limit 1), so this role is the only AWS access they have.
3. **The service account**: `ROLE_ARN=... envsubst < serviceaccount.yaml | kubectl apply -f -`.
4. **The node class** `nodeclass-xmem.yaml` (a 1200 GiB gp3 root volume at
   16000 IOPS and 1000 MiB/s, about $0.27 per hour while a node exists), referenced
   from the `gha-xmem` NodePool's `nodeClassRef`, and the xmem scale sets' pod
   templates set `serviceAccountName: arc-xmem-runner` and request 600 GiB (512 GiB
   pool) or 800 GiB (1 TiB pool) of `ephemeral-storage`. These edits are to files of
   the PR that adds the pools and are made after it merges.

Then set the variable `DREG_CKPT_WATCH=c1-deg5` to let the two-hourly watchdog
start the job and restart it after a node loss, or dispatch
`dreg-ckpt-job.yml` once by hand. Delete the variable to stop automatic restarts.
The job stops chaining after 40 segments.

## Checking it without AWS

`scripts/tests/ckpt-s3-sync-test.sh` and `scripts/tests/ckpt-segment-test.sh` run
the copy and the segment logic against a directory-backed stand-in for `aws s3`,
including a segment cut by its deadline, one cut by a signal, a restore on a
"new node", and a push that must not publish a generation without its manifest.
