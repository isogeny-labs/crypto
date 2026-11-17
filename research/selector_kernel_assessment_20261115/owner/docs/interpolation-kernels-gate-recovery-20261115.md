# Gate dependency recovery before the final interpolation audit

Canonical gate001 fails before axiom auditing: Lake4.35 reports
"unknown module source path" for the unregistered tracked final-assembly
probe. All three diagnostic probes and both theorem-module builds pass by
direct Lean compilation. The old gate recognizes only "unknown target" as
the signal for its existing direct-source compilation fallback.

Narrow prospective change: recognize both unregistered-target messages in
that classification branch. Keep source enumeration, direct compilation,
sorry-warning rejection, axiom allowlist and trusted-native policy unchanged.
Every tracked source must still compile; no file is removed from inventory.

Run the existing self-test suite, including unconfigured-good, omitted-bad,
configured-good and configured-bad project controls and standalone axiom
fixtures. Then run a fresh canonical gate002 over the same complete source
inventory. Preserve gate001 and every original build/probe receipt. Pin the
new gate source/commit separately as a dependency change. A success under
gate002 must not be described as a success of the old gate version.

Observed continuation: gate002 compiles every tracked source but the audit
CLI's String.toName parser turns the hyphenated diagnostic module path into
Name.anonymous. The valid-identifier control parses correctly. The probes
were moved, with byte-identical contents, into
docs/receipts/InterpolationKernels20261115/. No source was dropped from
auditing. Final gate003 passes167theorems,0failures,0trusted-native
exceptions. module-name001.log and probes.sha256 retain the diagnosis and
new paths. The axiom auditor itself was not changed.
