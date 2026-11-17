IPsec/Oakley group 3: reproducible GHS investigation

This directory provides finite-field checks, a necessary Hess genus bound,
exact class-group certificates, and a resumable search of the isogeny graph
of the IPsec curve over F_(2^155). It does not claim a successful attack or
an exhaustive exclusion of GHS vulnerabilities.

CURVE AND RESULTS
The field is F2[z]/(z^155+z^62+1). The IPsec model is
  y^2 + x*y = x^3 + 0x7338f
where the hexadecimal constant denotes a polynomial-basis bit string.
Its order from RFC 2409 is
  45671926166590716193865565914344635196769237316.

Direct GHS descent of the seed to F32 has m=26 and genus 2^25. The comparison
JMS vulnerable example has m=6 and genus 31. Within the specified fixed-curve
Hess two-factor construction over F32, a necessary genus lower bound is 992.
An independent binary product-span calculation excludes all 46 support pairs
below that bound. No genus-992 descent or actual factor decomposition is
constructed by these checks.

The isogeny class has an unconditionally certified subgroup lower bound of
74,173,379,224,089,213,153,040 curve classes. PARI's class-number computation
gives equality conditional on GRH. The lower-bound certificate checks exact
quadratic-form orders without relying on that completeness assumption.

The local search snapshot records 24,492 screened Frobenius orbits, with no
low-genus candidate in the implemented tests. This is a dated checkpoint,
not a full-class result. See search_snapshot.json for its scope and counters.

RUN THE PYTHON CHECKS
Use Python 3 on Linux; only the standard library is needed. Run from this
directory, using python3 or the Python interpreter of your choice:
  python3 analyze.py
  python3 validate_bound.py
  python3 verify_resumable.py
  python3 explicit_three_isogenies.py

The programs update their named JSON result files. Assertions are part of the
verification; run without Python's -O option. verify_resumable.py kills a test
worker during an expansion and compares the recovered database with an
uninterrupted run, including all graph witnesses and checkpoint counters.

OPTIONAL CLASS-GROUP CHECKS
Install PARI/GP separately, then run:
  python3 full-class/verify_certificate.py --gp /path/to/gp
The exact certificate is fast and does not assume GRH. The separate
full-class/class_size.gp recomputes the GRH-conditional class number and may
take several minutes. Its earlier output is retained in class_size.log.

RUN OR RESUME THE SEARCH
  python3 resumable_search.py --max-nodes 100
  python3 resumable_search.py --status
Omit --max-nodes for an unbounded run. Repeating the command resumes the same
live-search/search.sqlite3 database. It checkpoints each screen and completed
prime-degree expansion. Only one worker may hold the database lock.

Levels 3,17,31 supply modular-isogeny edges. Canonicalization identifies the
155 absolute-Frobenius conjugates sharing these GHS screens. The old bounded
degree-3/7 search could not reach the whole class; the added degrees address
that obstruction. Whole-class coverage with the selected generators remains
conditional on the completeness of the GRH-dependent class-group result.

The worker records direct GHS ranks over all three proper subfields and a
necessary Hess two-factor genus bound over F32. A low necessary bound is a
candidate for further analysis, not a constructed attack. These tests do not
cover every possible Weil-descent construction. See SEARCH_README.txt for
candidate thresholds, persistence, stopping, and backup instructions.

The source package excludes the live database, PID files, downloaded papers,
temporary test databases, and extracted PARI binaries. Starting the program
does not provision durable compute or a scheduler. Earlier bounded search
results are retained for reproducibility. Sample-point order checks are not
independent point counts. No discrete logarithm is solved here.

SOURCES AND DATA ATTRIBUTION
RFC 2409, section 6.3:
  https://www.rfc-editor.org/rfc/rfc2409.html#section-6.3
Jacobson--Menezes--Stein (2001):
  https://www.math.uwaterloo.ca/~ajmeneze/publications/124.pdf
Hess (2004), generalized GHS construction:
  https://doi.org/10.1112/S146115700000108X
Achter--Howe (2017), section 4, class-group torsors:
  https://msp.org/ant/2017/11-1/ant-v11-n1-p.pdf
PARI/GP 2.15 manual and class-number qualifications:
  https://pari.math.u-bordeaux.fr/pub/pari/manuals/2.15.5/users.pdf

The bundled classical modular-polynomial files are Andrew Sutherland's MIT
data, reduced modulo two by the parser at runtime:
  https://math.mit.edu/~drew/ClassicalModPolys.html
  https://math.mit.edu/~drew/modpolys/jfiles/phi_j_31.txt (and other levels)
Requested attribution:
  Reinier Broeker, Kristin Lauter, Andrew V. Sutherland,
  "Modular polynomials via isogeny volcanoes", Mathematics of Computation 81
  (2012), 1201-1231. https://arxiv.org/abs/1001.0402
The worker saves hashes of all active source and polynomial files in each
new database and refuses a configuration mismatch on restart.
