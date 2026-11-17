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

int main(int argc,char**argv){try{
    require(argc==2,"usage: quartic_descent study-directory");std::string root=argv[1];initialize_cover(read_json(root+"/inputs/p192-d7-prym.json"));Level2Input input;
    auto candidate=read_json(root+"/evidence/riemann-v1/result.json");Form curve_c=read_form(candidate.at("quartic"));
    auto moduli=read_json(root+"/evidence/quotient-theta2-v1/result.json");std::vector<T24> theta;for(auto& w:moduli.at("theta_squares").arr)theta.push_back(read_t24(w));
    const std::array<int,7> labels{63,38,52,41,13,26,19};std::array<Line,7> aronhold;std::map<int,Line> lines;
    for(int i=0;i<7;++i){aronhold[i]=read_line(candidate.at("Aronhold_bitangents").arr.at(i));lines.emplace(labels[i],normalize_line(aronhold[i]));}
    DerivativeProducts products(theta);int known_checks=0,extra_checks=0;
    for(int i=0;i<7;++i){std::vector<int> f;for(int j=0;j<7;++j)if(i!=j)f.push_back(j);std::array<int,4> frame{f[0],f[1],f[2],f[3]};
        auto sq=squared_coordinates(labels[i],frame,labels,aronhold,products);check_squared_coordinates(aronhold[i],frame,aronhold,sq);++known_checks;
    }
    for(int i=0;i<7;++i)for(int j=i+1;j<7;++j){
        int label=labels[i]^labels[j];require(theta_parity(label),"remaining odd characteristic");std::vector<int> f;for(int k=0;k<7;++k)if(k!=i&&k!=j)f.push_back(k);
        std::array<int,4> frame{f[0],f[1],f[2],f[3]};auto sq=squared_coordinates(label,frame,labels,aronhold,products);Line roots;
        for(int k=0;k<3;++k)roots[k]=sqrt24(sq[k]);
        std::vector<Line> candidates;
        for(int signs=0;signs<4;++signs){Line coefficients=roots;if(signs&1)coefficients[1]=-coefficients[1];if(signs&2)coefficients[2]=-coefficients[2];Line line{};
            for(int a=0;a<3;++a)for(int b=0;b<3;++b)line[b]=line[b]+coefficients[a]*aronhold[frame[a]][b];
            try{check_bitangent(curve_c,line);candidates.push_back(normalize_line(line));}catch(const std::exception&){}
        }
        require(candidates.size()==1,"unique bitangent sign choice");std::array<int,4> second{f[1],f[2],f[3],f[4]};
        auto second_sq=squared_coordinates(label,second,labels,aronhold,products);check_squared_coordinates(candidates[0],second,aronhold,second_sq);
        require(lines.emplace(label,candidates[0]).second,"unique odd label");++extra_checks;std::cerr<<"labelled bitangent "<<label<<'\n';
    }
    require(lines.size()==28,"all labelled bitangents");std::set<std::string> distinct;
    for(auto& [label,line]:lines){check_bitangent(curve_c,line);std::string key;for(auto& c:line)key+=c.key()+"|";distinct.insert(key);}require(distinct.size()==28,"distinct geometric bitangents");
    std::vector<int> permutation;
    for(auto& r:input.branch){auto a=r.power(prime);int where=-1;for(int i=0;i<7;++i)if(a==input.branch[i])where=i;require(where>=0,"branch Frobenius");permutation.push_back(where);}
    auto frobenius_code=[&](int code){int mask=input.masks[code],out=0;for(int i=0;i<7;++i)if((mask>>i)&1)out^=1<<permutation[i];out=canonical_subset(out);auto it=std::find(input.masks.begin(),input.masks.end(),out);require(it!=input.masks.end(),"Frobenius characteristic code");return int(it-input.masks.begin());};
    auto frobenius_label=[&](int label){int code=(label>>3)|((label&7)<<3);int out=frobenius_code(code^input.delta)^input.delta;return ((out&7)<<3)|(out>>3);};
    std::array<Line,3> target{lines.at(frobenius_label(labels[0])),lines.at(frobenius_label(labels[1])),lines.at(frobenius_label(labels[2]))};
    auto fourth=frame_coordinates(lines.at(frobenius_label(labels[3])),target);Matrix3<T24> left{},map{};
    for(int i=0;i<3;++i)left[i]=target[i];
    for(int j=0;j<3;++j){Line rhs{};rhs[j]=fourth[j].inv();auto col=solve3(left,rhs);for(int i=0;i<3;++i)map[i][j]=col[i];}
    int line_checks=0;
    for(auto& [label,line]:lines){Line fp=line;for(auto& c:fp)c=c.power(prime);require(normalize_line(row_times(lines.at(frobenius_label(label)),map))==normalize_line(fp),"all labelled Frobenius line correspondences");++line_checks;}
    require(normalized_form(transform_form(curve_c,map)).c==normalized_form(frobenius_form(curve_c)).c,"Frobenius quartic equation");
    Matrix3<T24> total=identity3(),current=map;
    for(int i=0;i<24;++i){total=multiply3(total,current);current=frobenius3(current);}
    T24 lambda=total[0][0];require(lambda!=T24(0),"nonzero projective norm scalar");
    for(int i=0;i<3;++i)for(int j=0;j<3;++j)require(total[i][j]==(i==j?lambda:T24(0)),"projective descent cocycle");
    require(lambda.a.degree()==0&&lambda.a.at(0).a.degree()<=0,"descent scalar lies in Fp");mpz_class inverse3,three=3,pminus=prime-1;
    require(mpz_invert(inverse3.get_mpz_t(),three.get_mpz_t(),pminus.get_mpz_t())!=0,"P192 inverse3 modulo p-minus1");
    F target_norm=lambda.a.at(0).inv().power(inverse3),mu=norm_preimage8(target_norm);
    for(auto& row:map)for(auto& c:row)c=c*T24(mu);
    std::vector<Matrix3<T24>> partials;total=identity3();current=map;
    for(int i=0;i<24;++i){partials.push_back(total);total=multiply3(total,current);current=frobenius3(current);}require(total==identity3(),"normalized linear cocycle");
    std::vector<Line> columns;int vectors_tried=0;
    for(int a=0;a<8&&columns.size()<3;++a)for(int b=0;b<3&&columns.size()<3;++b){
        std::vector<Z> coeff(size_t(a+1));coeff[a]=Z(1);F fa{Poly<Z>(coeff)};std::vector<F> outer(size_t(b+1));outer[b]=fa;
        T24 beta{P(outer)};std::vector<T24> conjugates;for(int k=0;k<24;++k){conjugates.push_back(beta);beta=beta.power(prime);}
        for(int coordinate=0;coordinate<3&&columns.size()<3;++coordinate){Line v{};
            for(int k=0;k<24;++k)for(int r=0;r<3;++r)v[r]=v[r]+partials[k][r][coordinate]*conjugates[k];
            ++vectors_tried;
            bool independent=false;
            if(columns.empty()){for(auto& c:v)independent=independent||c!=T24(0);}
            else if(columns.size()==1){for(int i=0;i<3;++i)for(int j=0;j<3;++j)independent=independent||(v[i]*columns[0][j]!=v[j]*columns[0][i]);}
            else{Matrix3<T24> test{};for(int i=0;i<3;++i){test[i][0]=columns[0][i];test[i][1]=columns[1][i];test[i][2]=v[i];}independent=determinant3(test)!=T24(0);}
            if(independent)columns.push_back(v);
        }
    }
    require(columns.size()==3,"Hilbert90 coefficient-basis bound");Matrix3<T24> change{};for(int j=0;j<3;++j)for(int i=0;i<3;++i)change[i][j]=columns[j][i];
    require(determinant3(change)!=T24(0)&&multiply3(map,frobenius3(change))==change,"matrix Hilbert90 equation");
    Form descended=normalized_form(transform_form(curve_c,change));for(auto& [e,c]:descended.c)require(c.a.degree()==0&&c.a.at(0).a.degree()<=0,"normalized quartic descends to Fp");
    require(jacobian_rank7(descended)==36,"descended quartic geometric smoothness");for(auto& [label,line]:lines)check_bitangent(descended,row_times(line,change));
    rejects([&]{auto corrupt=change;corrupt[0][0]=corrupt[0][0]+T24(1);require(multiply3(map,frobenius3(corrupt))==corrupt,"corrupted descent matrix");},"corrupted descent matrix");
    std::cout<<"{\"scope\":\"P192-d7-kernel1-candidate-curve-descent\",\"known_gradient_ratio_checks\":"<<known_checks<<",\"new_labelled_bitangents\":"<<extra_checks<<",\"labelled_Frobenius_checks\":"<<line_checks<<",\"Hilbert90_vectors_tried\":"<<vectors_tried<<",\"base_field_quartic\":true,\"geometric_smoothness_rank\":36,\"Serre_twist_checked\":false,\"divisor_map_constructed\":false,\"standing\":\"producer-unverified\",\"quartic_Fp\":[";
    bool first=true;for(auto& [e,c]:descended.c){if(!first)std::cout<<',';first=false;std::cout<<"{\"exponents\":["<<e[0]<<','<<e[1]<<','<<e[2]<<"],\"coefficient\":\""<<scalar(c.a.at(0)).x<<"\"}";}std::cout<<"],\"Frobenius_map\":";emit_matrix(map);std::cout<<",\"change_of_coordinates\":";emit_matrix(change);
    std::cout<<",\"labelled_bitangents\":[";first=true;for(auto& [label,line]:lines){if(!first)std::cout<<',';first=false;std::cout<<"{\"label\":"<<label<<",\"line\":[";for(int i=0;i<3;++i){if(i)std::cout<<',';emit_t24(line[i]);}std::cout<<"]}";}std::cout<<"]}\n";
}catch(const std::exception&e){std::cerr<<"REJECT "<<e.what()<<'\n';return 1;}}
