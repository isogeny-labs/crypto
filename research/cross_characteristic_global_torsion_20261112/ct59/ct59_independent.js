'use strict';
// CT59: independent Frobenius-matrix fixed-vector tests for small ell;
// BigInt modular-order certificate for the two 32-bit prime examples.
const PRIMES = [3,5,7,11,13,17,19,23,31,37,41,43,47,59,61,67,73,89,
                97,101,127,131,163,257,521,65537,2147483647,4294967291];
const FIX = new Map([[3,[1,2]], [5,[8,8]], [11,[5,10]], [19,[9,18]],
    [37,[72,72]], [43,[7,14]], [97,[96,96]], [163,[81,162]],
    [257,[32,32]], [521,[520,520]], [65537,[64,64]],
    [2147483647,[124,124]], [4294967291,[2147483645,4294967290]]]);
function need(x,why){if(!x)throw Error(why);}
function prime(n){if(n<2)return false;for(let d=2;d*d<=n;d += d===2?1:2)if(n%d===0)return false;return true;}
function pfactors(n){let a=[];for(let d=2;d*d<=n;d += d===2?1:2){if(n%d===0){a.push(d);do{n/=d;}while(n%d===0);}}
 if(n>1)a.push(n);return a;}
function powmod(a,n,p){let b=1n;while(n){if(n&1n)b=b*a%p;a=a*a%p;n>>=1n;}return b;}
function order(l){const p=BigInt(l),a=p-2n;let m=l-1;
 for(const r of pfactors(m))while(m%r===0&&powmod(a,BigInt(m/r),p)===1n)m/=r;
 need(powmod(a,BigInt(m),p)===1n,'invalid order');
 for(const r of pfactors(m))need(powmod(a,BigInt(m/r),p)!==1n,'not minimal');
 return m;
}
// Matrix of Frobenius in the basis (P,F(P)): F(P)=F(P), F(F(P))=-2P.
function mm(A,B,p){return [(A[0]*B[0]+A[1]*B[2])%p,(A[0]*B[1]+A[1]*B[3])%p,
 (A[2]*B[0]+A[3]*B[2])%p,(A[2]*B[1]+A[3]*B[3])%p];}
function residue(a,p){return (a%p+p)%p;}
function matrixAudit(l,maxF,some,full){let F=[0,l-2,1,0],M=[1,0,0,1],first=0,firstFull=0;
 for(let f=1;f<=maxF;f++){
   M=mm(M,F,l);
   const det=residue((M[0]-1)*(M[3]-1)-M[1]*M[2],l);
   if(!first&&det===0)first=f;
   if(!firstFull&&M[0]===1&&M[1]===0&&M[2]===0&&M[3]===1)firstFull=f;
 }
 need(first===some&&firstFull===full,`matrix mismatch ell=${l}: ${first}/${firstFull} v ${some}/${full}`);
}
function audit(){for(const l of PRIMES){need(prime(l),'nonprime test input');
 const m=order(l),some=m%2?m:2*m,full=2*m;
 if(l<=521)matrixAudit(l,full,some,full);
 if(FIX.has(l)){const [a,b]=FIX.get(l);need(some===a&&full===b,'fixture mismatch');}
 console.log(`ell=${l} single_f=${some} full_f=${full} ord_minus2=${m}`);
}
 let n=1;for(let x=0;x<2;x++)for(let y=0;y<2;y++)if(residue(y*y+y-x*x*x-x,2)===0)n++;
 need(n===5,'alternate trace control');
 console.log('PASS: fixed-twist only; five-point twist defeats universal single-point f>=8 at ell=5');
}
audit();
