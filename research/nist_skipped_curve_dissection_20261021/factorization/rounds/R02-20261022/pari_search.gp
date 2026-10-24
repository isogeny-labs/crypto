\\ Native bounded search; every returned divisor still requires certification.
default(nbthreads,1);
default(factor_proven,1);
qstr(n)=Str("\"",n,"\"");
{
  read(getenv("SEARCH_INPUT"));
  my(F,ok=0,seconds=if(type(SEARCH_SECONDS)=="t_INT",SEARCH_SECONDS,600));
  if(seconds<1 || seconds>600,error("invalid search limit"));
  iferr(F=alarm(seconds,factorint(C,6));alarm(0);ok=(type(F)=="t_MAT"),
    err,alarm(0);write(Str(OUT,"/pari-error.txt"),err));
  if(ok,
    write(Str(OUT,"/proposal.json"),"{\"status\":\"returned\",\"flag\":6,\"candidate_factors\":[",
      strjoin(apply(qstr,Vec(F[,1])),","),"]}"),
    write(Str(OUT,"/proposal.json"),"{\"status\":\"alarm_or_error\",\"flag\":6,\"candidate_factors\":[]}"));
}
quit;
