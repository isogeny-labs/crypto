{
 iferr(
 my(cv=read("case0.gpvec"),pr=cv[1],a2=Mod(16*cv[7],pr),b2=Mod(64*cv[8],pr),G2=[Mod(4*cv[12][1],pr),Mod(8*cv[12][2],pr)],C);
 C=ellinit([0,0,0,a2,b2],pr);
 if(!ellisoncurve(C,G2)||ellorder(C,G2)!=cv[11],error("coordinate control order"));
 write("scalar-coordinate-control.gpvec",lift([pr,a2,b2,cv[11],G2]));
 print("PASS coordinate u=2 original-subgroup control");
 ,failure,print(failure);quit(2));
}
quit(0);
