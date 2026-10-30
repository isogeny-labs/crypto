# Original dynamic mathematical inputs

Baseline d2e534f95ff573e9b304c2374f12009b4e7d0dd1. Sources reviewed: curves.rs generate/verify_public/verify_factor_base, solvers.rs Group::read/domain, dynamic replay Expected, execution.rs original launch limits. This source-only increment constructs independent replay inputs from original public objects and config, never the private witness or saved solver polynomial/terminal.

Use the explicit dormant f4-wide-dynamic-v1 solver identity; do not reinterpret historical f4 cells. Check prime family, n33..61,m4..15, positive original work/memory/trace and the existing fixed native degree cap12. Reject a different degree_bound instead of silently ignoring it; general configurable-degree support is still separate work. Public family/n/m/twist and factor-base requested_size must equal config. Verify public curve/order/twist and factor-base membership, then reconstruct signed domain in existing canonical order and sorted unique x codes. Check actual modulus width and profile capacities64 signed points/32 x codes. Own these originals so later expected-input views cannot be supplied with a different config.

The caller still must authenticate config and limits against frozen plan/run; this function does not manufacture a run binding or select a production route. No plan allowlist or CLI dispatch activation is included.

Authored controls: n47/n61, m4/m15, both twists, no private-witness dependence, retained original resource values, signed closure, Mersenne modulus at61, changed requested size/twist, noncanonical factor-base x, legacy solver identity, different degree cap, zero work/memory/trace and out-of-profile arity. Outside-only candidate until preceding immutable validation finishes; not compiled/tested yet.

Applied after baseline release compilation and582focused tests passed (0failed,7ignored). Constructor and new controls require fresh compilation/tests; no successful validation claimed at commit time. Production plan allowlist/dispatch remains unchanged. Parent integration must validate original run/config/limits before calling this constructor and reconcile outer claims after file replay.
