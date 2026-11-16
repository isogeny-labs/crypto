# C93: source guards and an exact symmetric pair solver

Three frozen experiments ran on one core with 4 GiB per-process limits: the guard and symmetric-pair oracle experiments passed; the matched timing diagnostic is inconclusive. All earlier sources, inputs, manifests and raw evidence remain unchanged. The scientific goal remains active and unpassed.

The first hypothesis was that shallow source guards would fix the free-source coefficient-span blind spot. The implementation expands the original S3 as a canonical field-valued Boolean polynomial and emits a dual witness only when it proves the entire guard empty. All guards are retained. Independent direct S3 evaluation passed 132 tiny cover cases and 2,292 guard conditions over GF(8) and GF(16), including zeros, repetitions and even extension degree.

The actual GF(2^83)/D18 profile checked 168 guard cases on four exposed separator cells at widths 2 and 4. It rejected 73 guards, but no whole separator cell. The planted source pairs survived. Guards were more effective on the supported examples than on the random cells, where rank often reached 83. Do not interpret rejected guard counts as a global search reduction or an end-to-end gain.

That limited result led to a different representation. Set s=x+y and t=xy. Then

    S3(x,y,u) = t^2 + ut + u^2 s^2 + B.

For source prefix D of dimension d, s lies in D and t lies in the product span E of dimension min(2d−1,n). The displayed equation is linear over F2 in these coefficient bits. Recover every affine coefficient state, then solve X^2+sX=t with X constrained to D. Returning (X,X+s) enforces the missing product support exactly. This retains ordered pairs, repeated sources, zeros, torsion and nonrational point-lift branches. Large coefficient fibers return unresolved, never an exact empty set.

Independent literal enumeration passed 120 cases over GF(4), GF(8), GF(16) and GF(32), with 188 complete source pairs. All nine fixed-separator GF(2^83)/D18 diagnostics completed: four coefficient-planted unsupported cases had zero pairs, two supported cases had two pairs each, u=0 had (1,1), u=1 had (0,1) and (1,0), and u=2 had none. The u=0/1 shortcuts have explicit no-field-wrap polynomial-degree proofs. These are complete conditional pair sets; they are not the original five-source solution set or a hardness certificate.

The matched diagnostic supplied all 36 original source bits and 83 original-coordinate equations to the unchanged genuine repository MatrixF4/MatrixF5 adapter. Algorithms receive only original field/domain/separator inputs; generation coefficients are excluded. The candidate completed the first exposed unsupported case in one cold process, 0.035470234 s and 12,672 KiB observed RSS. Native F4 then exceeded the frozen three-second native cap. The wrapper traceback, neutral input and empty wrapper stdout are preserved. F5, the second case and the remaining repetitions did not run. No speed ratio, median, completed matching panel or qualified gain is claimed.

The failed wrapper did not persist native partial stdout/stderr or failed-child observer metrics before raising. Preserve this limitation: the outer lab process measurements and wrapper files are available, but cannot substitute for missing native diagnostics. The next harness variant must write native streams to exclusive files and save observer records before assertions. This alone is not a reason to rerun an unchanged timeout; use a materially changed preprocessing/configuration or a different diagnostic.

A stronger comparator should extract linear source relations by eliminating quadratic monomial columns from the original equations, then substitute them before a genuine F4/F5 solve. The symmetric method may correspond to such degree-block preprocessing. A timeout of the current configuration establishes no hardness or superiority. Source identity, original-root equivalence, formulation costs and all failed arms must remain visible.

Next: emit independently checkable coefficient-rank and source-support certificates; test degree-block linear extraction and a genuinely different baseline configuration. Separately implement the odd-characteristic quadratic backend, with exact exceptions and cached square classes, to advance prime/odd-extension coverage. The actual 339-variable solve, full matching F6, 20 preregistered hard instances, three cold repetitions, isolation/A/A and requested field coverage remain open. Symmetric variables and binary elimination are established; novelty is not claimed.
