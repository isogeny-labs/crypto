#import "../local-report-template.typ": render
#let s = json("summary.json")
#let plan = json("runtime-plan.json")
#for (i, oracle) in ("Groebner", "F4Groebner", "PreparedSymmetricPairOnly").enumerate() {
 if i > 0 { pagebreak() }
 render(summary: s + (strata: s.strata.filter(r => r.oracle == oracle),), plan: plan,
 figure: image("ratios-" + oracle + ".svg", width: 100%),
 title: "Full cubic IC: " + oracle,
 run: "local-s4-f4-ic-small-20261115-09", duration: "181.220",
 findings: [All twelve strata retain five correct measured rounds and passing
 local controls. Native F4 is cheaper than the old Macaulay pipeline in these
 separate trajectories, but its full cold ratios remain 743–1356. Prepared pair
 ratios are 2.08–3.87. F4 and Macaulay streams are not asserted equal: charged
 fallback paths can differ. A representative native case spends most counted
 arithmetic in characteristic polynomial and eigenvectors. These counts do not
 constitute a native wall profile. [pcc:unverified]])
}
