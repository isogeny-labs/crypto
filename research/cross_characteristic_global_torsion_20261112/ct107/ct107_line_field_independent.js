#!/usr/bin/env node
'use strict';
// CT107 independent modular-first verifier: polynomial Euclid,
// Frobenius and exact rank-relation recovery at 2, 13 and 59.
function check(ok, why) { if (!ok) throw Error(why); }
function arith(p) {
  const norm = x => ((x % p) + p) % p;
  const trim = v => { v=v.map(norm); while(v.length>1 && v.at(-1)===0)v.pop(); return v; };
  const add=(a,b)=>trim(Array.from({length:Math.max(a.length,b.length)},(_,i)=> (a[i]||0)+(b[i]||0)));
  const neg=a=>trim(a.map(x=>-x));
  const sub=(a,b)=>add(a,neg(b));
  const mul=(a,b)=>{const c=new Array(a.length+b.length-1).fill(0);for(let i=0;i<a.length;i++)for(let j=0;j<b.length;j++)c[i+j]=norm(c[i+j]+a[i]*b[j]);return trim(c);};
  const pow=(a,n)=>{let v=[1];while(n){if(n%2)v=mul(v,a);a=mul(a,a);n=Math.floor(n/2);}return v;};
  const inv=x=>{x=norm(x);check(x!==0,'inverse of zero');let a=x,e=p-2,v=1;while(e){if(e%2)v=norm(v*a);a=norm(a*a);e=Math.floor(e/2);}check(norm(x*v)===1,'scalar inversion');return v;};
  const divide=(aa,bb)=>{
    let a=trim(aa),b=trim(bb);check(b.some(x=>x),'divide by zero');const lead=inv(b.at(-1));
    const q=new Array(Math.max(1,a.length-b.length+1)).fill(0);
    while(a.length>=b.length && (a.length!==1||a[0]!==0)){
      const v=norm(a.at(-1)*lead),k=a.length-b.length;q[k]=v;
      for(let i=0;i<b.length;i++)a[k+i]=norm(a[k+i]-v*b[i]);a=trim(a);
    }
    return [trim(q),a];
  };
  const rem=(a,b)=>divide(a,b)[1];
  const gcd=(a,b)=>{a=trim(a);b=trim(b);while(!(b.length===1&&b[0]===0)){[a,b]=[b,rem(a,b)];}return trim(a.map(x=>norm(x*inv(a.at(-1)))));};
  const invert=(a,m)=>{
    let r0=trim(m),r1=rem(a,m),t0=[0],t1=[1];
    while(!(r1.length===1&&r1[0]===0)){
      const [q,r]=divide(r0,r1);
      [r0,r1]=[r1,r];[t0,t1]=[t1,rem(sub(t0,mul(q,t1)),m)];
    }
    check(r0.length===1,'not a unit in quotient');
    return rem(t0.map(x=>norm(x*inv(r0[0]))),m);
  };
  return {norm,trim,add,neg,sub,mul,pow,inv,divide,rem,gcd,invert};
}
function divisionPolys(p){
 const {add,sub,mul,pow,trim}=arith(p),D=trim([-79,-40,-4,4]),f3=trim([-21,-237,-60,-4,3]),f4=trim([-5821,-1496,-210,-790,-100,-4,2]),X=[0,1];
 const f5=sub(mul(pow(D,2),f4),pow(f3,3));
 const f6=mul(f3,sub(f5,pow(f4,2)));
 const f7=sub(mul(f5,pow(f3,3)),mul(pow(D,2),pow(f4,3)));
 const f11=sub(mul(f7,pow(f5,3)),mul(mul(pow(D,2),f4),pow(f6,3)));
 check(f11.length===61 && f11[60]===11%p,'division polynomial degree');
 const nums=[X,sub(mul(X,D),f3),sub(mul(X,pow(f3,2)),mul(D,f4)),sub(mul(X,mul(D,pow(f4,2))),mul(f5,f3)),sub(mul(X,pow(f5,2)),mul(D,mul(f6,f4)))];
 const dens=[[1],D,pow(f3,2),mul(D,pow(f4,2)),pow(f5,2)];
 return {f11,nums,dens};
}
function quotient(p,mod){
 const a=arith(p), red=v=>a.rem(v,mod), plus=(x,y)=>red(a.add(x,y)),
    neg=v=>red(a.neg(v)), times=(x,y)=>red(a.mul(x,y));
 const pow=(x,n)=>{let z=[1];while(n){if(n%2)z=times(z,x);x=times(x,x);n=Math.floor(n/2);}return z;};
 const {nums,dens}=divisionPolys(p);
 let s=[0];
 for(let i=0;i<5;i++){
   const d=red(dens[i]),di=a.invert(d,mod);
   check(JSON.stringify(times(d,di))==='[1]','division polynomial denominator not invertible');
   s=plus(s,times(red(nums[i]),di));
 }
 const evalpoly=(coeff,x)=>{let r=[0];for(let i=coeff.length-1;i>=0;i--)r=plus(times(r,x),[coeff[i]]);return r;};
 return {...a,red,plus,neg,times,pow,s,evalpoly};
}
function minRelation(p,mod,maxdegree=12){
 const q=quotient(p,mod),d=mod.length-1,vectors=[];let x=[1];
 for(let k=0;k<=maxdegree;k++){
   vectors.push([...x,...new Array(d-x.length).fill(0)]);
   const mat=vectors.map(v=>v.slice()), u=vectors.map((_,i)=>vectors.map((_,j)=>+(i===j)));
   let rank=0;
   for(let j=0;j<d;j++){
      let i=rank;while(i<mat.length&&mat[i][j]===0)i++;
      if(i===mat.length)continue;
      [mat[rank],mat[i]]=[mat[i],mat[rank]];[u[rank],u[i]]=[u[i],u[rank]];
      const scale=q.inv(mat[rank][j]);mat[rank]=mat[rank].map(v=>q.norm(v*scale));u[rank]=u[rank].map(v=>q.norm(v*scale));
      for(let z=0;z<mat.length;z++)if(z!==rank){
        const v=mat[z][j];if(v){mat[z]=mat[z].map((w,t)=>q.norm(w-v*mat[rank][t]));u[z]=u[z].map((w,t)=>q.norm(w-v*u[rank][t]));}
      }
      rank++;if(rank===mat.length)break;
   }
   if(rank<mat.length){
     let i=rank;while(i<mat.length && u[i].at(-1)===0)i++;
     check(i<mat.length,'relation without highest coefficient');
     const scale=q.inv(u[i].at(-1)), coeff=u[i].map(z=>q.norm(z*scale));
     check(JSON.stringify(q.evalpoly(coeff,q.s))==='[0]','linear relation not verified');
     return coeff;
   }
   x=q.times(x,q.s);
 }
 throw Error('minimal polynomial exceeds bound');
}
const source=divisionPolys(59).f11;
const binary=divisionPolys(2).f11;
const q59=minRelation(59,source),q13=minRelation(13,divisionPolys(13).f11),q4=minRelation(2,binary);
const expected59=[19,26,16,12,35,43,33,58,29,29,46,39,1];
const expected13=[2,11,0,1,0,3,4,8,6,2,12,6,1],quartic=[1,1,0,0,1];
check(JSON.stringify(q59)===JSON.stringify(expected59)&&JSON.stringify(q13)===JSON.stringify(expected13),'modular line minpolys');
check(JSON.stringify(q4)===JSON.stringify(quartic),'binary collapsed invariant');
const a59=arith(59),a2=arith(2);
const q1=[27,1],q2=[36,1],g1=[41,22,35,11,14,1],g2=[16,2,57,45,21,1];
check(JSON.stringify(a59.mul(a59.mul(a59.mul(q1,q2),g1),g2))===JSON.stringify(q59),'source line factors');
check(JSON.stringify(a59.gcd(q59,q59.slice(1).map((c,i)=>a59.norm(c*(i+1)))))==='[1]','source line polynomial not squarefree');
check(JSON.stringify(a59.gcd(source,source.slice(1).map((c,i)=>a59.norm(c*(i+1)))))==='[1]','source eleven polynomial not squarefree');
for(const g of [g1,g2]){
 const q=quotient(59,g),X=[0,1];
 check(JSON.stringify(q.pow(X,59**5))==='[0,1]','source quintic fixed field');
 check(JSON.stringify(a59.gcd(g,q.sub(q.pow(X,59),X)))==='[1]','source quintic irreducibility');
}
const binaryFactors=[0x130027,0x131203,0x131211].map(w=>{const arr=[];for(let i=0;w;i++,w=Math.floor(w/2))arr.push(w%2);return arr;});
check(JSON.stringify(a2.mul(a2.mul(binaryFactors[0],binaryFactors[1]),binaryFactors[2]))===JSON.stringify(binary),'binary factorization');
for(const g of binaryFactors){
 const q=quotient(2,g),X=[0,1];
 check(JSON.stringify(q.pow(X,2**20))==='[0,1]','binary degree20 Frobenius');
 for(const d of [4,10])check(JSON.stringify(a2.gcd(g,q.sub(q.pow(X,2**d),X)))==='[1]','binary degree20 irreducibility');
 check(JSON.stringify(minRelation(2,g))===JSON.stringify(quartic),'binary invariant mismatch');
}
const qglobal2=a2.pow(quartic,3);
check(JSON.stringify(qglobal2)==='[1,1,1,1,1,0,1,0,1,1,0,0,1]','binary characteristic polynomial fifth root');
const out={binary_global_minpoly_reduction_asc:qglobal2,binary_index_2_adic_lower_bound:8,binary_invariant_minpoly_asc:quartic,binary_line_factor_degrees:[4,4,4],binary_prime:2,binary_residue_subalgebra_dimension:4,binary_x_factor_degrees:[20,20,20],case:'ct107-line-field-prime-index',complete_transfer_evaluated:false,global_char0_minpoly_expanded:false,global_line_degree:12,source_factor_degrees:[1,1,5,5],source_index_59_adic_valuation:0,source_minpoly_asc:q59,source_prime:59,source_rational_line_invariants:[23,32]};
console.log(JSON.stringify(out));
