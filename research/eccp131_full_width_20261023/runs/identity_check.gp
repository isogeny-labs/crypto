\ Exact published curve and target identity, independent PARI/GP check
p=1550031797834347859248576414813139942411;a=1399267573763578815877905235971153316710;b=1009296542191532464076260367525816293976;n=1550031797834347859219047037805205710577;
E=ellinit([a,b],p);
G=[Mod(1317953763239595888465524145589872695690,p),Mod(434829348619031278460656303481105428081,p)];Q=[Mod(1247392211317907151303247721489640699240,p),Mod(207534858442090452193999571026315995117,p)];
if(!isprime(p)||!isprime(n),error("prime check"));
if(ellcard(E)!=n,error("curve order"));
if(!ellisoncurve(E,G)||!ellisoncurve(E,Q),error("point off curve"));
if(ellmul(E,G,n)!=[0]||ellmul(E,Q,n)!=[0],error("subgroup order"));
print("IDENTITY_OK");
