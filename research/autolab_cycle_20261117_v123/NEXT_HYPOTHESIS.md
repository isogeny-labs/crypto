# Prepared C124: normalize XOR across nonlinear values with bounded cuts

Status: derived prototype, AST/global-name preflight ONLY; not scientifically executed. Known XORcircuit synthesis is prior art; no novelty or speedclaim.

Assign every primary input a formal XORatom,with bit0forconstant1. XOR/NOTcombine masks even when atoms denote nonlinear functions. For each AND,emit its exact two input masks and assign the result a new atom; structural reuse and equal/complement identities are exact. Atoms need not be probabilistically/algebraically independent: XORidentities hold for their actual correlated Boolean values. Induction on source nodes proves semantic equivalence for every primary assignment.

To bound expansion,if a combined mask exceeds16terms,emit the original exact XORof its two rewritten inputs and treat that value as a fresh atom. This is a semantic cut,not an approximation. Cap total materializedsupport at8million and keep the existing500000gatecap/4GiB/onecore90sworker limit. Cap failure is inconclusive,not a false/empty result. Outputconstraints and allfive recoveredrootwires are mapped and pruned together; source/control objects are never mutated.

This differs from earlier primary-affine-form synthesis: cancellations are now tracked OVERANDOUTPUTATOMS,such as a XOR(a XORb)=bwhen aandbare nonlinear. It may also increase gatecount by flattening useful sharing; that possibility is part of the falsifiable hypothesis.

Next independent oracle: exhaust random small AND/XOR/NOTcircuits at3..6inputs,with deep nonlinear XORreconvergence,constants/complements,cut-threshold crossings,all outputpredicates and recovery rows. Compare every outputvalue,not just SATmembership. Then rewrite the frozen C123exactverifier graph,compare actual controls/complete root recovery,source hashes and totalnodes/clauses/setup/RSS at declared termbounds. Freeze no hard retry until this changed representation is assessed. Originalallroots/broadfield/genuineF4/F5/F6releasegate remain required.
