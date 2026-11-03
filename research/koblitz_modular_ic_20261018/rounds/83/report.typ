#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= R83: direct fraction verifier rejected
The frozen attempt passed its source/donor gate, but Singular could not access the source module after switching rings. It printed a terminal marker despite native errors, so the strict gate rejected the run. Zero valid fraction identities were checked.

#image("failure83.svg", width: 100%)

The correction serializes source module polynomials as strings before switching rings; it will run under a fresh frozen protocol.
