#set page(paper:"a4",margin:(x:15mm,y:13mm))
#set text(font:"DejaVu Sans",size:9pt)
#set par(leading:0.5em)
= Retaining ungrounded edges produces more exact relations
*Deferred graph50 · recorded-input mathematics · 17 November 2026*

The new bounded Rust adjacency queue retains valid two-class edges until an endpoint gains an exact unit anchor. On the same108058validated Runpod49sources, it yields865full integer cancellation witnesses versus522for the existing grounded-discard graph. The recorded prefix reaches516rows at complete B batch155648, compared with185344for the baseline. This is a mathematical projection of retained sources; no new natural-yield, timing or DLP experiment ran. PCC-20261117-e9bd73 is UNVERIFIED. [pcc:unverified]

#image("recorded-projections.svg",width:100%)

== What the queue preserves
Every accepted source has zero, one or two distinct signed classes. An anchor is a source combination with large part exactly +D_v. A newly anchored vertex wakes its pending edges: one known endpoint propagates a signed anchor; two known endpoints produce an integer-zero large part. Unanchored cycles emit nothing. Coefficients are integers, including2when paths overlap; there is no modulo2or modular-inverse shortcut. [pcc:unverified]

Source ids remain original and consecutive. The queue processes pending edge ids in insertion order. Every deferred edge is activated at most once, and a bounded refusal consumes its original source id. Caps cover lifetime sources/vertices/deferred edges/witness support; malformed shapes leave state unchanged. [pcc:unverified]

== Checked scope and observed state
Thirteen combined Rust controls pass:32sign orientations, unanchored odd-cycle refusal, shared-path coefficient2, explicit vertex/edge/witness/source bounds, malformed inputs and64×257synthetic streams with independent integer incidence checks. Strict Clippy and optimized builds pass. [pcc:unverified]

Recorded replay hashes the original screened49raw, reproduces every baseline graph decision and checkpoint, and independently multiplies32408anchor/865full incidence witnesses. Every projected row annihilates the original normalized kernel. Dense u128row reduction reaches actual rank508at B154037/complete batch154624; this is not a new sparse-kernel timing or success claim. [pcc:unverified]

Final state:141145observed classes,32408anchors,90610deferred edges,15825activated and74785still live. Maximum stored/emitted witness support27/25; no vertex, edge or witness refusal. Default bounds262144sources/classes/deferred edges and256witness terms. Longer witnesses, memory and queue cost remain to be measured. [pcc:unverified]

== Prior art and next experiment
Read Gaudry–Thériault–Thomé corr2004-29,§3.1–3.3, primary URL https://cacr.uwaterloo.ca/techreports/2004/corr2004-29.pdf. Their full LP graph retains unrooted edges;§3.3discards them for simpler analysis. Their non-F2multiplicity warning motivates exact signed cancellation. This queue is a conservative retention variant, not a novelty or complexity claim. KB tools/wrapper unavailable; existing modular large_prime_filter.rs reviewed and not substituted for integer evidence. [pcc:unverified]

Next integrate a separate screened deferred collector and complete independent replay, preserving older schemas and consumed calls. New projections must record their activation source explicitly: a balanced cycle can cancel its latest anchor from the final witness, so witness.max_source alone cannot bind the emission checkpoint. Freeze a new source/protocol before Runpod measurement. [pcc:unverified]

Original49source bd69a76db876fa1da120de0edf1033489327d937; raw SHAe767e8a175a475645b3a899a32c2f0643ce20a750693aaa8797697934eb4bc56. Original complete mathematical replay is separately recorded at PCC3319ad, also UNVERIFIED. Original failures/results stay unchanged; all owned pods through49deleted. No complete-cost or scoreboard admission. [pcc:unverified]
