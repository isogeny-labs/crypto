# Published deployment artifacts

Source, tests, and the applied CloudFormation template live in this directory.
Compiled binaries and deployment-specific inventory remain in the private S3
bucket listed in `artifacts.json`. No AWS credentials, private SSH keys, Cairn
identity keys, Conductor credentials, or database files are in these artifacts.

`artifacts.json` records the object URI, SHA-256, and byte count for:

- `deployment_bundle`: the complete deployment folder, including the runtime,
  client installer, generated template, and observed deployment status.
- `client_installer`: the small workstation package. Extract it and run
  `bash install_client.sh` in `spot-client/` on the machine running the coding clients.
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

The applied stack enables wake/idle scaling and guarded automatic IP failover.
The applied Spot quota is 96 vCPUs. The original source and IP remain protected
while its remaining experiment runs. Private encrypted memory backups run at
15-minute boundaries, and automatic recovery passed two successive disposable
replacement hops. The newest independent Diem copy restore also passed.

The laptop hooks/SSH installer is prepared; actual laptop installation and
Codex `/hooks` trust remain client-side activation steps. New arbitrary programs
need tested native checkpoints or explicit tested CRIU enrollment. Memory
archives are private in the separate checkpoint namespace and are excluded from
all software releases and GitHub. See `VALIDATION.md` for exact evidence/limits.

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
