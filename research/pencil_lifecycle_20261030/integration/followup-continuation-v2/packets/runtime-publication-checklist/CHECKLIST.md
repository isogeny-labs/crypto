# Next runtime qualification and scoped publication checklist

Prepared read-only from current local files and /var/tmp/pencil-continuation-audit-20261030T2348Z. No build, image assembly, binary execution, cloud API, publication or deployment was performed. Root's final ecbench build is pending: final source manifest, binary hash, base-image digest and runtime-image digest remain null in status.json. Historical binaries/images are not substitutes.

## Bytes ready for concrete review

All hashes were recomputed from local bytes. Controller/progress source still matches all 19 retained manifest entries; fresh-owner source matches all 5. No reason to rebuild these unchanged artifacts was found.

| Artifact | SHA256 |
|---|---|
| Controller ZIP |7791cec77e94d1fa710383a5004dfb0628125c03ee54eaedfd1a007475600fd8|
| Runtime archive |2c10901aa723ba47e35e3cda1ca801331cb10428926d35a5b0074641b3d2bf95|
| Controller/supervisor |b0403250273d2e06e58b18a745f221f2c8d9881979afbe027fdb5f8c237628a6|
| Progress observer |65509d5a2ed54622b36212efc6ddf2505f4d7dacde40ac640a9a9e5e6faf0273|
| Fresh owner |f6f6016e549ab19d9e92a7a89a5a89d095de068775dd003ab5666217726b48b7|

Exact paths and sizes: status.json. ZIP and archive package manifests already verify internal checksums, unchanged dependencies and modes. Source manifests: controller/progress79047cc0133d522074e852bfbc592c5702d2d3732c9b52c84874ce969f5c4cfb; owner1e2afffc10f732503eb4680d64f879fe5cd735a8ed5c86cd09d545a8ea311cfe.

Prior explicit publication approval named ZIP0bd84d40… and runtime72b227c0…, not these replacement bytes. Obtain scoped approval for the replacements and concrete final images after their bindings are ready. No IAM/RBAC expansion is part of this checklist.

## Minimum build/assembly work remaining

1. Reconcile root's running final ecbench build; do not duplicate it. Required target recipe from build-bundle is `cargo +1.99.1 build --locked --release --no-default-features --target x86_64-unknown-linux-musl --bin ecbench`, with `RUSTFLAGS='-C target-cpu=x86-64'` and build-jobs2. Offline may be added only with existing dependencies. Final source identity, flags, toolchain, receipt and executable hash must describe that completed build. A host-only or tests-only build is not this image executable.
2. Once root accepts that build, stage the final ecbench and retained owner using `install -m755`, and emit build-info.json with both exact hashes, immutable source revision, rustc, architecture linux/amd64, musl target and default_features=false. Preserve the existing owner's own source manifest/build receipt rather than attributing its bytes to a new build. Do not run the full build-bundle wrapper solely to rebuild unchanged owner.
3. Assemble only the explicit base-image context: ecbench, isolab-fresh-owner, build-info.json and smoke-prime.json. Dockerfile.ecbench pins public.ecr.aws/aws-cli/aws-cli@sha256:b94fc7612d131f1a3cb76145ba3385c813ad7561ef2c2b60c89fbd5dfee93d39, places the owner at /usr/local/bin/isolab-fresh-owner, and runs USER1000:1000. Record the exact Dockerfile/context hashes and base digest. Its source_repository validator currently expects the historical aburan28 URL; record it accurately rather than silently changing the metadata contract.
4. Assemble the runtime overlay from the exact archive over that new immutable base digest. Dockerfile.runtime has ARG BASE_IMAGE, COPY runtime/ /opt/isolab/recovery/, USER0:0 and no RUN. Owner belongs in the base, not this overlay. The historical failure was image progress mode0700 despite archive0755; a reviewed explicit COPY --chmod=0755 for runtime executables, or equivalent verified mode-preserving context, must address actual final-image accessibility. Do not assume archive header modes establish final-layer modes.
5. Qualify the actual final image in an uncredentialed bounded environment: extracted file hashes/modes, static x86 ELF/no interpreter, base and runtime manifests, owner presence/hash, final ecbench/build-info equality, actual intended user/custody-path permissions and the intended helper/runner lifecycle interface. Preserve the prior private backstop result as only its scoped shutdown test. Qualification command/cap/fixtures still need to be frozen for the actual final image; no scientific target execution follows merely from image assembly.

Trusted script interfaces (not run): `publish-image BUNDLE_DIR COMMIT OUTPUT.json` builds/pushes the base and emits OUTPUT.json.fresh-owner.json; `publish-runtime BASE-IMAGE.json OUTPUT.json` version-GETs the overlay archive, verifies it, builds/pushes and emits the runtime/base/archive triple. These are publication actions, not local dry runs. Use only after scoped approval and source/publisher review. Prefer reusing locally assembled exact artifacts when supported; do not replace evidence with another unbound rebuild.

## Exact controller source-only change set

- Bucket: isolab-results-104186811093-us-east-2.
- Proposed controller key: inputs/recovery-controllers/7791cec77e94d1fa710383a5004dfb0628125c03ee54eaedfd1a007475600fd8.zip.
- Proposed runtime key: inputs/recovery-runtime/2c10901aa723ba47e35e3cda1ca801331cb10428926d35a5b0074641b3d2bf95.tar.gz.
- Object versions remain UNSET until authorized publication and versioned GET/hash readback. Do not fabricate or reuse earlier versions.
- On the existing isolab-recovery-controller stack, change only LauncherKey and LauncherVersion. Expected resource delta: logical Project (AWS::CodeBuild::Project), Properties.Source.Location and Properties.SourceVersion. Preserve every other parameter and the deployed template; compare the generated change set against the actual deployed configuration before execution. No Role, Access, Ingress, namespace/RBAC, retry/concurrency, VPC, policy or permissions changes.
- The repository deploy-controller wrapper is unsuitable for this narrow update: it rebuilds controller/MCP, uploads, applies namespace/RBAC and deploys a whole template with CAPABILITY_IAM. Do not call it as shorthand for source-property replacement.
- Runtime publisher binding must name the new runtime key, actual version, exact archive hash and final base image digest. Its current runtime.json is historical until those explicit bindings are reviewed. Retain both base-image fresh-owner sidecar and runtime/base/archive receipt; runtime output alone does not bind the owner.

## Admission remains a separate dependency

Reuse all nine retained enrollment inputs; do not regenerate1260 workloads. Replace pending preflight/source slots with root's final reviewed results, not audit-era defect status. Before scientific admission, finish the prospective amendment/protocol, request/image/source/enrollment bindings and runtime qualification; reconcile existing EKS/controller/owner receipts with the existing scoped role and compute remaining active-worker charges. Preserve the25200-second campaign ceiling and2026-10-31T06:00:00Z absolute deadline, one active payload and previously recorded failures. No remote state was queried for this checklist; remote idleness is unknown.
