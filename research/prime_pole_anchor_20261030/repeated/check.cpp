#include <gmpxx.h>
#include <algorithm>
#include <iostream>
#include <stdexcept>
#include <vector>

using Q = mpq_class;
using Poly = std::vector<Q>;
void need(bool b) { if (!b) throw std::runtime_error("certificate failed"); }
int valuation(Q q) {
    need(q != 0);
    mpz_class n = q.get_num();
    if (n < 0) n = -n;
    return int(mpz_scan1(n.get_mpz_t(), 0)) -
           int(mpz_scan1(q.get_den().get_mpz_t(), 0));
}
Poly linear(const Poly& p, Q c, Q s) {
    Poly out(p.size()+1);
    for (size_t i=0; i<p.size(); ++i) {
        out[i] += c*p[i]; out[i+1] += s*p[i];
    }
    return out;
}
Poly chart(int l, int t, int delta) {
    Q norm=1;
    for (int s=-l; s<l; ++s) if (s) norm *= -s;
    Poly p{Q(1)/norm};
    for (int s=-l; s<l; ++s) if (s!=delta) p=linear(p,t-s,2*l);
    return p;
}
int content(const Poly& p) {
    int v=100000;
    for (const Q& c:p) if (c!=0) v=std::min(v,valuation(c));
    return v;
}
mpz_class fib(unsigned n) {
    mpz_class x=0,y=1;
    for (unsigned i=0;i<n;++i) { mpz_class z=x+y; x=y; y=z; }
    return x;
}
Q diagonal(const std::vector<int>& m,int t) {
    Q f=1;
    for (size_t j=0;j<m.size();++j) {
        Q factor=Q((1<<j)-t)/Q(1<<j);
        for (int r=0;r<m[j];++r) f*=factor;
    }
    return f;
}
int main() {
    try {
        unsigned charts=0,coefficients=0,odd=0,levels=0,negative=0;
        for (int l:{4,8,16}) for (int k:{1,2,4,8}) {
            for (int t=-l;t<l;++t) for (int d=-l+1;d<l;++d) {
                Poly old=chart(l,t,d),p=old;
                if (d%2==0) for (int r=0;r<k;++r)
                    p=linear(p,Q(1-t)/Q(1-d),Q(-2*l)/Q(1-d));
                need(p[0]==old[0]); ++charts;
                for (const Q& c:p) { need(c==0 || valuation(c)>=0); ++coefficients; }
                if (t%2!=0 && d%2==0) {
                    need(content(p)>=content(old)+k); ++odd;
                }
            }
        }
        for (int a=1;a<=8;++a) for (int g:{1,2,4}) {
            std::vector<int> m(a);
            int total=0;
            for (int i=a-1;i>=0;--i) {
                m[i]=g;
                for (int j=i+1;j<a;++j) m[i]+=(j-i)*m[j];
                total+=m[i];
            }
            need(mpz_class(total)==g*fib(2*a));
            for (int i=0;i<a;++i) {
                need(valuation(diagonal(m,-(1<<i)))==g); ++levels;
                std::vector<int> ones(a,1);
                int expected=1;
                for (int j=i+1;j<a;++j) expected-=j-i;
                need(valuation(diagonal(ones,-(1<<i)))==expected);
            }
            --m[0];
            need(valuation(diagonal(m,-1))==g-1); ++negative;
            if (g==1) std::cout<<"a="<<a<<" L="<<(1<<a)<<" minimum_added_degree="<<total<<'\n';
        }
        std::cout<<"charts="<<charts<<" integral_coefficients="<<coefficients
                 <<" odd_content_checks="<<odd<<" direct_level_checks="<<levels
                 <<" insufficient_schedule_rejected="<<negative<<"\nPASS\n";
    } catch (const std::exception& e) { std::cerr<<"FAIL "<<e.what()<<'\n'; return 1; }
}
