\\ Generate kernel-checkable product equalities, not primality theorems.
{
  read(getenv("PRODUCT_INPUT"));
  my(path=getenv("LEAN_OUTPUT"));
  write(path,"import Init\n\n-- Product identities only. PARI primality certificates are separate.\nset_option maxRecDepth 4096\nnamespace SkippedNistFactors\n");
  foreach(PRODUCTS,r,
    my(P=r[3],value=prod(i=1,#P,P[i][1]^P[i][2]));
    if(value!=r[2],error("invalid product"));
    my(name=Str(strjoin(strsplit(r[1],"-"),"_"),"_product"));
    my(expr=strjoin(vector(#P,i,Str("(",P[i][1]," ^ ",P[i][2],")"))," * "));
    write(path,"theorem ",name," :\n    (",r[2]," : Nat) = ",expr," := by decide\n#print axioms ",name,"\n"));
  write(path,"end SkippedNistFactors");
}
quit;
