# Capacity correction before measurement

The complete-checkout vector/Gray run launched on-demand as requested.
Attempts to launch the baseline and final Semaev run on-demand were rejected
before boot by the account's shared32-vCPU Standard On-Demand quota. No
measurement was produced by those launch refusals. No other job was stopped
and no quota or account setting was changed.

Their corrected launches use ephemeral Spot instances of the SAME
c7i.2xlarge type, through the separate96-vCPU Standard Spot quota. This
supersedes the on-demand wording for those two retries only. Per-job host
manifests, single-worker/core reservation, A/A, five-pair comparisons,
output verification, budgets and all scientific gates are unchanged. Spot
interruptions remain incomplete runs. Do not pool different hosts.

All corrected jobs use complete checkouts: initial patch-preparation errors
from the two sparse attempts stay retained in preparation-failures/.
