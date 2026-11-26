#set page(paper: "a4", margin: 17mm)
#set text(font: "DejaVu Sans", size: 10pt)
= Candidate 1: pilot below the 50% target
The 16-call engineering pilot and all three phased/unphased calibrations pass. Every query output retains its original correctness checks. The fixed degree 131 uniform pilot records the following complete cold instruction counts. These are pilot observations, not a primary-panel result. [pcc:unverified]
#table(columns: 2, [Source variant], [Cold simulated instructions], [Original], [44,025,553], [Prior incumbent], [34,988,002], [Candidate 1], [23,798,322], [Fixed 50% goal ceiling], [22,011,702])
The frozen Rust smoke summary reports 45.9443% fewer instructions than the matched original control. The target is missed. No primary rows have been collected, and no primary subset is selected. Original inputs, source snapshot, draft protocol, all pilot rows and the earlier failed qualifier prefix remain preserved. [pcc:unverified]

Oracle instructions are 16,184,304 for candidate 1 versus 18,071,029 original and 18,096,046 incumbent. All field-operation tallies are unchanged; total simulated instructions price the implementation changes. [pcc:unverified]

The broader root release build reached its 600-second timeout compiling dependencies, before tests ran. The focused 47-test suite and 195-row three-executable qualifier pass, but do not discharge root integration or signed independent admission. [pcc:unverified]

Next test a separately profiled fused multiplication/reduction path and qualified field-cache lookup. Keep the exact query, original denominator, 50% threshold and every verification step.
#pagebreak()
= Candidate 1: remaining costs
#image("../profiles/candidate1-cold/hotspots.svg", width: 92%)
Two fresh source-bound profiles agree. The compact field reduction function alone costs 1,651,088 instructions; the hardware multiply stack costs 1,308,024 and its outer word wrapper 459,576 self instructions. These are descriptive call-stack views and are not added to inclusive ancestor costs. [pcc:unverified]

Protocol candidate1.profile.v2.protocol.json was published before these calls. Its v1 predecessor was unrun; v2 corrects a copied instrument-label value. The complete framework manifests/raw data remain under profiles/. The profile benchmark metadata was imported afterwards from the actual manifests, retaining their real timestamps and commands. No counter call was rerun for bookkeeping.
