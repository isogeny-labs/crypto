# r30 S3 index verification attempt 02

This read-only verifier was submitted to inspect the completed r30 point-base S3 file index and join the three family manifests to all 384 shard records. The Job never obtained a Pod/node and failed at its 600-second active deadline. It produced no logs and no result objects, so it provides no evidence about the contents or correctness of the r30 index.

Preserved S3 source data remain under s3://isolab-results-104186811093-us-east-2/runs/n83-fb-variants-r30-20261031-r1/attempt-01/. The verifier prefix is s3://isolab-results-104186811093-us-east-2/runs/n83-fb-index-verify-r30-20261031-02/. Retry with a fresh job name only after r31 releases its node.
