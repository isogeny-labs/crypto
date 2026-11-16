# Outer norm and pullback formula for the fixed chained transfers, version2

The frozen original CHAIN_COMPRESSION_PROOF.md/PCC71048c is preserved.
This version adds an inline standing marker required by prose lint;the
mathematical argument and hypotheses are unchanged. [pcc:unverified]

Scientific standing unsigned. This is an elementary composition consequence
of the existing quotient/norm identities, not a new isogeny or novelty
claim. It supplies the direct whole-Jacobian comparison route for the
accepted chain. Actual Picard, quotient and polarization constructions
remain written dependencies. [pcc:unverified]

Let A_i=Jac(T_i) and B_i=E_(i,109)×E_(i,113). The old product pullbacks
P_i and norms N_i satisfy N_i P_i=P_i N_i=[2]; the normalized Richelot
maps satisfy R_iHat R_i=R_i R_iHat=[2]. Put

    K_i=R_i P_i : B_i→A_i,
    H_i=N_i R_iHat : A_i→B_i.

These give both H_i K_i=[4] and K_i H_i=[4]. For the selected elliptic
product isogeny phi_i:B_(i-1)→B_i, the accepted global Jacobian transfer
satisfies

    [4]Psi_i = K_i phi_i H_(i-1).

For the first two hops, phi_1=alpha and phi_2=beta on their actual
normalized short models. All identities are homomorphism identities over
F101, not deductions from a finite list of outputs. [pcc:unverified]

## The middle Jacobian stages cancel algebraically

Put F_n=Psi_n ... Psi_1 and Phi_n=phi_n ... phi_1. We claim

    [4]F_n = K_n Phi_n H_0.

For n=1 this is the old factorization. Assuming the identity at n-1,
linearity and the middle relation give

    [16]F_n
      = K_n phi_n H_(n-1) [4]F_(n-1)
      = K_n phi_n H_(n-1) K_(n-1) Phi_(n-1) H_0
      = [4]K_n Phi_n H_0.

Cancel multiplication by4 in Hom(A_0,A_n). That group is torsion-free:
if[4]u=0, the connected image of u is contained in the finite kernel of[4]
and is zero. This is cancellation of homomorphisms, not a claim about
torsion in geometric point groups. The induction proves the displayed
formula. [pcc:unverified] In particular

    [4](Psi_2 Psi_1) = K_2 beta alpha H_0.

The composite remains the old degree2401 isogeny with adjoint norm49.
The formula evaluates that existing map through different stages; it does
not lower its geometric degree. [pcc:unverified]

ChainCompression.two_hop_double_cleared/PCC1b3e8b proves the16-versus4
identity over explicit modules and map-linearity hypotheses without any
injectivity assumption on point multiplication. The second declaration,
rational_two_hop_factorization/PCC167fc6, cancels only under an explicit
injectivity hypothesis for multiplication by4 on the destination. Both
are locally kernel checked with zero trusted native proofs; geometry and
global Hom cancellation above remain written. Old136 declarations are
unchanged;the new two bring the written census to138 across26 modules.
Scientific signing remains separate. [pcc:unverified]

## The implemented rational-point route

All fixed rational Jacobian groups have order12317=109*113. The selected
elliptic maps preserve their factors of orders109/113, and inverse4 is82
modulo109 and85 modulo113. Therefore the whole two-hop point algorithm is

1. Evaluate the two direct T0 norms H0,j(D).
2. Apply the corresponding public quarter scalar82/85 once.
3. Evaluate alpha_j, then beta_j, on those elliptic points.
4. Evaluate the two direct T2 pullbacks and add the balanced classes.

Multiplying the result by4 gives K2 beta alpha H0(D), and multiplication
by4 is injective on A2(F101), so this is the accepted composite. No
intermediate T1 class or T1 norm is needed in this expression. The same
argument uses only outer norms and pullbacks for any number of admitted
hops. Parameter discovery and fixed-map construction remain inherited
inputs that must be charged in a cold comparison. [pcc:unverified]

For original embedded elliptic inputs, the same global identities imply

    F_n K_0 = K_n Phi_n.

Indeed multiply by4, substitute H0K0=[4], commute the scalar and cancel4
in Hom. Thus for this source the direct route is the actual elliptic
composition followed by one final pullback. For two hops the full reverse
source norm remains[196], recovered with104 modulo109 or64 modulo113.
These are public integer multipliers, not labels for discrete logarithms.
[pcc:unverified]

## Native acceptance and comparison scope

Pipeline32 is a separate pending native test, not accepted by this written
argument. Its second-hop case must compare every12317 output against the
retained old second hop and check group/model/selected-beta controls. Its
compressed case must compare every12317 output against both oldsecond
after the accepted purefirst72e1d8 and separately staged puresecond after
purefirst, plus every109/113 original source scalar/direct endpoint/full
reverse196/recovery control. The accepted first-hop global/native result
is an explicit dependency of this reference composition. Wrong-quarter
mutations and all identity/negation/paired-sum/bijection checks remain.
[pcc:unverified]

The compressed route belongs in any future staged-chain cost comparison,
along with the direct elliptic-source route. Actual whole costs must include
construction, transfer, solver, recovery and verification on matched inputs
and references. No timing run or measured computational advantage is
inferred from this algebraic stage reduction. Current S3/livebudget admission
is unavailable;no solver or benchmark has been launched. [pcc:unverified]

Sources checked: PRIME_FIELD_PSI_PROOF.md/78176c,
PRIME_FIELD_ADJOINT_PROOF.md/526f9f, QUARTIC_PULLBACK_PROOF.md/4934b4,
T2_QUARTIC_PROOF.md/f9cf19 and SUCCESSOR_CHAIN_PROOF_V2.md for the actual
maps and norm identities; Milne AV isogeny/Hom facts and the Galbraith
Chapter10 Picard/norm sources already used there. Existing
TwoHopTransport/ChainTransport formal sources do not include this outer
norm/pullback composition statement. KB/ecdb unavailable;no exhaustive
prior-art or novelty assertion. [pcc:unverified]
