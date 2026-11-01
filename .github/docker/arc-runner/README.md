# Crypto ARC runner image

The stock ARC runner image has the GitHub Actions agent but lacks commands
used by existing `ubuntu-24.04` workflows, including `node` on `PATH` and a C
compiler. This image adds the common Linux build tools, Node 24, Python 3.12,
and Rust 1.98 with the musl target. It is based on the official runner release
`2.338.0`. Specialized GPU, perf, Sage and virtualization jobs still need
their own runners or should stay on GitHub-hosted runners.

Build from the repository root. Create the `isolab/gha-runner` ECR repository
once, then push with an IAM role permitted to write to it. Pin the resulting
immutable digest in both ARC scale set value files. The EKS Karpenter node role
can pull this repository.

```sh
image=104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/gha-runner
aws ecr create-repository --repository-name isolab/gha-runner --region us-east-2
docker build -t "$image:20261020-v1" .github/docker/arc-runner
aws ecr get-login-password --region us-east-2 \
  | docker login --username AWS --password-stdin 104186811093.dkr.ecr.us-east-2.amazonaws.com
docker push "$image:20261020-v1"
docker logout 104186811093.dkr.ecr.us-east-2.amazonaws.com
aws ecr describe-images --repository-name isolab/gha-runner --region us-east-2 \
  --image-ids imageTag=20261020-v1 --query 'imageDetails[0].imageDigest' --output text
```

Rebuild with a new tag when the official Actions runner release changes. A
stale runner binary was rejected as `Outdated` during deployment, leaving
jobs queued even though Karpenter had provisioned the node.
