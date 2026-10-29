# Exact cofactor donor integration

Design frozen before implementation on 2026-10-29. Base is merged root/main
and binary source `f6871ea3816161abcfd615954b035d1cdc84071b`. Parent delegates
T840 curves.rs and T803 solvers.rs/research scope. No cloud generation, new
budget, exporter edit or independent mathematical claim is authorized here.

Reuse runtime `40469deb0b4016873b4215b7a33c763f500deda7` from
`/home/ec2-user/crypto-ml-exact-cofactor-20261028` (retained head
`8bb842d7906ffca89e412e4c02fd6c6729ed80a9`, original base c14d4552).
Apply only its semantic two-file patch: dataset-local ExactPrimeCurve, exact
positive subgroup/cofactor product checking, removal of the artificial u32
cofactor rejection, direct FastCurve construction, and Group::Prime context.
Do not replace complete older files. Preserve both bounded native root paths,
their corrected auxiliary/cap replay, current prime-chain behavior, and F4/PKM
field/arity limits. Legacy ecc::CurveParams and all frozen draw identities stay
unchanged. No fake h=1, truncation, resampling or subgroup substitution.

Source review consulted retained DESIGN.md and VALIDATION.md, including their
SEC1 v2 C.2 cofactor definition and existing Point/FastCurve interface analysis.
This is reuse of a retained implementation, not a new algorithm or performance
claim. No fresh external source check was performed for this integration.

Retain the three donor tests: exact_prime_context_matches_legacy_small_cofactor_arithmetic,
exact_prime_cofactor_above_u32_replays_and_solves_without_witness, and
exact_prime_cofactor_rejects_invalid_metadata_without_overflow. The fixed
large-cofactor fixture has p=266287972351, r=31, h=8589934592; its independently
checked order and BSGS scalar-label replay are software controls, not corpus
coverage. Keep both-twist small-cofactor arithmetic equivalence and invalid
metadata / F4 / PKM admission checks.

Historical donor gates: 5186 full-library passes, zero failures, 123 ignored;
161 dataset passes, eight CLI passes, strict Clippy. Those results apply only
to donor runtime40469deb, not this combined source. Parent will run focused
tests, required full library, lint and relevant CLI/restore checks, and preserve
source/binary identities and failures. This author runs no builds or tests.

The original corpus scope, $200 total ceiling and worker IAM/admission gates
remain in force. Complete execution costs, broad dataset production, trained
model efficacy and first/last-fall-degree claims remain unresolved.

## Pre-build bridge-test amendment

Before any combined build or fixture execution, extend the existing frozen
large-cofactor test with a bounded m=2 native root invocation on precisely its
existing public target and factor-base object, with private_witness still null.
Require one retained prime-root certificate, parse it and invoke the independent
prime root artifact verifier against the exact public/base/config objects.
Do not demand a PDP solution, change seeds or regenerate a different fixture.
If the configured budget censors the call, retain and assert its incomplete
status rather than calling it completed. Preserve the original BSGS, scalar
tamper and F4/PKM boundary assertions. This is an arithmetic-context integration
bridge, not a full factor-base transcript/export or scientific corpus cell.
