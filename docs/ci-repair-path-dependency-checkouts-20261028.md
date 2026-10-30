# Restore root path dependencies in sparse CI

Main `d4e85a6a70875c6ad7fe90559882e1d172ed94e0` has 26 failed or cancelled
checks. At least eleven inspected compiler failures share the same cause:
the root Cargo manifest depends on `tools/factor-base`, which their sparse
checkout omits. This prevents Cargo from loading the package before the
intended compile, lint, replay or host-verification work begins.

Add the tracked factor-base source to all 65 matching root checkout
definitions across 41 workflows, including release, CodeQL, Rust, ecbench,
native host and research validators. Existing broad `tools/` checkouts already
cover it. The final checkout inventory contains no omission among those root
definitions. The two sealed proof workflows remain byte-exact; no event,
runner, timeout, scientific threshold or experiment selection changes.

The source materializer also restores both root path crates' Rust sources and
manifests, their transitive isogeny_algos sources/manifest, and literal embedded
inputs. A real cold temporary Git checkout first fails native offline Cargo
metadata resolution because factor-base is absent; after materialization,
the same command resolves all four local packages. Its dependency fixture is
restored while an unrelated run archive remains absent. All eleven scope
controls pass. Actual repository materialization restores 170 compile-time
inputs, and the maintained-target Clippy command passes with warnings denied.
The exact local Rust version is retained in the validation record.

The broader ShellCheck pass found eight existing naked negated greps in the
conductor arithmetic guards. Such commands suppress errexit and could let a
reported verifier error pass. Each guard now explicitly rejects a matching
error and any grep failure other than a clean no-match. The actual shell
guards pass 24 controls: clean stderr is accepted, while error output and a
missing stderr file are refused. No arithmetic experiment is run. The Jacobian
table validator's equivalent quoting and diagnostic message also avoid a
literal-dollar/backtick lint ambiguity. All 41 edited workflows pass both
Actionlint and ShellCheck, and all 15 budget/seal controls pass.

Raw outputs, exact original checkout inventory, changed workflow list,
unchanged sealed-workflow hashes and validation counts are retained under
`reports/root-path-dependency-repair-20261028/`. Initial local collection lacked
the two exact seal files; restoring their tracked bytes made the full budget
controls pass without changing a seal or its policy. The first metadata
control used `--no-deps`, which does not resolve path crates; the final native
control resolves the entire offline dependency graph and reproduces the error.

Other catalog-count, dated-ledger, replay-inventory and historical-validator
failures remain separate work. This repair establishes dependency availability
and stronger shell guards; current-head hosted checks and review still decide
merge, and no scientific status is promoted.
