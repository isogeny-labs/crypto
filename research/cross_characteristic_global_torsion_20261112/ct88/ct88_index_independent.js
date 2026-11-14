'use strict';
// CT88 independent BigInt Sylvester/Bareiss determinant, no Python dependencies.
const g=[-2102915n,3828478n,-6423347n,-1696464n,-39690n,-29862n,-840n,-8n,1n];
const der=g.slice(1).map((v,i)=>v*BigInt(i+1));
const a=g.slice().reverse(),b=der.slice().reverse();
const n=a.length-1,m=b.length-1,N=n+m;
let A=Array.from({length:N},()=>Array(N).fill(0n));
for(let j=0;j<m;j++)for(let k=0;k<=n;k++)A[j][j+k]=a[k];
for(let j=0;j<n;j++)for(let k=0;k<=m;k++)A[m+j][j+k]=b[k];
let detSign=1n,prev=1n;
for(let k=0;k<N-1;k++){
 if(A[k][k]===0n){let q=k+1;while(q<N&&A[q][k]===0n)q++;if(q===N)throw Error('singular');[A[k],A[q]]=[A[q],A[k]];detSign=-detSign;}
 const p=A[k][k];
 for(let i=k+1;i<N;i++)for(let j=k+1;j<N;j++){
  const z=A[i][j]*p-A[i][k]*A[k][j];
  if(z%prev)throw Error('nonexact Bareiss');
  A[i][j]=z/prev;
 }
 for(let i=k+1;i<N;i++)A[i][k]=0n;
 prev=p;
}
const D=detSign*A[N-1][N-1];
if(D!==-126276666637416719355500270478117660693463886295963968396774334577180672n)throw Error('discriminant');
let r=D,v=0;while(r%2n===0n){r/=2n;v++;}
if(v!==16)throw Error('valuation');
const canon=(x,y)=>{x=((x%7)+7)%7;y=((y%7)+7)%7;
 if(x===0)return [0,1];let inverse=[0,1,4,5,2,3,6][x];return [1,(y*inverse)%7];};
const mul=p=>canon(-2*p[1],p[0]-2*p[1]);
let seen=new Set(),cycles=[];
for(let p of [[0,1],...[0,1,2,3,4,5,6].map(i=>[1,i])]){
 if(seen.has(p.toString()))continue;
 let start=p.toString(),q=p,l=0;
 do{if(seen.has(q.toString()))throw Error('cycle');seen.add(q.toString());l++;q=mul(q);}while(q.toString()!==start);
 cycles.push(l);
}
cycles.sort((a,b)=>a-b);if(JSON.stringify(cycles)!=='[4,4]')throw Error('orbits');
let f=0,h=0b10011;
g.forEach((v,i)=>{if((v&1n)!==0n)f|=(1<<i);});
let sq=0;for(let i=0;i<=4;i++)if((h>>i)&1)sq|=(1<<(2*i));
if(f!==sq)throw Error('nonmaximal');
const qs=[23624165350186n,10252887761980724n];
if(qs[0]%4n!==2n||qs[1]%8n!==4n)throw Error('denoms');
const obj={curve_order_f2:5,disc_g:D.toString(),disc_v2:v,frobenius_trace:-2,g_mod2:f,order_index_v2:8,projective_orbits:cycles,relative_q2_v2:1,relative_q3_v2:2,square_root_mod2:h,test:'ct88-2-adic-index'};
console.log(JSON.stringify(obj));
