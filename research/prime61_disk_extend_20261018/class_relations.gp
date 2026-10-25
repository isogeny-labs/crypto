\\ Exact selected-subgroup certificate. No class-number oracle or GRH call.
lookup_log(target, generator, order, table, width, giant) = {
  my(z=target, j=0, k);
  for(i=0,width,
    if(mapisdefined(table,Str(z),&j),
      k=(i*width+j)%order;
      if(qfbpow(generator,k)!=target,error("bad logarithm witness"));
      return(k)
    );
    z=qfbcomp(z,giant)
  );
  error("square is outside the certified odd cyclic subgroup")
};
selected_component_certificate() = {
  my(d=-6649310858224389603, odd=34081013, n=68162026, principal,
     forms,A,G,T,tors,tormap,baby,width,z,giant,k,tm,exponent,rhs);
  if(qfbpow(Qfb(2,1,3),3)!=Qfb(1,1,6),error("D=-23 control"));
  if(qfbpow(Qfb(2,2,3),2)!=Qfb(1,0,5),error("D=-20 control"));
  if(!isprime(odd) || n!=2*odd,error("bad order factors"));
  principal=Qfb(1,1,(1-d)/4);
  forms=[Qfb(3,3,554109238185365801),Qfb(7,5,237475387793728201),
    Qfb(11,7,151120701323281583),Qfb(13,5,127871362658161339),
    Qfb(19,9,87490932345057759),Qfb(23,17,72275118024178151),
    Qfb(53,27,31364673859549011),Qfb(61,23,27251274009116353)];
  A=forms[2];
  if(qfbpow(A,n)!=principal || qfbpow(A,n/2)==principal || qfbpow(A,n/odd)==principal,
    error("old exact order certificate failed"));
  G=qfbpow(A,2);
  if(G==principal || qfbpow(G,odd)!=principal,error("odd generator order"));
  T=[forms[1],qfbpow(A,odd),qfbpow(forms[3],odd),qfbpow(forms[8],odd)];
  for(i=1,4,if(qfbpow(T[i],2)!=principal,error("torsion square")));
  tors=vector(16); tormap=Map();
  for(mask=0,15,
    z=principal;
    for(j=1,4,if(bittest(mask,j-1),z=qfbcomp(z,T[j])));
    tors[mask+1]=z;
    if(mapisdefined(tormap,Str(z)),error("dependent torsion basis"));
    mapput(tormap,Str(z),mask);
    print("{\"kind\":\"torsion\",\"mask\":",mask,",\"form\":\"",z,"\"}")
  );
  width=sqrtint(odd); if(width^2<odd,width++);
  baby=Map(); z=principal;
  for(j=0,width-1,mapput(baby,Str(z),j); z=qfbcomp(z,G));
  giant=qfbpow(G,-width);
  for(i=1,8,
    if(qfbpow(forms[i],n)!=principal,error("selected class not killed by n"));
    k=lookup_log(qfbpow(forms[i],2),G,odd,baby,width,giant);
    if(!mapisdefined(tormap,Str(qfbpow(forms[i],odd)),&tm),error("torsion projection outside basis"));
    exponent=(k*((odd+1)/2))%odd;
    rhs=qfbcomp(qfbpow(G,exponent),tors[tm+1]);
    if(rhs!=forms[i],error("selected-generator expression failed"));
    print("{\"kind\":\"relation\",\"index\":",i,",\"odd_exponent\":",exponent,
      ",\"square_log\":",k,",\"torsion_mask\":",tm,",\"left\":\"",forms[i],
      "\",\"right\":\"",rhs,"\"}")
  );
  print("{\"kind\":\"summary\",\"schema\":\"prime61.selected-subgroup/v1\",\"result\":\"PASS\",\"odd_prime\":",odd,
    ",\"odd_generator\":\"",G,"\",\"torsion_basis\":\"",T,
    "\",\"torsion_count\":16,\"baby_steps\":",width,",\"selected_generators\":8,\"subgroup_order\":",16*odd,
    ",\"whole_class_number_certified\":false,\"entire_horizontal_class_certified\":false}")
};
default(parisizemax,1024*1024*1024);
iferr(selected_component_certificate(),failure,print(Str(failure));quit(1));
quit(0);
