# Resume: crypto @ codex/curve25519-atlas-carry-rank-20261104

## Goal
Pursue the requested isogeny-representative, limb-carry encoding and rank-aware
factor-base ideas. Study: research/curve25519_atlas_carry_rank_20261104/.

## Lead / hypothesis
Existing lead atlas mechanisms: LEAD-ICP-isogeny-transported-relation-yield,
LEAD-STR-class-representative-ic-selection and
LEAD-ICP-digit-presentation-solving-degree. Compare identical transported
support, identical signed query models and training-only selection with
held-out queries. No novelty or performance claim is made.

## Spec
research/curve25519_atlas_carry_rank_20261104/PROTOCOL.md.
Small-prime panel p=2029,8173,32749, N=8, 32 targets per panel, seeds
13,29,47,71. Source model y^2=x^3+(486662 mod p)x^2+x. The actual RFC
Curve25519 fixture is functional arithmetic checking, not a relation run.
Protocol is review_required; prerequisites are named in it.

## State
Native C++/GMP development instrument, fixed product tests, emitted relation
constraints and fixture replay exist. Producer-only check observations are
in research/curve25519_atlas_carry_rank_20261104/checks/. No experiment was
launched and no measured cost or speed ratio exists. Full-query SMT tests
hit their development timeout; retain them as unresolved.

## Next steps
From this repo: make -C research/curve25519_atlas_carry_rank_20261104 test.
Finish general model replay and frozen solver orchestration. Read the study
README and checks before rerunning anything. Reconcile the branch PR and its
checks; do not duplicate it. Before experiments, get scripts/checkpoint.sh
gate to pass, publish the exact source, freeze a source-bound custom image
and canonical machine protocol, confirm budget headroom and the EKS route.
Do not substitute the stock ecbench validation fixture for this binary.

## Runs in flight
None. Inspect checks/ and /tmp/curve25519-encoding-check-6 for the pinned
fixture test. The root library build completed, but its broad test run was
stopped after isolated missing-fixture failures; raw output is in
checks/cargo-release-lib-4.log.gz. No remote job or paid resource was created.

## Stop and success conditions
Prospective EKS limit: USD 5 total including build/failures, four vCPUs,
4 GiB, 900 seconds per job, zero retries, original absolute deadline fixed
at launch. Stop if the checkpoint, identity, image, protocol or budget gate
fails. A component gain needs its frozen paired comparison and independent
replay; passing development tests alone is not such a gain.
