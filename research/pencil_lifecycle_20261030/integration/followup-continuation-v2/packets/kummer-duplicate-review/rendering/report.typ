#set page(paper:"a4",margin:18mm)
#set text(size:11pt)
#text(size:18pt,weight:"bold")[Independent duplicate-adapter review]
#image("diagram.svg",width:100%)

#text("Independent reviewer: root. Renderer and original producer: kummer_transitions. Findings below are root’s; rendering adds no review or replay.")

#text("Root inspected the helper, fixture, native scalar-order checks and graph forwarding, verified 32 producer hashes and 18 preexecution bindings, and ran one fresh offline replay with a separate target cache. The native exit was zero and parsed output matched the producer exactly: all eight frozen controls passed.")

#text("Both scalar labels are negated when the signed divisor is canonicalized. The first graph edge still receives its original signed divisor and original labels; a duplicate is evaluated before graph suppression. Prime r, nonidentity G and rG=0 establish exact order for the stated valid group domain. rQ=0 alone does not establish Q membership in that subgroup, and the helper correctly documents membership and D=aG+bQ as caller obligations. Incoming divisor validity and collector eligibility are also external obligations; this is not a hostile-input validator.")

#text("The informative labels are deliberately constructed from the public known scalar 31124 on p=739,r=34147. Both accepted candidates are verified by native scalar multiplication. The changed-target control deliberately violates the initial label invariant and is rejected at candidate verification; it is not evidence that first-edge insertion validates arbitrary labels. The graph retains one edge and zero cycles throughout every case.")

#text("Initialization, reference-label checking, graph processing, scalar arithmetic and candidate-verification fields are reported separately. Hashing, memory, comparison work and fixture construction are not fully priced. No natural collision rate, trajectory yield, end-to-end performance or complete-cost ratio is established. No blocking discrepancy was found at this isolated correctness scope.\n")