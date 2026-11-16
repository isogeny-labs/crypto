#set page(paper: "a4", margin: 17mm)
#set text(size: 10pt, font: "DejaVu Sans")
#set par(justify: false, leading: 0.6em, spacing: 0.7em)
#set heading(numbering: none)
= Literal norm and actual Euclidean inverse
#text("Both unchanged original Lean4.35/4.34.1 gates audit66/0 failing/0 trusted, with seventeen complete statement bindings35c3a5. This proves normalized raw norm and actual signed Euclid arithmetic at223. Claims remain unsigned. [pcc:unverified]")
== Original unsigned norm expression
#text("Keep t0=a*p0%223, t3=(b*p2+c*p1)%223, result=(t0+3*t3)%223. For canonical words<223, products≤49284, pair sum≤98568 and final sum≤888. The source proves no-overflow/exact toNat expression, canonical output, decoded polynomial and exact raw cofactor norm value. [pcc:unverified]")
== Actual original base inverse
#text("Fp.inv initializes(t,nt,r,nr)=(0,1,223,n), uses qq=trunc(r/nr), updates(nt,t−qq*nt,nr,r−qq*nr), stops at nr=0 and adds223 once if t<0. It does not use powmod; the earlier mathematical N^221 certificate is not substituted for this algorithm. Original source/counters remain unchanged. [pcc:unverified]")
#image("actual-Euclidean-state.svg", width: 100%)
#text("The model returns None for zero guard/fuel exhaustion. Kernel certificates over only223 base inputs prove every nonzero input succeeds, terminalnr0/gcd1, normalized output in[0,223), and the returned word's inverse identity. Sixteen is a proved sufficient bound, never an accepted truncated loop. No cubic coefficient population is enumerated. [pcc:unverified]")
#text("Every visited state and quotient/product/subtraction fits signed64. Remainders are nonnegative; actual divisors positive. Int.tdiv models toward-zero division, and the original Std lemma proves agreement with Euclidean division here. Final normalization is nonnegative/<223, so the unsigned cast fits. Structural lemmas expose actual returned values. [pcc:unverified]")
#pagebreak()
= Controls, complete bindings and scope
#table(columns: (36%,64%), inset: 6pt,
 [Zero/exhaustion], [Zero guard and zero-fuel nonterminal computation return None.],
 [Negative normalization], [Input2: t=−111, normalized112,112*2=1. Input222 returns222.],
 [Overflow boundary], [Noncanonical2^63 times2 wraps in the norm; unconditional decode multiplicativity is rejected.],
 [Exact audit], [42old+19new named+5generated=66. Seventeen complete types; four inherited deprecated-if warnings per build/original auditor warning retained.],
 [Resources], [Both original gatesj1/120s/default memory/default tactics; standard foundations only, no native decision/custom axiom.],
)
== Preserved source and binding failures
#text("First9e7844 leaves Fin223 constant3 value unreduced; entire source/outputs/rejected sorryAx retained. Explicit original value/rfl repairs only proof. Accepted64 source576848 then exposes the original extractor stopping at let p := in two types. Three complete raw-norm records are reused; original two proved let declarations remain unbound. Two explicit equivalent no-let corollaries supply full hypotheses/conclusions. Final66 preserves the entire accepted64 source prefix, no verifier/hypothesis/gate change. [pcc:unverified]")
== Primary sources and next bridge
#text("Preregistratione0cb48: exact original common.rs/candidate/Word07/finite08, full official Rust Reference operator source blob3b63884a5e445787078c19422fe627105e2a4933 (overflow/division/casts), actual Std division lemma, full response/byte pins retained. KBfive/conductor127, locked atlas unread; no novelty/absence inference. [pcc:unverified]")
#text("Final source SHA256 745d57b0a41459bd19de5c4b2932a10cee7470da87725bc76c26eaa94e98d06b. Whole sources/callers/builds/audits/bindings/pins/times are retained in complete-types-final-05; additive exact-source CI replays both original gates. [pcc:unverified]")
#text("This is source-level arithmetic, not compiled Rust/ABI/counter/runtime execution. A single typed composition with universal finite08, full cubic field/Jacobian/group maps and matched costs remain separate. Existing chain/cubic/F101 graphs stay unchanged. Active278/12cc ownCI20:55/original authenticated whole-store/gates still control integration. No native experiment, benchmark or original full population ran. [pcc:unverified]")
