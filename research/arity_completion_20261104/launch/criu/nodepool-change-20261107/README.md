# cpu-spot NodePool limit raise: attempt from this session (2026-11-07), not applied

Owner choice: raise NodePool `cpu-spot` to the policy limits in
`taskq/deploy/eks-karpenter/cost-controls.json` (cpu 16, memory 128Gi), and
temporarily allow-list this session's egress /32 for the EKS public endpoint.

| Time (UTC) | Step | Result |
|---|---|---|
| 21:31 | Snapshot endpoint access | `cluster-access-before.json`: `3.134.144.201/32`, `99.100.36.89/32` |
| 21:31–21:32 | Add `160.79.106.143/32` (update `75c870fc…`) | Successful |
| 21:32 | `patch-cpu-spot.sh` (get, dry-run, patch, readback) | **failed before any read**: TLS handshake timeout to the API endpoint |
| 21:33 | Diagnosis | session egress rotates across a pool (`160.79.106.131`, `.21`, `.244` seen) and the egress proxy re-terminates TLS and reset the tunnel; a /32 cannot work and a wider range was not approved |
| 21:33–21:34 | Remove the CIDR (update `0f76ba4b…`) | Successful; `cluster-access-after.json` equals the before set |

No Kubernetes object was read or changed through this path. The NodePool
limit is unchanged.

## For an operator on the harness host (3.134.144.201)

`patch-cpu-spot.sh` is the exact procedure (resourceVersion-tested JSON patch,
server dry-run, apply, readback that requires cpu "16" and memory "128Gi").
It needs a kubeconfig for an identity with NodePool patch rights. The harness
host's instance role `spot-experiments-WorkerRole` maps only to
`isolab:submitters` in EKS, so it is not enough on its own. Adjust the `SP`/`R`
paths for that host.
