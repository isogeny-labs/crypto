# Read-only closure of four packaging findings

All four scoped findings are addressed in the inspected root state:

1. Fresh-owner Cargo.lock is present in the Git index as mode100644, blob e3e1d6b622c8e0fc7132d94a3cac9c2d6dab889c. Both index bytes and local bytes have SHA256 ed3f704907cb840e3c219c41a9a9f100d5f5867eed5c0598afea60a32670ea36.
2. isolab-experiment.yml build artifact upload explicitly includes isolab-build/isolab-fresh-owner alongside ecbench and build-info.json.
3. The tested-source materialization command explicitly includes taskq/deploy/eks-karpenter/resume/fresh-owner under the minimal profile.
4. The root publisher mock fixture SHA25698301bb91c77f170f9de187f38c49e8fc6812dcf39d810bbf1502bcf2ee69aaa exactly matches the privately tested proposal.

This closes only the four identified local packaging/source-delivery mismatches. It does not establish final regression, CI, successful remote workflow publication, deployed image qualification or research execution. No builds, cloud calls, test executions or shared edits were performed during this closure check. Parent-reported root shell/YAML/mock validations remain separately owned evidence. Frozen research source was preserved.
