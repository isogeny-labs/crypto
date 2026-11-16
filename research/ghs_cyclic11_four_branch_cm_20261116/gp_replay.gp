a=ffgen(z^5+Mod(4,5)*z+Mod(1,5),'z);
print("field_order=",5^5);
print("ellcard_E2=",ellcard(ellinit([0,0,0,1,1+a^2],a)));
print("ellcard_E24=",ellcard(ellinit([0,0,0,2,2+a+2*a^3],a)));
print("irreducible_t2=",polisirreducible(x^10-2*x^5+3125));
print("irreducible_t24=",polisirreducible(x^10-24*x^5+3125));
quit;
