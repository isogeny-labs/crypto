\\ Common helpers. Field K = F_{q^n}, q prime, generator t. F_q embedded as integers*one.
\\ f(x) = x^3 + a2 x^2 + a4 x + a6 (odd char).
fval(c,x) = x^3 + c[1]*x^2 + c[2]*x + c[3];
\\ number of w in F_q (f(c0+c1 w) != 0) for which f is a nonzero square in K
linelift(c, c0, c1, q, one) = {
  my(s=0, r=0, z); for(w=0, q-1, z = fval(c, c0 + c1*w*one); if(z==0, r++, if(issquare(z), s++))); [s, r];
}
\\ is the polynomial f_L(w) = c1^-3 f(c0+c1 w) defined over subfield of degree e over F_q ?
\\ coefficients: w^2: (3c0+a2)/c1 ; w: (3c0^2+2a2 c0+a4)/c1^2 ; 1: f(c0)/c1^3
fLcoef(c,c0,c1) = [(3*c0+c[1])/c1, (3*c0^2+2*c[1]*c0+c[2])/c1^2, fval(c,c0)/c1^3];
insub(z, q, e) = (z^(q^e) == z);
desc(c,c0,c1,q,e) = my(v=fLcoef(c,c0,c1)); insub(v[1],q,e) && insub(v[2],q,e) && insub(v[3],q,e);
