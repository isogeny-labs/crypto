D=Mod(1,5)*(v^11-11*v^9+44*v^7-77*v^5+55*v^3-11*v);
P=v^5+v^4+v^3+2*v^2+3*v+1;
H=(v-2)*(D-1);
print("factor_identity=",D-2==Mod(1,5)*(v-2)*P^2);
print("squarefree_gcd=",gcd(H,deriv(H)));
print("degree_H=",poldegree(H));
print("hyperell_charpoly=",hyperellcharpoly(H));
quit;
