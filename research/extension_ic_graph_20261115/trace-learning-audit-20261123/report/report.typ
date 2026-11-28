#set page(paper:"a4",margin:15mm)
#set text(font:"DejaVu Sans",size:9pt)
#set par(leading:0.5em)
= Trace-learning bounds and replay acceptance
Rust recorded-input mathematics · 23 November 2026

Eight fixed F4 calls and six linear-stage checks pass on the first saved system. Learning, self replay and ordinary reference produce the same captured root for both arities. Producer evidence: PCC-20261123-ac5831, *UNVERIFIED*. [pcc:unverified]

#image("learning-bounds.svg",width:100%)
#table(columns:(1.5fr,1.25fr,1.1fr,1.3fr),align:(left,right,right,right),[Path],[Recorded sum],[Learning mul upper],[Included mul upper],[Five with trace],[1813876894],[2097520],[1815974414],
[Six with trace],[1751507290],[3589725],[1755097015],
)
The six-coordinate upper bound remains below the five-coordinate recorded lower bound for this *limited multiplication subtotal*. Every saved stream has one learning run; the new fixed control matches its emitted F4 counter and matrix maxima. This is not a complete F4/descent/pipeline cost or runtime ranking. [pcc:unverified]

== Where the missing bound comes from
For each step, recorded rows and free columns are subsets of the pair rows R and full columns C; rank k is at most min(R,C). The tracked dense reduction performs at most `R*C*k` row multiplications and `(R-1)*(C+1)*k` additions, including p-f. Add k pivot inversions, each using 19 Fermat multiplications at p1009. Contributor updates use at most `(R-1)*k*ceil(R/64)` word ORs. Empty matrices contribute zero; every sum/product is checked in u128.

#table(columns:(1.5fr,1.3fr,1fr,1.3fr),align:(left,right,right,right),[Path],[Additions upper],[Inverse calls upper],[Word ORs upper],[Five with trace],[2057260],[254],[10528],
[Six with trace],[3542800],[261],[25836],
)
Inverse calls and their expanded Fermat multiplications are alternative units, never added twice. Bounds are upper limits, not actual event counts. Allocation, row copies, pivot/contributor scans and other bookkeeping remain outside them.

== Acceptance control
All saved 66 five-coordinate and 33 six-coordinate mismatch cases meet the existing inconsistency-or-staircase-at-most64 condition. Injected invalid stored row indices return mismatch without that condition; the audit rejects them before linear extraction. That synthetic corruption is not a naturally observed worker failure. The stopping rule is not a general Buchberger certificate or completeness theorem. [pcc:unverified]

The fixed captured controls agree; no production change is made solely from injected corruption. Dedicated workers retain internally learned, arity-separated traces. Broader exceptional-input guarantees stay open.

== Scope and next step
This is a once-only recorded control from a published immutable native binary, not a timed benchmark. No new target or natural relation search was generated. No elapsed data is exported; internal budget clocks remain. Runpod funds/checkpoint access and complete isolated cold accounting are still required for the requested comparison and parity conclusion.

Sources: audit-original.json, PLAN.md, derived-comparison.json; trace_learning_audit.rs and trace_learning_report.rs; retained Root41 source and full saved-cohort counters. #link("https://github.com/isogeny-labs/crypto/pull/4986")[PR 4986].
