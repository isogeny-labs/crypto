// Tian 2020 sections 2 and 4: actual normalized level-3 Weil functions.
// This is a geometric prerequisite, not a reconstructed quotient curve.
#include "quotient_arithmetic.hpp"
#include <array>

P derivative(const P& a) {
    std::vector<F> out;
    for (int i=1;i<=a.degree();++i) out.push_back(a.at(i)*F(i));
    return P(out);
}

F resultant3(const P& u,const P& a) {
    require(u.degree()==3 && u.at(3)==F(1),"resultant monic cubic");
    std::array<std::array<F,3>,3> m{};
    P x{F(0),F(1)}, v=a%u;
    for(int j=0;j<3;++j) {
        for(int i=0;i<3;++i) m[i][j]=v.at(i);
        v=(v*x)%u;
    }
    return m[0][0]*(m[1][1]*m[2][2]-m[1][2]*m[2][1])
         - m[0][1]*(m[1][0]*m[2][2]-m[1][2]*m[2][0])
         + m[0][2]*(m[1][0]*m[2][1]-m[1][1]*m[2][0]);
}

std::array<F,7> null_vector(std::vector<std::array<F,7>> m) {
    std::vector<int> pivots;
    int row=0;
    for(int col=0;col<7 && row<int(m.size());++col) {
        int pick=row;
        while(pick<int(m.size()) && m[pick][col]==F(0)) ++pick;
        if(pick==int(m.size())) continue;
        std::swap(m[pick],m[row]);
        F inv=m[row][col].inv();
        for(auto& v:m[row]) v=v*inv;
        for(int i=0;i<int(m.size());++i) if(i!=row) {
            F c=m[i][col];
            for(int j=0;j<7;++j) m[i][j]=m[i][j]-c*m[row][j];
        }
        pivots.push_back(col); ++row;
    }
    require(pivots.size()==6,"principal function nullity must be one");
    int free=0;
    while(std::find(pivots.begin(),pivots.end(),free)!=pivots.end()) ++free;
    std::array<F,7> out{}; out[free]=F(1);
    for(int i=0;i<6;++i) out[pivots[i]]=-m[i][free];
    return out;
}

struct Weil3 {
    G index, support;
    P a,b;
    F norm_scale;
    bool identity;
    explicit Weil3(const G& p):index(p),support(p.neg()),identity(p==G{}) {
        if(identity) return;
        support.check();
        require(support.times(3)==G{} && support.u.degree()==3,"order3 function input");
        const P& u=support.u;
        P u2=u*u,u3=u2*u;
        auto [g,s,t]=xgcd(support.v.scale(F(2)),u);
        require(g.degree()==0,"unramified order3 support");
        P lift=support.v;
        lift=lift+u*((((curve-lift*lift).exact(u))*s)%u);
        require(((lift*lift-curve)%u2).degree()<0,"first Hensel lift");
        lift=lift+u2*((((curve-lift*lift).exact(u2))*s)%u);
        require(((lift*lift-curve)%u3).degree()<0,"second Hensel lift");
        std::array<P,7> columns;
        P x{F(0),F(1)},power{F(1)};
        for(int i=0;i<5;++i) {columns[i]=power; power=power*x;}
        columns[5]=lift; columns[6]=(x*lift)%u3;
        std::vector<std::array<F,7>> matrix(9);
        for(int i=0;i<9;++i) for(int j=0;j<7;++j) matrix[i][j]=columns[j].at(i);
        auto n=null_vector(matrix);
        a=P(std::vector<F>(n.begin(),n.begin()+5)); b=P{n[5],n[6]};
        require(((a+b*lift)%u3).degree()<0,"triple support vanishing");
        P quotient=(a*a-b*b*curve).exact(u3);
        require(quotient.degree()==0 && quotient.at(0)!=F(0),"principal norm identity");
        norm_scale=quotient.at(0);
        check();
    }
    void check()const {
        if(identity) return;
        require(a*a-b*b*curve==(support.u*support.u*support.u).scale(norm_scale),"exact h norm identity");
        require(((a+b*support.v)%support.u).degree()<0,"correct h support sign");
    }
    F operator()(const G& z)const {
        if(identity) return F(1);
        require(z.u.degree()==3,"Weil evaluation outside generic degree3 chart");
        auto [g,s,t]=xgcd(support.u,z.u);
        require(g.degree()==0,"Weil support collision");
        P value=((z.v+support.v)*s)%z.u;
        F ratio=value.at(2);
        return ratio*ratio*ratio*resultant3(z.u,a+b*z.v);
    }
};

std::vector<G> generic_inputs(const std::vector<G>& kernel) {
    std::vector<G> points;
    for(int i=1;i<=128 && points.size()<18;++i) {
        F x(i); Z value=scalar(curve.eval(x));
        mpz_class symbol,y;
        mpz_powm(symbol.get_mpz_t(),value.x.get_mpz_t(),mpz_class((prime-1)/2).get_mpz_t(),prime.get_mpz_t());
        if(symbol!=1) continue;
        mpz_powm(y.get_mpz_t(),value.x.get_mpz_t(),mpz_class((prime+1)/4).get_mpz_t(),prime.get_mpz_t());
        G point{P{-x,F(1)},P{F(Poly<Z>{Z(y)})}};point.check();points.push_back(point);
    }
    require(points.size()>=9,"generic support input bound");
    std::vector<G> result;
    for(int i=0;i<16 && i+2<int(points.size()) && result.size()<3;++i) {
        G z=points[i].add(points[i+1]).add(points[i+2]);
        bool usable=true;
        for(auto& k:kernel) {
            G translated=z.add(k);
            if(translated.u.degree()!=3) {usable=false;break;}
            auto [g,s,t]=xgcd(translated.u,derivative(translated.u));
            if(g.degree()!=0) {usable=false;break;}
            for(auto& p:kernel) if(!(p==G{})) {
                auto [gg,ss,tt]=xgcd(translated.u,p.u);
                if(gg.degree()!=0) {usable=false;break;}
            }
            if(!usable) break;
        }
        std::cerr<<"generic candidate "<<i<<" usable="<<usable<<'\n';
        if(usable) result.push_back(z);
    }
    require(result.size()==3,"three regular test classes within fixed bound");
    return result;
}

int main(int argc,char** argv) {try {
    require(argc==2,"usage: weil3 study-directory"); std::string root=argv[1];
    initialize_cover(read_json(root+"/inputs/p192-d7-prym.json"));
    auto actual=read_json(root+"/evidence/final-kernel-check/actual-cover-torsion.json");
    std::vector<G> basis;
    for(auto& v:actual.at("basis").arr) basis.push_back({polynomial(v.at("U")),polynomial(v.at("V"))});
    auto modules=read_json(root+"/../nist_cover_separator_20261115/data/modules-192.json");
    M frob{},form{};bool found=false;
    for(auto& row:modules.at("modules").arr) if(row.at("candidate").integer()==139 && row.at("d").integer()==7) {
        block(frob,row.at("E_Frobenius"),2,0);block(frob,row.at("Prym_Frobenius"),4,2);
        block(form,row.at("E_Weil"),2,0);block(form,row.at("Prym_Weil"),4,2);found=true;
    }
    require(found,"P192 d7 module");
    std::ifstream in(root+"/evidence/final-kernel-check/kernels.jsonl");std::string line;
    require(bool(std::getline(in,line)),"first kernel input");Parser parser{line};auto row=parser.value();
    require(row.at("kernel").integer()==1,"fixed kernel1");Basis coordinates;
    for(auto& r:row.at("rref_basis").arr) {V v{};for(int i=0;i<6;++i)v[i]=r.arr.at(i).integer();coordinates.push_back(v);}
    Quotient quotient(basis,coordinates,frob,form);auto kernel=quotient.kernel;
    std::map<std::string,int> index;
    for(int i=0;i<27;++i) index.emplace(gkey(kernel[i]),i);
    std::vector<Weil3> functions;
    for(int i=0;i<27;++i) {functions.emplace_back(kernel[i]);std::cerr<<"constructed h "<<i<<'\n';}
    auto inputs=generic_inputs(kernel);std::vector<F> alpha(27,F(1));
    const auto& z=inputs[0];
    for(int i=1;i<27;++i) {
        F a=functions[i](z),b=functions[i](z.add(kernel[i])),c=functions[i](z.neg());
        require(a!=F(0)&&b!=F(0)&&c!=F(0),"normalization outside zero/pole");alpha[i]=c/(a*b);
    }
    int equations=0,symmetries=0,invariances=0,frobenius_checks=0;
    F nonzero_theta;
    for(size_t test=0;test<inputs.size();++test) {
        const G& d=inputs[test];std::vector<std::vector<F>> values(27,std::vector<F>(27));
        for(int j=0;j<27;++j) {
            G shifted=d.add(kernel[j]);
            for(int i=0;i<27;++i) values[i][j]=alpha[i]*functions[i](shifted);
        }
        F theta;
        for(int i=0;i<27;++i) {
            require(values[i][0]!=F(0),"regular nonzero Weil value");theta=theta+values[i][0];
            int opposite=index.at(gkey(kernel[i].neg()));
            require(alpha[i]*functions[i](d.neg())==values[opposite][0],"symmetric normalization");++symmetries;
            F repeated=functions[i](d.neg())/(functions[i](d)*functions[i](d.add(kernel[i])));
            require(repeated==alpha[i],"normalization independent of evaluation point");
            int fi=index.at(gkey(gfrob(kernel[i])));
            require(alpha[fi]*functions[fi](gfrob(d))==values[i][0].power(prime),"normalized Weil Frobenius covariance");++frobenius_checks;
            for(int j=0;j<27;++j) {
                int sum=index.at(gkey(kernel[i].add(kernel[j])));
                require(values[i][0]*values[j][i]==values[sum][0],"normalized theta-group multiplication");++equations;
            }
        }
        require(theta!=F(0)&&theta.power(prime)==theta,"nonzero base-field invariant theta value");
        nonzero_theta=theta;
        for(int j=0;j<27;++j) {
            F shifted;for(int i=0;i<27;++i) shifted=shifted+values[i][j];
            require(values[j][0]*shifted==theta,"level-subgroup invariant theta section");++invariances;
        }
        std::cerr<<"completed generic class "<<test<<'\n';
    }
    rejects([&]{auto bad=functions[1];bad.a=bad.a+P{F(1)};bad.check();},"corrupted principal function");
    rejects([&]{F bad=alpha[1]+F(1);require(bad*functions[1](z.neg())==alpha[index.at(gkey(kernel[1].neg()))]*functions[index.at(gkey(kernel[1].neg()))](z),"corrupted normalization");},"corrupted normalization");
    Basis bad{V{1,0,0,0,0,0},V{0,1,0,0,0,0},V{0,0,1,0,0,0}};
    rejects([&]{Quotient wrong(basis,bad,frob,form);},"non-isotropic kernel");
    std::cout<<"{\"scope\":\"P192-d7-kernel1-normalized-Weil3\",\"field_degree\":8,\"kernel_size\":27,\"principal_norm_identities\":26,\"generic_classes\":3,\"normalized_group_law_checks\":"<<equations<<",\"symmetry_checks\":"<<symmetries<<",\"Frobenius_checks\":"<<frobenius_checks<<",\"theta_invariance_checks\":"<<invariances<<",\"rejected_controls\":3,\"nonzero_theta_value\":\""<<scalar(nonzero_theta).x<<"\",\"quartic_constructed\":false,\"standing\":\"producer-unverified\",\"functions\":[";
    for(int i=0;i<27;++i) {
        if(i) std::cout<<',';
        std::cout<<"{\"P_U\":";emit_p(kernel[i].u);std::cout<<",\"P_V\":";emit_p(kernel[i].v);
        std::cout<<",\"A\":";emit_p(functions[i].a);std::cout<<",\"B\":";emit_p(functions[i].b);
        std::cout<<",\"norm_scale\":";emit_f(functions[i].norm_scale);std::cout<<",\"alpha\":";emit_f(alpha[i]);std::cout<<'}';
    }
    std::cout<<"]}\n";
}catch(const std::exception& e){std::cerr<<"REJECT "<<e.what()<<'\n';return 1;}}
