# Published deployment artifacts

Source, tests, and the staged CloudFormation template live in this directory.
Compiled binaries and deployment-specific inventory remain in the private S3
bucket listed in `artifacts.json`. No AWS credentials, private SSH keys, Cairn
identity keys, Conductor credentials, or database files are in these artifacts.

`artifacts.json` records the object URI, SHA-256, and byte count for:

- `deployment_bundle`: the complete deployment folder, including the runtime,
  client installer, generated template, and observed deployment status.
- `client_installer`: the small workstation package. Extract it and run
  `python3 setup.py` in `spot-client/` on the machine running the coding clients.
- `runtime`: the digest-pinned worker bundle containing the supervisor, activity
  reporter, node services and pinned node executables.
- `checkpoint_discovery`: the portable native scanner executable, also included
  in the runtime. Its runbook is `CHECKPOINT_DISCOVERY.md`.
- `checkpoint_selftest`: the native kill/corrupt/restore bootstrap gate, also
  included in the runtime. Its build/deployment instructions are in `AWS_DEPLOYMENT.md`.
- `solver_checkpoint_selftest`: the real sparse-solver kill/restore bootstrap gate.
- `native_experiment`: the portable checkpoint-capable factor-base executable.

The release's `artifacts.json` sits beside the deployment bundle and is also
committed here. It is excluded from the bundle to avoid a circular self-hash.

These objects require an authenticated AWS identity with `s3:GetObject` access.
Their bucket blocks public access, uses server-side encryption and versioning,
and rejects non-TLS requests. The client installer itself needs only its bundled
`lambda:InvokeFunction` policy once installed; it contains no AWS credentials.

For example, download the deployment bundle with the URI recorded in the
manifest, verify its recorded hash, then extract its `spot-system/` directory:

```bash
aws s3 cp S3_URI_FROM_ARTIFACTS_JSON spot-system.tar.gz --region us-east-2
sha256sum spot-system.tar.gz  # compare to deployment_bundle.sha256
tar -xzf spot-system.tar.gz
```

The artifact names are content-addressed. Preserve previous objects for rollback;
do not replace a bundle's content under another bundle's digest.

## Deployment status

The applied stack enables managed wake/idle scaling in an isolated workspace;
IP cutover is disabled. The applied Standard Spot quota is 96 vCPUs; cold boot,
actual native state replay, worker replacement and 30-minute idle shutdown have
passed. The separate
original host still runs its existing experiments and local Cairn/Conductor
node bundle. `AWS_DEPLOYMENT.md` and `validation/aws-deployment.json` record the
applied settings, quota requests and validation limits.

Before activation, complete the recovery, workload migration, node writer-lock
handoff and client installation gates in `README.md`. Existing non-checkpointed
experiments do not acquire RAM recovery from these software uploads. Their
approved CRIU memory snapshots remain private on the original retained disk;
S3 memory export and actual legacy restore have separate unresolved gates.

## Validation

The standalone Python tests exercise infrastructure control and synthetic process
recovery. They implement no cryptographic computation, scientific benchmark or
research result generation. Run them from the repository root:

```bash
python3 -m unittest discover -s infra/spot-system -p 'test_*.py' -v
python3 -m py_compile infra/spot-system/*.py
cfn-lint infra/spot-system/template.json
```

Native discovery and its supervisor restore tests run with:

```bash
cargo test --locked --manifest-path infra/spot-system/checkpoint-discovery/Cargo.toml
```

See `VALIDATION.md` for the publication checks and their precise scope.
