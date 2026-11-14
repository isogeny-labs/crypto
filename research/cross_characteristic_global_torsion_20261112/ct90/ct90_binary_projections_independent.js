'use strict';
// CT90 independent BigInt polynomial arithmetic, triangular integral coordinates,
// residue-field CRT, and all 65,536 multiplication-pair checks.
const G=[-2102915n,3828478n,-6423347n,-1696464n,-39690n,-29862n,-840n,-8n,1n];
const P=Array.from({length:8},()=>Array(8).fill(0n));
for(let j=0;j<4;j++)P[j][j]=4n;
const ncols=[[1,3,0,0,1,0,0,0],[0,1,3,0,0,1,0,0],
             [0,0,1,3,0,0,1,0],[1,3,0,1,0,0,0,1]];
for(let j=4;j<8;j++)for(let i=0;i<8;i++)P[i][j]=BigInt(ncols[j-4][i]);
const bs=Array.from({length:8},(_,j)=>Array.from({length:8},(_,i)=>P[i][j]));
const must=(condition,msg)=>{if(!condition)throw Error('CT90 '+msg);};
function times(a,b){let r=Array(15).fill(0n);
 for(let i=0;i<8;i++)for(let j=0;j<8;j++)r[i+j]+=a[i]*b[j];
 for(let k=14;k>=8;k--)for(let j=0;j<8;j++)r[k-8+j]-=r[k]*G[j];
 return r.slice(0,8);
}
// An element with rational power-basis coefficients c/q.  The triangular
// inverse basis has numerators below, each divided by q.
function numer(c){return [c[0]-c[4]-c[7],c[1]-3n*c[4]-c[5]-3n*c[7],
 c[2]-3n*c[5]-c[6],c[3]-3n*c[6]-c[7],4n*c[4],4n*c[5],4n*c[6],4n*c[7]];}
function bits(a){return a.reduce((r,x,i)=>r^(Number((x%2n+2n)%2n)<<i),0);}
const M=bs.map(a=>bs.map(b=>{let c=numer(times(a,b));
 // (A_i/4)*(A_j/4): denominator 16.
 must(c.every(x=>x%16n===0n),'product not integral');
 return bits(c.map(x=>x/16n));
}));
function mul8(a,b){let z=0;
 for(let i=0;i<8;i++)if(a&(1<<i))for(let j=0;j<8;j++)if(b&(1<<j))z^=M[i][j];
 return z;
}
const es=[240,241];
for(let e of es)must(mul8(e,e)===e,'idempotency');
must((es[0]^es[1])===1,'sum');
function basis(e){let z=[e];for(let i=1;i<4;i++)z.push(mul8(z[i-1],2));return z;}
function mapper(e){let B=basis(e),table=new Map();
 for(let k=0;k<16;k++){
  let x=0;for(let i=0;i<4;i++)if(k&(1<<i))x^=B[i];
  must(!table.has(x),'dependent basis');table.set(x,k);
 }
 must(table.size===16,'bad field dimension');
 return Array.from({length:256},(_,a)=>{
  let t=table.get(mul8(e,a));must(t!==undefined,'outside component');return t;
 });
}
let maps=es.map(mapper);
const projected=Array.from({length:256},(_,i)=>maps.map(a=>a[i]));
function mul4(a,b){let z=0;
 for(let i=0;i<4;i++)if(a>>i&1)for(let j=0;j<4;j++)if(b>>j&1)z^=1<<(i+j);
 for(let k=6;k>=4;k--)if(z>>k&1)z^=0x13<<(k-4);
 return z;
}
for(let a=0;a<256;a++)for(let b=0;b<256;b++){
 let m=projected[mul8(a,b)],x=projected[a],y=projected[b];
 must(m.every((v,i)=>v===mul4(x[i],y[i])),'multiplication mismatch');
 let s=projected[a^b];must(s.every((v,i)=>v===(x[i]^y[i])),'addition mismatch');
}
function local(c0,q0){let c=c0.map(BigInt),q=BigInt(q0),a=numer(c);
 // exact integral at 2: cancel powers of two in every numerator.
 let v=0n;while(q%2n===0n){q/=2n;v++;}
 let d=1n<<v;must(a.every(x=>x%d===0n),'not 2-integral');
 must(q%2n!==0n,'odd unit missing');
 return bits(a.map(x=>x/d));
}
const c2=['51746818064829','-330291288560641','-95029425174429','-1711075375204','-1596513316848','-44914399152','-455319459','54044109'];
const c3=['-201756977283948362','563360952055619683','149521192983511524','3495575179577110','2638006865239263','74263835359077','711827220769','-88442189137'];
let e2=local(c2,'23624165350186'),e3=local(c3,'10252887761980724');
function mul12(x,y){let z=0;while(y){if(y&1)z^=x;y>>>=1;x<<=1;if(x&4096)x^=0x1225;}return z&4095;}
function eval12(t){let a=1545,z=0,p=1;for(let i=0;i<4;i++){if(t>>i&1)z^=p;p=mul12(p,a);}return z;}
let v2=projected[e2].map(eval12),v3=projected[e3].map(eval12);
must(e2===4&&e3===254&&String(v2)==='1906,1906'&&String(v3)==='1261,1260','target embeddings');
must(String(projected[1])==='1,1'&&String(projected[2])==='2,2','source generator');
must(String(projected[240])==='1,0'&&String(projected[241])==='0,1','CRT units');
let out={basis_images:Array.from({length:8},(_,i)=>projected[1<<i]),
 e2_basis_mask:e2,e2_binary_values:v2,e2_images:projected[e2],
 e3_basis_mask:e3,e3_binary_values:v3,e3_images:projected[e3],
 idempotents:es,selected_idempotent:240,target_s:1545,
 tested_products:65536,test:'ct90-binary-residue-selection'};
// JSON keys ordered lexicographically to match independent Python output.
console.log(JSON.stringify(Object.fromEntries(Object.entries(out).sort(([a],[b])=>a.localeCompare(b)))));
