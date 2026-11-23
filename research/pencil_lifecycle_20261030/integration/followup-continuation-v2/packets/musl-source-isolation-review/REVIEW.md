# Source-isolated musl build proposal: independent review

The amendment preserves the original wrapper and its build/ELF/link-map behavior. One new failure-receipt gap should be corrected before adopting the wrapper: source-root validation occurs after creation of the receipt directory but before receipt initialization and the try/finally block. A rejected symlink, noncanonical path or wrong prefix therefore leaves an empty receipt directory, with no recorded failure. Move that validation inside the initialized receipt scope, or explicitly receipt the preparation rejection. This is a source-control-flow finding; no wrapper was executed.

All six proposal hashes match manifest f955229acbba4efb9f8734350121fcb424f690151e2da1fd5f456f241f7825ee. The preserved original is byte-identical to /var/tmp/pencil-musl-build-supervision-20261031/build-receipted.py. Proposed Python AST parses. No native execution, checkout, cache acquisition or source mutation occurred.

## What the amendment establishes

The selected root must resolve to itself under a direct /var/tmp/pencil-musl-source-* path, Git top-level must match, HEAD must be detached and equal the requested commit. Commit/tree checks remain. Both untracked checks now include ignored files. Build target, compiler/toolchain selection, locked/offline flags, feature set, environment restrictions, ELF checks and linker-map candidate extraction are unchanged. The existing nonblocking target-cache lock and process ownership probe remain; source/cwd changes do not silently authorize concurrent cache use.

The wrapper by itself does not establish full immutable source closure. It hashes Cargo manifests and records Git tree, but does not enforce the proposal's complete source-file inventory, submodule/symlink/path-dependency review, source lease or effective Cargo configuration. These are explicit preparation obligations, not demonstrated wrapper results. Pre/post clean checks are evidence under ownership, not protection from an adversary that can mutate and restore files during compilation.

## Substantive requirements versus optional implementation choices

Required for this build claim: an exact committed input snapshot, exclusive source ownership for the build, complete materialization of every consumed input, explicit binding of external path dependencies and symlink targets, and a pre/post source inventory. Resolve worktree/ancestor/CARGO_HOME Cargo configuration and inherited tool selection without printing credentials. Preserve existing offline/locked behavior and pinned native compiler inputs. The target-cache owner must be reconciled beyond the cooperative flock when existing processes could use an implicit/configured target path.

A full detached checkout is a sufficient way to achieve source materialization, but not a mathematical requirement. A separately reviewed sparse worktree could suffice if the complete build-input closure—including build-script scans, include_str/include_bytes, generated inputs and path dependencies—is shown materialized and pinned. That closure evidence is not present in this proposal, so the sparse alternative is not already qualified. Full checkout is the proposal's simple default, not a reason to delete unrelated artifacts or inflate scientific scope.

Making tracked files/directories read-only is optional accidental-write defense and may not prevent owner writes. It does not replace the source lease or hashes. A build script that writes the source tree should fail rather than silently reopening permissions. Rejecting all submodules/escaping symlinks is sufficient; separately pinned and reviewed dependencies are an alternative. Worktree removal is optional later cleanup and must wait for terminal ownership/evidence preservation.

## Timing and failure boundaries

The requested seconds cap applies to the Cargo invocation, plus5seconds kill grace and supervisor wait allowance. Pre/post commands each have their existing120second timeout; preparation and these checks are not charged to the same single wall-clock cap. No stronger total-duration guarantee is added. Offline compilation can still wait on registry locks, within the build cap.

Most failures inside try/finally record status/error and preserve logs; live supervised process groups are terminated. Startup argparse/receipt-directory failures already predate this amendment. The new root-validation gap is the new issue identified above. No post-failure auto-retry is proposed; reconcile an ambiguous checkout or build in its original location. Successful static ELF remains a candidate requiring the stated independent CRT/libc map review and final publication binding.

This review grants no execution or publication and makes no scientific-result claim. Root retains preparation and current-build ownership decisions. The reviewer's initial hash helper assumed a nested manifest schema; its KeyError and correction are recorded in static-checks.json.
