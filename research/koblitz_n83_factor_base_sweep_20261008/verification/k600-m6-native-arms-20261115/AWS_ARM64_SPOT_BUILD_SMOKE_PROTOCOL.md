# Frozen AWS ARM64 build feasibility smoke

Status: **prepared, not launched**. This is a larger-memory engineering
control following the two local [6 GiB OOM results](LINUX_ARM64_BUILD_SMOKE_ATTEMPT2.md).
It tests whether the exact code revision
`0ae1150458da16d5becc3316d0cf9217c43b3209` can compile the
five K600 Linux ARM64 examples. It does not run a K600 relation search
and cannot supply a cold index-calculus cost.

The proposed host is one EC2 `m7g.xlarge` Spot instance in
`us-west-2c`: AWS reports four ARM64 vCPUs and 16 GiB RAM. Use
Amazon Linux 2023 ARM64 AMI `ami-0c09db4d8b86a32d2` from the
public SSM parameter on 2026-11-15, a 50 GiB encrypted gp3 root
volume deleted on termination, default-VPC public subnet
`subnet-00eb1bfd197da78b8`, and default security group
`sg-0b8f33eabbf97881c`. No key pair, IAM instance profile,
new inbound rule, or S3 permission is requested. The account's
`SpotOnlyEC2Launch` policy explicitly denied an on-demand dry run;
the equivalent Spot dry run returned `DryRunOperation`. The latest
read-only Spot price-history entry was $0.0878/hour at 2026-11-15
09:00 UTC; it is a quote, not a fixed bill. Set a $0.15/hour maximum
Spot price and a persistent request valid only until
`2026-11-15T19:00:00Z`, with stop on interruption. AWS allows stop
for persistent Spot requests; a one-time request would not preserve
the root volume on shutdown. The stopped request must be cancelled
and its instance terminated after the console receipt is secured.

Launch exactly one instance using `ecc-challenges-new` in
`us-west-2`, with this protocol's committed user-data file:

```sh
aws ec2 run-instances --profile ecc-challenges-new --region us-west-2 \
  --image-id ami-0c09db4d8b86a32d2 --instance-type m7g.xlarge \
  --subnet-id subnet-00eb1bfd197da78b8 \
  --security-group-ids sg-0b8f33eabbf97881c \
  --count 1 --associate-public-ip-address \
  --instance-market-options 'MarketType=spot,SpotOptions={MaxPrice=0.15,SpotInstanceType=persistent,InstanceInterruptionBehavior=stop,ValidUntil=2026-11-15T19:00:00Z}' \
  --block-device-mappings 'DeviceName=/dev/xvda,Ebs={VolumeSize=50,VolumeType=gp3,DeleteOnTermination=true,Encrypted=true}' \
  --instance-initiated-shutdown-behavior stop \
  --metadata-options 'HttpTokens=required,HttpPutResponseHopLimit=1' \
  --tag-specifications 'ResourceType=instance,Tags=[{Key=Name,Value=n83-k600-arm64-build-smoke-20261115}]' \
  --user-data file://research/koblitz_n83_factor_base_sweep_20261008/verification/k600-m6-native-arms-20261115/aws-arm64-spot-build-smoke-user-data.sh
```

The exact command with `--dry-run` returned
`DryRunOperation: Request would have succeeded` on 2026-11-15;
this checks authorization only, not available Spot capacity. The
user-data file is 3,335 bytes and passed `bash -n` and
`shellcheck -s bash` before launch.

The exact [user-data script](aws-arm64-spot-build-smoke-user-data.sh)
fetches only the public GitHub branch and checks out the full source
SHA above. It requires a clean sparse source tree, pulls the same
Rust 1.99.1 ARM64 image ID as the local attempts, and builds the same
five examples with `--locked --no-default-features --release -j 2`.
The Docker cgroup has 12 GiB RAM, no swap above that, four CPUs and
512 PIDs; the Cargo build is capped at 2,400 seconds. A systemd
safety timer powers off the host after 65 minutes, and the script
powers it off on both success and failure. Use instance-initiated
shutdown behavior `stop`, retrieve the console log and exact
exit/output hashes, then terminate the stopped instance and verify
the root volume's deletion. The user-data script sends its compressed
log to the EC2 serial console with explicit delimiters and a raw-log
SHA-256; it sends nothing to S3. If the console log is truncated,
mark the run incomplete and retain the stopped volume until another
read path is authorized.

Success requires shell exit 0 and five executable Linux ARM64 ELF
binaries with recorded sizes and SHA-256 values. Preserve network,
clone, source, package, build, timeout, OOM, interruption and
console-custody failures distinctly. This smoke uses an otherwise
fresh target, but host provisioning and toolchain setup are separate
from the source-bound `charge build` receipt. The solver installation,
runtime image source label, frozen host-specific build plan, zero-trial
preflight and K600 arm controls remain pending even if the smoke passes.
No performance graph or factor-base ranking can change from this
build check.
