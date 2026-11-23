# Item 1: the big-host run that decides P2 (blocked; specification)

2026-11-04 · status: **not run**. Everything below is fixed, so the run needs
only access and a launch.

## Why it did not run here

1. **Credentials.** The AWS key pair in this container's environment is
   rejected by STS: `aws sts get-caller-identity` returns `InvalidClientTokenId`.
   The isolab-eks skill requires the caller to verify as account
   `104186811093` before submitting.
2. **Contract fit.** The portable `isolab.eks.request/v1` request
   (`taskq/deploy/eks-karpenter/contracts/request.schema.json`) runs only
   `ecbench.spec/v1` jobs, with `seconds ≤ 3600` and no custom command. This
   workload is msolve, not ecbench. It needs the EKS MCP's `isolab_k8s_run`,
   which runs an authorized arbitrary command and is not installed in this
   container, or an ecbench harness extension (`ecbench-extend`).

## Workload (frozen)

- **Source.** Main after #3380 merges, which brings `round3/SYSTEMS.sha256`;
  `gen.sing` and `instances_r3.txt` are already on main.
- **Tools.** Singular 4.3.2 and msolve 0.6.5, from Ubuntu 24.04 packages
  `singular` and `msolve`.
- **Inputs.** Regenerate the systems, then check them byte-for-byte:
  ```sh
  cd research/ic_tree_gluing_20261103
  INSTFILE=instances_r3.txt CELLS="m5_t4_k0_T m6_t5_k0_T" OUTDIR=$PWD/bighost Singular -q gen.sing </dev/null
  ```
  Each `.ms` file must match its sha256 in `round3/SYSTEMS.sha256`; stop if
  either one differs.
- **Run.** One job per cell, alone on the node:
  ```sh
  TLIMIT=21600 VLIMIT=<node memory minus 2 GiB, in KiB> ORDER="m5_t4_k0_T m6_t5_k0_T" bash run.sh bighost 1
  ```
- **Resources.** At least 64 GiB of memory (128 GiB preferred) and one core,
  since msolve runs with `-t 1`. A 6-hour cap per cell.
- **Stop condition.** Each cell ends at the first of: GB complete, TLIMIT,
  or memory exhaustion. Record the log either way.
- **Records.** The two `.log` files plus `parse.sh` output, uploaded to S3
  with their sha256 and committed under `round4/logs/bighost/`.

## What each outcome means (registered in `.resume/RESUME.md`, round 3)

| outcome | reading |
|---|---|
| D_T(5,4) = 9 and D_T(6,5) ≤ 10 | P2 holds (E ≤ 5, 6). The tree stays the only scalable encoding, and CROSSOVER.md's β = 0 rows remain possible |
| D_T(6,5) ≥ 11 | P2 fails (E(6) > 6), and the tree line closes |
| any cell cut off again | still undecided; report the lower bound |

None of these outcomes moves the 256-bit comparison with rho. CROSSOVER.md
puts even the most optimistic law at 2^{156} or more against rho's 2^{128}.
