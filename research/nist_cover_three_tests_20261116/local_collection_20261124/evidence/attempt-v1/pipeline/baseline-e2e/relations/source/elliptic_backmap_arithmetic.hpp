#pragma once
#include "quartic_descent_arithmetic.hpp"
#include <fstream>
constexpr int EB_PRECISION=112;
using EBExponent=std::array<int,2>;
Z eb_z(const T24& c){require(c.a.degree()<=0&&c.a.at(0).a.degree()<=0,"backmap original-field coefficient");return c.a.at(0).a.at(0);}
T24 eb_t(const Z& c){return T24(F(Poly<Z>{c}));}
struct EBSeries {
    std::array<Z,EB_PRECISION> c{};
    EBSeries()=default;
    explicit EBSeries(const Z& a){c[0]=a;}
    EBSeries scale(const Z& a)const{EBSeries r;for(int i=0;i<EB_PRECISION;++i)r.c[i]=c[i]*a;return r;}
    EBSeries operator+(const EBSeries& b)const{EBSeries r;for(int i=0;i<EB_PRECISION;++i)r.c[i]=c[i]+b.c[i];return r;}
    EBSeries operator-()const{return scale(Z(-1));}
    EBSeries operator-(const EBSeries& b)const{return *this+(-b);}
    EBSeries operator*(const EBSeries& b)const{EBSeries r;for(int i=0;i<EB_PRECISION;++i)if(c[i]!=Z(0))for(int j=0;j+i<EB_PRECISION;++j)if(b.c[j]!=Z(0))r.c[i+j]=r.c[i+j]+c[i]*b.c[j];return r;}
    EBSeries inv()const{require(c[0]!=Z(0),"backmap series unit");EBSeries r;r.c[0]=c[0].inv();for(int n=1;n<EB_PRECISION;++n){Z sum;for(int i=1;i<=n;++i)sum=sum+c[i]*r.c[n-i];r.c[n]=-sum*r.c[0];}return r;}
    EBSeries operator/(const EBSeries& b)const{return *this*b.inv();}
    EBSeries shift(int n)const{EBSeries r;for(int i=0;i<EB_PRECISION;++i)if(i+n>=0&&i+n<EB_PRECISION)r.c[i+n]=c[i];return r;}
    EBSeries derivative()const{EBSeries r;for(int i=1;i<EB_PRECISION;++i)r.c[i-1]=c[i]*Z(i);return r;}
    EBSeries integral()const{EBSeries r;for(int i=1;i<EB_PRECISION;++i)r.c[i]=c[i-1]/Z(i);return r;}
    int order()const{int n=0;while(n<EB_PRECISION&&c[n]==Z(0))++n;return n;}
};
struct EBPoly {
    std::map<EBExponent,Z> terms;
    EBPoly()=default;
    explicit EBPoly(const Z& a){if(a!=Z(0))terms.emplace(EBExponent{},a);}
    void add(EBExponent e,const Z& a){if(a==Z(0))return;auto p=terms.find(e);if(p==terms.end())terms.emplace(e,a);else{p->second=p->second+a;if(p->second==Z(0))terms.erase(p);}}
    EBPoly scale(const Z& a)const{EBPoly r;for(auto& [e,c]:terms)r.add(e,c*a);return r;}
    EBPoly operator+(const EBPoly& b)const{EBPoly r=*this;for(auto& [e,c]:b.terms)r.add(e,c);return r;}
    EBPoly operator-()const{return scale(Z(-1));}
    EBPoly operator-(const EBPoly& b)const{return *this+(-b);}
    EBPoly operator*(const EBPoly& b)const{EBPoly r;for(auto& [e,c]:terms)for(auto& [f,d]:b.terms)r.add({e[0]+f[0],e[1]+f[1]},c*d);return r;}
    EBPoly partial(int i)const{EBPoly r;for(auto& [e,c]:terms)if(e[i]){auto f=e;--f[i];r.add(f,c*Z(e[i]));}return r;}
    Z eval(const Z& x,const Z& y)const{Z r;for(auto& [e,c]:terms){Z a=c;for(int i=0;i<e[0];++i)a=a*x;for(int j=0;j<e[1];++j)a=a*y;r=r+a;}return r;}
    EBSeries eval(const EBSeries& x,const EBSeries& y)const{std::array<EBSeries,33> xp,yp;xp[0]=EBSeries(Z(1));yp[0]=EBSeries(Z(1));for(int i=1;i<33;++i){xp[i]=xp[i-1]*x;yp[i]=yp[i-1]*y;}EBSeries r;for(auto& [e,c]:terms){require(e[0]<33&&e[1]<33,"backmap polynomial series exponent");r=r+(xp[e[0]]*yp[e[1]]).scale(c);}return r;}
    EBPoly reduced(const EBPoly& f)const{auto leading=f.terms.find({0,4});require(leading!=f.terms.end()&&leading->second!=Z(0),"quartic monic-y reduction chart");EBPoly r=*this;int steps=0;for(;;){auto p=std::find_if(r.terms.begin(),r.terms.end(),[](const auto& t){return t.first[1]>=4;});if(p==r.terms.end())break;require(++steps<10000,"quartic polynomial reduction limit");auto e=p->first;Z a=p->second/leading->second;for(auto& [g,c]:f.terms)r.add({e[0]+g[0],e[1]-4+g[1]},-a*c);}return r;}
};
using EBRow=std::vector<Z>;
struct EBEchelon {
    int width;
    std::map<int,EBRow> rows;
    explicit EBEchelon(int n):width(n){}
    void insert(EBRow row){require(int(row.size())==width,"backmap linear row width");for(auto& [p,r]:rows){Z a=row[p];if(a!=Z(0))for(int j=p;j<width;++j)row[j]=row[j]-a*r[j];}int pivot=0;while(pivot<width&&row[pivot]==Z(0))++pivot;if(pivot==width)return;Z inv=row[pivot].inv();for(int j=pivot;j<width;++j)row[j]=row[j]*inv;rows.emplace(pivot,row);}
    std::vector<EBRow> kernel()const{std::vector<EBRow> out;for(int j=0;j<width;++j)if(!rows.count(j)){EBRow v(size_t(width),Z(0));v[j]=Z(1);for(auto it=rows.rbegin();it!=rows.rend();++it){Z sum;for(int k=it->first+1;k<width;++k)sum=sum+it->second[k]*v[k];v[it->first]=-sum;}out.push_back(v);}return out;}
};
void eb_emit_poly(std::ostream& out,const EBPoly& p){out<<'[';bool first=true;for(auto& [e,c]:p.terms){if(!first)out<<',';first=false;out<<"{\"exponents\":["<<e[0]<<','<<e[1]<<"],\"coefficient\":\""<<c.x<<"\"}";}out<<']';}
void eb_emit_series(const EBSeries& s,int count){std::cout<<'[';for(int i=0;i<count;++i)std::cout<<(i?",":"")<<'"'<<s.c[i].x<<'"';std::cout<<']';}
int eb_hilbert(const EBPoly& curve,const EBPoly& a,const EBPoly& b,int n,int& rank){std::map<EBExponent,int> index;int width=0;for(int i=0;i<=n;++i)for(int j=0;j+i<=n;++j)index.emplace(EBExponent{i,j},width++);EBEchelon echelon(width);
    std::array<const EBPoly*,3> generators{&curve,&a,&b};for(int k=0;k<3;++k){int degree=k?6:4;for(int i=0;i<=n-degree;++i)for(int j=0;i+j<=n-degree;++j){EBRow row(size_t(width),Z(0));for(auto& [e,c]:generators[k]->terms)row.at(size_t(index.at({i+e[0],j+e[1]})))=c;echelon.insert(row);}}
    rank=int(echelon.rows.size());return width-rank;
}
