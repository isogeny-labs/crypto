# Protocol: Brent cycle detection in the fully priced Frobenius rho (frozen before data, 2026-11-06)

**Question.** Does `rho.signed_frobenius_v2_full` cost less than `rho.signed_frobenius_full` (the `rho-frob` reference of the Koblitz orbit IC panels), per target, in fully priced GAE?
- The v2 method is the v2 walk with Brent cycle detection beside the 16-point window, priced exactly as the legacy walk.
- The read-only survey of the committed lazy-orbit records (`research/ecbench_lazy_orbit_members_20261018`) found that capped walks take 17–59 % of `rho-frob`'s walk operations at n17–n31, 7.9 % at n37 and 1.1 % at n41.
- A cheaper `rho-frob` is a stronger matched-rho reference, not an index-calculus improvement: every IC row's distance to it grows.

**Arms** (one session per curve, every arm a `*_full` method, nothing else differs):
- `rho-frob`: `rho.signed_frobenius_full` (legacy walk), the reference.
- `rho-frob-v2`: `rho.signed_frobenius_v2_full`, the candidate.
- `rho-frob-aa`: `rho.signed_frobenius_full` again, the A/A control.

**Panel.** The eight Koblitz curves of the lazy-orbit panel: a0 n13, a1 n17, a0 n19, a0 n23, a1 n29, a0 n31, a0 n37, a0 n41.
- Each curve gets 32 public targets and 3 rounds, with the arm order alternating, 1 warm-up and isolation L0.
- Target seeds are the panel's plus 1 (fresh targets).

**Pricing.** `full_prices.json` as committed. The v2 walk's extra counters (lost steps, long cycles, table queries and inserts) are descriptive: the lost steps are inside `walk_operations`, and the one lookup per distinguished point is charged through `distinguished_points` for both walks. The comparison is repeated with every native counter priced at 0, 0.1 and 1 GAE per operation (AGENTS.md §5a).

**Decision rule, per curve.** Paired by target, `ecbench compare --a rho-frob-v2 --b rho-frob`:
- **cheaper:** the 95 % interval of S(v2)/S(legacy) lies below 1 at all three native weights;
- **dearer:** it lies above 1 at all three;
- **no difference:** otherwise.

The run is valid only if the A/A interval (`rho-frob-aa` vs `rho-frob`) contains 1 on every curve, and every record verifies with `--replay-all`.

**Not claimed.** No index-calculus result and no leaderboard or `benchmarks/README.md` row: the validation gate needs a replay certificate, a validator report and a matched reference, which this run does not file. Counted GAE does not depend on the host; wall time is not compared.
