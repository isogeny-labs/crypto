#pragma once
#include "quotient_theta2_arithmetic.hpp"

T24 read_t24(const Json& row){require(row.arr.size()==3,"tower value shape");std::vector<F> a;for(auto& r:row.arr)a.push_back(field(r));return T24(P(a));}
std::array<T24,3> solve3(Matrix3<T24> a,const std::array<T24,3>& b){
    T24 denominator=determinant3(a);require(denominator!=T24(0),"Riemann linear system nonsingular");std::array<T24,3> out;
    for(int j=0;j<3;++j){auto m=a;for(int i=0;i<3;++i)m[i][j]=b[i];out[j]=determinant3(m)/denominator;}return out;
}
using Exponent=std::array<int,3>;
struct Form {
    std::map<Exponent,T24> c;
    void put(Exponent e,const T24& value){c[e]=c[e]+value;if(c[e]==T24(0))c.erase(e);}
    Form operator+(const Form& b)const{Form r=*this;for(auto& [e,v]:b.c)r.put(e,v);return r;}
    Form operator-()const{Form r;for(auto& [e,v]:c)r.put(e,-v);return r;}
    Form operator-(const Form& b)const{return *this+-b;}
    Form operator*(const Form& b)const{Form r;for(auto& [e,v]:c)for(auto& [f,w]:b.c)r.put({e[0]+f[0],e[1]+f[1],e[2]+f[2]},v*w);return r;}
    Form scale(const T24& a)const{Form r;for(auto& [e,v]:c)r.put(e,v*a);return r;}
    Form partial(int i)const{Form r;for(auto& [e,v]:c)if(e[i]){auto f=e;--f[i];r.put(f,v*T24(e[i]));}return r;}
    T24 at(const Exponent& e)const{auto i=c.find(e);return i==c.end()?T24(0):i->second;}
    T24 eval(const std::array<T24,3>& p)const{
        T24 sum;for(auto& [e,v]:c){T24 w=v;for(int i=0;i<3;++i)for(int j=0;j<e[i];++j)w=w*p[i];sum=sum+w;}return sum;
    }
};
Form linear(const std::array<T24,3>& a){Form f;for(int i=0;i<3;++i){Exponent e{};e[i]=1;f.put(e,a[i]);}return f;}
std::vector<Exponent> monomials(int d){std::vector<Exponent> out;for(int i=0;i<=d;++i)for(int j=0;i+j<=d;++j)out.push_back({i,j,d-i-j});return out;}
int jacobian_rank7(const Form& f){
    auto cols=monomials(7),mult=monomials(4);std::map<Exponent,int> col;
    for(int i=0;i<int(cols.size());++i)col.emplace(cols[i],i);
    std::vector<std::vector<T24>> a;
    for(int i=0;i<3;++i){auto partial=f.partial(i);for(auto& m:mult){std::vector<T24> row(cols.size());for(auto& [e,v]:partial.c)row[col.at({e[0]+m[0],e[1]+m[1],e[2]+m[2]})]=v;a.push_back(row);}}
    int rank=0;
    for(int c=0;c<int(cols.size());++c){int pick=rank;while(pick<int(a.size())&&a[pick][c]==T24(0))++pick;if(pick==int(a.size()))continue;
        std::swap(a[pick],a[rank]);T24 inv=a[rank][c].inv();for(int j=c;j<int(cols.size());++j)a[rank][j]=a[rank][j]*inv;
        for(int i=rank+1;i<int(a.size());++i){T24 factor=a[i][c];for(int j=c;j<int(cols.size());++j)a[i][j]=a[i][j]-factor*a[rank][j];}++rank;
    }
    return rank;
}
void check_bitangent(const Form& f,const std::array<T24,3>& line){
    int k=0;while(k<3&&line[k]==T24(0))++k;require(k<3,"nonzero bitangent");int i=(k+1)%3,j=(k+2)%3;
    std::array<T24,3> p{},q{};p[i]=T24(1);p[k]=-line[i]/line[k];q[j]=T24(1);q[k]=-line[j]/line[k];
    auto first=p;bool found=false;
    for(int shift=0;shift<5&&!found;++shift){for(int a=0;a<3;++a)p[a]=first[a]+T24(shift)*q[a];found=f.eval(p)!=T24(0);}
    require(found,"bitangent not a component; leading coefficient bound");
    std::array<Poly<T24>,3> coordinates;for(int a=0;a<3;++a)coordinates[a]=Poly<T24>{q[a],p[a]};Poly<T24> restriction;
    for(auto& [e,v]:f.c){Poly<T24> term{v};for(int a=0;a<3;++a)for(int b=0;b<e[a];++b)term=term*coordinates[a];restriction=restriction+term;}
    require(restriction.degree()==4,"bitangent restriction degree");T24 leading=restriction.at(4),b=restriction.at(3)/(T24(2)*leading),c=(restriction.at(2)/leading-b*b)/T24(2);
    Poly<T24> square{c,b,T24(1)};require(restriction==(square*square).scale(leading),"exact bitangent square restriction");
}

