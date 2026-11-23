# Local image assembly proposal — final binary unbound

This proposal prepares two local images from an accepted final static ecbench build and retained runtime/owner bytes. It performs no Docker operation, publication, controller update or scientific execution. The completed b985 binary is historical and is not assigned to the final slot. Source revision/tree, final binary/receipt hashes and local image IDs remain null until the corrected-source build completes and is accepted.

## Exact recipe and retained inputs

Use the unchanged Dockerfile.ecbench: upstream public.ecr.aws/aws-cli/aws-cli@sha256:b94fc7612d131f1a3cb76145ba3385c813ad7561ef2c2b60c89fbd5dfee93d39; four explicit COPY inputs; source revision label; HOME=/work; WORKDIR/work; USER1000:1000; empty entrypoint. Its ecbench --help CMD is metadata and is not executed by assembly. Use unchanged Dockerfile.runtime: ARG BASE_IMAGE, FROM the accepted local base image ID, COPY runtime/ to /opt/isolab/recovery/, USER0:0, empty entrypoint. Neither Dockerfile has RUN. Reject nonempty upstream OnBuild before building.

The controller ZIP is custody evidence, not an image input. The runtime tar already contains isolab-eks-recovery and isolab-criu-progress, plus criu/tar/shared libraries. Fresh owner is a base input. No helper recompilation or archive regeneration is needed. Recompute hashes of retained bytes before staging and again in the completed context; bind all paths and hash values in bindings.json.

Create fresh exclusive base-context and overlay-context directories. Base context contains exactly ecbench0755, isolab-fresh-owner0755, smoke-prime.json0644 copied from retained l2-validation.json, and build-info.json0644. Dockerfile stays outside the context and is pinned separately. No repository, credentials, socket or home mount enters either context.

For overlay preparation, first inspect the verified tar without extraction; allow only exact listed ordinary-file/directory members under runtime/, reject symlinks, hardlinks, traversal, duplicates and unexpected members. Copy each verified ordinary member through an explicit allowlist into fresh overlay-context/runtime. Validate SHA256SUMS and BUILD.json against retained archive evidence. Do not blindly extract owner/mode bits.

Record a mode-only normalization ledger: runtime/ and runtime/lib/0700 ->0755; BUILD.json and SHA256SUMS0600 ->0644; isolab-eks-recovery0775 ->0755; progress/criu/tar/libraries remain0755. Preserve every file byte/hash. This explicitly fixes directory traversal as well as executable access for the UID1000 qualification cases. Docker COPY preserves staged modes under the intended recipe, but Q0 must verify actual final-layer modes, ownership and all ancestor-directory traversal; staging evidence alone is not final-image evidence. No new Dockerfile syntax or BuildKit chmod extension is required.

## Metadata contract

Emit the existing build-info fields: source_repository=https://github.com/aburan28/crypto (historical validator contract), source_revision=accepted final40hex, ecbench_sha256=accepted final64hex, fresh_owner_sha256=f6f6016e549ab19d9e92a7a89a5a89d095de068775dd003ab5666217726b48b7, built_at=actual binary-build timestamp, architecture=linux/amd64, target=x86_64-unknown-linux-musl, default_features=false, rustc=actual build-receipt version text, rustflags=actual flags. Do not invent a fresh owner rebuild or attribute owner bytes to the final ecbench source; retain its separate source/build manifest. Additional assembly timestamp, context hashes, binary receipt, full link flags/toolchain and retained owner provenance belong in a sidecar receipt. Reject a final build whose target/features/flags do not meet the accepted recipe; do not rewrite metadata to conceal a difference.

## Concrete existing local route, gates and caps

Existing terminal backstop receipts used `sudo -n docker`; current availability is not assumed. The first eventual gate is read-only explicit-local-endpoint inspection using bounded noninteractive sudo. Use an empty private DOCKER_CONFIG, clear Docker endpoint/context/TLS selectors and specify `--host unix:///var/run/docker.sock`; do not print credentials, change socket groups/modes or start another daemon. Record client/server versions and endpoint. If this existing route is unavailable, stop before assembly.

Require the exact upstream pinned reference already present locally and inspect it for linux/amd64 and empty OnBuild. No pull/login is included. Record its immutable ID and RepoDigests. `--pull=false` is not a substitute for this check; the selected builder must resolve this existing local image without registry fetch. If it requires remote metadata/pull, stop rather than redirecting to another builder or downloading. Do not invoke publish-image or publish-runtime: those wrappers authenticate, build, push and query AWS.

Proposed one-attempt local software envelope:600seconds total, with two builds capped240seconds each and final60seconds reserved for exact-state readback/reconciliation. Staging/static validation cap30seconds; each inspect/version/readback command cap15seconds. Use one build at a time, --network=none, --pull=false, --platform=linux/amd64 and unique scoped local tags/IID files. No scientific payload is executed; these two recipes have no RUN. Caps bound client supervision, not guaranteed daemon termination. A timed-out/ambiguous build leaves ownership retained: inspect only the exact IID/tag and known builder operation, record incomplete state and stop. No automatic retry, broad prune/remove, daemon restart or unrelated cache cleanup.

Proposed command templates after exact paths and bindings are frozen:

    timeout -k 2s 240s sudo -n env DOCKER_CONFIG=EMPTY_CONFIG docker --host unix:///var/run/docker.sock build --pull=false --network=none --platform=linux/amd64 --iidfile BASE_IID --build-arg SOURCE_REVISION=FINAL_REVISION --tag UNIQUE_BASE_TAG --file PINNED_DOCKERFILE_ECBENCH BASE_CONTEXT
    timeout -k 2s 240s sudo -n env DOCKER_CONFIG=EMPTY_CONFIG docker --host unix:///var/run/docker.sock build --pull=false --network=none --platform=linux/amd64 --iidfile RUNTIME_IID --build-arg BASE_IMAGE=BASE_IMAGE_ID --tag UNIQUE_RUNTIME_TAG --file PINNED_DOCKERFILE_RUNTIME OVERLAY_CONTEXT

The supervisor must enforce one shared monotonic deadline rather than blindly spending every individual cap, clear selector environment at the outer invocation, capture stdout/stderr/exit and record exact source/context identities before either build. These templates are not an executable supervisor and are not execution admission. Post-build inspect IDs, layer IDs, user, entrypoint, CMD, source label and OnBuild. A local image ID is not a remote repository digest. No remote digest or object version is invented.

## Handoff to the separate qualification sequence

Once final image IDs are frozen, Q0-Q4 use the already drafted300-second qualification sequence. The fixture packet is /var/tmp/pencil-runtime-fixtures-20261031-v1; mount prepared/fixtures at /fixtures read-only. Its manifest is d31e50811beb532bc44c9b4468a4ac31529385f4a46f201e3cb981cf6f516dbc. The relocated request hash is8a8cdbda79fd92a2fc0d62d3add10e6f8a79ce7222c4dba491ae703645e3ac92; expected loaded bytes5248862. Files are0444 and directory0555. These are fixture preparation results, not container qualification.

Final-image Q0 must separately verify binary/owner/runtime bytes and actual modes, including /opt/isolab/recovery and lib traversal. Active diagnostic/owner rejection cases explicitly use UID1000, despite overlay's configured UID0. The owner negative-check case is not successful owner authentication. Assembly does not establish controller lifecycle, timing, scientific admission or remote publication. Existing scientific campaign limits and prior costs remain unchanged.
