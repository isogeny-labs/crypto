#include "destination_relations_arithmetic.hpp"
#include <openssl/sha.h>
constexpr int evaluation_count=85;
using Vector=std::vector<Z>;
using Space=std::vector<Vector>;
using Monomial=std::array<int,2>;

Space row_basis(Space rows,int width){
    std::map<int,Vector> pivots;
    for(auto row:rows){
        require(int(row.size())==width,"linear-space width");
        for(auto& [j,pivot]:pivots){Z c=row[j];if(c!=Z(0))for(int k=j;k<width;++k)row[k]=row[k]-c*pivot[k];}
        int j=0;while(j<width&&row[j]==Z(0))++j;
        if(j<width){Z inv=row[j].inv();for(int k=j;k<width;++k)row[k]=row[k]*inv;pivots.emplace(j,row);}
    }
    for(auto it=pivots.rbegin();it!=pivots.rend();++it)for(auto jt=pivots.begin();jt->first<it->first;++jt){Z c=jt->second[it->first];if(c!=Z(0))for(int k=it->first;k<width;++k)jt->second[k]=jt->second[k]-c*it->second[k];}
    Space out;for(auto& [j,row]:pivots)out.push_back(row);return out;
}
Space nullspace(const Space& matrix,int width){
    Space rows=row_basis(matrix,width);std::vector<int> pivots;
    for(auto& row:rows){int j=0;while(j<width&&row[j]==Z(0))++j;require(j<width,"nonzero pivot row");pivots.push_back(j);}
    Space out;
    for(int free=0;free<width;++free)if(std::find(pivots.begin(),pivots.end(),free)==pivots.end()){
        Vector v(size_t(width),Z(0));v[free]=Z(1);for(size_t i=0;i<pivots.size();++i)v[pivots[i]]=-rows[i][free];out.push_back(v);
    }
    return out;
}
Vector linear_combination(const Vector& coeff,const Space& basis){
    require(coeff.size()==basis.size(),"linear-combination shape");Vector out(evaluation_count,Z(0));
    for(size_t i=0;i<coeff.size();++i)if(coeff[i]!=Z(0))for(int j=0;j<evaluation_count;++j)out[j]=out[j]+coeff[i]*basis[i][j];
    return out;
}
Vector hadamard(const Vector& a,const Vector& b){require(a.size()==b.size(),"pointwise product shape");Vector out;out.reserve(a.size());for(size_t i=0;i<a.size();++i)out.push_back(a[i]*b[i]);return out;}
Vector residual(Vector v,const Space& basis){
    for(auto& row:basis){int j=0;while(j<int(row.size())&&row[j]==Z(0))++j;require(j<int(row.size()),"nonzero residual pivot");Z c=v[j];if(c!=Z(0))for(int k=j;k<int(row.size());++k)v[k]=v[k]-c*row[k];}return v;
}
bool all_zero(const Vector& v){for(auto& c:v)if(c!=Z(0))return false;return true;}
Space preimage(const Space& candidate,const Space& multiplier,const Space& target){
    Space canonical=row_basis(target,evaluation_count),constraints;
    for(auto& e:multiplier){Space remainder;for(auto& v:candidate)remainder.push_back(residual(hadamard(v,e),canonical));
        for(int j=0;j<evaluation_count;++j){Vector row;for(auto& v:remainder)row.push_back(v[j]);constraints.push_back(row);}
    }
    Space coeff=nullspace(row_basis(constraints,int(candidate.size())),int(candidate.size())),out;
    for(auto& c:coeff)out.push_back(linear_combination(c,candidate));
    return row_basis(out,evaluation_count);
}
Space intersection(const Space& a,const Space& b){
    Space equations;for(int j=0;j<evaluation_count;++j){Vector row;for(auto& v:a)row.push_back(v[j]);for(auto& v:b)row.push_back(-v[j]);equations.push_back(row);}
    Space coeff=nullspace(equations,int(a.size()+b.size())),out;
    for(auto& c:coeff){Vector left(c.begin(),c.begin()+a.size());out.push_back(linear_combination(left,a));}return row_basis(out,evaluation_count);
}
std::vector<Monomial> monomial_basis(int degree){std::vector<Monomial> out;for(int i=0;i<=3;++i)for(int j=0;j+i<=degree;++j)out.push_back({i,j});return out;}
using Jet3=std::array<Z,3>;
Jet3 jet_product(const Jet3& a,const Jet3& b){Jet3 out{};for(int i=0;i<3;++i)for(int j=0;i+j<3;++j)out[i+j]=out[i+j]+a[i]*b[j];return out;}
struct Class {Space w;bool operator==(const Class& b)const{return w==b.w;}};

struct QuarticKM {
    std::map<std::array<int,3>,Z> terms;
    std::vector<BasePoint> points;
    std::vector<Fiber> fibers;
    std::vector<int> pool;
    std::vector<Monomial> mons;
    Space v1,v2,v3,v4;
    Class identity;
    int flips=0;
    explicit QuarticKM(const Json& model,const Json& receipt){
        prime=mpz_class(model.at("p").scalar);require(prime==mpz_class("6277101735386680763835789423207666416083908700390324961279"),"P192 target field");
        for(auto& row:model.at("terms").arr){std::array<int,3> e{};for(int j=0;j<3;++j)e[j]=row.at("exponents").arr.at(j).integer();terms.emplace(e,Z(mpz_class(row.at("coefficient").scalar)));}
        require(terms.at({4,0,0})!=Z(0),"declared x-leading monomial chart");
        for(auto& p:receipt.at("points").arr){BasePoint point{p.at("x").integer(),Z(mpz_class(p.at("y").scalar))};require(derivative_at(point,0,0)==Z(0),"target pool equation");points.push_back(point);}
        for(auto& f:receipt.at("split_fibers").arr){Fiber fiber{f.at("x").integer(),{}};for(auto& i:f.at("point_indices").arr)fiber.support.push_back(i.integer());fibers.push_back(fiber);}
        require(fibers.size()>=2&&fibers[0].support.size()==4&&fibers[1].support.size()==4,"split support input");
        int p=fibers[1].support[0],q=fibers[1].support[1];
        for(int i=0;i<int(points.size())&&pool.size()<evaluation_count;++i)if(points[i].x!=fibers[0].x&&i!=p&&i!=q)pool.push_back(i);
        require(pool.size()==evaluation_count,"85 disjoint evaluation points");
        v1=row_basis(evaluate_basis(1),evaluation_count);v2=row_basis(evaluate_basis(2),evaluation_count);
        v3=evaluate_basis(3);v4=row_basis(evaluate_basis(4),evaluation_count);mons=monomial_basis(9);
        require(v1.size()==10&&v2.size()==22&&row_basis(v3,evaluation_count).size()==34&&v4.size()==46,"target Riemann-Roch dimensions");identity={v2};
    }
    Z derivative_at(const BasePoint& p,int dx,int dy)const{
        Z value;for(auto& [e,c]:terms)if(e[0]>=dx&&e[1]>=dy){long factor=1;for(int j=0;j<dx;++j)factor*=e[0]-j;for(int j=0;j<dy;++j)factor*=e[1]-j;value=value+c*Z(factor)*zpow(Z(p.x),e[0]-dx)*zpow(p.y,e[1]-dy);}return value;
    }
    Space evaluate_basis(int n)const{
        Space out;for(auto m:monomial_basis(3*n)){Vector row;for(int i:pool){const auto& p=points[i];Z pole=Z(p.x-fibers[0].x);require(pole!=Z(0),"evaluation point at pole");row.push_back(zpow(Z(p.x),m[0])*zpow(p.y,m[1])/zpow(pole,3*n));}out.push_back(row);}return out;
    }
    std::vector<Jet3> monomial_jets(const BasePoint& p)const{
        require(derivative_at(p,0,0)==Z(0),"jet point on actual curve");Z fy=derivative_at(p,0,1);require(fy!=Z(0),"declared unramified local x chart");
        Z first=-derivative_at(p,1,0)/fy,second=-(derivative_at(p,2,0)+Z(2)*derivative_at(p,1,1)*first+derivative_at(p,0,2)*first*first)/(Z(2)*fy);
        Jet3 x{Z(p.x),Z(1),Z(0)},y{p.y,first,second};std::vector<Jet3> out;
        for(auto m:mons){Jet3 value{Z(1),Z(0),Z(0)};for(int j=0;j<m[0];++j)value=jet_product(value,x);for(int j=0;j<m[1];++j)value=jet_product(value,y);out.push_back(value);}return out;
    }
    std::map<int,int> base_divisor()const{std::map<int,int> d;for(int i:fibers[0].support)d[i]=3;return d;}
    Class effective(const std::map<int,int>& d)const{
        int degree=0;Space constraints;
        for(auto& [point,multiplicity]:d){require(multiplicity>=0&&multiplicity<=3,"effective support multiplicity bound");degree+=multiplicity;if(!multiplicity)continue;auto jets=monomial_jets(points.at(size_t(point)));
            for(int k=0;k<multiplicity;++k){Vector row;for(auto& j:jets)row.push_back(j[k]);constraints.push_back(row);}
        }
        require(degree==12,"effective representative degree12");Space coeff=nullspace(constraints,int(mons.size())),out;require(coeff.size()==22,"effective target class RR dimension");for(auto& c:coeff)out.push_back(linear_combination(c,v3));return {row_basis(out,evaluation_count)};
    }
    Class point_class(int point)const{if(point==fibers[0].support[0])return identity;auto d=base_divisor();--d[fibers[0].support[0]];++d[point];return effective(d);}
    Class flip(const Class& a,const Class& b){
        ++flips;Space products;for(auto& u:a.w)for(auto& v:b.w)products.push_back(hadamard(u,v));auto w1=row_basis(products,evaluation_count);require(w1.size()==46,"KM product space dimension");
        auto w2=preimage(v3,v3,w1);require(w2.size()>=10,"KM variable intermediate dimension");auto w3=preimage(v4,v2,w1);require(w3.size()==22,"KM divisor preimage dimension");
        Space fourth;for(auto& v:v4)fourth.push_back(hadamard(w2[0],v));auto w4=row_basis(fourth,evaluation_count);require(w4.size()==46,"KM multiplication injectivity");
        auto w5=preimage(v3,w3,w4);require(w5.size()==22,"KM addflip output dimension");std::cerr<<"completed target addflip "<<flips<<'\n';return {w5};
    }
    Class negate(const Class& a){return a==identity?a:flip(a,identity);}
    Class add(const Class& a,const Class& b){if(a==identity)return b;if(b==identity)return a;return negate(flip(a,b));}
    bool equal(const Class& a,const Class& b)const{
        if(a==b)return true;
        Space image;
        for(auto& row:b.w)image.push_back(hadamard(a.w[0],row));
        auto target=row_basis(image,evaluation_count);
        require(target.size()==22,"nonzero-section multiplication injectivity");
        return !preimage(v3,a.w,target).empty();
    }
    bool zero(const Class& a)const{return equal(a,identity);}
};

int main(int argc,char**argv){try{
    require(argc==3,"usage: quartic_km model.json point-receipt.json");QuarticKM km(read_json(argv[1]),read_json(argv[2]));int p=km.fibers[1].support[0],q=km.fibers[1].support[1],r0=km.fibers[0].support[0];
    auto a=km.point_class(p),b=km.point_class(q);require(!km.zero(a)&&!km.zero(b)&&km.zero(km.identity),"target point/zero classes");
    auto direct_divisor=km.base_divisor();direct_divisor[r0]-=2;++direct_divisor[p];++direct_divisor[q];auto direct=km.effective(direct_divisor);
    auto sum=km.add(a,b);require(km.equal(sum,direct),"target addition equals direct effective-divisor class");
    auto negative=km.negate(a);require(km.zero(km.add(a,negative)),"target cancellation");
    auto principal=km.base_divisor();for(int i:km.fibers[0].support)--principal[i];for(int i:km.fibers[1].support)++principal[i];auto relation=km.effective(principal);require(km.zero(relation),"actual vertical relation is zero in target Jacobian");
    Vector function;for(int i:km.pool)function.push_back(Z(km.points[i].x-km.fibers[1].x)/Z(km.points[i].x-km.fibers[0].x));
    require(all_zero(residual(function,relation.w))&&all_zero(residual(function,km.v1)),"explicit principal function in both RR spaces");
    rejects([&]{auto point=km.points[p];point.y=point.y+Z(1);km.monomial_jets(point);},"corrupted target curve point");
    rejects([&]{auto bad=function;bad[0]=bad[0]+Z(1);require(all_zero(residual(bad,relation.w)),"corrupt principal function evaluation");},"corrupt principal function evaluation");
    std::cout<<"{\"scope\":\"P192-kernel1-candidate-target-KM\",\"evaluation_points\":85,\"base_divisor_degree\":12,\"RR_dimensions\":[10,22,34,46],\"point_classes\":2,\"addition_vs_direct_effective_divisor\":true,\"cancellation\":true,\"principal_relation_zero\":true,\"explicit_function_membership\":true,\"addflip_calls\":"<<km.flips<<",\"rejected_controls\":2,\"source_image_bound\":false,\"Serre_twist_checked\":false,\"standing\":\"producer-unverified\",\"test_point_indices\":["<<p<<','<<q<<"],\"reference_point_index\":"<<r0<<",\"evaluation_point_indices\":[";
    for(size_t i=0;i<km.pool.size();++i)std::cout<<(i?",":"")<<km.pool[i];
    std::cout<<"]}\n";
}catch(const std::exception&e){std::cerr<<"REJECT "<<e.what()<<'\n';return 1;}}
