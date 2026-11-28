# CM-CACHE-COLD-V2: calibration admission refused

The one calibration launch stopped at native isolation preflight with exit3, before fork/exec of cache-eval. The receipt records other_cpu_seconds=1.8 over the two-second preflight, above the protocol's0.2 threshold; CPU/memory PSI are both0. No calibration rows file exists, no held-out launch occurred, and no unchanged retry was made. No point-action timing, phase cost, A/A value or speedup exists for this attempt. This is an operational admission outcome, not an algorithmic comparison.

Native software qualification completed before this attempt:24 synthetic analyzer checks,60 independent GMP expected point outputs,60 subgroup memberships and three changed-input/output rejections. Existing prefix/cold qualification records are linked from CACHE_ANALYSIS.md. The independent field implementation uses no OpenSSL point arithmetic. Allfive curves remain in the protocol, with P-192 primary.

Source/checker/launch9f59b94289896ead81df3b7f190df71832005220 and frozen protocol d4b2f1be961c67b20526b8184907fc5b39bb1f68 were verified hosted before the attempt. Canonical protocol body digest:0bbaa17b83ae883e2b72c62c611177b4d4f134e9099c102933a082ba054cbf4f. The exact launch is protocol/cache-cold-v2/launch.sh, invoked from the repository root. Its copied bytes, empty stdout/stderr, exit, refusal and stopping record are immutable raw artifacts. The budget was one120-second/2GiB/thread1 attempt per panel, held-out conditional on valid calibration; no cloud submission/cost, cap/role/budget change or generator restart occurred.

## Custody and replay

The original attempt-001 tree remains locally ignored and unchanged. An exact copy is archived under runs/attempt-001 to follow the repository archive convention. runs.tar.xz is1232bytes, SHA2569d1ce72c11cf4cb726d76c7792789ff563141497b809522d9bd9d110fb52c648. runs-manifest.sha256 binds every raw member. admission-receipt.json and summary.json expose the recorded outcome without needing to extract the raw tree.

From this directory verify `sha256sum -c runs.tar.xz.sha256`, then extract with `tar -xJf runs.tar.xz` into a fresh directory and verify `sha256sum -c runs-manifest.sha256` against that copy. These are artifact replays, not benchmark retries. No comparative analysis is run on absent data. Preserve the refusal; another execution needs a distinct authorized protocol amendment and a genuinely quiet admission environment. Existing EKS cost/quota restrictions do not authorize moving this timed run to a new cloud job.
