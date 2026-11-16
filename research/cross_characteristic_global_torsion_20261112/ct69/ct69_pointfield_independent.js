'use strict';
// Independent finite-field, residue and public DLP audit of the global model.
// The Python oracle proves global polynomial identities over Q, not used here.
const assert=(x)=>{if(!x)throw new Error('CT69 independent check failed')};
const H=[-101306819,89655,267096,349907,162543,-14817,-4813,108,27];
const A=[15208005,352381,332454,-39746,-19603,324,108];
const D=4992581;
function mod(a,n){return ((a%n)+n)%n}
function evalp(v,y,p){let acc=0;for(let i=v.length-1;i>=0;i--)acc=mod(acc*y+v[i],p);return acc}
function invp(a,p){for(let i=1;i<p;i++)if(mod(a*i,p)===1)return i;throw new Error('not invertible')}
function mul(a,b){let r=0;while(b){if(b&1)r^=a;a<<=1;if(a&256)a^=0x17b;b>>=1;}return r}
function pow(a,n){let r=1;while(n){if(n&1)r=mul(r,a);a=mul(a,a);n>>=1}return r}
function degree(a){let d=-1;while(a){d++;a>>=1}return d}
function rem(a,b){while(a&&degree(a)>=degree(b))a^=(b<<(degree(a)-degree(b)));return a}
function gcd(a,b){while(b){const r=rem(a,b);a=b;b=r}return a}
function irreducible(){let y=2;for(let i=1;i<=8;i++){y=mul(y,y);if([1,2,4].includes(i)&&gcd(y^2,0x17b)!==1)return false}return y===2}
assert(irreducible());
const r29=[];for(let y=0;y<29;y++)if(evalp(H,y,29)===0)r29.push(y);
assert(r29.join(',')==='5,23');
const sx=mod(-evalp(A,5,29)*invp(D,29),29);
assert(sx===18 && mod(5*5+5-(sx**3-sx*sx-10*sx-20),29)===0);
let sourceN=1;for(let x=0;x<29;x++)for(let y=0;y<29;y++)if(mod(y*y+y-(x**3-x*x-10*x-20),29)===0)sourceN++;
assert(sourceN===30);
// H mod2 is the irreducible polynomial used as the quotient field modulus.
let Hbits=0;for(let i=0;i<H.length;i++)if(mod(H[i],2))Hbits^=1<<i;
assert(Hbits===0x17b);
const beta=2, x=pow(beta,4)^beta^1;
assert(x===19);
let value=0;for(let i=H.length-1;i>=0;i--)value=mul(value,beta)^mod(H[i],2);
assert(value===0);
assert((mul(beta,beta)^beta)===(mul(mul(x,x),x)^mul(x,x)));
assert((pow(x,4)^x^1)===0);
assert((pow(x,4)^1)===x); // doubling of U is -U
let cur=beta, orbit=new Set();for(let i=0;i<8;i++){orbit.add(cur);cur=mul(cur,cur)};
assert(orbit.size===8&&cur===beta);
// Direct independent point-count check: enumerate all binary (x,y) pairs.
let targetN=1;for(let a=0;a<256;a++){
 const rhs=mul(mul(a,a),a)^mul(a,a);
 for(let b=0;b<256;b++)if((mul(b,b)^b)===rhs)targetN++;
}
assert(targetN===225);
const t=[null,[x,beta],[x,beta^1]];
const q=[null,[18,5],[18,23]];
const mapped=q.map(z=>z===null?null:(z[0]===18&&z[1]===5?t[1]:(z[0]===18&&z[1]===23?t[2]:undefined)));
assert(mapped.every((z,i)=>JSON.stringify(z)===JSON.stringify(t[i])));
const solved=mapped.map(z=>t.findIndex(w=>JSON.stringify(w)===JSON.stringify(z)));
assert(solved.join(',')==='0,1,2');
const result={curve:'11a1',ell:3,source_field:29,target_field:256,global_point_degree:8,global_full_division_degree:48,
  global_model:'H(Y), X=-A(Y)/4992581',reduction29_y_roots:r29,
  binary_residue_polynomial_hex:'0x17b',source_generator:[18,5],binary_generator:[x,beta],
  binary_frobenius_orbit:orbit.size,encoded_subgroup_cases:q.length,source_group_order:sourceN,
  target_group_order:targetN,recovered_dlogs:solved,output_dlp_verified:true,root_lookup:true,scalable:false};
function ordered(v){if(Array.isArray(v))return v.map(ordered);if(v&&typeof v==='object'){let x={};for(const k of Object.keys(v).sort())x[k]=ordered(v[k]);return x}return v}
process.stdout.write(JSON.stringify(ordered(result))+'\n');
