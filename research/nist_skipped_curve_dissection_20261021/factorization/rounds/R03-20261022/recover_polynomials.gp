\\ Native exact validation and layout of saved YAFU polynomial candidates.
integer(s) = {
  my(v=Vecsmall(s), x=0, sign=1, start=1, stop=#v);
  if(!#v,error("empty integer"));
  while(start<=stop && (v[start]==32||v[start]==9),start++);
  while(stop>=start && (v[stop]==32||v[stop]==9||v[stop]==13),stop--);
  if(start<=stop && v[start]==45,sign=-1;start++);
  if(start>stop,error("missing digits"));
  for(i=start,stop,if(v[i]<48||v[i]>57,error("nondecimal integer"));x=10*x+v[i]-48);
  sign*x;
};
{
  my(N=integer(getenv("NFS_INPUT")), path=getenv("POLY_INPUT"), out=getenv("POLY_OUTPUT"));
  if(N<=1,error("invalid input"));
  my(lines=readstr(path), block=List(), a=vector(6), seen=vector(6), y0=0, hasy0=0, count=0, hasn=0, highest=0);
  write(out,"n: ",N);
  foreach(lines,line,
    my(chars=Vecsmall(line));
    if(!#chars,next);
    my(header=chars[1]==35, tokens=strsplit(line,":"));
    if(tokens[1]=="n",
      if(hasn||count||#block||#tokens!=2||integer(tokens[2])!=N,error("wrong or misplaced input header"));
      hasn=1;next;
    );
    if(tokens[1]=="time",
      if(!hasn||#block||#tokens!=2||integer(tokens[2])<0,error("invalid native time record"));
      write(out,line);next;
    );
    if(header,
      if(#block,error("incomplete candidate before next header"));
      block=List();a=vector(6);seen=vector(6);hasy0=0;
    );
    if(!#block && !header,error("candidate must begin with native score header"));
    listput(block,line);
    my(name=Vecsmall(tokens[1]));
    if(#name==2 && name[1]==99 && name[2]>=48 && name[2]<=53,
      if(#tokens!=2,error("invalid coefficient"));
      my(k=name[2]-47);
      if(seen[k],error("duplicate coefficient"));
      a[k]=integer(tokens[2]);seen[k]=1;
    );
    if(tokens[1]=="Y0",
      if(#tokens!=2||hasy0,error("invalid Y0"));
      y0=integer(tokens[2]);hasy0=1;
    );
    if(tokens[1]=="Y1",
      if(#tokens!=2||!hasy0||vecmin(seen)!=1||a[6]==0,error("incomplete degree-5 candidate"));
      my(y1=integer(tokens[2]),g=gcd(y1,N));
      if(g!=1,error("noninvertible Y1: gcd=",g));
      my(m=-Mod(y0,N)/Mod(y1,N));
      if(subst(Polrev(a),'x,m)!=0,error("wrong input or invalid common root"));
      highest=max(highest,a[6]);
      write(out,strjoin(Vec(block),"\n"));
      count++;block=List();
    );
  );
  if(#block,error("trailing incomplete candidate"));
  if(!count,error("no complete candidates"));
  print("{\"verified\":true,\"input\":\"",N,"\",\"candidate_count\":",count,
    ",\"highest_leading_coefficient\":",highest,
    ",\"common_root_verified\":true,\"elapsed_time_record_added\":false}");
}
quit;
