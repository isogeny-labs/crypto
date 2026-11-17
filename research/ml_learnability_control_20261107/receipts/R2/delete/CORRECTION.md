# Correction to the delete rationale (2026-11-08, 08:07Z)

The delete request and commit b502a9e8dc attributed the `cpu-spot` scheduling block to the terminal mllc pod. That
attribution was wrong.

After the delete, the scheduler still reported "1 node(s) didn't match pod anti-affinity rules" for
`sgsecant-panel-20261108-a2-8q5t6`. The EKS audit log shows a second job pod bound to the same node
(`ip-10-84-107-139`, the only `cpu-spot` node at r8i.4xlarge / 16 vCPU) at 05:53:42Z:
`pq2-msolve-n37-e63bf8e5-s8kgm`. It is not part of this study round, and the mllc pod had shared the node with it. The
blocking pod is therefore that job, not the deleted mllc pod. It is not this session's to touch.

The delete itself removed only the failed mllc Job. Its outputs had already been salvaged (card R2-S2) and its S3
results are untouched, so the delete was harmless but unnecessary.
