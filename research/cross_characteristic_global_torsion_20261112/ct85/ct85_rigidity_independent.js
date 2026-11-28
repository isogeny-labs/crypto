'use strict';
// Separate BigInt polynomial construction and independently represented F4096.
const check=q=>{if(!q)throw Error('CT85 certificate failed')};
const pm=(a,b)=>{let z=Array(a.length+b.length-1).fill(0n);for(let i=0;i<a.length;i++)for(let j=0;j<b.length;j++)z[i+j]+=a[i]*b[j];return z};
const ps=(a,b)=>Array.from({length:Math.max(a.length,b.length)},(_,i)=>(a[i]??0n)-(b[i]??0n));
const ev=(a,x,m)=>{let z=0n;for(let i=a.length-1;i>=0;i--){z=z*x+a[i];if(m)z=(z%m+m)%m;}return z};
const b2=-4n,b4=-20n,b6=-79n,b8=-21n;
const c4=b2*b2-24n*b4,delta=-b2*b2*b8-8n*b4**3n-27n*b6*b6+9n*b2*b4*b6;
check(c4===496n&&delta===-(11n**5n)&&c4**3n%11n!==0n);
const D=[-79n,-40n,-4n,4n],psi3=[-21n,-237n,-60n,-4n,3n];
const f4=[-5821n,-1496n,-210n,-790n,-100n,-4n,2n];
const m=[-10n,-2n,3n];
const A2=ps(pm(m,m),pm([-1n,2n],D)),A3=ps(pm([0n,1n],pm(psi3,psi3)),pm(D,f4));
const funcs=[[[0n,1n],[1n]],[A2,D],[A3,pm(psi3,psi3)]];
const RED=(1<<12)|(1<<9)|(1<<5)|(1<<2)|1;
function fm(a,b){let z=0;while(b){if(b&1)z^=a;b>>=1;a<<=1;if(a&4096)a^=RED;}return z&4095;}
function fp(a,n){let z=1;for(;n;n=Math.floor(n/2)){if(n%2)z=fm(z,a);a=fm(a,a);}return z;}
function fEval(a){let z=0;for(let i=a.length-1;i>=0;i--)z=fm(z,2)^Number(a[i]&1n);return z;}
let rows=[];
for(let i=0;i<3;i++){
 const n=i+1,[A,B]=funcs[i],N=BigInt(n);
 check(A.length===n*n+1&&B.length===n*n&&A.at(-1)===1n&&B.at(-1)===N*N);
 let den=ev(B,9n,37n),num=ev(A,9n,37n),a=-1n;
 for(let k=1n;k<37n;k++)if(den*k%37n===1n)a=k;
 check(a>0n&&Number(num*a%37n)===[9,27,28][i]);
 let bd=fEval(B);check(bd!==0&&fm(fEval(A),fp(bd,4094))===[2,17,1562][i]);
 rows.push([n,n*n,n*n-1,n*n]);
}
console.log(JSON.stringify({binary_x:[2,17,1562],c4:Number(c4),delta:Number(delta),j_num:Number(c4**3n),map_degrees:rows,non_cm:true,source_x:[9,27,28]}));
