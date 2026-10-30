# Clean CLI surface 01

Classification: SOFTWARE command registration smoke check. This does not run a
solver, startup grant, dataset generation, or the native acceptance workflow.

After clean-build-01 completes with exit 0 and both source guards true, use only
its retained executables and verify their recorded hashes before and after.
Run these four fixed commands once, in order, stopping at the first failure:

1. `crypto dataset --help`
2. `crypto dataset start --help`
3. `crypto dataset capabilities`
4. `dataset_startup_cli_fixture --help`

Each command has a 30-second timeout and 5-second TERM grace. Retain separate
stdout/stderr and the exact argv/exit/hash records. Require ordinary exit 0,
empty stderr, nonempty stdout. The capabilities document must parse as
`ml-dataset.capabilities/v1`, expose V1/V2 policy plans, and list all ten named
backend identifiers. Help output must expose the expected dataset subcommands,
the startup resume option, and the controller's policy-acceptance option.

Report this as CLI surface validation only. Capability declarations describe
their own applicability and gaps; this check establishes neither solver coverage
nor successful generation. No retry or substitution of an older binary.
