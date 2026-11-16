\\ Reconstruct exact timeout targets from raw hexadecimal curve parameters.
\\ Large integers are always written as decimal strings in JSON outputs.
qstr(n) = Str("\"", n, "\"");
emit(id, value, pieces) = print("{\"id\":\"",id,"\",\"value\":",qstr(value),",\"bits\":",#binary(value),",\"pieces\":[",strjoin(apply(qstr,pieces),","),"]}");
{
  read(getenv("CURVE_INPUTS"));
  my(curves=Map());
  foreach(INPUTS,c, mapput(curves,c[1],c));
  foreach(["skipped_224_5736","skipped_256_876","published_224","published_256"],name,
    my(c=mapget(curves,name), p=c[2], n=c[5], t=p+1-n);
    emit(Str(name,"-twist2"),(p-1)^2+t^2,[(p-1)^2+t^2]));
  my(c=mapget(curves,"skipped_224_1485"),p=c[2],n=c[5],t=p+1-n);
  emit("skipped_224_1485-conductor4",abs(t*(t^2-2*p)),[abs(t),abs(t^2-2*p)]);
  c=mapget(curves,"skipped_256_876"); p=c[2]; n=c[5];
  emit("skipped_256_876-4n-minus1",4*n-1,[4*n-1]);
  emit("p256-4p-minus1",4*p-1,[4*p-1]);
  c=mapget(curves,"published_256"); n=c[5];
  emit("published_256-6n-plus1",6*n+1,[6*n+1]);
  emit("published_256-7n-minus1",7*n-1,[7*n-1]);
}
quit;
