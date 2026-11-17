#set page(paper: "a4", margin: 17mm)
#set text(size: 10pt, font: "DejaVu Sans")
#set par(justify: false, leading: 0.65em, spacing: 0.8em)
#set heading(numbering: none)
= Concrete cubic norm descent
#text("The original cubic coefficient model cannot square to (12,0,0) or (24,0,0). Both unchanged original Lean gates pass15 audited declarations/0 trusted on the identical final source, Lean4.35 and4.34.1. Twelve original PCC statement bindings and all complete source/log/failure evidence are preserved in data26ec43. All claims remain unsigned. [pcc:unverified]")

== A coefficient proof of the retained obstruction
#text("Use the basis 1,z,z² with z³=d. Multiplication by a+bz+cz² has the following matrix. Its determinant is a³+d b³+d² c³−3dabc. The determinant expansion and square identity are Lean theorems over any commutative coefficient ring. [pcc:unverified]")
$ M = mat(a, d c, d b; b, a, d c; c, b, a) $
$ N_d(a,b,c) = a^3 + d b^3 + d^2 c^3 - 3 d a b c $
#text("The square coefficients are s0=a*a+d*(b*c+c*b), s1=a*b+b*a+d*c*c, s2=a*c+b*b+c*a, in exactly the original order. Polynomial normalization proves N_d(s0,s1,s2)=N_d(a,b,c)². If the square is (D,0,0), this gives N_d(a,b,c)²=D³. [pcc:unverified]")
#text("With the explicit unit hypothesis invD*D=1, the base-ring element N_d(a,b,c)*invD squares to D. Consequently a base-ring nonsquare unit cannot acquire a square root in this cubic coefficient algebra. This argument needs no unspoken field, primality or irreducibility assumption. It constructs no quotient field or Jacobian. Bindings25bee3,559c7e,739945,9bfaad. [pcc:unverified]")

== Exact constants and the original arithmetic model
#text("Kernel reduction in canonical Fin223 checks the223 base residues: no square is12 or24. It also checks93*12=1 and158*24=1. Specializing d=3 therefore excludes both constants for every coefficient triple. The proof does not enumerate223 cubed elements. The positive control (2,0,0)²=(4,0,0) preserves an allowed square case. Bindings112101,f4c1d2,3d5c0c,f3fe26,0abe9b. [pcc:unverified]")
#text("A separate Lean definition spells out the five original natural-number temporaries and every intermediate/final modular reduction in Kf.mul's square. The coefficient-equality theorem links that exact arithmetic model to s0,s1,s2; the final two statements exclude(12,0,0) and(24,0,0) directly. The entire original common.rs is byte-pinned in the evidence. Bindings8c325d,6ade5a,2f1fe7. [pcc:unverified]")

#pagebreak()
= Scope, provenance and next work
#text("The previous fixed second/third elliptic-hop native records reported D12 and D24 and unsupported specified-product identifications. Their derivations used a written tower-law argument; the older NonsquareResidues module covered only base residues. This component adds a concrete coefficient implication and original arithmetic model. It neither reruns nor changes either old experiment. [pcc:unverified]")

#table(columns: (34%,66%), inset: 6pt,
 [Requirement], [Evidence or remaining boundary],
 [Polynomial algebra], [Generic determinant, norm-square and explicit unit descent, with exact Lean statements.],
 [Both fixed constants], [Universal Fin223 coefficient exclusions plus the allowed square4 control.],
 [Original arithmetic model], [Five temporaries, all reductions and coefficient order; complete original source retained.],
 [Original proof tools], [Both gates15 audited/0 trusted; only standard propext, Classical.choice and Quot.sound.],
 [Native and geometric objects], [Word-level binary execution, field/Jacobian construction and global polarized identification remain separate.],
 [Other models and costs], [No general Jacobian/twist impossibility or computational advantage follows.],
)
#text("Every row is unsigned component evidence. The source/proof-map data are in research/chained_jacobian_transfers_20261110/evidence/cubic-norm-descent-final-04. Named statements12, audited declarations15 including generated helpers. Source SHA256 begins18384e633660a419. The original common.rs SHA256 begins832fd28810909194. [pcc:unverified]")

== Retained attempts and reproduction
#text("The first abstract-ring zero-simplification compile failed; the first Consumer invocation targeted the wrong configured repository root; two finite-coordinate normalization attempts failed. Complete sources, diagnostics and exits are retained. An intermediate eight-statement module and the final extended arithmetic model have separate complete audit records. No failing module with sorryAx was accepted. [pcc:unverified]")
#text("Run the unchanged Root gate with Lean4.35 and the unchanged Consumer gate with Lean4.34.1 on byte-identical source copies inside each gate's actual configured repository. Use one compiler worker,120s per gate and the original static proof memory defaults. The source-pinned CI workflow is implemented; its own published-source result remains prospective. No new field-engine, original360, LP60 or other scientific native execution occurred. [pcc:unverified]")
#text("Sources: original Kf.mul; both fixed nondual13 negative derivations; earlier NonsquareResidues; Howe–Leprévost–Poonen Proposition15 and following remark, already retained in publication17. KB search127 and the locked vault do not imply novelty. Next: publish/replay this proof component, preserve complete scientific custody, and continue global Jacobian/program bridges and the frozen full LP population when actual budget/reservation/capacity admission permits. [pcc:unverified]")
