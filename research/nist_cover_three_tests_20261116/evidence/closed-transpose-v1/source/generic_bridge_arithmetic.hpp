#pragma once
#include "public_classes_arithmetic.hpp"
struct GBSource {
    inline static CFPoly modulus;
    CFPoly a;
    GBSource(long n=0):a{Z(n)}{}
    explicit GBSource(CFPoly p):a(p%modulus){}
    GBSource operator+(const GBSource& b)const{return GBSource(a+b.a);}
    GBSource operator-()const{return GBSource(-a);}
    GBSource operator-(const GBSource& b)const{return GBSource(a-b.a);}
    GBSource operator*(const GBSource& b)const{return GBSource((a*b.a)%modulus);}
    GBSource inv()const{auto [g,s,t]=xgcd(a,modulus);require(g.degree()==0,"global evaluation source denominator unit");GBSource r(s.scale(g.at(0).inv()));require(*this*r==GBSource(1),"global evaluation source inverse identity");return r;}
    GBSource operator/(const GBSource& b)const{return *this*b.inv();}
    bool operator==(const GBSource& b)const{return a==b.a;}
    bool operator!=(const GBSource& b)const{return !(*this==b);}
};
using GBP=Poly<GBSource>;
GBSource gb_eval_poly(const CFPoly& p,const GBSource& x){GBSource r;for(int i=p.degree();i>=0;--i)r=r*x+GBSource(CFPoly{p.at(i)});return r;}
GBSource gb_field(const Json& row,const GBSource& x,const GBSource& y){auto a=gb_eval_poly(pc_read_poly(row.at("a")),x),b=gb_eval_poly(pc_read_poly(row.at("b")),x),c=gb_eval_poly(pc_read_poly(row.at("c")),x);return (a+b*y)/c;}
Vector gb_flat(const GBP& q,int d){Vector r(size_t(3*d),Z(0));for(int j=0;j<3;++j)for(int i=0;i<d;++i)r[size_t(j*d+i)]=q.at(j).a.at(i);return r;}
Vector gb_solve(const std::vector<Vector>& columns,const Vector& target){int n=int(target.size());require(int(columns.size())==n,"global image square basis");Space rows(size_t(n),Vector(size_t(n+1),Z(0)));for(int i=0;i<n;++i){for(int j=0;j<n;++j)rows[i][j]=columns[j][i];rows[i][n]=target[i];}
    for(int j=0;j<n;++j){int p=j;while(p<n&&rows[p][j]==Z(0))++p;require(p<n,"global image full power basis");std::swap(rows[p],rows[j]);Z inv=rows[j][j].inv();for(int k=j;k<=n;++k)rows[j][k]=rows[j][k]*inv;for(int i=0;i<n;++i)if(i!=j){Z c=rows[i][j];for(int k=j;k<=n;++k)rows[i][k]=rows[i][k]-c*rows[j][k];}}
    Vector out;for(int i=0;i<n;++i)out.push_back(rows[i][n]);Vector replay(size_t(n),Z(0));for(int j=0;j<n;++j)for(int i=0;i<n;++i)replay[i]=replay[i]+out[j]*columns[j][i];require(replay==target,"global power basis solve replay");return out;
}
struct GBImage {CFPoly source_u,source_v,u,v;int degree=0;};
GBImage gb_image(const G& source,const Json& global,const EBPoly& curve_c){source.check();GBImage r;r.source_u=rational(source.u);r.source_v=rational(source.v);r.degree=r.source_u.degree();require(r.degree>=1&&r.degree<=3,"fixed positive source divisor degree");auto source_h=rational(curve);require(((r.source_v*r.source_v-source_h)%r.source_u).degree()<0,"global evaluator source equation");GBSource::modulus=r.source_u;GBSource x(CFPoly{Z(0),Z(1)}),y(r.source_v);std::vector<GBSource> uc,vc;for(auto& a:global.at("U").arr)uc.push_back(gb_field(a,x,y));for(auto& a:global.at("V").arr)vc.push_back(gb_field(a,x,y));GBP U(uc),V(vc),T{GBSource(0),GBSource(1)};require(U.degree()==3&&U.at(3)==GBSource(1),"global evaluator monic cubic");int n=3*r.degree;GBP power{GBSource(1)};std::vector<Vector> basis;for(int k=0;k<n;++k){basis.push_back(gb_flat(power,r.degree));power=(power*T)%U;}auto relation=gb_solve(basis,gb_flat(power,r.degree)),values=gb_solve(basis,gb_flat(V,r.degree));for(auto& c:relation)c=-c;relation.push_back(Z(1));r.u=CFPoly(relation);r.v=CFPoly(values);
    auto eval=[&](const CFPoly& p){GBP z;for(int i=p.degree();i>=0;--i)z=(z*T+GBP{GBSource(CFPoly{p.at(i)})})%U;return z;};require(eval(r.u).degree()<0&&eval(r.v)==V,"global closed image power reconstruction");require(cf_evaluate(curve_c,r.u,r.v).degree()<0,"global closed image quartic equation");require(cf_evaluate(curve_c,r.u,r.v+CFPoly{Z(1)}).degree()>=0,"altered global closed image y rejects");return r;
}
Class gb_class(const Json& row,QuarticKM& km){Class c;for(auto& a:row.arr){Vector v;for(auto& z:a.arr)v.emplace_back(mpz_class(z.scalar));require(v.size()==evaluation_count,"public class85 coordinates");c.w.push_back(v);}require(c.w.size()==22&&row_basis(c.w,evaluation_count).size()==22,"public class dimension22");auto ambient=row_basis(km.v3,evaluation_count);for(auto& v:c.w)require(all_zero(residual(v,ambient)),"public class section membership");return c;}
struct GBAdapter {
    QuarticKM& km;std::vector<int> reference,auxiliary;
    GBAdapter(QuarticKM& k,const Json& formal,const std::vector<GBImage>& images):km(k){for(auto& i:formal.at("seed_indices").arr)reference.push_back(i.integer());require(reference.size()==3,"global reference triple");std::set<int> excluded(km.pool.begin(),km.pool.end());excluded.insert(reference.begin(),reference.end());for(int i=int(km.points.size())-1;i>=0&&auxiliary.size()<3;--i){bool good=!excluded.count(i)&&km.points[i].x!=km.fibers[0].x;for(auto& d:images)good&=d.u.eval(Z(km.points[i].x))!=Z(0);if(good)auxiliary.push_back(i);}require(auxiliary.size()==3,"global map common auxiliary3");}
    Class baseline()const{std::map<int,int> d;for(int i:reference)d[i]=3;for(int i:auxiliary)++d[i];return km.effective(d);}
    Class encode(const GBImage& image)const{int n=image.u.degree();require(n==3*image.degree,"global image degree3d");Space constraints(size_t(n),Vector(km.mons.size(),Z(0)));CFPoly x{Z(0),Z(1)};for(size_t j=0;j<km.mons.size();++j){auto e=km.mons[j];auto q=(pc_power_mod(x,e[0],image.u)*pc_power_mod(image.v,e[1],image.u))%image.u;for(int i=0;i<n;++i)constraints[i][j]=q.at(i);}for(int i:reference){require(image.u.eval(Z(km.points[i].x))!=Z(0),"global image disjoint reference chart");auto jets=km.monomial_jets(km.points[i]);for(int m=0;m<3-image.degree;++m){Vector row;for(auto& j:jets)row.push_back(j[m]);constraints.push_back(row);}}for(int i:auxiliary){Vector row;for(auto e:km.mons)row.push_back(zpow(Z(km.points[i].x),e[0])*zpow(km.points[i].y,e[1]));constraints.push_back(row);}require(constraints.size()==12,"global degree12 constraint count");auto coeff=nullspace(constraints,int(km.mons.size()));require(coeff.size()==22,"global class coefficient dimension22");Space values;for(auto& c:coeff)values.push_back(linear_combination(c,km.v3));auto out=row_basis(values,evaluation_count);require(out.size()==22,"global class evaluation dimension22");return {out};}
};
