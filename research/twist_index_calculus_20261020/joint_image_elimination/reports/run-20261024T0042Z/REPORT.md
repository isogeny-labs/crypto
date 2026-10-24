# Native global joint S5 regression v1

Run `regression-v1`, started 2026-10-24 00:42:41 UTC. Frozen solver source:
`950e15f38c4a2646a14518ee89cc672687ac2d6a`. The copied runner hash and
exact commands are retained in `runs/regression-v1/metadata/`.

![Construction and root-solving boundary](../../figures/joint-mechanism.svg)

The first six global p37/seed0/query2 solves are terminal. Phi and doubling
finite-domain processes exit 134 with allocation-failure stderr. Pure L0
finite-domain returns an explicit incomplete result at the cooperative
limit. All three global geometric processes exit 134 with allocation
failure. Empty root output from an incomplete solve is not zero true yield.
The attempted pure incomplete result fails the complete-set oracle check.

The three source-block exports completed. Their initial union launchers
were interrupted before producing solve directories or exit receipts;
continuation is in `runs/regression-v2/`, not overwritten v1 outputs.
See [migration reconciliation](../../MIGRATION_RECONCILIATION_20261024.md).

The protocol retains one thread, 8 GiB address space, a 1,200 s outer
watchdog and a shared 1,180 s cooperative budget per query. Heavy phases
use the busy reservation. These durations are unadmitted diagnostics.
The first allocation failure additionally incurred a core-dump delay;
the saved adjustment disables core dumps for subsequent owned processes.
It changes neither mathematical inputs nor address-space/time caps.

No complete joint collector, measured speedup or impossibility statement
is established by these six terminal outcomes. The completed source-block
and later variant results require their separate reports and audits.
