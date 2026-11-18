# Moving-divisor Smith transport [pcc:unverified]

This implements the known Smith correspondence on the fixed original F61 public inputs, using the already constructed canonical models. It is not a novelty or speed claim. Prior model/trace/connectedness derivations and native Pic arithmetic remain independent-review dependencies.

## A quadratic for the images of one source point

Over a separable generic trigonal fibre let x1,x2,x3 be the roots of G(t,x), and put lambda_i=y_i/G'(x_i). For the delta1 orientation, the four quotient coordinates rho are sums of lambdas with fixed product kappa=delta1/(8delta4). Set S=sum lambda_i²=-delta2/(2delta4). For a given source point (x,y) at x=x1, lambda=y/G'(x) fixes one sign. Its two images therefore have

    (rho-lambda)²=(lambda2+lambda3)²
                 =S-lambda²+2kappa/lambda,

or

    H(rho)=rho²-2lambda*rho+2lambda²-S-2kappa/lambda=0.

The complementary sign-triple correspondence has exactly these two images on the open where the denominators and discriminants are units. This selects source y, rather than merely producing arbitrary roots of the scalar quartic. Flipping y exchanges the appropriate images, as tested through negation on classes.

For a degree3 source Mumford divisor (u,v), work over L=F61[x]/u, with y=v and t=N(x)/D(x). The image algebra is the degree6 etale F61 algebra L[rho]/H. Squarefreeness of u, unit denominators and discriminant(H), plus the scalar-quartic discriminant check, exclude the exceptional chart points. Canonical sections give three homogeneous coordinate elements in this algebra; the code checks the entire canonical quartic equation there. A coordinate-unit check excludes a common homogeneous base point.

Evaluate the ten homogeneous cubics in these coordinates. A rank6 evaluation map gives the4dimensional space H0(3K-E) of cubics vanishing on the effective degree6 image divisor E. Rank6 also rejects collisions that would make these six algebra points fail to define a degree6 embedded divisor. Since deg(3K-E)=6>=2g for g=3, this complete linear system is base-point free and generates E's ideal on the curve. No splitting of u or extraction of extension-field point lists is required.

## Cancelling the source base divisor

An original input class X need not have safe support and its Mumford reference includes infinity. Choose a public auxiliary A=[j]D_aux and form B=X+A. If A and B have degree3 effective representatives in the safe chart, then X=B-A in the source Jacobian. Map those representatives to E_A and E_B. The Pic module's degree6 reducer returns [E-2D_inf], so

    phi(X) = [E_B-2D_inf]-[E_A-2D_inf] = [E_B-E_A].

Thus neither phi(infinity) nor a guessed quartic base-point correction is used. All three source infinities and target reference terms cancel before mapping. The bounded search checks X=B-A natively at every attempt, uses only public class inputs and the public auxiliary base, and has512attempts. Exhaustion is a retained failure, not a universal moving-lemma guarantee. Separate starting points1and37 test that the result is independent of the chosen auxiliary representative.

The expected target scalar is absent from transport.rs. It appears only in the driver that recreates the original fixed public fixtures and verifies the transported equality. This is a map qualification, not a quartic DLP solve.

## Class arithmetic and verification

The copied plane_quartic_pic64.rs module represents Pic0 canonically and handles degree6 generating spaces via ideal quotients and finite-field linear algebra. Its constructor supplies a separate homogeneous Jacobian smoothness check. The zero reference D_inf=K-P0 is internal to the quotient arithmetic and cancels as above. Exact class-key equality, nonzero source image, multiplication by the original odd prime ell and additive/negation/invalid-source controls qualify the implementation.

Source: Smith https://arxiv.org/abs/0806.2995 section6 equation11 and section7; the point-image quadratic above is a deduction from its interpolation/sign-triple description. The repository Pic module describes its representation and arithmetic; source provenance is in PIC-PROVENANCE.txt. No full end-to-end cost result or universal total transport algorithm is established by these bounded tests.
