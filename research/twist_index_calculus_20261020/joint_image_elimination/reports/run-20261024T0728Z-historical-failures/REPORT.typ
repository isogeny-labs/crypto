#set page(paper: "a4", margin: 17mm)
#set text(size: 9pt)
#set par(justify: true)
#show raw: set text(size: 7.6pt)
= #text("Historical S5 fixture binding: two retained checker failures")

#text("These are provenance-checker failures, not missing physical relations or evidence about S5 solver cost. Neither attempt ran a collector or BSGS.")


#image("../../figures/historical-match-failures.png", width: 100%)

== #text("Version 1: match assertion")

#text("twist-joint-historical-s5-match-v1.service builds the native checker, then its match exits 101 at compatible(). The raw mixed trace stores query triples [a,b,oracleHitCount]; the frozen public exporter retains all 61 query coefficient pairs [a,b] in order. Comparing these arrays directly fails despite matching public coefficients.")


#text("The correction constructs exactly the exporter's 13-key public projection and removes only the query oracle-count column. It retains the full trace, its order, every coefficient and the parameter/physical-set checks. Coordinate, query and omitted-tuple corruption checks remain; a truncated trace check is added. The original source and failure are not rewritten.")


== #text("Version 2: compilation")

#text("twist-joint-historical-s5-match-v2.service records compile exit 101: the diagnostic JSON macro rejects a method call on an unparenthesized array. No matching or corruption control runs in this version. The syntax correction in commit 5e8611ed78 adds parentheses to that diagnostic expression and changes none of the acceptance checks.")


#text("Raw commands, source hashes, stdout, stderr and exits remain in runs/historical-s5-match-v1/metadata/ and runs/historical-s5-match-v2/metadata/. Heavy work is serialized by the shared busy reservation; the match has one thread, 8 GiB and a 60-second watchdog after separate compilation. No wall-time ratio is admitted. There are fewer than two comparable scientific values, so the figure shows executed failure phases rather than a quantitative yield chart.")


== #text("Required result and limitation")

#text("A corrected checker must bind p37/seed0/query2 to the completed pure source-block solution, the separate signed-set/RREF audit and the identical historical physical tuple; all four corruption controls must reject. Only an actual new receipt can establish that result. The old report lacks a complete runtime input hash. Even a passing source-and-record match cannot create a retroactive runtime seal, complete the 61-query collector, prove generic ideal equality or establish a cost improvement.")
