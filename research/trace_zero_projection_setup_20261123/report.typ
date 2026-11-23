#set page(paper: "a4", margin: 15mm)
#set text(font: "DejaVu Sans", size: 9pt)
#set heading(numbering: none)
#set par(leading: 0.55em)
= Faster complete trace-zero S3 projector setup
TZA-20261123 · ECC2K-130 algebraic index-calculus relation finding

The fixed degree131 uniform query uses *13.82% fewer complete-oracle simulated instructions* and *6.18% fewer complete-cold-query instructions* than the current merged solver. All90 recorded results match. These are unsigned producer observations, PCC-20261123-435311. [pcc:unverified]

#image("instructions.svg", width: 100%)

#table(columns: (1.35fr, 1.25fr, 1.25fr, 1fr, 1fr), inset: 4pt, [Fixed d12 query], [Baseline cold Ir], [Adjoint cold Ir], [Cold removed], [Oracle removed],
[83/planted], [29,542,434], [28,519,674], [3.46%], [9.79%],
[83/uniform], [34,457,766], [33,333,976], [3.26%], [7.24%],
[83/cancellation], [24,752,165], [24,698,227], [0.22%], [0.95%],
[131/planted], [39,031,522], [36,264,943], [7.09%], [21.41%],
[131/uniform], [46,922,548], [44,024,513], [6.18%], [13.82%],
[131/cancellation], [32,452,734], [32,399,425], [0.16%], [0.84%],)
Five repeats per arm/cell; plot whiskers are observed min/max, not population intervals. All primary cell ranges are degenerate in this fixed panel. Cancellation does not construct a projector: its small changes are compiled-path effects, not projector-work measurements. [pcc:unverified]

#pagebreak()
= Exact same equations, cheaper construction
The existing oracle eliminates the product variable through Artin–Schreier and constructs all quotient constraints modulo the product span W. At d12 it still solves13 coefficient bits; this change does not reduce system size or establish additional Z[tau] leverage.

Each coefficient mask c for a linear functional r and multiplier h must satisfy dot(c,z)=dot(r,hz). The previous constructor generates every hX^k, reduces each modulo W and transposes all coordinate values. The replacement computes the same mask through an adjoint map and one ordinary field multiplication.

For f=X^n+sum X^t, let R reverse exactly n coefficient bits, S(a)=sum over positive t of a>>(n-t), and N=(I+S)R. Direct coefficient comparison gives N M(X)^T=M(X) N. Since M(h)=h(M(X)), the required mask is N^(-1)(h N(r)). Because S strictly lowers degree, N^(-1)=R(I+S+S^2+...). The131-bit modulus needs one nonzero inverse fold; the83-bit modulus needs at most two. This is established transposed multiplication, with no novelty claim.

Dispatch checks the complete polynomial, including its degree and every tail term. The only enabled moduli are X^131+X^13+X^2+X+1 and X^83+X^45+X^2+X+1. Other fields retain original construction. Full-W membership, roots, supplied signs, model order, all quotient rows, deficient-system fallback and group replay are retained.

= Controls, custody and validation
The panel has6 fixed cases ×3 interleaved arms ×5 repeats:90 planned,90 attempted,90 completed,0 excluded,0 retries. A/A cold discrepancies and the no-regression criterion pass in all6 cells. The n131 uniform thresholds were fixed at3% oracle and1% cold reduction before comparative data. [pcc:unverified]

Twenty-four focused development tests include every bilinear basis triple on both exact moduli, forward/inverse basis identities, full projector coefficient equality over five dimensions and all65 retained query outputs. Separate schoolbook polynomial/group arithmetic checks all65 fixture admissions and negative cases and replays all60 saved primary point tuples. These checks are producer-written, not an independently signed verdict. [pcc:unverified]

Whole cold Ir includes parsing the same single-query input, curve/context setup, full relation search, all projector preparation, roots/signs, internal and wrapper replay, serialization, destruction and exit. The oracle partition includes projector construction. Field-operation tallies are secondary: generator and general multiplication share an informational event name but have different actual costs, priced by total Ir.

The two fixed fields are ECC2K-130 K0-131 and an E0 degree83 control; the latter is not the standard K1-83 curve. This panel supports no population, native-runtime, fastest-external or complete-DLP conclusion. Prior-host ratios are not compounded.

#pagebreak()
= Reproduction and remaining gate
Frozen numeric source:68a0dece3c745cea3980bf0a2d383abf0f8d24fb. Freeze publication:f86b451e9ed60339021931943b8cdb02564923d7, before primary data. Protocol hash:d19325b556ce274bea13085fb969f53220551819431f7dc87a3f9a50b48fb7fd.

Compiler:Rust1.100.0 (0ef1bc386,2026-11-09). Valgrind3.24.0. Intel Xeon Platinum8573C, one pinned CPU0,2GiB child address-space cap. Combined calibration/pilot/freeze/primary completed inside600seconds; zero retries and zero paid resources. No isolated native clock is admitted. [pcc:unverified]

The focused facade compiles actual current numeric/field/curve/kernel modules. It additionally imports frozen SAT/chain support for unused interfaces; those solvers are not executed. The full root release-test gate is reported separately in checks/root-tests.txt and checks/root-tests.exit. Its terminal result is stated in README.md; focused tests do not replace it.

Evidence: summary.json, grid-audit.json, polynomial-replay.json, export-final.json, RAW-CALLGRIND.tar.xz, checks/source-inputs.json, native/Cargo.lock. Initial canonical-export adapter scaffolds and their errors are preserved; export-final.json supplies explicit instance identities and observed instrument/frozen analysis metadata and passes strict file checks. No scientific row or analysis changed. [pcc:unverified]

= Sources checked
KB unavailable:repository wrapper returned127 and no KB MCP tool is loaded. Repository sources include trace_zero_linearized.rs, trace_zero_word_field.rs and the retained rowmap study. Primary source:De Feo, Doliskani and Schost, Fast arithmetic for the algebraic closure of finite fields, ISSAC2014,section2 on dual maps and transposition: #link("https://cs.uwaterloo.ca/~eschost/publications/embed.pdf")[author PDF]. Current canonical protocol files were VLT1 encrypted; the last plaintext maintained tool0c76edb5 was recovered exactly from Git, with its hash recorded.

= Next focused optimization
Test reuse of the existing compact WordField half-trace/square maps while building the h(i) columns. The present constructor still recomputes half-traces with general elements even after that compact field workspace exists. This is a proposed independent follow-up, not part of this measured change.

Existing admitted runtime, benchmark-table and leaderboard values remain historical. These local instruction counts have no signed independent admission. Report copies do not promote a leaderboard row.
