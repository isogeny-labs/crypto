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

int main(int argc,char** argv){try{
    require(argc==2,"usage: riemann_quartic study-directory");std::string root=argv[1];
    initialize_cover(read_json(root+"/inputs/p192-d7-prym.json"));Level2Input input;
    auto source=read_json(root+"/evidence/quotient-theta2-v1/result.json");
    require(source.at("odd_theta_zeros").integer()==28&&source.at("even_theta_zeros").integer()==0,"nonhyperelliptic moduli precondition");
    std::vector<T24> w;for(auto& v:source.at("theta_squares").arr)w.push_back(read_t24(v));require(w.size()==64,"theta-square vector size");
    std::array<std::array<int,6>,3> ratios{{{{5,12,33,40,21,28}},{{5,27,40,54,2,28}},{{12,27,33,54,2,21}}}};
    std::array<std::array<int,2>,3> den_b{{{{49,56}},{{47,49}},{{47,56}}}},num_c{{{{7,14}},{{14,16}},{{7,16}}}},den_c{{{{35,42}},{{35,61}},{{42,61}}}};
    Matrix3<T24> alpha{};
    for(int i=0;i<3;++i){auto r=ratios[i];T24 da=w[r[2]]*w[r[3]],db=w[den_b[i][0]]*w[den_b[i][1]],dc=w[den_c[i][0]]*w[den_c[i][1]];
        T24 a2=w[r[0]]*w[r[1]]/da,b2=w[r[4]]*w[r[5]]/db,c2=w[num_c[i][0]]*w[num_c[i][1]]/dc;
        T24 p2=a2*da*da,q2=b2*db*db,r2=c2*dc*dc;
        T24 pq=(p2+q2-r2)/T24(2),pr=(i==0?(p2+r2-q2):(q2-p2-r2))/T24(2),sign(i==2?-1:1);
        alpha[i][0]=sqrt24(a2);alpha[i][1]=sign*pq/(da*db*alpha[i][0]);alpha[i][2]=sign*pr/(da*dc*alpha[i][0]);
        require(alpha[i][0]*alpha[i][0]==a2&&alpha[i][1]*alpha[i][1]==b2&&alpha[i][2]*alpha[i][2]==c2,"Aronhold squared ratios");
    }
    Matrix3<T24> matrix{};std::array<T24,3> minus{T24(-1),T24(-1),T24(-1)};
    for(int j=0;j<3;++j)for(int i=0;i<3;++i)matrix[j][i]=alpha[i][j].inv();
    auto u=solve3(matrix,minus);
    for(int j=0;j<3;++j)for(int i=0;i<3;++i)matrix[j][i]=u[i]*alpha[i][j];
    auto k=solve3(matrix,minus);
    Matrix3<T24> left{};for(int j=0;j<3;++j){left[0][j]=T24(1);left[1][j]=alpha[0][j].inv();left[2][j]=alpha[1][j].inv();}
    Matrix3<T24> xi{};
    for(int coordinate=0;coordinate<3;++coordinate){std::array<T24,3> rhs{T24(-1),-k[0]*alpha[0][coordinate],-k[1]*alpha[1][coordinate]};auto solution=solve3(left,rhs);
        T24 check;for(int j=0;j<3;++j){xi[j][coordinate]=solution[j];check=check+solution[j]/alpha[2][j];}
        require(check==-k[2]*alpha[2][coordinate],"fourth Riemann linear relation");
    }
    Form x=linear({T24(1),T24(0),T24(0)}),y=linear({T24(0),T24(1),T24(0)}),z=linear({T24(0),T24(0),T24(1)});
    Form a=linear(xi[0]),b=linear(xi[1]),c=linear(xi[2]),s=x*a+y*b-z*c;
    Form quartic=s*s-(x*y*a*b).scale(T24(4));require(!quartic.c.empty(),"nonzero reconstructed quartic");quartic=quartic.scale(quartic.c.begin()->second.inv());
    std::vector<std::array<T24,3>> lines{{T24(1),T24(0),T24(0)},{T24(0),T24(1),T24(0)},{T24(0),T24(0),T24(1)},{T24(1),T24(1),T24(1)},alpha[0],alpha[1],alpha[2]};
    for(auto& line:lines)check_bitangent(quartic,line);
    int rank=jacobian_rank7(quartic);require(rank==36,"geometric quartic smoothness matrix");
    rejects([&]{require(jacobian_rank7(x*x*x*x)==36,"singular quartic smoothness");},"singular quartic");
    rejects([&]{auto line=lines[4];line[0]=line[0]+T24(1);check_bitangent(quartic,line);},"corrupt bitangent");
    bool base=true;for(auto& [e,v]:quartic.c)if(v.a.degree()>0||v.a.at(0).a.degree()>0)base=false;
    std::cout<<"{\"scope\":\"P192-d7-kernel1-Riemann-candidate\",\"working_degree\":24,\"bitangent_checks\":7,\"jacobian_degree7_rank\":"<<rank<<",\"rejected_controls\":2,\"normalized_coefficients_in_Fp\":"<<(base?"true":"false")<<",\"same_field_descent_checked\":false,\"divisor_map_constructed\":false,\"standing\":\"producer-unverified\",\"quartic\":[";
    bool first=true;for(auto& [e,v]:quartic.c){if(!first)std::cout<<',';first=false;std::cout<<"{\"exponents\":["<<e[0]<<','<<e[1]<<','<<e[2]<<"],\"coefficient\":";emit_t24(v);std::cout<<'}';}
    std::cout<<"],\"Aronhold_bitangents\":[";for(size_t i=0;i<lines.size();++i){if(i)std::cout<<',';std::cout<<'[';for(int j=0;j<3;++j){if(j)std::cout<<',';emit_t24(lines[i][j]);}std::cout<<']';}std::cout<<"]}\n";
}catch(const std::exception&e){std::cerr<<"REJECT "<<e.what()<<'\n';return 1;}}
