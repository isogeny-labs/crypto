# Amendment 2: authorized local Rust measurements

The user answered the missing-runner question with **"Use local or runpod"**
on 15 November 2026. This authorizes the local backend. Because that answer
directly addressed the unavailable checkpoint/EKS route, this continuation
uses a local checkpoint and published source/spec in place of that route.
It does not report S3 custody or EKS/L2 admission that did not occur.

## First local pilot, frozen before observations

- One known-answer cubic instance: `p=67`, fixture/base/oracle seed `3`.
- All five oracle engines and both protected eliminators; actual same-instance
  rho A/A controls as in Amendment 1.
- One retained warmup followed by five measured rounds. Default five-arm
  ordering and odd-round reversal are unchanged.
- Maximum 10000 residuals per IC arm; whole-process wall limit 900 seconds;
  memory ceiling 8 GiB; zero automatic measurement retries.
- Pin one allowed CPU and capture CPU/cgroup/OS/toolchain provenance. Wait for
  compiler processes to finish before timing. Native execution and subsequent
  research analysis use Rust; shell is thin capture/build/run orchestration.
- Published source SHA, binary SHA256, exact command, runtime manifest, raw
  JSONL, exit code and hashes live in a create-new local run directory under
  `/workspace/ic-local-runs/`. Oversized raw files stay outside Git, with their
  local paths/hashes recorded; remote upload remains pending availability.
- Retain every failure, timeout, incomplete block and warmup. A censored or
  incomplete stratum cannot yield a parity verdict. Do not select successful
  executions out of failed cases for an acceptance result.

This pilot selects which component to investigate next. It is not a
large-subgroup crossover test. Stronger reference implementations and larger
fresh holdouts remain necessary for the broader rho-parity objective.

## Local timing and analysis scope

Report wall ratios separately from the incomplete multiplication-equivalent
model. Compare graph and pivot on matching fingerprints, and check both rho
and pivot A/A work/scalars. For local timing controls, the predeclared median
A/A ratio must be within `[1/1.10, 1.10]` for each stratum. Report paired
geometric-mean ratios and deterministic bootstrap intervals when all planned
rounds are present. Label these local observations and producer-run checks;
the standard lab admission remains a different status. [pcc:unverified]

The Rust driver gains an explicit oracle selector, preserving the previous
all-five default. Successor experiments must freeze their selected subset,
new inputs and limits before observations; this is not permission to alter a
running or completed cohort. [pcc:unverified]
