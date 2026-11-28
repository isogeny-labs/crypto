# Offline L2 request admission control

`check-l2-request.sh` accepts a scratch frozen source/checkpoint/image/request closure and rejects changed isolation, retry, schedule, image, checkpoint and runner identities. It checks the maintained source-checkpoint path layout and exact native result job shape. The checked-in source manifest remains blocked, so no real request is admitted here. [pcc:unverified]

`development.tar.xz` retains full final control logs, primary source blob IDs and the two initial local rejection diagnostics. `source.sha256` pins the current guard, test and protocol prose. The mock checkpoint is not a real S3 upload/readback, and the checker does not authenticate a remote ECR image or submit an EKS job. Live service verification and independent result replay remain required. [pcc:unverified]
