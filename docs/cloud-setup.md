# Cloud session setup (Claude Code and Codex)

`scripts/cloud-setup.sh` is the setup script for cloud agent environments. It
installs the toolchain, clones the sibling repositories and wires credentials,
and it keeps the session inside its disk allowance. Put one line in the
environment's setup-script field:

| Runtime | Where the script starts | Setup-script field |
| --- | --- | --- |
| Claude Code on the web | one directory above the repositories (`/home/user`) | `bash crypto/scripts/cloud-setup.sh` |
| Codex cloud | inside the repository (`/workspace/crypto`) | `bash scripts/cloud-setup.sh` |

`bash crypto/scripts/cloud-setup.sh --dry-run` prints every step and changes
nothing. Each step warns instead of failing, so a missing tool never fails the
session's setup.

## What it does

1. **Disk.** Applies the `harness` sparse profile to the crypto checkout
   (`scripts/sparse-checkout.sh`; [sparse-checkout.md](sparse-checkout.md))
   and to the crypto-autoresearcher checkout (`tools/sparse_checkout.py`; its
   `docs/sparse-checkout.md`). The large `research/` archives, experiment run
   directories and reference bundles stay in git but off disk, and each tool's
   `add --path P/` brings a path back. `CRYPTO_SPARSE_PROFILE=minimal|full`
   and `CRYPTO_AR_SPARSE_PROFILE=full` change the profiles.
2. **Tools and repositories.** Runs `scripts/setup-workstation.sh` without its
   control-plane step. That covers apt build tools, PARI/GP, FLINT,
   PostgreSQL, Rust, Go, Node, uv, Conductor, cairn, the codex, opencode and gh
   CLIs, aws and kubectl. SageMath is the lean conda-forge build (`sagelib` +
   `maxima`, about 3 GiB) and installs only when 4 GiB are free. Missing
   siblings (`crypto cryptanalysis crypto-autoresearcher cryptanalysis-rl`
   under `isogeny-labs`) are cloned as partial clones: blobs over 1 MiB stay on
   the server until a checkout needs them. Each repository then gets its
   virtualenv.
3. **Credentials.** Installs a git credential helper for `https://github.com`
   that reads `GH_APP_*` or `GH_SERVICE_TOKEN` when git needs a credential.
   gh, aws, kubectl and the agent CLIs read their own variables. Writes
   `~/.config/cloud-setup/env` (PATH, mode 600) and sources it from `~/.bashrc`
   and `~/.profile`. It prints the names of the credential variables it can
   see, never their values.
4. **Cleanup.** Clears the apt, uv, Go build, npm, micromamba and cargo caches
   (`SETUP_CLEANUP=0` keeps them).

The Conductor control plane and the cairn node are processes. The SessionStart
hook (`.claude/hooks/session-start.sh`) starts them in each session.

## Credentials

The script writes no credentials. Set them in the environment's settings:

- **Claude Code.** Use environment variables, or Network secrets where the
  settings offer them. The setup script cannot read them, but the session can.
  That is why everything above reads credentials when it is used, not at setup.
- **Codex.** Environment variables last for the whole task. Secrets exist only
  while the setup script runs. `PERSIST_SECRETS=1` copies the recognized
  variables present during setup into `~/.config/cloud-setup/env` (mode 600),
  so the agent can use them later. It is off by default because Codex removes
  secrets before the agent runs on purpose.

| Service | Variables |
| --- | --- |
| GitHub | `GH_TOKEN` (gh), or `GH_SERVICE_TOKEN`, or `GH_APP_ID` + `GH_APP_INSTALLATION_ID` + `GH_APP_PRIVATE_KEY` (git and gh) |
| AWS, S3 checkpoints | `AWS_ACCESS_KEY_ID` + `AWS_SECRET_ACCESS_KEY` [+ `AWS_SESSION_TOKEN`], `AWS_REGION`, `CHECKPOINT_BUCKET` |
| Iso Lab | `ISOLAB_URL`, or `KUBECONFIG` + `ISOLAB_KUBE_CONTEXT` |
| Conductor | `CONDUCTOR_ENDPOINT` + `CONDUCTOR_TOKEN` (shared control plane) |
| Agent CLIs | `ANTHROPIC_API_KEY`, `OPENAI_API_KEY` |
| Compute | `RUNPOD_API_KEY`, `MODAL_TOKEN_ID` + `MODAL_TOKEN_SECRET`, `HF_TOKEN` |

## Disk

Claude Code clones the repositories attached to the environment in full before
the setup script runs. With all four attached, that left 821 MiB free in a
session on 2026-11-01. The two harness profiles freed 11.3 GiB there:
5.6 GiB from crypto (39,782 of 164,471 files off disk) and 5.7 GiB from
crypto-autoresearcher (44,704 of 151,565). That leaves room for the lean Sage
build.

crypto-autoresearcher's whole-ledger sweeps (`validate_ledger`,
`goal_portfolio_health`, `portfolio_kpis`) refuse a sparse checkout, and the
coordination skills run them. For a coordination session, opt that checkout
out on the setup line itself, since the setup script cannot read the
environment's variables in Claude Code:

```sh
CRYPTO_AR_SPARSE_PROFILE=full bash crypto/scripts/cloud-setup.sh
```

or run `python3 tools/sparse_checkout.py disable` in the session. That needs
about 6 GiB free again.

`WITH_SAGE=0` skips Sage, `WITH_LEAN=1` adds elan for cryptanalysis's Lean
proofs, and `SETUP_SKIP` takes more `setup-workstation.sh` components to skip
(`bash scripts/setup-workstation.sh --list`). The header of
`scripts/cloud-setup.sh` lists every override.
