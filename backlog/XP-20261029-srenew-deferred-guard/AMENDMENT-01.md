# XP-20261029-srenew-deferred-guard — amendment 01 (route only, before any execution)

**Status:** recorded before the first execution of this protocol. No session of
`spec-P{24,28,32}.json` has run. `PROTOCOL.md` and the three specs are unchanged and
keep their frozen hashes. The amendment changes only who builds and launches §4's
image. It changes no arm, workload, seed, size, decision rule or prediction.

## What happened

§4 named `isolab-experiment.yml` as the route, with `source_revision`
`be2969cebf30a0cfb79088fd10094324c3efb3bb`.
- **Dispatches:** P28 (run 40828477966) and P24 (run 40828482134).
- **What succeeded:** in both runs the contracts and build jobs succeeded.
- **What failed:** both `experiment` jobs failed at "Obtain temporary AWS credentials", with `Not authorized to perform sts:AssumeRoleWithWebIdentity`.
- **What did not happen:** no image was published, no job was launched, and the S3 prefixes `runs/gh-40828477966-1/` and `runs/gh-40828482134-1/` are empty.
- **Why:** the trust policy of the workflow's role requires the OIDC subject `repo:aburan28/crypto:ref:refs/heads/main`. The repository now reports itself as `isogeny-labs/crypto`.
- **History:** no `workflow_dispatch` of this workflow has ever reached the launch step.

## Amended route

The same commit is built with the same tested-commit scripts, and launched by the
executor's existing AWS identity, through the AWS launcher route the stride study used.

1. **Build.** `taskq/deploy/eks-karpenter/build-bundle`, taken from `be2969ce` itself, is run on a clean checkout of `be2969ce`:
   - the checkout is a sparse checkout of the Cargo manifests, `src/`, `tools/factor-base/` and every target path in `Cargo.toml`;
   - the flags are as the script sets them: `cargo +1.99.1 build --locked --release --no-default-features --target x86_64-unknown-linux-musl --bin ecbench` with `RUSTFLAGS='-C target-cpu=x86-64'`;
   - its `build-info.json` records the binary's sha256.
2. **Image.** The steps are those of `publish-image` at `be2969ce`:
   - the same three-file context and the same `Dockerfile.ecbench`;
   - `docker build --platform linux/amd64`, pushed to `isolab/tasks`;
   - the image digest is resolved from ECR.

   The tag is `local-20261029-dg-<revision>` instead of `gh-<run>-<attempt>-<revision>`, because there is no workflow run.
3. **Launch.** `taskq/deploy/eks-karpenter/remote/launch-remote` is run with an `isolab.eks.request/v1` request:
   - `job.build = {image_uri@digest, source_revision, ecbench_sha256}`, checked by `validate-request`;
   - `provenance.harness = "claude-code-local"`;
   - job names `ic-dg-p24-20261029`, `ic-dg-p28-20261029` and `ic-dg-p32-20261029`;
   - c8a.2xlarge, 3,600 s;
   - the spec embedded byte for byte from `spec-P<b>.json` at `be2969ce`.

   Frozen launch cards from `experiment-director` are run unchanged by `executor-mechanical`.
4. **Integrity.** These are unchanged from §4:
   - each job's `build-info.json` must name `be2969ce`;
   - results are checked against `SHA256SUMS`;
   - a host loss reruns the size whole under a new job name.
