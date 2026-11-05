from pathlib import Path
import json
D=Path(__file__).parent;v=json.loads((D/'result.json').read_text());assert v['cases']==12 and v['all_pass'];report='''# Label information retained across geometric deduplication

Derived a precise duplicate-label equation and its source insertion point. If two valid signed divisors normalize to the same canonical divisor, normalize both label pairs by the same signs. Their differences satisfy Delta_a+Delta_b*k=0. A nonzero Delta_b gives a candidate k, checked by kG=Q; zero Delta_b gives no new equation when labels are valid.

The existing collector suppresses canonical signed duplicates with a HashSet before graph insertion. The proposed next change stores the first normalized label pair and evaluates the coefficient difference before suppressing the duplicate edge. This preserves graph deduplication while retaining potential target information. It is a proposal, not an implemented collector result.

The lemma assumes exact prime order r for G, Q=kG, valid labels and a common target identity. Its proof does not require individual graph nodes to lie in that subgroup. By contrast, the graph-rank formulation A*x=a+b*k assumes a consistent F_r node-coordinate model; that additional geometric correspondence remains explicit.

Twelve frozen deterministic algebra controls passed with exit0: four valid signed-duplicate cases, four invalid-assumption rejection controls, and four graph-rank cases over F11. Two informative duplicate cases return3; two redundant duplicates produce the zero equation. A balanced triangle with target-bearing dependence returns3; a balanced redundant triangle does not. The forest and the unbalanced triangle have zero left nullspace. These are fixed algebra checks, not Jacobian trajectory measurements or a formal proof checker.

This distinguishes geometric topology from target-rank information. The retained114-edge forest still has no distinct-edge cycle. Its excluded duplicates were not proved incapable of constraints; the lemma identifies exactly which coefficient difference matters. No cross-target pooling or reuse of planted target values is admitted as naturally discovered information.

A future deterministic geometric fixture can relabel a retained recovered D using (a+k,b-1), explicitly constructing the alternate labels from known k. That would test the duplicate-handler interface, not demonstrate a naturally produced recurrence collision. No such geometric fixture or new trajectory was run here.

Symbolic extra work: at most two scalar negations per accepted signed divisor; two scalar subtractions per duplicate; for an informative duplicate, one inversion, multiplication and negation plus kG=Q verification. Store two u64 labels per key, excluding container overhead. Canonical word bounds and all verification costs remain explicit. No speed estimate is reported.

Full equations, elementary proofs, source placement, assumptions and future-fixture scope are in LEMMA.md. Primary precedent is the Handbook of Applied Cryptography, chapter3 section3.6.3/Algorithm3.60 (https://cacr.uwaterloo.ca/hac/about/chap3.pdf); signed recovery follows Chung–Costello–Smith. The extraction mechanism is established labelled-collision algebra, not a novelty claim. Independent review pending.
''';(D/'REPORT.md').write_text(report)
svg='''<svg xmlns="http://www.w3.org/2000/svg" width="850" height="190"><rect width="850" height="190" fill="white"/><g font-family="sans-serif" fill="#17334c"><text x="20" y="32" font-size="21">Same geometric divisor does not imply same label equation</text><text x="20" y="74" font-size="19">Normalize signs → subtract labels → Delta_a + Delta_b k = 0</text><text x="20" y="113">Delta_b ≠ 0: verify candidate k    |    Delta_b = 0: require Delta_a = 0</text><text x="20" y="149">Graph edge remains deduplicated; first label pair remains available</text><text x="20" y="178">12 fixed algebra controls passed · no new geometric cycle or timing</text></g></svg>''';(D/'diagram.svg').write_text(svg)
t='#set page(paper:"a4",margin:18mm)\n#set text(size:10pt)\n#text(size:17pt,weight:"bold")[Signed duplicates and target rank]\n#image("diagram.svg",width:100%)\n'
for p in report.split('\n\n')[1:]:t+='\n#text('+json.dumps(p,ensure_ascii=False)+')\n'
(D/'report.typ').write_text(t)
(D/'status.json').write_text(json.dumps({'schema':'research.algebraic-construction/v1','lemma':'signed_duplicate_coefficient_difference','proof':'elementary derivation in LEMMA.md; not kernel-checked','fixed_algebra_cases':12,'all_pass':True,'source_change':'proposed only','geometric_trajectory_executed':False,'target_rank_distinct_from_graph_cycle':True,'independent_review':'pending','scientific_execution':False},indent=2)+'\n')
