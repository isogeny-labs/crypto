freeze;

/*****************************************************************************
  bielliptic.m -- bielliptic structure of genus-3 hyperelliptic curves.

  Intrinsics:
    IsBielliptic(X::CrvHyp) -> BoolElt
        True iff the genus-3 hyperelliptic curve X admits a bielliptic
        involution (a degree-2 map to a genus-1 curve).

    BiellipticDecomposition(X::CrvHyp) -> SeqEnum
        Returns a list of pairs <C, E> -- C a genus-2 hyperelliptic curve, E an
        elliptic curve, with Jac(X) ~ Jac(C) x E -- one for each bielliptic
        involution of X (equivalently, each genus-2 curve covered by X).

  Method.  Write X : y^2 = F(x) with F = h^2 + 4f (degree 7 or 8).  A bielliptic
  involution of X is the lift of a Moebius involution mu of the x-line that is a
  symmetry of the binary octic F.  With iota the hyperelliptic involution and
  sigma a bielliptic involution, <iota, sigma> = (Z/2)^2 and

        Jac(X) ~ Jac(X/sigma) x Jac(X/(iota sigma)),

  where E := X/sigma has genus 1 and C := X/(iota sigma) has genus 2 (Kani-Rosen).
  Concretely, centring the fixed points of mu to  u -> Cc/u  and setting
  t = u + Cc/u, the symmetric function F~(u)/u^4 is a quartic G(t), and

        E : Z^2 = G(t),     C : W^2 = (t^2 - 4 Cc) G(t).

  When F carries a Klein-four group of such symmetries the genus-2 quotient
  splits further and Jac(X) ~ E1 x E2 x E3.
 *****************************************************************************/

powersums := function(Cc, kmax)
  Qt := PolynomialRing(Rationals()); t := Qt.1;
  P := [Qt | 2, t];
  for k in [2..kmax] do Append(~P, t*P[k] - Cc*P[k-1]); end for;
  return P;                         // P[k+1] = u^k + (Cc/u)^k as a poly in t
end function;

// genus-2 quotient C and elliptic quartic G for the involution u -> Cc/u (u = x-a)
buildFinite := function(F, a, Cc)
  Qx := Parent(F); xx := Qx.1;
  Ft := Evaluate(F, xx + a);
  b := [Coefficient(Ft, i) : i in [0..8]];
  P := powersums(Cc, 4);
  G := b[5];
  for i in [0..3] do G +:= b[i+1]*Cc^(i-4)*P[(4-i)+1]; end for;   // F~(u)/u^4 in t
  Qt := Parent(G); t := Qt.1;
  return G, HyperellipticCurve((t^2 - 4*Cc)*G);
end function;

// genus-2 quotient C and elliptic quartic G for the reflection x -> kk - x (v = x-kk/2)
buildReflection := function(F, kk)
  Qx := Parent(F); xx := Qx.1;
  Ft := Evaluate(F, xx + kk/2);
  Qu := PolynomialRing(Rationals()); u := Qu.1;
  G := &+[Coefficient(Ft, 2*i)*u^i : i in [0..4]];                // F~(v) = G(v^2)
  return G, HyperellipticCurve(u*G);
end function;

// all Q-rational bielliptic involutions of X, as a list <G_quartic, C_genus2>
octicSyms := function(F)
  Qx := Parent(F); x := Qx.1;
  res := [];
  // chart A: mu fixes a finite pair, M = [[a,b],[1,-a]], F(Mv) = (a^2+b)^4 F(v)
  Pab := PolynomialRing(Rationals(), 2); aa := Pab.1; bb := Pab.2;
  PXZ := PolynomialRing(Pab, 2); X := PXZ.1; Z := PXZ.2;
  Fh := &+[Coefficient(F,i)*X^i*Z^(8-i) : i in [0..8]];
  Fn := Evaluate(Fh, [aa*X + bb*Z, X - aa*Z]);
  lam := aa^2 + bb;
  IA := ideal<Pab | Coefficients(Fn - lam^4*Fh)>;
  if Dimension(IA) le 0 then
    for v in Variety(IA) do
      Cc := v[1]^2 + v[2];
      if Cc ne 0 then G, D := buildFinite(F, v[1], Cc); Append(~res, <G, D>); end if;
    end for;
  end if;
  // chart B: mu fixes infinity, mu(x) = k - x, F(k-x) = F(x)
  Pk := PolynomialRing(Rationals()); k := Pk.1;
  Pkx := PolynomialRing(Pk); xx := Pkx.1;
  IB := ideal<Pk | Coefficients(Evaluate(F, k - xx) - Evaluate(F, xx))>;
  gens := [c : c in Generators(IB) | c ne 0];
  if #gens gt 0 then
    g := GCD(gens);
    if g ne 0 and Degree(g) ge 1 then
      for r in Roots(g) do
        if Evaluate(F, r[1] - x) eq F then
          G, D := buildReflection(F, r[1]); Append(~res, <G, D>);
        end if;
      end for;
    end if;
  end if;
  return res;
end function;

// the elliptic curve (minimal model) whose Jacobian is that of the genus-1 quotient
// Z^2 = G(.); works even when Z^2 = G has no rational point (uses the genus-one model).
ellModel := function(G)
  if Degree(G) le 3 then
    E := EllipticCurve(HyperellipticCurve(G));
  else
    E := Jacobian(GenusOneModel(G));
  end if;
  return MinimalModel(E);
end function;

intrinsic IsBielliptic(X::CrvHyp) -> BoolElt
{ True iff the genus 3 hyperelliptic curve X admits a bielliptic involution
  (a degree-2 map to a genus-1 curve). }
  require Genus(X) eq 3: "X must be a genus 3 hyperelliptic curve";
  f, h := HyperellipticPolynomials(X);
  return #octicSyms(h^2 + 4*f) ge 1;
end intrinsic;

intrinsic BiellipticDecomposition(X::CrvHyp) -> SeqEnum
{ For a bielliptic genus 3 hyperelliptic curve X, return the list of pairs <C, E> -- C
  a genus-2 hyperelliptic curve and E an elliptic curve, with Jac(X) ~ Jac(C) x E -- one
  pair for each bielliptic involution of X (equivalently, each genus-2 curve covered by
  X via a degree-2 map).  The genus-2 model is a MinimalWeierstrassModel; the caller can
  call ReducedModel if a reduced model is desired. }
  require Genus(X) eq 3: "X must be a genus 3 hyperelliptic curve";
  f, h := HyperellipticPolynomials(X);
  syms := octicSyms(h^2 + 4*f);
  require #syms ge 1: "X is not bielliptic";
  return [<MinimalWeierstrassModel(s[2]), ellModel(s[1])> : s in syms];
end intrinsic;
