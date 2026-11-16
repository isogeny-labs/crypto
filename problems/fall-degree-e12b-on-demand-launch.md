# E12b On-Demand launch denied by the worker role

- **Status:** open
- **Kind:** access/tool limit
- **Affected work:** [Amendment 12](../research/fall_degree_bounds_20261008/PREREGISTRATION.md), E12b cell `3:24:7`
- **Next action owner:** AWS account administrator; experiment executor after access is resolved
- **Last updated:** 2026-10-28 UTC (2026-10-27 Pacific)

## Problem

The registered E12b completion cannot launch because the current worker role
explicitly denies On-Demand `ec2:RunInstances` through `SpotOnlyEC2Launch`.
Amendment 12 requires On-Demand after two Spot interruptions, so a Spot launch
would not execute the registered continuation.

The required cell is `(m,n,ell)=(3,24,7)`, `N=45`, four draws from seed
`20261012`, closure cap 4, from the unchanged archive named in Amendment 12.
Only one of these four draws completed in each previous attempt. The E12b
reading remains undecided until the complete registered cell and gates pass.

## Evidence and uncertainty

- **Observed:** [preflight receipt and command](../research/fall_degree_bounds_20261008/e12/on-demand-preflight-20261028/README.md)
  record AWS `UnauthorizedOperation`, shell exit 254, and the named explicit
  deny. Config and S3 reads succeed. Zero instances and zero draws were started.
- **Derived or proved:** no new mathematical result. The denial identifies
  an infrastructure restriction, not a solver failure or degree bound.
- **Suspected explanation:** none needed for the observed explicit deny;
  any further permission or capacity gaps are untested.
- **Missing capability:** administrator-authorized On-Demand launch access
  for the registered ephemeral runner.
- **What this does not establish:** it does not consume a launched experiment
  attempt, prove the role can launch Spot, or guarantee that other permissions
  and capacity will suffice after this restriction is resolved.

## What would unblock it

- [ ] An administrator provides an authorized environment permitting the
  registered On-Demand instance, with required source/config reads and runner
  profile access. Do not bypass the explicit deny by changing accounts or
  policies without authorization.
- [ ] Reconcile existing EC2 and S3 state, then a dry run of the intended
  launch returns `DryRunOperation` rather than `UnauthorizedOperation`.

These conditions clear the observed launch blocker only. Successful execution,
gates 0–4, four complete output rows, and the registered lower-median reading
remain separate obligations; none proves an asymptotic fall-degree theorem.

## Next discriminating step

- **Action:** after authorized access changes, repeat the launch preflight and
  execute only the existing Amendment 12 continuation if admitted.
- **Inputs and controls:** unchanged `fd-src-371035216942.tgz`; seed `20261012`;
  `keep=4`, `cmax=4`; gates 0–3 and exact non-timing equality of draw 0 with r1/r2.
- **Execution limits:** the existing protocol and account limits govern.
  An interrupted or failed actual run is not completed, with no further
  relaunch allowed by Amendment 12. No E12a/E12c retry is added here.
- **Outcomes:** permission success permits launch preparation; another denied
  preflight preserves the blocker; complete validated output permits the
  registered E12b reading. Partial output remains partial.
- **Recheck trigger:** administrator-confirmed change to the authorized
  execution environment, or discovery of an already-launched E12b-OD job.

## Resolution and history

Unresolved. Recorded after the 2026-10-28 UTC dry run. This record and its PR
do not amend the experiment, launch a job, or complete E12b.
