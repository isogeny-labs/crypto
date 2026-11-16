# Optional resumable memory envelope: source validation

The immutable request now accepts only `memory: "4Gi"` or `"8Gi"`, defaults to
4Gi, and omits the default from serialization. The legacy-layout regression
compares exact serialized bytes and hashes; explicit 8Gi must change identity.
Computation and fresh L2 job manifests both receive equal request/limit values.

Controller tests invoke the maintained submit-job script only with --dry-run,
in an isolated child test process, and check 4Gi/8Gi manifests, CPU4, existing
deadline, no retries, L2 pool and request binding. No AWS, Kubernetes, controller
deployment or runtime publication is performed. Existing storage caps remain
unchanged; these tests do not establish 8Gi checkpoint storage sufficiency.

Commands (CARGO_TARGET_DIR=/tmp/pencil-memory-controller-target):

- cargo test --locked --offline --manifest-path taskq/deploy/eks-karpenter/resume/controller/Cargo.toml
- cargo test --release --locked --offline --manifest-path taskq/deploy/eks-karpenter/resume/controller/Cargo.toml
- cargo clippy --locked --offline --manifest-path taskq/deploy/eks-karpenter/resume/controller/Cargo.toml --all-targets -- -D warnings

The schema field regression uses installed Python jsonschema's Draft7Validator
for the schema's shared keywords. Draft202012Validator is unavailable here;
this is not certification of the draft2020-12 metaschema. Native Rust tests
also compare the schema enum/default/optional status against the parser.

Sources reviewed: resume/controller/src/{contract,cloud,model}.rs,
resume/{README,CONTROLLER}.md, the maintained parent submit-job renderer, and
https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/ .
The Kubernetes documentation motivates applying the same explicit request and
limit to both execution stages; the backward-compatible default follows the
existing immutable-request contract.

The generated external contracts/mcp-tools.json embeds this schema for submit
and status. Both embedded schemas are synchronized in this repair under expanded scope.
The MCP source includes resume/request.schema.json and picks up changes when
rebuilt. Deployment and any publication require their existing authorization.
