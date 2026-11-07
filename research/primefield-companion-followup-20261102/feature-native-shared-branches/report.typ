#set page(paper:"a4",margin:17mm)
#set text(size:10pt)
= Share coefficients across phase branches
T-1873 · 2026-11-07 · Prime-field compressed recovery

One bounded reader now supplies all surviving phase branches. Quotient supports, selected roots and power caches remain separate. The adapter applies C0 across the branches, rebuilds survivor rings, then shares the F1 and F2 traversals before applying each C2 constraint.

#image("flow.svg",width:100%)

*Correctness.* Componentwise induction on the balanced tree gives the same value and arithmetic for each branch. Constraints on distinct branches are independent; changing their processing order preserves successful final states.

#table(columns:(3fr,1fr),inset:4pt,
 [Native control],[Observed],
 [Mode groups / branch value equalities],[225 / 450],
 [Canonical state equalities],[2,516],
 [Invalid width/field/scale rejections],[15],
 [Mode reader deliveries, separate → shared, cap64],[59,920 → 15,400],
 [Mode reader deliveries, separate → shared, cap16],[120,800 → 37,500],
 [Source calls:281targets at cap64 / cap16],[2,881 / 3,152])

Public recovery routes remain identical with zero unresolved targets. Maximum active branch count is4. The synthetic mode panel has180shared versus450separate streams and6,540versus16,350reader deliveries. Per-branch scalar/pair product counts agree exactly.

*Costs.* For q live branches and L mode coefficients, q readers/qL deliveries become1reader/L deliveries. Counts cover completed mode calls only; initial support and later extraction streams remain charged. Up to four rings coexist. Synthetic maxima:346cached coefficient slots and34returned slots; public cache maximum89. These separate maxima exclude capacities, map overhead, clones and recursive temporaries and are not peak memory. No field-operation or timing claim.

*Evidence.* validation01passes with76source pins; inspected PDF/SVG and manifest. No new compiler/native/render failure. Independent signed verdict pending. No cloud/timing run. Original4,563cohort unchanged; historical fullCargo fixture/deadline remains. Full derivation and replay: DERIVATION.md, RESULTS.md, README.md.

*Sources and next.* Exact reused-power predecessor and official FLINT quotient conventions, flintlib.org/doc/nmod_poly/index.html. KB unavailable. Next derive a joint neighboring-mode coefficient recurrence, checking index alignment and denominators before implementation.
