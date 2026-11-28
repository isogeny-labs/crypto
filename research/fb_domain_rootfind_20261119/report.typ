#set page(paper: "a4", margin: 1.8cm)
#set text(size: 10pt)
#set par(justify: true)

= Factor-base domain root extraction in the `f4_fp` visitor

*2026-11-19 · isogeny-labs/crypto PR \#4826 · record `20261119T1919Z-fb-domain-rootfind`* \
*Standing:* every number is `[pcc:unverified]`. This is a single-host, unisolated (L0) component diagnostic. There is no replay certificate, validator report or `VALIDATION.json`, so it is not an admitted benchmark row.

== Question
The recursive visitor of `f4_fp` solves the algebraic index-calculus decomposition systems, whose leaf variables are restricted to the factor-base x-set $D$. For a univariate in such a variable it evaluated the polynomial at every element of $D$: $|D| dot "terms" dot log("deg")$ multiplications for each univariate at each node, and none of it was counted. Can that root step be made independent of $|D|$ without changing the traversal?

== Change (opt-in; the scan is the default and the control)
`DomainRootMode::Frobenius` finds the $FF_p$-roots by $gcd(g, x^p - x)$ and Cantor–Zassenhaus, which costs $O(log p dot "deg"^2)$ multiplications. It keeps the roots in $D$ through a value → position index built once per traversal and returns them in domain order. This is exact because $D subset FF_p$, and the order makes the branch values, recursion and visitor sequence the scan's. `Auto` picks per univariate by an estimated multiplication count.

== Correctness
- 4 new tests:
  - extractor equality on more than 600 adversarial univariates ($p$ from 2 to $2^32 - 5$);
  - identical traced branch sequences in all three modes on planted F4 systems;
  - unchanged default serialisation.
- `f4_fp` suite: 69/69.
- The example asserted identical solution sequences, F4 work and completion in every mode on 100 targets over 11 configurations, with every planted target decomposed.

== Results
#figure(image("root_step.svg", width: 100%), caption: [Root-step multiplications (top, counted) and wall time (bottom, L0). Labels: scan ÷ Frobenius as printed by the example; above 1 means Frobenius is cheaper.])

#table(
  columns: 9,
  align: right,
  [m], [bits], [$|D|$], [fmul scan], [fmul Frob], [count ×], [wall scan], [wall Frob], [wall ×],
  [2], [16], [64], [524,288], [728,409], [0.7], [0.010 s], [0.018 s], [0.5],
  [2], [16], [256], [8,388,608], [2,914,627], [2.9], [0.063 s], [0.069 s], [0.9],
  [2], [20], [256], [8,388,608], [3,678,558], [2.3], [0.054 s], [0.074 s], [0.7],
  [2], [20], [1024], [67,108,864], [7,442,442], [9.0], [0.367 s], [0.162 s], [2.3],
  [2], [24], [1024], [67,108,864], [8,942,083], [7.5], [0.388 s], [0.185 s], [2.1],
  [2], [31], [1024], [67,108,864], [11,909,735], [5.6], [0.363 s], [0.221 s], [1.6],
  [2], [31], [4096], [536,870,912], [23,800,840], [*22.6*], [2.757 s], [0.434 s], [*6.4*],
  [3], [16], [16], [507,904], [2,822,840], [0.2], [0.098 s], [0.144 s], [0.7],
  [3], [20], [32], [3,096,576], [10,904,975], [0.3], [0.275 s], [0.454 s], [0.6],
  [3], [31], [32], [3,096,576], [17,434,767], [0.2], [0.345 s], [0.645 s], [0.5],
  [3], [31], [64], [16,646,144], [47,226,203], [0.4], [0.901 s], [1.642 s], [0.5],
)

== What this shows and does not show
- The Frobenius root step does not grow with $|D|$, while the scan's grows linearly. The advantage rises with $|D|$ and falls with $log p$: 22.6× fewer root-step multiplications and 6.4× less wall time at $m = 2$, 31 bits, $|D| = 4096$.
- Wall-time gains are smaller than count ratios, because per-call overhead (allocation, gcd, splitting) is uncounted. At $|D| = 256$ the Frobenius path is slower in wall time, and `Auto`'s count model is miscalibrated (1.36× slower than the scan at 20 bits).
- For $m = 3$ chains with $|D| <= 64$ the scan wins.
- Not shown: any relation-collection, linear-algebra or complete-solve change; any comparison with rho; a scaling law.

== Owed
An isolated (L2) host, an independent-host replay, a validator recompute and a `VALIDATION.json`, before any admitted row.

== Next
Proposed: batched sibling root extraction. One product tree and remainder tree over $D$, shared by all children of a branching node, would remove the per-call overhead that limits the wall-time gain.
