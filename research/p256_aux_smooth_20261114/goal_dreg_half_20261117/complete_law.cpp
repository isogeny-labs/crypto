// Exact hypothesis/circuit qualification; not a relation solver or benchmark.
#include "membership_prefix.inc"
struct F {
    Z z; F(long n=0):z(mod(Z(n))){} explicit F(const Z& n):z(mod(n)){}
    F operator+(const F& b)const{return F(z+b.z);} F operator-(const F& b)const{return F(z-b.z);}
    F operator*(const F& b)const{return F(z*b.z);}
};
template<class T> struct Proj { T x,y,z; };
template<class T> Proj<T> rcb(const Proj<T>& u,const Proj<T>& v) {
    // RCB Algorithm1, instructions1--40, projective x=X/Z,y=Y/Z.
    T t0=u.x*v.x,t1=u.y*v.y,t2=u.z*v.z;
    T t3=(u.x+u.y)*(v.x+v.y)-t0-t1;
    T t4=(u.x+u.z)*(v.x+v.z)-t0-t2;
    T t5=(u.y+u.z)*(v.y+v.z)-t1-t2;
    T z=T(P-3)*t4+T(mod(3*B))*t2;
    T x=t1-z; z=t1+z; T y=x*z;
    t1=T(3)*t0+T(P-3)*t2;
    t2=T(P-3)*(t0-T(P-3)*t2);
    t4=T(mod(3*B))*t4+t2;
    return {t3*x-t5*t4,y+t1*t4,t5*z+t3*t1};
}
Proj<F> encode(const Point& p,const Z& scale=1) {
    require(scale!=0,"zero projective scale");
    return p.infinity?Proj<F>{F(0),F(scale),F(0)}:Proj<F>{F(scale*p.x),F(scale*p.y),F(scale)};
}
Point decode(const Proj<F>& p) {
    require(p.x.z!=0 || p.y.z!=0 || p.z.z!=0,"RCB zero output");
    require(mod(p.y.z*p.y.z*p.z.z-p.x.z*p.x.z*p.x.z+3*p.x.z*p.z.z*p.z.z-B*p.z.z*p.z.z*p.z.z)==0,"RCB off-curve");
    if(p.z.z==0){require(p.x.z==0,"invalid infinity");return {};}
    Z iz=inv(p.z.z);return point(p.x.z*iz,p.y.z*iz);
}
template<class T> Proj<T> balanced(const std::vector<Proj<T>>& ps,size_t lo,size_t hi) {
    require(hi>lo,"empty sum circuit");if(hi-lo==1)return ps[lo];
    size_t mid=lo+(hi-lo)/2;return rcb(balanced(ps,lo,mid),balanced(ps,mid,hi));
}
struct Node {std::string op,name;Z value=0;size_t a=0,b=0;unsigned degree=0;};
std::vector<Node> nodes;
struct C {
    size_t id;explicit C(size_t i,bool):id(i){}
    C(long n=0):C(Z(n)){} explicit C(const Z& n):id(nodes.size()){nodes.push_back({"constant","",mod(n),0,0,0});}
    static C var(const std::string& name){size_t i=nodes.size();nodes.push_back({"input",name,0,0,0,1});return C(i,true);}
    C op(const std::string& op,const C& b)const {
        size_t i=nodes.size();unsigned d=op=="multiply"?nodes[id].degree+nodes[b.id].degree:std::max(nodes[id].degree,nodes[b.id].degree);
        nodes.push_back({op,"",0,id,b.id,d});return C(i,true);
    }
    C operator+(const C& b)const{return op("add",b);}C operator-(const C& b)const{return op("subtract",b);}
    C operator*(const C& b)const{return op("multiply",b);}
};
std::vector<F> evaluate_circuit(const std::map<std::string,Z>& assignment) {
    std::vector<F> v;v.reserve(nodes.size());
    for(const auto& n:nodes) {
        if(n.op=="constant")v.emplace_back(n.value);
        else if(n.op=="input")v.emplace_back(assignment.at(n.name));
        else if(n.op=="add")v.push_back(v[n.a]+v[n.b]);
        else if(n.op=="subtract")v.push_back(v[n.a]-v[n.b]);
        else {require(n.op=="multiply","unknown circuit op");v.push_back(v[n.a]*v[n.b]);}
    }return v;
}
struct Line {Z x,y,z,parameter,character;bool target_infinity;};
Z line_eval(const Line& l,const Proj<F>& p){return mod(l.x*p.x.z+l.y*p.y.z+l.z*p.z.z);}
Line isolate(const Point& t,unsigned m,const std::string& kind,const std::string& fb) {
    for(size_t i=0;i<64;++i) {
        Z s=mod(Z(hash(SEED+"|GCB|"+fb+"|"+kind+"|m="+num(m)+"|line="+num(i)),16));
        if(t.infinity) {
            Z chi=power(mod(s*s*s-3*s+B),(P-1)/2);
            if(chi==P-1)return {1,0,mod(-s),s,chi,true};
        }else {
            Z A=mod(3*t.x-s*s),D=mod(3*t.x*t.x-3-2*s*t.y),disc=mod(A*A-4*D);
            Z chi=power(disc,(P-1)/2);
            if(chi==P-1)return {mod(-s),1,mod(s*t.x-t.y),s,chi,false};
        }
    }throw std::runtime_error("64 target-line candidates exhausted; cell open");
}
std::string line_json(const Line& l,const Point& target) {
    require(line_eval(l,encode(target))==0 && l.character==P-1,"line certificate");
    Object o{{"coefficients",array({dec(l.x),dec(l.y),dec(l.z)})},{"parameter",dec(l.parameter)},
      {"target",point_json(target)},{"legendre_character","-1"},{"infinity_case",l.target_infinity?"true":"false"}};
    if(!target.infinity) {
        const Z A=mod(3*target.x-l.parameter*l.parameter),D=mod(3*target.x*target.x-3-2*l.parameter*target.y);
        o["residual_quadratic_coefficients"]=array({dec(D),dec(A),dec(Z(1))});o["discriminant"]=dec(mod(A*A-4*D));
        // Exact substitution identity in u, including the root u=0.
        Poly u=Poly::var("u"),xx=u+Poly(target.x),yy=Poly(l.parameter)*u+Poly(target.y);
        Poly diff=xx*xx*xx-Poly(3)*xx+Poly(B)-sq(yy);
        require((diff-u*(sq(u)+Poly(A)*u+Poly(D))).terms.empty(),"target-line coefficient identity");
    }else o["vertical_rhs"]=dec(mod(l.parameter*l.parameter*l.parameter-3*l.parameter+B));
    return object(o);
}
std::string no_two_torsion() {
    Poly x=Poly::var("x"),g=x*x*x-Poly(3)*x+Poly(B),xp(1);
    size_t squarings=0,multiplications=0;
    for(int i=static_cast<int>(mpz_sizeinbase(P.get_mpz_t(),2))-1;i>=0;--i) {
        xp=divide(sq(xp),{g},{"x"}).remainder;++squarings;
        if(mpz_tstbit(P.get_mpz_t(),static_cast<mp_bitcnt_t>(i))) {xp=divide(xp*x,{g},{"x"}).remainder;++multiplications;}
    }
    Poly f=xp-x,r0=g,r1=f,s0(1),s1(0),t0(0),t1(1);
    while(!r1.terms.empty()) {
        auto dr=divide(r0,{r1},{"x"});Poly q=dr.multipliers[0];
        Poly nr=dr.remainder,ns=s0-q*s1,nt=t0-q*t1;
        r0=r1;r1=nr;s0=s1;s1=ns;t0=t1;t1=nt;
    }
    require(r0.degree()==0 && r0.terms.size()==1,"rational2torsion exists");
    Z unit=inv(r0.terms.begin()->second);s0=Poly(unit)*s0;t0=Poly(unit)*t0;
    require((s0*g+t0*f-Poly(1)).terms.empty(),"gcd Bezout identity");
    bool rejects=!( (s0+Poly(1))*g+t0*f-Poly(1) ).terms.empty();require(rejects,"corrupt Bezout accepted");
    return object({{"cubic",g.json()},{"x_to_p_mod_cubic",xp.json()},{"bezout_cubic_multiplier",s0.json()},
      {"bezout_residue_minus_x_multiplier",t0.json()},{"gcd","1"},{"pow_squarings",num(squarings)},
      {"pow_multiplications",num(multiplications)},{"altered_bezout_rejected","true"}});
}
std::string dense_CI(const Base& b) {
    if(b.columns.size()%3!=0)return object({{"applicable","false"},{"reason",jquote("original retained column count not divisible by3; no subset silently substituted")}});
    Poly x=Poly::var("x"),y=Poly::var("y"),g=x*x*x-sq(y)-Poly(3)*x+Poly(B),Fpoly(1);
    for(const auto& col:b.columns)Fpoly=Fpoly*(x-Poly(col.target.x));
    auto d=divide(Fpoly,{g},{"x","y"});const Poly& reduced=d.remainder;
    unsigned k=static_cast<unsigned>(2*b.columns.size()/3);require(reduced.degree()==k,"dense CI degree");
    Poly at_infinity;
    for(const auto& [mon,c]:reduced.terms)if(!mon.contains("x")) {
        unsigned deg=0;for(const auto& [v,e]:mon){(void)v;deg+=e;}
        if(deg==k)at_infinity.terms[mon]=c;
    }
    require((at_infinity-term(Monomial{{"y",k}},Z(1))).terms.empty(),"dense CI leading forms meet at infinity");
    for(const auto& col:b.columns)for(unsigned sign=0;sign<2;++sign) {
        std::map<std::string,Z> p{{"x",col.target.x},{"y",sign?mod(-col.target.y):col.target.y}};
        require(reduced.eval(p)==0 && g.eval(p)==0,"dense CI point");
    }
    return object({{"applicable","true"},{"curve",g.json()},{"retained_abscissa_product",Fpoly.json()},
      {"division_multiplier",d.multipliers[0].json()},{"second_generator",reduced.json()},
      {"CI_degrees",array({"3",num(k)})},{"quotient_regularity",num(k+1)},
      {"identity",jquote("F=Q*curve+second_generator; leading forms have no common nonzero zero")}});
}
std::string circuit_json(const Proj<C>& sum,const C& f) {
    std::vector<std::string> ns;
    for(const auto& n:nodes)ns.push_back(object({{"op",jquote(n.op)},{"name",jquote(n.name)},
      {"value",dec(n.value)},{"a",num(n.a)},{"b",num(n.b)},{"degree_upper",num(n.degree)}}));
    return object({{"nodes",array(ns)},{"sum_outputs",array({num(sum.x.id),num(sum.y.id),num(sum.z.id)})},
      {"relation_output",num(f.id)},{"representation",jquote("polynomial straight-line circuit, no new ring variables")}});
}
int main(int argc,char** argv) {
    try {
        require(argc==2,"usage: complete_law FRESH_OUT");std::filesystem::path out=argv[1];require(!std::filesystem::exists(out),"fresh output");
        write(out/"no-two-torsion.json",no_two_torsion());
        Base b=build(8);crosscheck(b);auto domain=signed_points(b);
        write(out/"factor-base.json",b.dump);write(out/"identity.json",b.identity);write(out/"construction.json",b.certificate);
        write(out/"dense-CI.json",dense_CI(b));
        Variables vars;auto gs=tower_polynomials(b,vars);const auto pairs=certify_tower(gs,vars,9);
        std::vector<std::string> gps,coordinates;
        for(const auto& g:gs)gps.push_back(g.json());
        std::set<Z> full_roots;Point q=b.coset;
        for(size_t i=0;i<256;++i) {
            require(!q.infinity && full_roots.insert(q.x).second,"full fibre distinctness");
            std::map<std::string,Z> a{{"H",1}};Point current=q;
            for(size_t j=0;j<8;++j){a["X"+num(j)]=current.x;current=b.chain[j].apply(current);}
            require(current==b.terminal,"full chain terminal");for(const auto& g:gs)require(g.eval(a)==0,"full chain equation");
            Object co;for(const auto& [v,c]:a)co[v]=dec(c);coordinates.push_back(object(co));q=AUX.add(q,b.kernel);
        }
        require(q==b.coset,"full fibre closure");
        for(const auto& col:b.columns)require(full_roots.contains(col.target.x) && col.target.y!=0,"retained inclusion/nonzero sign");
        write(out/"structured-span.json",object({{"full_fibre_points",array(coordinates)},
          {"quadratic_generators",array(gps)},{"s_pairs",pairs},{"full_fibre_K_H_basis",jquote("all256 squarefree monomials in X0,...,X7")},
          {"full_fibre_quotient_regularity","8"},{"retained_signed_quotient_regularity_upper","9"},
          {"exact_retained_generator_list_materialized","false"},
          {"explanation",jquote("Restriction spans retained abscissae by degree8; even/odd target functions add at most one y. This is a span bound, not a supplied retained homogeneous ideal.")}}));
        OpenSSLCheck openssl(TARGET);size_t checks=0;
        auto test_pair=[&](const Point& u,const Point& v,const Z& s,const Z& t) {
            Point expected=TARGET.add(u,v),actual=decode(rcb(encode(u,s),encode(v,t)));
            require(actual==expected,"RCB/GMP discrepancy");openssl.add(u,v,expected);++checks;
        };
        for(size_t i=0;i<std::min<size_t>(16,domain.size());++i) {
            const auto& p=domain[i];test_pair({},p,2,3);test_pair(p,{},5,7);test_pair(p,TARGET.neg(p),11,13);test_pair(p,p,17,19);
        }
        test_pair({},{},23,29);
        for(size_t i=0;i<64;++i)test_pair(domain[(i*17+3)%domain.size()],domain[(i*31+5)%domain.size()],Z(i+1),Z(i+2));
        std::vector<std::string> cells;size_t replays=0;
        for(unsigned m=2;m<=7;++m) {
            auto indices=planted_indices(b,m);Point planted=replay(domain,indices),seq;
            std::vector<Proj<F>> ps;
            for(size_t j=0;j<indices.size();++j){Point p=domain[indices[j]],next=TARGET.add(seq,p);openssl.add(seq,p,next);seq=next;ps.push_back(encode(p,Z(j+2)));}
            auto sum=balanced(ps,0,ps.size());require(decode(sum)==planted && seq==planted,"balanced group replay");++replays;
            for(const std::string kind:{"planted","unplanted"}) {
                const Point target=kind=="planted"?planted:natural_target(b,m);const Line l=isolate(target,m,kind,b.fb_id);
                nodes.clear();C H=C::var("H");std::vector<Proj<C>> leaves;
                for(unsigned i=0;i<m;++i)leaves.push_back({C::var("x"+num(i)),C::var("y"+num(i)),H});
                auto symbolic=balanced(leaves,0,leaves.size());C f=C(l.x)*symbolic.x+C(l.y)*symbolic.y+C(l.z)*symbolic.z;
                unsigned e=nodes[f.id].degree;std::map<std::string,Z> a{{"H",1}},pure{{"H",0}};
                for(unsigned i=0;i<m;++i){a["x"+num(i)]=domain[indices[i]].x;a["y"+num(i)]=domain[indices[i]].y;pure["x"+num(i)]=0;pure["y"+num(i)]=1;}
                auto ev=evaluate_circuit(a),top=evaluate_circuit(pure);
                require(decode(Proj<F>{ev[symbolic.x.id],ev[symbolic.y.id],ev[symbolic.z.id]})==planted,"circuit/GMP sum");
                require((ev[f.id].z==0)==(planted==target),"target line misclassification on replay witness");
                if(!target.infinity)require(top[f.id].z==1,"pure-Y top-degree coefficient");
                const std::string stem="m"+num(m)+"-"+kind,circuit=circuit_json(symbolic,f);
                write(out/(stem+"-circuit.json"),circuit);write(out/(stem+"-line.json"),line_json(l,target));
                std::vector<std::string> is;for(size_t i:indices)is.push_back(num(i));
                cells.push_back(object({{"cell",jquote(stem)},{"m",num(m)},{"kind",jquote(kind)},
                  {"target",point_json(target)},{"circuit_sha256",jquote(hash(circuit+"\n"))},
                  {"circuit_nodes",num(nodes.size())},{"relation_polynomial_degree_upper",num(e)},
                  {"pure_Y_degree_witness",target.infinity?"false":"true"},
                  {"point_variables_dense",num(2*m)},{"point_variables_chain",num(9*m)},
                  {"chain_added_variables",num(7*m)},{"free_group_witness_variables","0"},
                  {"planted_replay_indices",kind=="planted"?array(is):"null"},
                  {"relation_reported",kind=="planted"?"true":"false"},
                  {"full_relation_regularity","null"},{"solver_status",jquote("not-run")}}));
            }
        }
        write(out/"summary.json",object({{"factor_base",jquote(b.fb_id)},{"columns",num(b.columns.size())},
          {"signed_points",num(domain.size())},{"group_controls",num(checks)},{"planted_group_replays",num(replays)},
          {"cells",array(cells)},{"all_reported_relations_replayed","true"},{"full_relation_goal_achieved","false"},
          {"complete_homogeneous_retained_ideal_exported","false"},{"runtime_gain_claim","false"},
          {"scope",jquote("deterministic depth8 construction, published complete-law controls and bound hypotheses; not a complete degree certificate or solve")}}));
        std::cout<<b.fb_id<<" columns="<<b.columns.size()<<"; six planted replays; complete bound still conditional.\n";
    }catch(const std::exception& e){std::cerr<<"complete_law: "<<e.what()<<'\n';return 1;}
}
