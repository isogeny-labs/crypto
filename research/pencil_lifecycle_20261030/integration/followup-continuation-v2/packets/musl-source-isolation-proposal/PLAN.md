# Exact-commit isolated musl build source — prospective proposal

The original wrapper is preserved byte-for-byte. No Git command, checkout, source mutation or build was performed by this proposal. Python AST parsing of the proposed wrapper passed; there is no native build result. Root owns source preparation and execution after its active commit completes.

## Why a separate source root

The original wrapper runs `git ls-files --others --exclude-standard` in the shared research worktree and requires an empty result. Unrelated retained render/target caches and active receipt outputs can legitimately violate this gate. Do not delete those outputs, add ignore rules to conceal them, weaken the clean-source requirement, or build while a source commit is in flight.

Root should create one NEW detached linked worktree at the exact approved forty-character commit, in /var/tmp/pencil-musl-source-UNIQUE. The linked worktree shares Git objects, not its index or checked-out source files. Use a full checkout, not sparse checkout, to preserve all tracked build inputs. Require no existing destination. Do not copy the original filesystem, since that would import untracked caches and mutable files. Archive-only extraction is not proposed because the existing wrapper's HEAD/tree/clean checks require Git metadata.

Prospective root-owned preparation, after commit and source/cache ownership are resolved:

```
git -C /home/ec2-user/crypto-pencil-signed-graph-20261029 worktree add --detach /var/tmp/pencil-musl-source-UNIQUE EXACT_COMMIT
```

Record exact argv, native exit and final directory; on ambiguous checkout inspect the same destination before any retry. Bind source object's tree and commit and complete `git ls-files --stage` listing. Verify the destination's actual HEAD equals EXACT_COMMIT, detached state, clean tracked source, and empty ALL untracked inventory (including ignored files). Confirm complete materialization; reject submodule gitlinks or escaping source symlinks unless their exact dependency sources are separately pinned and reviewed. Review any local path dependencies outside the worktree and Cargo configuration from the worktree, its ancestors, and CARGO_HOME. Record hashes/config provenance without printing credentials. Parent /var/tmp and / should have no unexpected Cargo configuration; existing CARGO_HOME dependency/cache configuration must be bound. This is necessary because moving the source root changes ancestor configuration lookup.

Establish root's exclusive source lease across preparation/build/review. Make ordinary tracked source files and source directories read-only after checkout if compatible with this platform; preserve executable bits and do not alter symlink targets or the shared Git administrative directory. Keep .git pointer file intact. Read-only permissions protect against accidental tool writes but are not adversarial immutability for the owning user; source ownership plus pre/post hash/clean checks remains required. Record the exact source inventory before and after. A build script demanding source-tree writes fails closed; do not reopen write permission to make it pass without source review.

## Narrow wrapper amendment

`build-receipted-isolated.py` adds required --source-root, validates a direct nonsymlink /var/tmp/pencil-musl-source-* root, records it, verifies --show-toplevel and detached HEAD, and retains exact commit/tree checks. It strengthens both untracked checks to `git ls-files --others` without exclusions, so ignored source artifacts also fail. It does not change the compiler, feature set, target, flags, deadlines, native logs, final static-ELF requirements or linker-map candidate binding. All original failure receipts and process-group termination remain.

The wrapper does not create or authorize the worktree and does not itself establish the source lease, complete external-input inventory, or reviewed source permission policy. Those are root's preparation gates above. Do not call this a complete immutable build merely from its AST check.

Prospective invocation, only after exact preparation and review:

```
python3 /var/tmp/pencil-musl-source-isolation-proposal-20261031-v1/build-receipted-isolated.py \
  --source-root /var/tmp/pencil-musl-source-UNIQUE \
  --commit EXACT_COMMIT --seconds ROOT_SELECTED_CAP \
  --receipt-dir /var/tmp/pencil-final-musl-build-UNIQUE
```

## Shared cache and ownership

Keep the existing external CARGO_TARGET_DIR=/tmp/pencil-musl-target and its exclusive nonblocking .pencil-musl-owner.lock. Keep the existing pinned Cargo registry/toolchain caches via the wrapper's bounded environment; no dependency download or update. Rust1.99.1/offline/locked/no-default-features/x86_64-unknown-linux-musl/jobs2 and the reviewed musl compiler remain fixed. Source-path changes may invalidate package fingerprints; reuse of dependency artifacts is permitted but a cache hit is not guaranteed. Do not modify Cargo fingerprints, copy old final binaries as fresh results, or use unsafe timestamp manipulation.

Reconcile active Cargo/rustc ownership before using this cache; the wrapper observes its source cwd or explicit target directory, but this is not a global proof that every arbitrary process respects its lock. Root must confirm no existing owner uses this target through an implicit/configured path. Shared Cargo registry locks may create waits; those waits remain under the existing build deadline. No second build should be started while root's current build/check owns the same cache.

## Evidence and stopping

Retain worktree preparation receipt, source commit/tree/listing/hashes, source lease, external path/config review, source permissions, wrapper hashes, toolchain/compiler/cache identity and complete native build receipt. Preserve failed/ambiguous preparation and stop rather than retry automatically. A dirty source or added ignored/untracked file remains a failure. Root should independently review selected CRT/libc from actual link output and map, then bind final binary to publication metadata. Worktree removal is optional later cleanup, only after source/build ownership is terminal and evidence copied; do not remove an active or dirty worktree.

This is software build preparation only. Scientific allowance25200workerseconds and deadline2026-10-31T06:00:00Z remain unchanged; no scientific execution, runtime qualification or publication is asserted.
