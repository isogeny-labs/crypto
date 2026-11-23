#!/usr/bin/env node
"use strict";
// CT96 independent F3 irreducibility, GL2 orbit census, Frobenius trace recurrence.
// Scientific standing [pcc:unverified].
const chk=(b,why)=>{if(!b)throw new Error(why)};
const r3=n=>((n%3)+3)%3;
const trim=a=>{while(a.length>1&&a[a.length-1]===0)a.pop();return a};
const sub=(a,b)=>trim(Array.from({length:Math.max(a.length,b.length)},(_,i)=>
  r3((a[i]||0)-(b[i]||0))));
const mul=(a,b)=>{let c=Array(a.length+b.length-1).fill(0);
  for(let i=0;i<a.length;i++)for(let j=0;j<b.length;j++)
    c[i+j]=r3(c[i+j]+a[i]*b[j]);
  return trim(c)};
const power=(a,k)=>{let c=[1];while(k-->0)c=mul(c,a);return c};
const f3=a=>a.map(r3);
function psi7(){
  const D=f3([-79,-40,-4,4]),P3=f3([-21,-237,-60,-4,3]),
        F4=f3([-5821,-1496,-210,-790,-100,-4,2]);
  return sub(mul(sub(mul(power(D,2),F4),power(P3,3)),power(P3,3)),
             mul(power(D,2),power(F4,3)))}
function rem(a,f){
  a=trim(a.slice());const inv=f[f.length-1]===1?1:2;
  while(a.length>=f.length&&!(a.length===1&&a[0]===0)){
    let v=r3(a[a.length-1]*inv),shift=a.length-f.length;
    for(let i=0;i<f.length;i++)a[i+shift]=r3(a[i+shift]-v*f[i]);
    trim(a)
  }return a}
function gcd(a,b){
  while(!(b.length===1&&b[0]===0)){let c=rem(a,b);a=b;b=c}
  return a.map(v=>r3(v*(a[a.length-1]===1?1:2)))}
function field(){
  const f=psi7();chk(f.length===25&&f[24]===1,"degree and LC mod3");
  let x=[0,1],w=x,checks={};
  for(let k=1;k<=24;k++){
    w=rem(mul(mul(w,w),w),f);
    if(k===8||k===12){
      let h=gcd(f,sub(w,x));checks[String(k)]=h.length-1;
      chk(h.length===1&&h[0]===1,"Rabin gcd k="+k)
    }
  }chk(w.length===2&&w[0]===0&&w[1]===1,"Rabin full Frobenius");
  return checks}
function census(l){
  let gl=0,sLine=0,sX=0,sVec=0;
  const lines=new Set(),xs=new Set(),vec=new Set();
  for(let a=0;a<l;a++)for(let c=0;c<l;c++){
    if(a===0&&c===0)continue;
    const lead=a||c;let inv=1;
    while(inv*lead%l!==1)inv++;
    lines.add([a*inv%l,c*inv%l].join(","));
    const neg=[(l-a)%l,(l-c)%l];
    const v=(a<neg[0]||(a===neg[0]&&c<=neg[1]))?[a,c]:neg;
    xs.add(v.join(","));vec.add([a,c].join(","));
    let count=0;
    for(let b=0;b<l;b++)for(let d=0;d<l;d++)
      if((a*d-b*c)%l!==0)count++;
    chk(count===l*l-l,"second column noncollinearity");
    gl+=count;
    if(c===0)sLine+=count;
    if(c===0&&(a===1||a===l-1))sX+=count;
    if(a===1&&c===0)sVec+=count;
  }
  const [line_degree,x_degree,oriented_degree]=[lines.size,xs.size,vec.size];
  chk(gl===l*(l-1)*(l-1)*(l+1),"GL2 size");
  chk(gl/sLine===line_degree&&gl/sX===x_degree&&gl/sVec===oriented_degree,"orbit stabilizers");
  chk(line_degree===l+1&&x_degree===(l*l-1)/2&&oriented_degree===l*l-1,"tower size");
  return {ell:l,gl2:gl,line_degree,x_degree,oriented_degree,
          x_over_line:(l-1)/2,oriented_over_x:2}}
const mod=(x,l)=>((x%l)+l)%l;
function binary(l){
  // Independently work in F_l[t]/(t^2+2t+2), and use the trace recurrence
  // s_0=2, s_1=-2, s_d=-2s_(d-1)-2s_(d-2) for #E(F_(2^d)).
  let a=1,b=0,full=0,first=null,s0=2,s1=mod(-2,l),p2=1;
  for(let k=1;k<l*l;k++){
    [a,b]=[mod(-2*b,l),mod(a-2*b,l)];
    p2=p2*2%l;
    let s;
    if(k===1)s=s1;
    else {s=mod(-2*s1-2*s0,l);s0=s1;s1=s}
    if(first===null&&mod(p2+1-s,l)===0)first=k;
    if(a===1&&b===0){full=k;break}
  }
  chk(first!==null&&full>0&&(l*l-1)%full===0,"binary orders");
  return {ell:l,binary_first_nonzero_torsion_degree:first,
          binary_full_torsion_degree:full}}
function canonical(o){
  if(Array.isArray(o))return "["+o.map(canonical).join(",")+"]";
  if(o!==null&&typeof o==="object")return "{"+Object.keys(o).sort().map(
    k=>JSON.stringify(k)+":"+canonical(o[k])).join(",")+"}";
  return JSON.stringify(o)}
const towers=[3,5,7,11,13].map(census),
      binary_orders=[3,5,7,11,13,17,19,23,31].map(binary);
chk(towers[2].line_degree===8&&towers[2].x_degree===24&&
    towers[2].oriented_degree===48&&
    binary_orders[2].binary_full_torsion_degree===24,"CT80-94 controls");
process.stdout.write(canonical({binary_orders,field_7_division_degree:24,
  mod3_irreducible:true,rabin_gcd_degrees:field(),towers})+"\n");
