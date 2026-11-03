Classical APS follow-up investigation
Updated scope: 2026-10-26

This bundle contains the classical source catalogue and reference checks.
The main line is APS's labelled-base, one-relation construction, followed by
McGuire–Mueller's searches and Kudo–Yokota–Takahashi–Yasuda's acceleration.
JMC 2020 supplies conditional complexity analysis. Lanteri documents independent
implementations. MultiSolve and its oracle-based successor supply candidate
polynomial-solving technology. Matrix/minor attacks are recorded as adjacent
classical methods; they do not meet the same linear-algebra-avoidance objective.

SOURCE ACCESS AND CONTRIBUTIONS

APS: published in Finite Fields and Their Applications 51 (2018), 168–182,
doi:10.1016/j.ffa.2018.01.009; an accepted manuscript is openly available.

McGuire–Mueller: ePrint 2017/1262; complete public Magma definitions inspected
and executed with the included driver. All three modes passed for m=3 and
m=4 on a fixed order-991 curve. Direct retrieval of the paper PDF failed.

Kudo et al.: CANS 2018, 377–393, doi:10.1007/978-3-030-00434-7_19.
Publisher and institution records confirm hybrid search and symmetry methods.
The SCIS 2018 precursor abstract is public. The complete CANS chapter remains
unavailable in this investigation. WCC 2017 predecessor data concern PKM.

Yokoyama et al.: JMC 14 (2020), 460–485, doi:10.1515/jmc-2019-0029.
Full published text inspected. The lower bounds have explicit assumptions
and concern a specified naive index-calculus ideal; they are not an
unconditional barrier covering every APS solver or coordinate-based attack.

Lanteri: Genoa master's thesis, 2023-09-27. Public abstracts document C++/CoCoA
APS and PKM implementations; the thesis is restricted. A public 2025 F4/FGLM
tutorial coauthored by Lanteri links educational Sage F4 source.

MultiSolve: FFA 98 (2024), 102452, doi:10.1016/j.ffa.2024.102452. Adaptive
hybrid polynomial solving, with experiments on Trivium and an at-most-one-
solution framework. Oracle-based successor: AMC 23 (2026), 255–273,
doi:10.3934/amc.2026024; applications concern Aradi. A September 2026
application concerns syndrome decoding. No demonstrated APS improvement
over rho was found in these sources.

REPRODUCING THE CHECKS

Python 3.8+, standard library:
    python verify_aps.py

Python with SymPy 1.14.0:
    python compare_relation_solvers.py --samples 4 --timeout 3
    python compare_relation_solvers.py --stratified --timeout 3

Optional public Magma calculator run, requiring network:
    python run_magma_check.py

The runner downloads the public author source from
https://math.ie/ecdlp/sourcecode.magma, keeps the definitions unchanged, and
appends magma_check_harness.magma in place of the example driver. It submits
that mathematical code to the public endpoint linked at
https://magma-maths.org/calc/. The saved response includes source SHA256,
Magma version, stdout, and scope. It reported Magma 2.30-1.

The Magma correctness fixture is E/F_1021: y^2=x^3+x+6, #E=991,
P=(2,4), Q=7P. The m=4 Groebner path required 80 fresh bases. The three
author modes search different spaces, so their trial counts do not provide
a controlled performance comparison.

The independent Python comparison holds the labelled, partitioned bases
fixed across five methods: S3 evaluation, signed point sums with hashing,
univariate GCDs, unsplit Groebner solving, and hybrid Groebner solving.
All completed solution sets agree, and usable relations recover the DLP.
Singular, composite-order, and anomalous curves are excluded.

The stratified run deliberately selects one successful and one failed base
per prime. The separate unfiltered run uses four bases per prime. Timings
include method-specific polynomial construction and root extraction, but
exclude base generation and process startup. Root extraction uses the known
finite coordinate grid. Groebner computations use SymPy F5B and have a
three-second per-base cap. These are tiny reference experiments, not
reproductions of Magma F4 timing tables or end-to-end comparisons with rho.

Third-party papers and author source code are not redistributed in this
bundle. sources.json gives their primary locations and access limitations.
