# Iso Lab result URI correction

The maintained `taskq/deploy/eks-karpenter/submit-job` blob `3ccdbd09438e14fc5b2cfed5127d8f00e1a919e6` sets `ISOLAB_RESULTS_URI` to `s3://isolab-results-104186811093-us-east-2/runs/<job>/<pod-uid>/`. The first proposed runner expected the separate checkpoint bucket and would have failed in a real pod. The corrected runner checks the exact maintained URI, job, pod and node; an offline test accepts that contract and rejects the checkpoint bucket. No job or image ran. [pcc:unverified]

`development.tar.xz` losslessly retains the pinned launcher source, blob identities, final syntax/test/gate/whitespace logs. `source.sha256` pins corrected branch files. This is source review and mock verification, not authenticated L2 execution or S3 result readback. [pcc:unverified]
