#set page(paper: "a4", margin: 17mm)
#set text(size: 10pt, font: "DejaVu Sans")
#set par(justify: false, leading: 0.6em, spacing: 0.7em)
#set heading(numbering: none)
= Cubic trace and faithful cyclic descent
#text("The additive trace has an explicit fixed-part projection and constraints on faithful cyclic descent. Both unchanged original Lean4.35/4.34.1 gates pass17 named audited/0 failing/0 trusted, with seventeen complete original-extractor bindings1421da. Claims remain unsigned. This additive module trace is distinct from the multiplicative coefficient norm04. [pcc:unverified]")

== Stated module and normalization
#text("Let N(x)=x+F(x)+F²(x). Each statement exposes its needed IntModule/additivity/scalar-linearity/cubic-action hypotheses. For normalization, r annihilates the stated module or point and3u+rv=1. This never assumes scalar injectivity on an ambient geometric point group. [pcc:unverified]")
#table(columns: (34%,66%), inset: 6pt,
 [Trace transport], [N(x+y)=N(x)+N(y); N(nx)=nN(x)],
 [Fixed image], [F(Nx)=Nx and N(Fx)=Nx],
 [Fixed input], [F(x)=x implies N(x)=3x],
 [Normalized projection], [pi(x)=uN(x); pi²=pi; pi(x)=x iff F(x)=x],
 [Complement], [x=pi(x)+(x−pi(x)); N(x−pi(x))=0],
)
#image("fixed-and-kernel.svg", width: 100%)

== A faithful stable cyclic subgroup is fixed
#text("Assume F(P)=lambda P. Faithfulness means aN(P)=bN(P) implies aP=bP. Scalar transport and N(FP)=N(P) give lambda N(P)=N(P). Faithfulness applied to lambda and1 yields lambda P=P, hence F(P)=P. This is a necessary condition with explicit stability/faithfulness assumptions. [pcc:unverified]")
#text("For a fixed r-annihilated generator, normalized_fixed_input supplies the explicit left inverse uN(P)=P under3u+rv=1. These abstract identities construct no new lower-field curve/Jacobian map or faster DLP algorithm. [pcc:unverified]")

#pagebreak()
= Eigenline annihilation, controls and limits
== A covariant map to a fixed target
#text("Let f preserve integer scalars and satisfy f(Fx)=f(x). For F(P)=lambda P and rP=0, scalar transport gives r f(P)=0 and (lambda−1)f(P)=0. A supplied Bezout certificate u(lambda−1)+vr=1 forces f(P)=0. Trace is a special case. Mere lambda!=1 is not substituted for invertibility at an arbitrary composite modulus. [pcc:unverified]")
#text("Composition covariance is proved for two composable maps and can be reused along a chain. Actual geometric maps/linearity/covariance and the fixed-target identification remain inputs. Maps lacking those hypotheses are outside this obstruction; no general impossibility of prime-field descent or ECDLP transfers is asserted. [pcc:unverified]")

#table(columns: (36%,64%), inset: 6pt,
 [Abstract control], [Kernel-checked scope],
 [Fin109, scalar45], [F³=id and N=0 on every coefficient. This is not an actual curve Frobenius instance.],
 [Fin109, fixed action], [73(x+x+x)=x, the inverse3 normalization.],
 [Fin3 exception], [3=0 and no inverse3 exists.],
)
#text("All seventeen bindings are complete under the unchanged original extractor. The actual audits count17 named declarations and no generated equations; only propext,Classical.choice,Quot.sound occur. Source SHA256: 46e4993775a2c0c64f1aa079b28fb1d13b6e671df035a182f038a16f2e07ef43. Each original gate uses one worker/120s/original memory defaults, no native decision or curve/LP population. [pcc:unverified]")

== Rejected attempts and remaining bridges
#text("First07960c fails redundant rewrite/eigenline automation; second82b8d6 fails abstract scalar distribution. Both complete sources/outputs print rejected sorryAx. Explicit original module laws repair derivations only; statements/hypotheses/gates stay unchanged. Full04e804 and all original modules/verifiers/keyring/policy remain immutable. [pcc:unverified]")
#text("Sources: exact rational-bridge/Hom-sign/matching/odd-cancellation/full04 modules, actual Lean module laws. Preregistration091010 records competing hypotheses/controls; KB wrappers127, conductor unavailable, locked atlas unread, no novelty/absence inference. Canonical chain/cubic/F101 graphs unchanged: no geometric edge or cost added. [pcc:unverified]")
#text("Actual field/native/compiler/counter/Jacobian maps, original full LP60/240/62/original360/witness/seven controls/feedback/post, runtime/admission and matched solving costs remain open. Current CI store403/startup failures115681 separately block merge. No native scientific run, benchmark, paid launch or signed standing. [pcc:unverified]")
