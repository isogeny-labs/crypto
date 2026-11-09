# Branch helper cast successor: source review

The only Lean change adds Nat.cast_ofNat to the existing simp-only list. The retained line153 failure showed natural numeral2 casts remaining in the goal. The pinned Mathlib declaration rewrites casting ofNat(n) to ofNat(n), under NatCast and Nat.AtLeastTwo; its proof is rfl. ENat supplies the cast structure and n=2 meets the numeral condition. This is a source-level repair rationale, not a completed kernel check.

The coefficient theorem, rational controls, separate thirteen-name readback and expected False rejection remain byte-identical. All26 retained prerequisite source/artifact bindings and runtime pins remain unchanged. No failed helper artifact is added. Full16736-file pre/post custody remains required.

The concrete runner changes fresh workspace/unit/release identifiers and finite budgets:600seconds total,180precheck,240compiler,180postcheck,1CPU8GiB/no swap. All five calls share the compiler budget. Predecessor phase timings motivate a bounded attempt but do not demonstrate that all calls fit. Numeric ledger fields remain null and release false pending parent binding/publication.

No remaining blocking source finding was identified. Prior failed support sorryAx remains preserved. No target theorem or final audit is newly established, and no compiler or test was run here.

The reviewer authored the helper replay ancestor; renew_norm authored the correction. This is a disclosed non-blind review of the exact delta, primary API, controls and runtime adaptation. KB unavailable; exact retained source, failure output and pinned Mathlib files were used.
