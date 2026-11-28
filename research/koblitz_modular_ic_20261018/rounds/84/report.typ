#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= R84: corrected direct reduction is inconclusive
The source-polynomial ring-scope repair worked. The first denominator-cleared fraction polynomial reached reduction, but Singular warned that the saved candidate generators are not a standard basis and returned a nonzero remainder. This is not a nonmembership certificate. The strict success gate rejected the run with zero positive memberships.

#image("inconclusive84.svg", width: 100%)

The next verifier needs a coefficient witness from the normal.lib construction or a certified local basis. The accepted rank-32 RR chart offers an independent route to the proper source normalization.
