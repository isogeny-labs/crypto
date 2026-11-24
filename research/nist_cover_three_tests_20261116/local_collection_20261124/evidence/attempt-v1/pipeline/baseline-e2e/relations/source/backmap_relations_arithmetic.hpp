#pragma once
#include "elliptic_backmap_arithmetic.hpp"
EBPoly br_read_poly(const Json& row){EBPoly p;for(auto& term:row.arr){const auto& e=term.at("exponents");require(e.arr.size()==2,"backmap point polynomial dimension");p.add({e.arr[0].integer(),e.arr[1].integer()},Z(mpz_class(term.at("coefficient").scalar)));}return p;}
void br_emit_point(const EC& p){std::cout<<"{\"infinity\":"<<(p.zero?"true":"false");if(!p.zero)std::cout<<",\"x\":\""<<scalar(p.x).x<<"\",\"y\":\""<<scalar(p.y).x<<'"';std::cout<<'}';}
struct BRImage {EC point;bool local=false;std::array<int,4> orders{};};
struct BRMap {
    EBPoly curve,fx,fy,A,B,V,M;
    explicit BRMap(const Json& model,const Json& map){for(auto& t:model.at("terms").arr){const auto& e=t.at("exponents");curve.add({e.arr[0].integer(),e.arr[1].integer()},Z(mpz_class(t.at("coefficient").scalar)));}fx=curve.partial(0);fy=curve.partial(1);A=br_read_poly(map.at("X_numerator"));B=br_read_poly(map.at("X_denominator"));V=br_read_poly(map.at("Y_numerator"));M=br_read_poly(map.at("Y_denominator"));require(map.at("curve_map_degree").integer()==6,"degree6 backmap input");}
    BRImage image(const Z& xr,const Z& yr)const{
        require(curve.eval(xr,yr)==Z(0),"point norm input on actual quartic");Z av=A.eval(xr,yr),bv=B.eval(xr,yr),vv=V.eval(xr,yr),mv=M.eval(xr,yr);BRImage out;
        if(bv!=Z(0)&&mv!=Z(0)){out.point=EC(F(Poly<Z>{av/bv}),F(Poly<Z>{vv/mv}));return out;}
        out.local=true;EBSeries x(xr),y(yr);if(fy.eval(xr,yr)!=Z(0)){x.c[1]=Z(1);for(int n=1;n<EB_PRECISION;n*=2)y=y-curve.eval(x,y)/fy.eval(x,y);}else{require(fx.eval(xr,yr)!=Z(0),"smooth local backmap chart");y.c[1]=Z(1);for(int n=1;n<EB_PRECISION;n*=2)x=x-curve.eval(x,y)/fx.eval(x,y);}
        auto curve_series=curve.eval(x,y);for(auto& c:curve_series.c){require(c==Z(0),"local norm curve equation");}
        auto aa=A.eval(x,y),bb=B.eval(x,y),v=V.eval(x,y),m=M.eval(x,y);out.orders={aa.order(),bb.order(),v.order(),m.order()};for(int n:out.orders)require(n<EB_PRECISION,"resolved local norm valuation");int ox=out.orders[0]-out.orders[1],oy=out.orders[2]-out.orders[3];
        if(ox<0||oy<0){require(ox<0&&oy<0&&3*ox==2*oy,"elliptic pole2-to3 valuation ratio");out.point=EC{};return out;}
        Z xx=ox?Z(0):aa.c[out.orders[0]]/bb.c[out.orders[1]],yy=oy?Z(0):v.c[out.orders[2]]/m.c[out.orders[3]];out.point=EC(F(Poly<Z>{xx}),F(Poly<Z>{yy}));return out;
    }
};
int br_rank(std::vector<std::vector<mpz_class>> rows,const mpz_class& r){if(rows.empty())return 0;int rank=0,width=int(rows[0].size());for(int j=0;j<width&&rank<int(rows.size());++j){int pivot=rank;while(pivot<int(rows.size())&&rows[pivot][j]==0)++pivot;if(pivot==int(rows.size()))continue;std::swap(rows[pivot],rows[rank]);mpz_class inverse;require(mpz_invert(inverse.get_mpz_t(),rows[rank][j].get_mpz_t(),r.get_mpz_t())!=0,"projected relation pivot inverse");for(int k=j;k<width;++k){rows[rank][k]=rows[rank][k]*inverse%r;if(rows[rank][k]<0)rows[rank][k]+=r;}for(int i=rank+1;i<int(rows.size());++i){mpz_class a=rows[i][j];for(int k=j;k<width;++k){rows[i][k]=(rows[i][k]-a*rows[rank][k])%r;if(rows[i][k]<0)rows[i][k]+=r;}}++rank;}return rank;}
