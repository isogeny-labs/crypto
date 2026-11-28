// Kaarelson v2 Theorem 3 coverage reproduction, m = 2, exact integer counts.
// See PROTOCOL.md in this directory. No Python in the measurement.
// Build: g++ -std=c++20 -O3 -Wall -Wextra -Werror coverage_t3.cpp -o coverage_t3
#include <cstdint>
#include <cstdio>
#include <cmath>
#include <string>
#include <vector>
#include <algorithm>

using i64 = long long;

static i64 P;
static inline i64 md(i64 x){ x%=P; return x<0?x+P:x; }
static i64 pw(i64 x,i64 e){ i64 y=1; x=md(x); for(;e;e>>=1,x=md(x*x)) if(e&1) y=md(y*x); return y; }
static inline i64 inv(i64 x){ return pw(x,P-2); }
static inline int leg(i64 t){ t=md(t); if(t==0) return 0; return pw(t,(P-1)/2)==1?1:-1; }

struct Pt{ i64 x,y; bool inf; };
static i64 A,B;
static Pt ptInf(){ return {0,0,true}; }
static Pt padd(const Pt&s,const Pt&t){
    if(s.inf) return t;
    if(t.inf) return s;
    if(s.x==t.x && md(s.y+t.y)==0) return ptInf();
    i64 num,den;
    if(s.x==t.x && s.y==t.y){ num=md(3*s.x%P*s.x+A); den=md(2*s.y); }
    else { num=md(t.y-s.y); den=md(t.x-s.x); }
    i64 sl=md(num*inv(den));
    i64 x=md(sl*sl-s.x-t.x);
    i64 y=md(sl*md(s.x-x)-s.y);
    return {x,y,false};
}
// |E(F_p)| = p + 1 + sum_x legendre(x^3+Ax+B)
static i64 order(){ i64 s=0; for(i64 x=0;x<P;x++) s+=leg((x*x%P*x + A*x + B)); return P+1+s; }
static bool isprime(i64 n){ if(n<2) return false; for(i64 d=2; d*d<=n; d++) if(n%d==0) return false; return true; }

int main(int argc,char**argv){
    std::string out = argc>1? argv[1] : "per_dilation.csv";
    FILE*csv=fopen(out.c_str(),"w");
    fprintf(csv,"p,arm,a,b,n,X,u,s,f,coverage,E2overX2\n");
    std::vector<i64> primes={1009,4099,10007};
    printf("# Kaarelson Theorem 3 coverage reproduction (m=2)\n");
    printf("# p | arm | a b | n | X | f/X mean | coverage mean (SD) [min,max] | cov(u=1) | E2/X^2 mean\n");
    for(i64 p: primes){
        P=p;
        i64 X=(i64)std::floor(std::sqrt((double)p));
        // ---- select curves ----
        struct Sel{ std::string name; i64 a,b; bool ok; };
        std::vector<Sel> sels;
        // generic a!=0
        { i64 fa=0,fb=0; bool found=false;
          for(i64 a=1;a<p&&!found;a++) for(i64 b=1;b<p&&!found;b++){ A=a;B=b; if(md(4*a%P*a%P*a+27*b%P*b)==0) continue; i64 n=order(); if(isprime(n)){fa=a;fb=b;found=true;} }
          sels.push_back({"generic",fa,fb,found}); }
        // j=0 a=0
        { i64 fb=0; bool found=false;
          for(i64 b=1;b<p&&!found;b++){ A=0;B=b; if(md(27*b%P*b)==0) continue; i64 n=order(); if(isprime(n)){fb=b;found=true;} }
          sels.push_back({"j0",0,fb,found}); }
        for(auto&sel: sels){
            if(!sel.ok){ const char* why = (p%3==2) ? "supersingular, order p+1 (p=2 mod 3)" : "no prime order among the a=0 sextic twists"; printf("%lld | %s | (no prime-order curve: %s) | X=%lld\n",(long long)p,sel.name.c_str(),why,(long long)X); continue; }
            A=sel.a; B=sel.b; i64 a=sel.a,b=sel.b;
            i64 n=order();
            // generator
            Pt G=ptInf();
            for(i64 x=0;x<p&&G.inf;x++){ i64 rhs=md((x*x%P)*x + a*x + b); if(leg(rhs)>=0){ i64 y=-1; for(i64 t=0;t<P;t++) if(t*t%P==rhs){y=t;break;} if(y>=0) G={x,y,false}; } }
            std::vector<i64> xOf(n,-1);
            { Pt cur=ptInf(); for(i64 e=1;e<n;e++){ cur=padd(cur,G); xOf[e]=cur.x; } }
            // sanity: xOf[e]==xOf[n-e]
            bool symOK=true; for(i64 e=1;e<n;e++) if(xOf[e]!=xOf[n-e]){symOK=false;break;}
            // ---- per-dilation loop over distinct squares s=u^2, u=1..(p-1)/2 ----
            std::vector<int> covStamp(n,0), rStamp(n,0); std::vector<i64> rVal(n,0);
            int tok=0;
            double covSum=0,covSq=0,enSum=0,fSum=0,covMin=1e9,covMax=-1; double cov_u1=-1; i64 cnt=0;
            std::vector<i64> Bu; Bu.reserve(2*X+8);
            for(i64 u=1; u<=(p-1)/2; u++){
                i64 s=(u*u)%p;
                // build B_u: exponents e with (s*xOf[e])%p in [1,X]
                Bu.clear();
                for(i64 e=1;e<n;e++){ i64 v=(s* (xOf[e]%P))%p; if(v>=1 && v<=X) Bu.push_back(e); }
                i64 f=(i64)Bu.size();
                // coverage: distinct reachable sums from generic unordered pairs
                tok++; i64 covered=0;
                for(size_t i=0;i<Bu.size();i++) for(size_t j=i+1;j<Bu.size();j++){
                    if(xOf[Bu[i]]==xOf[Bu[j]]) continue; // non-generic (same x)
                    i64 t=(Bu[i]+Bu[j])%n; if(covStamp[t]!=tok){covStamp[t]=tok; covered++;}
                }
                double cov=(double)covered/(double)n;
                // energy: ordered pairs (incl doubling & inverse), E2 = sum r^2
                tok++; std::vector<i64> touched; touched.reserve(2*f*f+8);
                for(size_t i=0;i<Bu.size();i++) for(size_t j=0;j<Bu.size();j++){
                    i64 t=(Bu[i]+Bu[j])%n; if(rStamp[t]!=tok){rStamp[t]=tok; rVal[t]=0; touched.push_back(t);} rVal[t]++;
                }
                i64 E2=0, chk=0; for(i64 t: touched){ E2+=rVal[t]*rVal[t]; chk+=rVal[t]; }
                (void)chk; // chk == f*f by construction
                double enX2=(double)E2/((double)X*(double)X);
                // accumulate
                covSum+=cov; covSq+=cov*cov; enSum+=enX2; fSum+=(double)f/(double)X; cnt++;
                covMin=std::min(covMin,cov); covMax=std::max(covMax,cov);
                if(u==1){ cov_u1=cov; }
                fprintf(csv,"%lld,%s,%lld,%lld,%lld,%lld,%lld,%lld,%lld,%.6f,%.6f\n",
                    (long long)p,sel.name.c_str(),(long long)a,(long long)b,(long long)n,(long long)X,(long long)u,(long long)s,(long long)f,cov,enX2);
            }
            double covMean=covSum/cnt, covVar=covSq/cnt-covMean*covMean, covSD=covVar>0?std::sqrt(covVar):0;
            double enMean=enSum/cnt, fXmean=fSum/cnt;
            printf("%lld | %-7s | a=%lld b=%lld | n=%lld%s | X=%lld | f/X=%.3f | cov=%.4f (SD %.4f) [%.4f,%.4f] | u1=%.4f | E2/X^2=%.3f\n",
                (long long)p,sel.name.c_str(),(long long)a,(long long)b,(long long)n,(symOK?"":" [SYM?!]"),(long long)X,fXmean,covMean,covSD,covMin,covMax,cov_u1,enMean);
        }
    }
    fclose(csv);
    printf("# per-dilation rows -> %s\n", out.c_str());
    return 0;
}
