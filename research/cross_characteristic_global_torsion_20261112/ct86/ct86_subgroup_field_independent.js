'use strict';
// CT86 independent BigInt certificate and binary/source EC arithmetic; no Python imports.
const insist=(v,m='CT86 certificate')=>{if(!v)throw Error(m)};
function strip(a){while(a.length>1&&a.at(-1)===0n)a.pop();return a;}
function op(a,b,k=1n){let z=Array.from({length:Math.max(a.length,b.length)},(_,i)=>(a[i]??0n)+k*(b[i]??0n));return strip(z);}
function prod(a,b){let z=Array(a.length+b.length-1).fill(0n);for(let i=0;i<a.length;i++)for(let j=0;j<b.length;j++)z[i+j]+=a[i]*b[j];return strip(z);}
function exp(a,n){let z=[1n];while(n){if(n&1)z=prod(z,a);a=prod(a,a);n=Math.floor(n/2);}return z;}
function premainder(a,b){let r=strip([...a]);while(r.length>=b.length&&r.some(x=>x!==0n)){let sh=r.length-b.length,lc=r.at(-1),q=Array(sh).fill(0n).concat(b.map(x=>lc*x));r=op(r.map(x=>x*b.at(-1)),q,-1n);}return r;}
const X=[0n,1n], D=[-79n,-40n,-4n,4n],p3=[-21n,-237n,-60n,-4n,3n],f4=[-5821n,-1496n,-210n,-790n,-100n,-4n,2n];
const p5=op(prod(exp(D,2),f4),exp(p3,3),-1n);
const p7=op(prod(p5,exp(p3,3)),prod(exp(D,2),exp(f4,3)),-1n);
insist(p7.length===25&&p7[24]===7n);
const sq=exp([-10n,-2n,3n],2);
const A2=op(sq,prod([1n,-2n],D)), B2=D;
const B3=exp(p3,2),A3=op(prod(X,B3),prod(D,f4),-1n);
const denom=prod(D,B3),numer=op(op(prod(X,denom),prod(A2,B3)),prod(A3,D));
const H=[-2102915n,3828478n,-6423347n,-1696464n,-39690n,-29862n,-840n,-8n,1n];
let exact=[0n];for(let i=0;i<=8;i++)exact=op(exact,prod([H[i]],prod(exp(numer,i),exp(denom,8-i))));
insist(premainder(exact,p7).length===1&&premainder(exact,p7)[0]===0n);
// Independent brute irreducibility certificate mod 3: try every monic factor of degrees 1..4.
function norm3(a){let r=a.map(x=>((Number(x)%3)+3)%3);while(r.length>1&&r.at(-1)===0)r.pop();return r;}
function rem3(a,b){let r=norm3(a),d=norm3(b),unit=d.at(-1)===1?1:2;while(r.length>=d.length&&r.some(v=>v!==0)){let offset=r.length-d.length,c=r.at(-1)*unit%3;for(let i=0;i<d.length;i++)r[offset+i]=(r[offset+i]-c*d[i]+18)%3;r=norm3(r);}return r;}
for(let degree=1;degree<=4;degree++)for(let c=0;c<3**degree;c++){let a=[],z=c;for(let j=0;j<degree;j++){a.push(z%3);z=Math.floor(z/3);}a.push(1);insist(!rem3(H,a).every(v=>v===0),'g reducible mod3');}
const red=(a,p)=>(a%p+p)%p;
function inverse(a,p){for(let v=1;v<p;v++)if(red(v*a,p)===1)return v;throw Error('inverse');}
function sum37(P,Q){let p=37;if(!P)return Q;if(!Q)return P;let [x,y]=P,[u,v]=Q;
 if(x===u&&red(y+v+1,p)===0)return null;
 let slope=(x===u&&y===v)?red((3*x*x-2*x-10)*inverse(red(2*y+1,p),p),p):red((v-y)*inverse(red(u-x,p),p),p);
 let b=red(y-slope*x,p), xx=red(slope*slope+1-x-u,p),yy=red(-slope*xx-b-1,p);
 insist(red(yy*yy+yy-xx**3+xx*xx+10*xx+20,p)===0);
 return [xx,yy];}
let source=[null],q=null;for(let i=1;i<=7;i++){q=sum37(q,[9,4]);source.push(q);}
insist(source[7]===null&&JSON.stringify(source.slice(1,7))===JSON.stringify([[9,4],[27,9],[28,0],[28,36],[27,27],[9,32]]));
let x=source.slice(1,4).map(P=>P[0]),sc=[red(x[0]+x[1]+x[2],37),red(x[0]*x[1]+x[1]*x[2]+x[0]*x[2],37),red(x[0]*x[1]*x[2],37)];
insist(JSON.stringify(sc)==='[27,30,33]');
function eval37(poly,v){return poly.reduceRight((a,c)=>red(a*v+Number(c%37n),37),0);}
insist(eval37(H,27)===0&&eval37(denom,9)!==0&&red(eval37(numer,9)*inverse(eval37(denom,9),37),37)===27);
const MOD=0x1225;
function fmul(a,b){let z=0;while(b){if(b&1)z^=a;b>>=1;a<<=1;if(a&4096)a^=MOD;}return z&4095;}
function fpow(a,n){let z=1;while(n>0){if(n%2)z=fmul(z,a);a=fmul(a,a);n=Math.floor(n/2);}return z;}
const h=fmul(fmul(2,2),2)^fmul(2,2),one=[1,0],A=[2,0],Y=[0,1];
const fadd=(a,b)=>[a[0]^b[0],a[1]^b[1]];
const eq=(a,b)=>JSON.stringify(a)===JSON.stringify(b);
function mult(a,b){let t=fmul(a[1],b[1]);return [fmul(a[0],b[0])^fmul(t,h), fmul(a[0],b[1])^fmul(a[1],b[0])^t];}
function power(a,n){let o=one;while(n){if(n%2)o=mult(o,a);a=mult(a,a);n=Math.floor(n/2);}return o;}
function add24(P,Q){if(P===null)return Q;if(Q===null)return P;
 const [x,y]=P,[u,v]=Q;if(eq(x,u)&&eq(fadd(fadd(y,v),one),[0,0]))return null;
 const slope=eq(P,Q)?mult(x,x):mult(fadd(y,v),power(fadd(x,u),2**24-2));
 const b=fadd(y,mult(slope,x));const xx=fadd(fadd(fadd(mult(slope,slope),one),x),u), yy=fadd(fadd(mult(slope,xx),b),one);
 insist(eq(fadd(mult(yy,yy),yy),fadd(mult(mult(xx,xx),xx),mult(xx,xx))));return [xx,yy];}
let target=[null],z=null;for(let i=1;i<=7;i++){z=add24(z,[A,Y]);target.push(z);}
insist(target[7]===null && new Set(target.slice(0,7).map(a=>JSON.stringify(a))).size===7);
const roots=target.slice(1,4).map(pt=>pt[0]);
const c1=fadd(fadd(roots[0],roots[1]),roots[2]);
const c2=fadd(fadd(mult(roots[0],roots[1]),mult(roots[0],roots[2])),mult(roots[1],roots[2]));
const c3=mult(mult(roots[0],roots[1]),roots[2]);
insist(JSON.stringify([c1,c2,c3])===JSON.stringify([[1545,0],[1906,0],[1261,0]]));
for(const c of [c1,c2,c3])insist(c[1]===0&&eq(power(c,16),c)&&!eq(power(c,4),c));
const frob=(P,n)=>[power(P[0],2**n),power(P[1],2**n)];
insist(eq(frob(target[1],4),target[3]));
insist(eq(frob(target[2],4),target[6]));
insist(!target.slice(1,7).some(pt=>eq(pt,frob(target[1],1))));
insist(H.map(x=>Number((x%2n+2n)%2n)).join(',')==='1,0,1,0,0,0,0,0,1');
insist((fpow(1545,4)^1545^1)===0);
console.log('CT86 PASS: K_aut=6 L_aut=3 kernel_degree=8 src=(27,30,33) binary=(1545,1906,1261) residue_degree=4 frob4=3 wrong_Q=6');
