// Kaarelson v2 Theorem 3 coverage, m=2, averaged over several curves per prime.
#include <cstdlib>
// Follow-up to coverage_t3.cpp: tests whether averaging over curves brings mean
// coverage into the paper's 0.39-0.42 band and whether coverage(u=1) ~ the
// over-dilation mean after averaging over curves. Exact integer counts.
// Factor base per dilation is built in O(X) via an x -> exponent map (same set
// as coverage_t3.cpp's O(n) scan). See PROTOCOL.md / RESULT.md.
// Build: g++ -std=c++20 -O3 -Wall -Wextra -Werror coverage_multi.cpp -o coverage_multi
#include <cstdint>
#include <cstdio>
#include <cmath>
#include <vector>
#include <algorithm>
using i64 = long long;

static i64 P;
static inline i64 md(i64 x){ x%=P; return x<0?x+P:x; }
static i64 pw(i64 x,i64 e){ i64 y=1;x=md(x);for(;e;e>>=1,x=md(x*x))if(e&1)y=md(y*x);return y; }
static inline i64 inv(i64 x){ return pw(x,P-2); }
static inline int leg(i64 t){ t=md(t); if(t==0) return 0; return pw(t,(P-1)/2)==1?1:-1; }
static i64 A,B;
struct Pt{ i64 x,y; bool inf; };
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
    return {x,md(sl*md(s.x-x)-s.y),false};
}
static i64 order(){ i64 s=0; for(i64 x=0;x<P;x++) s+=leg((x*x%P)*x + A*x + B); return P+1+s; }
static bool isprime(i64 n){ if(n<2) return false; for(i64 d=2; d*d<=n; d++) if(n%d==0) return false; return true; }

struct CurveStat{ i64 a,b,n; double covMean, cov_u1, enMean; };

static CurveStat measureCurve(i64 a,i64 b){
    A=a; B=b; i64 n=order();
    i64 X=(i64)std::floor(std::sqrt((double)P));
    // generator
    Pt G=ptInf();
    for(i64 x=0;x<P&&G.inf;x++){ i64 rhs=md((x*x%P)*x+a*x+b); if(leg(rhs)>=0){ i64 y=-1; for(i64 t=0;t<P;t++) if(t*t%P==rhs){y=t;break;} if(y>=0) G={x,y,false}; } }
    std::vector<i64> xOf(n,-1);
    std::vector<std::vector<int>> xToExp(P);
    { Pt cur=ptInf(); for(i64 e=1;e<n;e++){ cur=padd(cur,G); xOf[e]=cur.x; xToExp[cur.x].push_back((int)e); } }
    std::vector<int> covStamp(n,0), rStamp(n,0); std::vector<i64> rVal(n,0);
    int tok=0;
    double covSum=0,enSum=0,cov_u1=0; i64 cnt=0;
    std::vector<i64> Bu; Bu.reserve(2*X+8);
    for(i64 u=1; u<=(P-1)/2; u++){
        i64 s=(u*u)%P, sinv=inv(s);
        Bu.clear();
        for(i64 t=1;t<=X;t++){ i64 xt=(t*sinv)%P; for(int e: xToExp[xt]) Bu.push_back(e); }
        // coverage
        tok++; i64 covered=0;
        for(size_t i=0;i<Bu.size();i++) for(size_t j=i+1;j<Bu.size();j++){
            if(xOf[Bu[i]]==xOf[Bu[j]]) continue;
            i64 r=(Bu[i]+Bu[j])%n; if(covStamp[r]!=tok){covStamp[r]=tok; covered++;}
        }
        double cov=(double)covered/(double)n;
        // energy
        tok++; std::vector<i64> touched;
        for(size_t i=0;i<Bu.size();i++) for(size_t j=0;j<Bu.size();j++){
            i64 r=(Bu[i]+Bu[j])%n; if(rStamp[r]!=tok){rStamp[r]=tok; rVal[r]=0; touched.push_back(r);} rVal[r]++;
        }
        i64 E2=0; for(i64 r: touched) E2+=rVal[r]*rVal[r];
        covSum+=cov; enSum+=(double)E2/((double)X*X); cnt++;
        if(u==1) cov_u1=cov;
    }
    return {a,b,n,covSum/cnt,cov_u1,enSum/cnt};
}

int main(int argc,char**argv){
    int K = argc>1? atoi(argv[1]) : 16;           // curves per prime
    const char* out = argc>2? argv[2] : "per_curve.csv";
    FILE*csv=fopen(out,"w");
    fprintf(csv,"p,idx,a,b,n,mean_coverage,coverage_u1,E2overX2\n");
    std::vector<i64> primes={1009,4099,10007};
    printf("# Kaarelson Theorem 3 coverage averaged over %d prime-order generic curves per prime (m=2)\n",K);
    printf("# p | X | curves | mean cov over curves (SD) | mean cov(u=1) over curves (SD) | mean E2/X^2 | per-curve cov range\n");
    for(i64 p: primes){
        P=p; i64 X=(i64)std::floor(std::sqrt((double)p));
        std::vector<CurveStat> cs;
        i64 a=1,b=1; int found=0;
        while(found<K && a<p){
            A=a;B=b;
            if(!(md(4*a%P*a%P*a+27*b%P*b)==0)){ i64 n=order(); if(isprime(n)){ CurveStat st=measureCurve(a,b); cs.push_back(st); found++; fprintf(csv,"%lld,%d,%lld,%lld,%lld,%.6f,%.6f,%.6f\n",(long long)p,found,(long long)a,(long long)b,(long long)st.n,st.covMean,st.cov_u1,st.enMean);} }
            b++; if(b>=p){ a++; b=1; }
        }
        // aggregate over curves
        double cM=0,cS=0,uM=0,uS=0,eM=0,cmin=1e9,cmax=-1;
        for(auto&st:cs){ cM+=st.covMean; uM+=st.cov_u1; eM+=st.enMean; cmin=std::min(cmin,st.covMean); cmax=std::max(cmax,st.covMean); }
        cM/=cs.size(); uM/=cs.size(); eM/=cs.size();
        for(auto&st:cs){ cS+=(st.covMean-cM)*(st.covMean-cM); uS+=(st.cov_u1-uM)*(st.cov_u1-uM); }
        cS=std::sqrt(cS/cs.size()); uS=std::sqrt(uS/cs.size());
        printf("%lld | X=%lld | %zu | cov=%.4f (SD %.4f) | cov(u=1)=%.4f (SD %.4f) | E2/X^2=%.3f | [%.4f,%.4f]\n",
            (long long)p,(long long)X,cs.size(),cM,cS,uM,uS,eM,cmin,cmax);
    }
    fclose(csv);
    printf("# per-curve rows -> %s\n", out);
    return 0;
}
