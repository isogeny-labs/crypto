#import "../local-report-template.typ": render
#let s = json("summary.json")
#let plan = json("runtime-plan.json")
#for (i, oracle) in ("F4Groebner", "PreparedSymmetricPairOnly").enumerate() {
 if i > 0 { pagebreak() }
 render(summary: s + (strata: s.strata.filter(r => r.oracle == oracle),), plan: plan,
 figure: image("ratios-" + oracle + ".svg", width: 100%),
 title: "Censored full cubic IC: " + oracle,
 run: "local-s4-f4-ic-scale-20261115-10", duration: "900.003",
 flow: [The process timed out at 900.003 seconds (exit124), retaining 181 of
 240 planned completed execution rows: 31 warmup and 150 measured. Six strata
 have all five measured rounds; both p1039/seed79 strata are incomplete. Exact
 additional in-flight attempts and partial-arm costs are unknown because there
 are no execution-start records. No retry or cost imputation. [pcc:unverified]],
 findings: [Two complete strata fail local A/A controls and remain diagnostic.
 The p1039/seed73 prepared-pair comparison passes at 1.85 (95% interval
 1.51–2.08). Missing last-target ratios stay missing; the native F4 p1039
 completed comparison fails A/A. The whole-process censor remains part of the
 result. No tested eligible graph interval reaches 1. [pcc:unverified]])
}
