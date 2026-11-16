#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= R46: exact polynomial-root collection
Fixed-chart observations, not an independent verdict or IC speed comparison.
#image("round46.svg", width: 100%)

== Exact substitution and complete roots
The registered equation gives physical nu(lambda)=N(lambda)/C. Its gamma
coefficient is checked identically equal to b. Substitute into every source
equation and take the common GCD of both coefficient components. Intersect
with lambda raised to q minus lambda, then split with field-basis trace
polynomials. Divisions and final linear factors reconstruct exactly; every
root re-evaluates in the original GCD. All original equations, eight units,
coefficient descent, Tate order 11 and deck/Frobenius checks remain mandatory.

The complete degree-7/9/11 output lists match the exhaustive R45 collector
byte for byte before the degree-13 candidate starts. The original degree-13
producer was terminal first. The new degree-13 export also matches exactly:
580 closed points in 58 deck orbits, comprising 7540 rational source points.
No sample replaces the full output comparison. All native attempts and proof
receipts are indexed in results.json.

== Counted scope and proof boundary
The two candidate counts describe different detector inputs. Polynomial
construction, GCD, Frobenius and trace splitting have additional costs;
no matched complete timing or counted-work panel was measured. This is not
an IC speedup claim. Polynomial certificates are checked in-process and are
not separately exported. Full-list equality supplies the independent
exhaustive-detector control, not an independent scientific replay.

The Lean module proves conditional completeness from a reconstructed product
of linear factors and a nonzero leading scalar. Native arithmetic, the GCD
implementation and proper geometry are not formalized. Audit outcomes and
all failed attempts, if any, remain in results.json.

Next: actual complete divisor evaluation on R45's new supports, signed Picard
classes, nonzero target action and dependence checks. Proper level-77 map,
quotient saturation/polarization, theta and node187 remain open. Fixed n=5
carrier; larger residue degree is not a larger-n Koblitz transfer.
