# Local-disk replay proposal; not executed

## Observed prerequisite state

The previous compilation timeout remains unchanged in `qualification-receipt.json`. Its processes were reaped. Before this diagnosis no live Lean or Lake build was observed. No compiler was launched during this diagnosis.

`/home/ec2-user` is an NFS4 mount. It contains both the imported package cache and Lean executable/shared library. `/var/tmp` is local XFS with about30GiB available at this check. A source-only private directory on `/home/ec2-user` therefore did not isolate the previous run from RPC latency. The current candidate and both helper hashes match the original qualification inputs and the live helper sources.

Fresh bounded reads successfully hashed the exact pinned Lean executable, its233,708,536-byte `libleanshared.so`, and a sampled534,520-byte Mathlib `.olean`; these files are accessible. This is not a guarantee of sustained RPC performance. Subsequent metadata/source enumeration again observed transient `rpc_wait_bit_killable` waits.

`cache-diagnosis-v2.json` enumerates a conservative header-import closure of2,528 modules in50.2seconds, no missing source modules, with504,145,272 bytes of main `.olean` files. The first parser scanned embedded strings and falsely listed JavaScript imports; that superseded diagnostic is retained in `cache-diagnosis.json`. V2 reads only the Lean header and handles `import all`; it is an inventory heuristic, not a compiler-certified dependency graph.

## Narrow mirror contract

`local-mirror-plan.json` selects only those modules' existing `.olean`, `.olean.private`, `.olean.server`, `.ir`, `.ir.sig`, and source files; the exact pinned executable plus its four linked Lean shared libraries; and the three private source modules/pin files. It does not copy the entire Mathlib checkout or installation. It excludes build traces, archives, Lake state, unrelated research and credentials. Existing package binary provenance remains a trust boundary; byte-copying them is not a clean dependency rebuild.

The future staging helper must:

1. Atomically create the fresh destination `/var/tmp/pencil-same-support-lean-local-v1`; refuse an existing destination. Check filesystem is local and free-space allowance covers the plan. Reject missing dependency `.olean` files before copy; the three private modules are compiled from copied source instead.
2. Copy regular bytes to exactly the planned relative destinations, without symlinks back to NFS. Record source and destination SHA256, bytes, before/after source metadata, actual copy duration and every failure. Verify against available existing pins, including the three private sources, toolchain executable/shared-library hashes and manifest. A future qualification must use the resulting fixed manifest, not re-resolve shared paths.
3. Refuse compilation until every copy/readback succeeds and all replay paths resolve within the local mirror, apart from ordinary system dynamic libraries. Confirm dynamic dependencies with `ldd` on the copied executable; retain output.
4. Reconcile live jobs immediately before a replay. Use one compiler process at a time and retain separate stdout/stderr/status for each command.

## Proposed finite envelope and commands

Proposed staging limits: one copy process, at most4GiB selected input bytes,600seconds cumulative wall time, and no install/download or change of pin. Abort if the generated plan exceeds the size ceiling. A timed-out NFS read can delay reaping; record the owned PID and do not retry until it is gone. These are proposed bounds, not a renewal of the scientific worker campaign.

Proposed first replay limits: one Lean thread,4096MiB Lean allocation limit,120seconds per module,360seconds cumulative across the three modules. No whole-project `lake build`. Once staging is qualified, the concrete invocation form is:

```sh
mirror=/var/tmp/pencil-same-support-lean-local-v1
export LEAN_SYSROOT="$mirror/toolchain"
export LEAN_PATH="$mirror/work:$mirror/deps"
cd "$mirror/work"
# Each command gets its own log, exit receipt and source/dependency hash binding.
timeout -k 5s 120s "$mirror/toolchain/bin/lean" -j1 -M4096 -o ProjectiveLawBoundary.olean ProjectiveLawBoundary.lean
timeout -k 5s 120s "$mirror/toolchain/bin/lean" -j1 -M4096 -o RuntimeSemantics.olean RuntimeSemantics.lean
timeout -k 5s 120s "$mirror/toolchain/bin/lean" -j1 -M4096 -o SameSupport.olean SameSupport.lean
```

The driver must stop on the first nonzero exit and reconcile reaping; the displayed lines are a command plan, not permission to run past an earlier failure. Verify option support from the copied executable before use. The candidate's `#print axioms` output is required; successful source compilation alone is not independent semantic review.

A source-level candidate elaboration error is distinct from this previous import-access timeout and can be repaired in a new private revision with its own log. The candidate has not yet been elaborated, so no syntax/API correctness is asserted. The mathematical target remains the validity-coefficient connection for one branch; a successful check would not identify the whole compiled dispatcher with Lean or establish a global divisor-class theorem.

## Subsequent staging authorization

The parent authorized copy-only staging with an **8GiB ceiling and300seconds wall**, replacing the proposed4GiB/600second staging envelope above. The completed plan has15,160 files and2,274,679,663 bytes, with no missing dependency oleans. The copy owner is recorded in the unique local destination's `owner.json`; it uses a295second internal stop and outer `timeout -k 5s 300s`. No compilation is authorized by this staging receipt. The original120seconds per compiler invocation remains unchanged.
