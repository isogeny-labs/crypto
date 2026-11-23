# Follow-up integration review

Status: independent written checking by the coordinator; **[pcc:unverified]**. This record accompanies the source-bound review of PR #4979 and the leaf-to-pair norm note. It is not executable replay, a signed verifier verdict, or a novelty claim.

## Cancellation-reduced support count

I checked the count independently of the scheduled producer's multiset argument. Four signed leaves reduce to an integer coefficient vector of l1 norm 0, 2, or 4. The norm-two count is 2B + 4 binom(B,2) = 2B^2. The norm-four partitions 4, 3+1, 2+2, 2+1+1, and 1+1+1+1 give respectively 2B, 4B(B-1), 4 binom(B,2), 8B binom(B-1,2), and 16 binom(B,4). Their sum is (2B^4+4B^2)/3. Quotienting nonzero vectors by global negation gives the cap (B^4+5B^2)/3. Further group relations or zero images only decrease support. Repeated leaves and cancelling pairs remain included. [pcc:unverified]

For B=31 the cap is 309442. The inherited order n=2147446831 gives (n-1)/2=1073723415 nonidentity abscissae; at least 1073413973 are outside that support. These are conditional combinatorial bounds, not an actual support census or a new order certificate. The count does not assert useful relation rank or change the leading B^4 exponent. [pcc:unverified]

## Leaf norm and the finite-field counterexample

I checked the diagonal/off-diagonal factorization R_M J_M = kappa C_M^2, where kappa=(-4)^B Res(M,f) Disc(M)^2. Each off-diagonal unordered chart occurs twice in R_M, while each diagonal occurs once; the extra monic doubling factor J_M supplies the missing diagonal multiplicity. Distinct nonzero abscissae in an odd-order subgroup make kappa nonzero. The radical must account for characteristic-p multiplicities, as the note expressly does. [pcc:unverified]

I also checked the displayed F7 example by substitution: y^2=x^3+3 has twelve affine points and O; P=(3,3) and Q=(4,2) lie on the regular D2=0 fibre. Their chart abscissae are 2,3,1,4. The product of W-1 through W-4 is W^4+4W^3+6W+3 in F7. D2 takes values 6,2,0,0, giving a dimension-three subalgebra and unequal fibre sizes 1,1,2. This excludes only automatic uniform normalized D2 folding of this actual pair spectrum. It is a written check, not a native test or a counterexample for all alternative coordinates. [pcc:unverified]

## Complexity and source scope

The requested component objective and the stronger complete-pipeline target must stay distinct. An exact O-tilde(D^theta + h) query with fixed theta<1, comparable charged setup, and complete output/replay costs would improve the present O-tilde(D) component baseline. The condition theta<1/2 is stronger: it is the conditional complete-pipeline exponent target when D=Theta(B^2), setup is O-tilde(D), matrix work is B^2, and the stated uniform-target/full-row model applies. Neither new note supplies such an algorithm. [pcc:unverified]

The coordinator opened the author PDF of Bostan-Flajolet-Salvy-Schost, Fast Computation of Special Resultants, and Chang's Short character sums for composite moduli. Polynomial-construction complexity is not a sublinear indexed-query bound. Chang Corollary 11 is established prior art used for the existing chosen-field parameter argument; it supplies no elliptic query algorithm. The frozen core and its four mathematical review files remain unchanged. [pcc:unverified]

## Integration decision

Preserve PR #4979's source at head 1a9f3e3c6966258f1a94068d95ec3f3e337b7196 and our core at d6e1d03aff64ff168bd1d1d565a08c8f5032ef62. Move only our mutable iteration10 state to PENCIL_FAMILY_STATE.json to avoid the observed add/add metadata collision. Retain both lanes' ACTIVE and dated-log entries. The nine-page core report explicitly covers its sixteen original claims; the new follow-up claims and actual total belong in the updated root state and continuation.
