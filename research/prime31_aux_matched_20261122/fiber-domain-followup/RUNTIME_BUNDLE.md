# Constructing the source payload without running an experiment

`prepare-runtime-bundle.sh` consumes **existing compiled binaries** and exact
tracked input files. It constructs a source payload in a new directory. It does
not compile, run a cell, freeze a protocol, upload, publish an image or launch a
job. Successful packaging always records `launch_admitted:false` and
`container_runtime_qualified:false`.

## Inputs and interface

The source checkout must be clean and non-shallow. The constructor's own Python
and shell files must be tracked and unchanged in that checkout. Give the two
supplied executables as absolute paths; they need not sit inside the checkout.
Select the role using the correct executable from the existing build receipts.
The shared native source metadata binds file contents, but does not independently
prove which Cargo binary target the compiler selected.

Example argument structure, **not a build or launch command**:

```sh
bash research/prime31_aux_matched_20261122/fiber-domain-followup/prepare-runtime-bundle.sh \
  --repository /absolute/clean/crypto \
  --choice-binary /absolute/existing/prime31-isogeny-choices \
  --incumbent-binary /absolute/existing/prime31-ic-incumbent \
  --cell research/prime31_aux_matched_20261122/fiber-domain-followup/selected/cell.json \
  --protocol research/prime31_aux_matched_20261122/fiber-domain-followup/selected/protocol.json \
  --selection research/prime31_aux_matched_20261122/fiber-domain-followup/selected/selection.json \
  --output /absolute/new-payload
```

Repeat `--cell` for **every** cell in the existing selection. The selected IDs,
input hashes and protocol hash must match exactly. The constructor recognizes
the v3 choice and v1 incumbent cell envelopes and their 120-second/4096-MiB caps.
`--extra-path` may add explicit tracked regular files to the small source closure.
No untracked file, symlink, submodule or path traversal is exported.

The output must be outside the source checkout, its parent must already exist,
and the output itself must not exist, including as a dangling symlink. A failed
construction preserves its new partial directory with `construction-failure.json`;
it is not complete without `bundle.json` and `SHA256SUMS`. Rerunning against that
directory is rejected. Existing outputs are never replaced or repaired in place.

## Source custody

For each copied executable, only `source-manifest` and `source-check` are invoked.
These are metadata commands, not scientific arithmetic. The constructor checks
the reported running-binary hash, embedded manifest hash, exact file inventory
and source revision. Every inventoried file must match its actual HEAD blob.

A real Git v3 bundle with `--filter=blob:none` preserves the original commit
ancestry and trees. A fresh Git repository is initialized without templates;
that bundle is imported, and only the selected HEAD blobs are added using their
real Git object identities. A non-cone sparse checkout materializes precisely
the inventory paths, selected input/protocol/selection files and explicit closure.
The original HEAD and complete parent graph are compared after export. Native
`source-check` is then run against the sanitized real checkout for both binaries.

There are no synthetic commits or substituted histories. Unselected file bodies
and historical blob bodies are omitted; real commit messages and tree metadata
remain because ancestry is preserved. Source Git config, remotes, hooks,
credential files and alternates are not copied. Automatic lazy fetching is
disabled; an incomplete local object set causes failure instead of a new fetch.
Git operations are metadata-only and use the existing local objects.

## Exact observed runtime dependencies

The payload does **not** assume musl or compatibility with the old AWS CLI image.
For the supplied ELF64 x86-64 executables and the host Git executable, it retains:

- Binary content hash, size, ELF interpreter and direct `DT_NEEDED` libraries.
- Full `readelf` headers/dynamic/version listings and required GLIBC versions.
- The actual host loader's `--list` resolution under a scrubbed loader
  environment, including transitive dependencies.
- Each resolved runtime file's original/resolved path, content hash, byte count
  and intended absolute installation path, plus copied exact bytes.

Only the trusted interpreter already used by the selected host Git is used for
dynamic dependency inspection. `--list` does not call the target executable's
main or constructors. Unknown interpreters, RPATH/RUNPATH, unsupported ELF
formats, unresolved dependencies and ambiguous loader output fail rather than
guessing. Static ELF startup is recorded explicitly when no interpreter or
dynamic dependencies exist. Git's own dependency closure is included because
the current native source-check invokes Git.

This is the **observed host startup dependency closure**, not an OCI compatibility
certificate. Runtime `dlopen`/NSS plugins, application data, kernel/procfs behavior,
and future entrypoint shell/timeout/affinity/uploader tools remain unqualified.
Do not blindly overlay captured glibc files onto an unrelated base image. A future
image must qualify its complete runtime and actual source/binary/OCI identity.
No runtime base digest is hardcoded here.

## Payload layout

| Path | Contents |
| --- | --- |
| `rootfs/usr/local/bin/` | Exact supplied native executables |
| `rootfs/usr/bin/git` and declared loader/library paths | Captured Git and ELF startup dependency bytes |
| `rootfs/opt/prime31/source/` | Sanitized real Git sparse checkout with original HEAD/ancestry |
| `source.bundle` | Blob-filtered real Git ancestry/tree bundle |
| `metadata/` | Original/sanitized native source checks, source manifests, ELF and loader evidence |
| `metadata/runtime-dependencies.json` | Exact executable/library hash and placement inventory |
| `bundle.json` | Complete constructor result and explicit limits of its claims |
| `SHA256SUMS` | Every generated file other than the checksum list itself |

## Qualification and remaining work

`test_runtime_bundle_contract.py` uses tiny real Git repositories and existing
system ELF files. It exercises real ancestry preservation, exact sparse material,
omitted historical/unselected bodies, clean config, file/JSON rejection, fresh
outputs and dependency-byte capture. It invokes no compiler or Prime31 arithmetic.

The **full constructor against the actual Prime31 binaries and final selected
protocol is not yet qualified**. No final selected cohort or container is created
by these tests. Protocol validation/publication, checkpoint upload/readback, live
lane budget/capacity, original deadline and OCI/execution receipts remain separate.
The existing **USD5 aggregate ceiling**, one computational thread/affinity CPU,
zero retries and deadline invariants are unchanged; the supported command job
still requires an honestly charged allocation of at least two vCPUs.
