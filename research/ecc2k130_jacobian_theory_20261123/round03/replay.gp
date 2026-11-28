read("field.gp");
print("degree=",poldegree(H));
print("irreducible=",polisirreducible(H));
print("real_roots=",polsturm(H));
for(t=-2,2,print("h_A(",t,")=",subst(H,x,t)));
print("derivative_mod263=",lift(Mod(subst(deriv(H),x,-1),263)));
print("quadratic_norm_numerator=",395^2-917*13^2);
quit;
