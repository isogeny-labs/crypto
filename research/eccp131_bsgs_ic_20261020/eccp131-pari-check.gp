\\ Independent PARI/GP point and group-order check of Certicom ECCp-131.
p = 0x048E1D43F293469E33194C43186B3ABC0B;
a = 0x041CB121CE2B31F608A76FC8F23D73CB66;
b = 0x02F74F717E8DEC90991E5EA9B2FF03DA58;
n = 0x048E1D43F293469E317F7ED728F6B8E6F1;
G = [0x03DF84A96B5688EF574FA91A32E197198A, 0x014721161917A44FB7B4626F36F0942E71];
Q = [0x03AA6F004FC62E2DA1ED0BFB62C3FFB568, 0x009C21C284BA8A445BB2701BF55E3A67ED];
E = ellinit([a,b],p);
print("G_on_curve=", ellisoncurve(E,G));
print("Q_on_curve=", ellisoncurve(E,Q));
print("nG=", ellmul(E,G,n));
print("nQ=", ellmul(E,Q,n));
print("ellcard=", ellcard(E));
print("published_order_matches=", ellcard(E)==n);
