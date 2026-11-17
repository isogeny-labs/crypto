# Read-only capacity reconciliation

The EKS overview showed nine failed and one suspended existing job; no new job or restart was requested. The overview's 64 configured vCPUs and 32/128 pool values are compiled-in descriptions, not authoritative live enforcement.

Both direct live reads exited 1. Their stderr preserves an AccessDenied at STS AssumeRole for the kubeconfig remote-launcher role. `kube-exec-role.json` initially looked only for --role-arn, so its null value is an incomplete argument extraction, not evidence that no role was requested. No credential bytes are retained. The caller is the worker role; its current context name spot-job-submitter does not prove the executable is using the approved research job-submitter role. No role/IAM change was attempted. Cloud budget balance and live caps remain unknown; no cloud build/job is authorized by this state.
