# Alternate AWS EC2 capacity screen, 2026-11-15 UTC

**Status: READ_ONLY_CAPACITY_GATE.** This checked a second AWS account for one Linux ARM64 host after the earlier `ecc-challenges-new` Spot launch was rejected before allocation. It did not launch an instance, build a binary, run the K600 solver, or measure IC runtime.

`aws sts get-caller-identity --profile default --query '{Account:Account,Arn:Arn}' --output json` identified account `590183823895`, user `adam`. The `ecc2k130` profile returned the same account and user. The existing `ecc-challenges-new` profile identifies account `104186811093`, user `coreuser`; its earlier exact m7g.xlarge Spot launch was rejected with `Blocked: This account is currently blocked and not recognized as a valid account` and is documented in [the rejection record](AWS_ARM64_SPOT_LAUNCH_REJECTION.md).

In account `590183823895`, AWS Service Quotas returned `0.0` vCPUs for both EC2 standard-family on-demand quota `L-1216C47A` and standard-family Spot quota `L-34B43A08` in each region below. The same command was run for each profile/region/code, substituting the region and quota code:

```sh
aws service-quotas get-service-quota --profile default --region us-west-2 --service-code ec2 --quota-code L-1216C47A --query 'Quota.Value' --output text
aws service-quotas get-service-quota --profile default --region us-west-2 --service-code ec2 --quota-code L-34B43A08 --query 'Quota.Value' --output text
```

| Region | Standard on-demand vCPU quota | Standard Spot vCPU quota |
| --- | ---: | ---: |
| us-west-2 | 0.0 | 0.0 |
| us-east-1 | 0.0 | 0.0 |
| us-east-2 | 0.0 | 0.0 |
| us-west-1 | 0.0 | 0.0 |
| eu-west-1 | 0.0 | 0.0 |
| ap-southeast-1 | 0.0 | 0.0 |

`m7g.xlarge` was listed as an offered instance type in all four `us-west-2` availability zones. An offering is not quota or allocation. With the reported zero vCPU quotas, this account provides no admitted m7g.xlarge route in the six checked regions. This is a selected-region quota screen, not a claim about every AWS region, instance family, or other provider. No quota increase was requested. A usable larger-memory host, a restored local container engine, or an authorized quota increase would reopen the charged build gate; the exact source, input, resource and control checks would still apply.
