#pragma once
#include "riemann_arithmetic.hpp"
#include <functional>
using Line=std::array<T24,3>;
int theta_parity(int a){return __builtin_popcount(unsigned((a&7)&(a>>3)))%2;}
int theta_pair(int a,int b){return (__builtin_popcount(unsigned((a&7)&(b>>3)))+__builtin_popcount(unsigned((b&7)&(a>>3))))%2;}
bool azygetic(int a,int b,int c){return theta_pair(a^b,a^c)==1;}
Line normalize_line(Line a){int i=0;while(i<3&&a[i]==T24(0))++i;require(i<3,"nonzero line");T24 inv=a[i].inv();for(auto& c:a)c=c*inv;return a;}
Matrix3<T24> identity3(){Matrix3<T24> m{};for(int i=0;i<3;++i)m[i][i]=T24(1);return m;}
Matrix3<T24> multiply3(const Matrix3<T24>& a,const Matrix3<T24>& b){Matrix3<T24> m{};for(int i=0;i<3;++i)for(int j=0;j<3;++j)for(int k=0;k<3;++k)m[i][j]=m[i][j]+a[i][k]*b[k][j];return m;}
Matrix3<T24> frobenius3(Matrix3<T24> a){for(auto& row:a)for(auto& c:row)c=c.power(prime);return a;}
Line row_times(const Line& a,const Matrix3<T24>& b){Line r{};for(int j=0;j<3;++j)for(int k=0;k<3;++k)r[j]=r[j]+a[k]*b[k][j];return r;}
Form normalized_form(const Form& f){require(!f.c.empty(),"nonzero form");return f.scale(f.c.begin()->second.inv());}
Form transform_form(const Form& f,const Matrix3<T24>& b){
    std::array<Form,3> coordinates{linear(b[0]),linear(b[1]),linear(b[2])};Form out;
    for(auto& [e,v]:f.c){Form term;term.put({0,0,0},v);for(int j=0;j<3;++j)for(int k=0;k<e[j];++k)term=term*coordinates[j];out=out+term;}return out;
}
Form frobenius_form(const Form& f){Form out;for(auto& [e,v]:f.c)out.put(e,v.power(prime));return out;}
Form read_form(const Json& j){Form f;for(auto& r:j.arr){Exponent e{};for(int i=0;i<3;++i)e[i]=r.at("exponents").arr.at(i).integer();f.put(e,read_t24(r.at("coefficient")));}return f;}
Line read_line(const Json& j){require(j.arr.size()==3,"line shape");Line r;for(int i=0;i<3;++i)r[i]=read_t24(j.arr[i]);return r;}
Line frame_coordinates(const Line& line,const std::array<Line,3>& frame){Matrix3<T24> matrix{};for(int j=0;j<3;++j)for(int i=0;i<3;++i)matrix[j][i]=frame[i][j];return solve3(matrix,line);}

struct DerivativeProducts {
    const std::vector<T24>& theta;
    std::map<std::array<int,3>,T24> cache;
    explicit DerivativeProducts(const std::vector<T24>& w):theta(w){}
    T24 product(int a,int b,int c){
        std::array<int,3> key{a,b,c};std::sort(key.begin(),key.end());auto hit=cache.find(key);if(hit!=cache.end())return hit->second;
        require(theta_parity(a)&&theta_parity(b)&&theta_parity(c)&&azygetic(a,b,c),"Jacobi derivative azygetic triple");
        std::vector<int> system{a,b,c},chosen,solution;int count=0;
        std::function<void(int)> visit=[&](int start){
            if(chosen.size()==5){solution=chosen;++count;return;}
            for(int d=start;d<64;++d)if(!theta_parity(d)){
                bool valid=true;
                for(size_t i=0;i<system.size()&&valid;++i)for(size_t j=i+1;j<system.size();++j)if(!azygetic(system[i],system[j],d)){valid=false;break;}
                if(valid){system.push_back(d);chosen.push_back(d);visit(d+1);chosen.pop_back();system.pop_back();}
            }
        };
        visit(0);require(count==1,"unique five-even fundamental completion");T24 value(1);
        for(int d:solution){require(theta[d]!=T24(0),"nonzero even Jacobi factor");value=value*theta[d];}
        cache.emplace(key,value);return value;
    }
};

std::array<T24,3> squared_coordinates(int label,const std::array<int,4>& frame,
                                    const std::array<int,7>& labels,const std::array<Line,7>& lines,
                                    DerivativeProducts& products){
    std::array<Line,3> base{lines[frame[0]],lines[frame[1]],lines[frame[2]]};auto d=frame_coordinates(lines[frame[3]],base);
    std::array<T24,3> out;
    for(int k=0;k<3;++k){int a=(k+1)%3,b=(k+2)%3;
        T24 numerator=products.product(label,labels[frame[a]],labels[frame[b]]);
        T24 denominator=products.product(labels[frame[3]],labels[frame[a]],labels[frame[b]]);
        require(d[k]!=T24(0),"Aronhold projective frame");out[k]=d[k]*d[k]*numerator/denominator;
    }
    return out;
}
void check_squared_coordinates(const Line& line,const std::array<int,4>& frame,
                               const std::array<Line,7>& lines,const Line& squared){
    std::array<Line,3> base{lines[frame[0]],lines[frame[1]],lines[frame[2]]};auto c=frame_coordinates(line,base);
    for(int i=0;i<3;++i)for(int j=0;j<3;++j)require(c[i]*c[i]*squared[j]==c[j]*c[j]*squared[i],"labelled Jacobi determinant ratios");
}

F norm_preimage8(const F& target){
    Z a=scalar(target);mpz_class m=(prime-1)/2,d;mpz_class eight=8;require(mpz_invert(d.get_mpz_t(),eight.get_mpz_t(),m.get_mpz_t())!=0,"odd part inverse8");
    mpz_class sign;mpz_powm(sign.get_mpz_t(),a.x.get_mpz_t(),m.get_mpz_t(),prime.get_mpz_t());Z odd=a*Z(sign);mpz_class value;
    mpz_powm(value.get_mpz_t(),odd.x.get_mpz_t(),d.get_mpz_t(),prime.get_mpz_t());F mu(Poly<Z>{Z(value)});
    mpz_class q;mpz_pow_ui(q.get_mpz_t(),prime.get_mpz_t(),8);mpz_class exponent=(q-1)/(prime-1);
    if(sign!=1){
        require(sign==prime-1,"target norm sign");mpz_class odd_part=q-1;while(mpz_even_p(odd_part.get_mpz_t()))odd_part>>=1;
        F root;bool found=false;
        for(int shift=0;shift<16&&!found;++shift){F x(Poly<Z>{Z(shift),Z(1)});root=x.power(odd_part);found=root.power(exponent)==F(-1);}
        require(found,"norm sign preimage bound");mu=mu*root;
    }
    require(mu.power(exponent)==target,"exact Fp8 norm preimage");return mu;
}
void emit_matrix(const Matrix3<T24>& m){std::cout<<'[';for(int i=0;i<3;++i){if(i)std::cout<<',';std::cout<<'[';for(int j=0;j<3;++j){if(j)std::cout<<',';emit_t24(m[i][j]);}std::cout<<']';}std::cout<<']';}

