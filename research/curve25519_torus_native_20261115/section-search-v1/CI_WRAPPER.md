# Normal Lake context for hosted target discovery

The report/cd9874 preserve the historical successful maintained standalone gate
on SectionSearchAssembly.lean. Its explicit pinned LEAN_PATH is supplied by
check-lean.sh. The unmodified repository workflow discovers standalone targets
without that environment, so the same Mathlib import would fail after merge.

Add a normal Lake project and dependency manifest for automatic discovery.
They wrap the exact same checked source and dependency revisions. No theorem,
native snapshot, prior receipt, helper, trust file or workflow is changed.
The project wrapper is publication integration, not a new scientific result.
The old dependency-context manifest remains byte-equal as historical evidence.

Focused Lake build and direct maintained audit are recorded separately here.
The full project wrapper invokes a broad Mathlib cache fetch; it is not rerun
on the local2.5GiB remaining envelope after the previous actual disk exhaustion.
The normal hosted project's cache/build/audit outcome remains pending. A prior
standalone pass is not represented as that hosted project receipt. No gate
or cache policy is bypassed.
