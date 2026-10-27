# Primary N83 Linux XOR panel: provider admission stopped

The [four exact extended-DIMACS inputs](admission.json) are the source-pinned
hash/Gray 5+1 and 4+2 instances in the preceding
[external-XOR handoff](../external-xor-20261025/PROTOCOL.md). Their local
SHA-256 digests match that handoff's committed receipt. They were uploaded
with `If-None-Match: *` and AES-256 server-side encryption to the dedicated
`s3://crypto-autoresearcher/factor-bases/icv1/etc/koblitz_n83_factor_base_sweep_20261008/linux-xor-panel-20261025/inputs/`
prefix. Each object was fetched independently; all four downloaded byte
streams match both the source receipt and S3's SHA-256 checksums.

Conductor allowed the unique directory and task T-5132 claimed it. The
dedicated `n83-xor-panel-20261025` EC2 role can read only this panel's
input prefix and put receipts only under its output prefix. The instance
profile also carries AWS's managed SSM core policy. The intended worker was
one Ubuntu 24.04 `c7i.2xlarge` in `us-west-2`, with IMDSv2, an encrypted
30-GiB root volume deleted on termination, no SSH key, and a 90-minute
instance-initiated shutdown set to terminate. AWS first returned
`DryRunOperation`. The actual `RunInstances` request then returned
`Blocked`: “This account is currently blocked and not recognized as a
valid account.” A subsequent EC2 query for task T-5132 returned no instance.
The launch was not retried after the provider admission failure.

No Linux solver process ran. There are no Linux timings, models, relations,
rank measurements, target descent, or matched rho from this attempt. The
Mac panel remains a censored solver-stage diagnostic, and
`selected_best_total_runtime` stays null. The S3 inputs, policy, intended
launch controls and [local replay script](audit_admission.py) are retained
for a later admitted provider run. A future run must pin its own Linux solver
binary and resource envelope before timing; Darwin intervals cannot be
pooled with Linux intervals. The dedicated IAM role is unused but retained
for this exact-input AWS continuation.
