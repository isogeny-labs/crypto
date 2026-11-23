#pragma once
// Separate arithmetic path: no producer field, group, polynomial or KM headers.
#include "json.hpp"
#include <gmpxx.h>
#include <openssl/ec.h>
#include <openssl/obj_mac.h>
#include <openssl/crypto.h>
#include <algorithm>
#include <array>
#include <chrono>
#include <ctime>
#include <iostream>
#include <memory>
#include <set>

const mpz_class modulus_p("6277101735386680763835789423207666416083908700390324961279");
const mpz_class order_r("6277101735386680763835789423176059013767194773182842284081");
mpz_class reduce(mpz_class a,const mpz_class& m=modulus_p){mpz_mod(a.get_mpz_t(),a.get_mpz_t(),m.get_mpz_t());return a;}
mpz_class inverse(const mpz_class& a,const mpz_class& m=modulus_p){mpz_class b;require(mpz_invert(b.get_mpz_t(),a.get_mpz_t(),m.get_mpz_t())==1,"checker inverse");return b;}
mpz_class power(mpz_class a,int n){mpz_class b=1;while(n--){b=reduce(b*a);}return b;}
struct Term {int x,y;mpz_class c;};
using Polynomial=std::vector<Term>;
Polynomial read_polynomial(const Json& j){Polynomial f;for(auto& t:j.arr){auto& e=t.at("exponents").arr;f.push_back({e.at(0).integer(),e.at(1).integer(),mpz_class(t.at("coefficient").scalar)});}return f;}
mpz_class evaluate(const Polynomial& f,const mpz_class& x,const mpz_class& y){mpz_class value=0;for(auto& t:f)value+=t.c*power(x,t.x)*power(y,t.y);return reduce(value);}
Polynomial derivative(Polynomial f,int axis){Polynomial out;for(auto t:f){int n=axis?t.y:t.x;if(n){t.c=reduce(t.c*n);if(axis)--t.y;else --t.x;out.push_back(t);}}return out;}
constexpr size_t precision=16;
using Series=std::array<mpz_class,precision>;
Series multiply(const Series& a,const Series& b){Series out{};for(size_t i=0;i<precision;++i)for(size_t j=0;i+j<precision;++j)out[i+j]+=a[i]*b[j];for(auto& n:out)n=reduce(n);return out;}
Series series_evaluate(const Polynomial& f,const Series& x,const Series& y){
    int nx=0,ny=0;for(auto& t:f){nx=std::max(nx,t.x);ny=std::max(ny,t.y);}
    std::vector<Series> xp(size_t(nx+1)),yp(size_t(ny+1));xp[0][0]=1;yp[0][0]=1;
    for(int i=1;i<=nx;++i)xp[size_t(i)]=multiply(xp[size_t(i-1)],x);
    for(int i=1;i<=ny;++i)yp[size_t(i)]=multiply(yp[size_t(i-1)],y);
    Series out{};for(auto& t:f){auto z=multiply(xp[size_t(t.x)],yp[size_t(t.y)]);for(size_t i=0;i<precision;++i)out[i]+=t.c*z[i];}
    for(auto& n:out){n=reduce(n);}return out;
}
struct Affine {bool infinity=true;mpz_class x=0,y=0;};
Affine read_point(const Json& j){Affine q;q.infinity=j.at("infinity").scalar=="true";if(!q.infinity){q.x=mpz_class(j.at("x").scalar);q.y=mpz_class(j.at("y").scalar);require(q.x>=0&&q.x<modulus_p&&q.y>=0&&q.y<modulus_p,"canonical coordinates");}return q;}
bool same(const Affine& a,const Affine& b){return a.infinity==b.infinity&&(a.infinity||(a.x==b.x&&a.y==b.y));}
struct SeparateMap {
    Polynomial curve,a,b,v,m;
    SeparateMap(const Json& model,const Json& map):curve(read_polynomial(model.at("terms"))),a(read_polynomial(map.at("X_numerator"))),b(read_polynomial(map.at("X_denominator"))),v(read_polynomial(map.at("Y_numerator"))),m(read_polynomial(map.at("Y_denominator"))){}
    Affine image(const mpz_class& x,const mpz_class& y)const{
        require(evaluate(curve,x,y)==0,"checker quartic equation");auto aa=evaluate(a,x,y),bb=evaluate(b,x,y),vv=evaluate(v,x,y),mm=evaluate(m,x,y);
        if(bb!=0&&mm!=0)return {false,reduce(aa*inverse(bb)),reduce(vv*inverse(mm))};
        auto fx=derivative(curve,0),fy=derivative(curve,1);bool free_x=evaluate(fy,x,y)!=0;
        auto d=evaluate(free_x?fy:fx,x,y);require(d!=0,"checker smooth local chart");
        Series xs{},ys{};xs[0]=x;ys[0]=y;(free_x?xs:ys)[1]=1;mpz_class di=inverse(d);
        for(size_t n=1;n<precision;++n){auto residual=series_evaluate(curve,xs,ys);(free_x?ys:xs)[n]=reduce(-residual[n]*di);}
        for(auto& n:series_evaluate(curve,xs,ys))require(n==0,"checker coefficientwise local lift");
        auto as=series_evaluate(a,xs,ys),bs=series_evaluate(b,xs,ys),vs=series_evaluate(v,xs,ys),ms=series_evaluate(m,xs,ys);
        auto valuation=[](const Series& s){size_t n=0;while(n<precision&&s[n]==0)++n;require(n<precision,"checker unresolved fixed16 valuation");return int(n);};
        int na=valuation(as),nb=valuation(bs),nv=valuation(vs),nm=valuation(ms),ox=na-nb,oy=nv-nm;
        if(ox<0||oy<0){require(ox<0&&oy<0&&3*ox==2*oy,"checker elliptic pole valuations");return {};}
        return {false,ox?mpz_class(0):reduce(as[size_t(na)]*inverse(bs[size_t(nb)])),oy?mpz_class(0):reduce(vs[size_t(nv)]*inverse(ms[size_t(nm)]))};
    }
};
using BN=std::unique_ptr<BIGNUM,decltype(&BN_free)>;
using EP=std::unique_ptr<EC_POINT,decltype(&EC_POINT_free)>;
BN number(const mpz_class& z){BIGNUM* p=nullptr;auto s=z.get_str();require(BN_dec2bn(&p,s.c_str())>0,"OpenSSL decimal parse");return BN(p,BN_free);}
struct OpenSSLGroup {
    std::unique_ptr<BN_CTX,decltype(&BN_CTX_free)> ctx{BN_CTX_new(),BN_CTX_free};
    std::unique_ptr<EC_GROUP,decltype(&EC_GROUP_free)> group{EC_GROUP_new_by_curve_name(NID_X9_62_prime192v1),EC_GROUP_free};
    OpenSSLGroup(){require(bool(ctx)&&bool(group),"OpenSSL P192 initialization");auto p=number(modulus_p),a=number(reduce(-3)),b=number(mpz_class("2455155546008943817740293915197451784769108058161191238065"));
        auto q=BN(BN_new(),BN_free),r=BN(BN_new(),BN_free),s=BN(BN_new(),BN_free);require(bool(q)&&bool(r)&&bool(s),"OpenSSL BN allocation");
        require(EC_GROUP_get_curve(group.get(),q.get(),r.get(),s.get(),ctx.get())==1,"OpenSSL curve parameters");require(BN_cmp(p.get(),q.get())==0&&BN_cmp(a.get(),r.get())==0&&BN_cmp(b.get(),s.get())==0,"exact OpenSSL curve identity");
        auto order=number(order_r);require(EC_GROUP_get_order(group.get(),q.get(),ctx.get())==1&&BN_cmp(q.get(),order.get())==0,"OpenSSL exact subgroup order");}
    EP empty()const{EP p(EC_POINT_new(group.get()),EC_POINT_free);require(bool(p),"OpenSSL point allocation");require(EC_POINT_set_to_infinity(group.get(),p.get())==1,"OpenSSL identity");return p;}
    EP point(const Affine& a)const{auto p=empty();if(!a.infinity){auto x=number(a.x),y=number(a.y);require(EC_POINT_set_affine_coordinates(group.get(),p.get(),x.get(),y.get(),ctx.get())==1,"OpenSSL point coordinates");require(EC_POINT_is_on_curve(group.get(),p.get(),ctx.get())==1,"OpenSSL curve equation");}return p;}
    void add(EP& sum,const EP& p,const mpz_class& coefficient)const{auto n=number(coefficient<0?mpz_class(-coefficient):coefficient);auto term=empty();require(EC_POINT_mul(group.get(),term.get(),nullptr,p.get(),n.get(),ctx.get())==1,"OpenSSL scalar multiplication");if(coefficient<0)require(EC_POINT_invert(group.get(),term.get(),ctx.get())==1,"OpenSSL signed term");require(EC_POINT_add(group.get(),sum.get(),sum.get(),term.get(),ctx.get())==1,"OpenSSL sum");}
    bool zero(const EP& p)const{return EC_POINT_is_at_infinity(group.get(),p.get())==1;}
    void composition(const EP& p,const EP& norm)const{auto six=empty();add(six,p,6);require(EC_POINT_cmp(group.get(),six.get(),norm.get(),ctx.get())==0,"OpenSSL public composition6");}
};
int separate_rank(std::vector<std::vector<mpz_class>> a){
    if(a.empty()){return 0;}size_t row=0;
    for(size_t col=0;col<a[0].size()&&row<a.size();++col){size_t pivot=row;while(pivot<a.size()&&reduce(a[pivot][col],order_r)==0)++pivot;if(pivot==a.size())continue;std::swap(a[row],a[pivot]);mpz_class iv=inverse(reduce(a[row][col],order_r),order_r);
        for(auto& n:a[row])n=reduce(n*iv,order_r);
        for(size_t i=0;i<a.size();++i){if(i!=row){mpz_class c=a[i][col];for(size_t j=col;j<a[i].size();++j)a[i][j]=reduce(a[i][j]-c*a[row][j],order_r);}}++row;}
    return int(row);
}
template<class F> void must_reject(F f){bool rejected=false;try{f();}catch(const std::exception&){rejected=true;}require(rejected,"checker corruption control accepted");}
