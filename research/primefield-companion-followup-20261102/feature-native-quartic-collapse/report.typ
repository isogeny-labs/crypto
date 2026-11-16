#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= Quartic collapsed-signature target
2026-11-05 · T-1667

A width14 decoder now represents the mean of each character-signature fiber using465 terms, exactly four quartic. The recorded cubic control rejects this target; the quartic packet passes a separate full-field check. [pcc:unverified]

#image("pipeline.svg", width: 100%)

== Why this target
Equal signatures cannot be separated by raising polynomial degree. Instead define F(x) as the mean of the complete signature fiber of x. It equals x on singleton fibers. Reflection permutes fibers and commutes with their means, so F(Z-c)+c is odd for c=(q-1)/2. This target enters the existing odd coefficient solve. [pcc:unverified]

The compiler enumerates the bounded public coefficient field, creates point-indicator corrections to X, and constructs polynomial coefficients. It receives no elliptic target or unknown logarithm. Enumeration and field-sized storage remain setup costs; the certificate fibers are not a runtime scalar-answer table.

== Quartic columns
For h=(r-1)/2 and degree d at most4, D=dh is at most2r-2. The forward Miller prefix covers0 through min(D,r-1); a reciprocal prefix covers original indices D down to r. Its maximum recurrence index D-r is at most r-2. Thus both sweeps avoid the singular index and cover all coefficients. [pcc:unverified]

== Exact records
At r479/q14, the cubic rejection has observed rank231. The quartic result has observed rank233 and721 orbit columns. The eight ambiguous pairs contain16 inputs; the remaining463 inputs retain X. The checker evaluates direct character products using a square bitmap and independently reconstructs the fiber means. It accepts all479 values and rejects a coefficient-sign corruption. Ranks remain producer outputs, not independent rank certificates. [pcc:unverified]

Twelve native tests pass, including width15 cubic retention, five small-prime fixtures and domain/degree/size controls. Five source/data hashes bind the run; three compiler warnings are retained. The separate checker has no constructor, recurrence or decoder imports, but was written in the same producing session. [pcc:unverified]

== Scope and next construction
This is the collapsed polynomial target, not yet a width14 public-point recovery route. Next add conditional separating characters for the eight pairs and connect them to the existing lazy source extension. Extra offsets and complete costs must be checked. No cloud job or timing panel ran. The original4563-case cohort is preserved.

== Evidence
Claims PCC-20261105-90e4fe and PCC-20261105-3d5a49 await signed coverage. Source, packet, fibers, receipts and logs accompany DERIVATION.md. Prior work: saved reflection, fiber-repair and bidirectional constructions; #link("https://dlmf.nist.gov/27.9")[NIST DLMF27.9] for the character reflection law. No novelty claim.
