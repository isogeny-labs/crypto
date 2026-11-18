#include "elliptic_backmap_arithmetic.hpp"
#include <memory>
constexpr int FS_N=224;
using FSVector=std::array<Z,FS_N>;
struct FSDag;
FSDag* fs_graph=nullptr;
struct FSExpr {
    int id=0;
    FSExpr()=default;
    FSExpr(long);
    explicit FSExpr(const Z&);
    static FSExpr node(int n){FSExpr r;r.id=n;return r;}
    FSExpr operator+(FSExpr)const;
    FSExpr operator-()const;
    FSExpr operator-(FSExpr b)const{return *this+(-b);}
    FSExpr operator*(FSExpr)const;
    FSExpr inv()const;
    FSExpr operator/(FSExpr b)const{return *this*b.inv();}
};
struct FSNode {char op;int a=-1,b=-1;FSVector c{};};
struct FSDag {
    std::vector<std::unique_ptr<FSNode>> nodes;
    std::map<std::string,int> constants;
    std::map<std::tuple<char,int,int>,int> cache;
    int constant(const Z& z){auto key=z.x.get_str();auto p=constants.find(key);if(p!=constants.end())return p->second;auto n=std::make_unique<FSNode>();n->op='c';n->c[0]=z;int id=int(nodes.size());nodes.push_back(std::move(n));constants.emplace(key,id);return id;}
    int input(const Z& z){auto n=std::make_unique<FSNode>();n->op='i';n->c[0]=z;int id=int(nodes.size());nodes.push_back(std::move(n));return id;}
    int operation(char op,int a,int b=-1){
        if(op=='+'||op=='*'){if(a>b)std::swap(a,b);if(op=='+'&&a==0)return b;if(op=='*'&&a==0)return 0;}
        bool ca=nodes[a]->op=='c',cb=b>=0&&nodes[b]->op=='c';
        if(op=='*'){if(ca&&nodes[a]->c[0]==Z(1))return b;if(cb&&nodes[b]->c[0]==Z(1))return a;}
        if(ca&&(op=='-'||op=='/'||cb)){auto x=nodes[a]->c[0];return constant(op=='-'?-x:op=='/'?x.inv():op=='+'?x+nodes[b]->c[0]:x*nodes[b]->c[0]);}
        auto key=std::make_tuple(op,a,b);auto p=cache.find(key);if(p!=cache.end())return p->second;
        auto n=std::make_unique<FSNode>();n->op=op;n->a=a;n->b=b;int id=int(nodes.size());nodes.push_back(std::move(n));cache.emplace(key,id);return id;
    }
    void coefficient(int k){for(auto& np:nodes){auto& n=*np;if(n.op=='c'||n.op=='i')continue;auto& a=nodes[n.a]->c;Z z;
        if(n.op=='+')z=a[k]+nodes[n.b]->c[k];
        else if(n.op=='-')z=-a[k];
        else if(n.op=='*'){auto& b=nodes[n.b]->c;for(int j=0;j<=k;++j)if(a[j]!=Z(0)&&b[k-j]!=Z(0))z=z+a[j]*b[k-j];}
        else {require(n.op=='/',"formal DAG opcode");if(k==0)z=a[0].inv();else{for(int j=1;j<=k;++j)z=z+a[j]*n.c[k-j];z=-z*n.c[0];}}
        n.c[k]=z;
    }}
    const FSVector& value(FSExpr a)const{return nodes[a.id]->c;}
};
FSExpr::FSExpr(long n):id(fs_graph->constant(Z(n))){}
FSExpr::FSExpr(const Z& z):id(fs_graph->constant(z)){}
FSExpr FSExpr::operator+(FSExpr b)const{return node(fs_graph->operation('+',id,b.id));}
FSExpr FSExpr::operator-()const{return node(fs_graph->operation('-',id));}
FSExpr FSExpr::operator*(FSExpr b)const{return node(fs_graph->operation('*',id,b.id));}
FSExpr FSExpr::inv()const{return node(fs_graph->operation('/',id));}
using FSCubic=std::array<FSExpr,3>;
FSCubic fs_add(const FSCubic& a,const FSCubic& b){FSCubic r;for(int i=0;i<3;++i)r[i]=a[i]+b[i];return r;}
FSCubic fs_neg(const FSCubic& a){FSCubic r;for(int i=0;i<3;++i)r[i]=-a[i];return r;}
FSCubic fs_scale(const FSCubic& a,FSExpr c){FSCubic r;for(int i=0;i<3;++i)r[i]=a[i]*c;return r;}
FSCubic fs_mul(const FSCubic& a,const FSCubic& b,const FSCubic& u){std::array<FSExpr,5> t{};for(int i=0;i<3;++i)for(int j=0;j<3;++j)t[i+j]=t[i+j]+a[i]*b[j];for(int k=4;k>=3;--k)for(int j=0;j<3;++j)t[k-3+j]=t[k-3+j]-t[k]*u[j];return {t[0],t[1],t[2]};}
FSCubic fs_curve(const EBPoly& f,const FSCubic& x,const FSCubic& y,const FSCubic& u){FSCubic r{};for(auto& [e,c]:f.terms){FSCubic term{FSExpr(c),FSExpr(0),FSExpr(0)};for(int i=0;i<e[0];++i)term=fs_mul(term,x,u);for(int i=0;i<e[1];++i)term=fs_mul(term,y,u);r=fs_add(r,term);}return r;}
FSExpr fs_eval(const EBPoly& f,FSExpr x,FSExpr y){FSExpr r;for(auto& [e,c]:f.terms){FSExpr term(c);for(int i=0;i<e[0];++i)term=term*x;for(int i=0;i<e[1];++i)term=term*y;r=r+term;}return r;}
FSExpr fs_at(const FSCubic& a,FSExpr x){return a[0]+x*(a[1]+x*a[2]);}
FSExpr fs_trace(const FSCubic& a,const FSCubic& u){return FSExpr(3)*a[0]-u[2]*a[1]+(u[2]*u[2]-FSExpr(2)*u[1])*a[2];}
FSExpr fs_det(const std::array<FSCubic,3>& m){return m[0][0]*(m[1][1]*m[2][2]-m[1][2]*m[2][1])-m[0][1]*(m[1][0]*m[2][2]-m[1][2]*m[2][0])+m[0][2]*(m[1][0]*m[2][1]-m[1][1]*m[2][0]);}
Z fs_det_z(const std::array<std::array<Z,3>,3>& m){return m[0][0]*(m[1][1]*m[2][2]-m[1][2]*m[2][1])-m[0][1]*(m[1][0]*m[2][2]-m[1][2]*m[2][0])+m[0][2]*(m[1][0]*m[2][1]-m[1][1]*m[2][0]);}
FSVector fs_product(const FSVector& a,const FSVector& b){FSVector r{};for(int i=0;i<FS_N;++i)for(int j=0;i+j<FS_N;++j)if(a[i]!=Z(0)&&b[j]!=Z(0))r[i+j]=r[i+j]+a[i]*b[j];return r;}
void fs_emit(std::ostream& out,const std::vector<Z>& v){out<<'[';for(size_t i=0;i<v.size();++i)out<<(i?",":"")<<'"'<<v[i].x<<'"';out<<']';}
void fs_emit(std::ostream& out,const FSVector& v){fs_emit(out,std::vector<Z>(v.begin(),v.end()));}
struct FSFit {int rank=0,nullity=0,heldout=0;bool success=false;std::vector<Z> chosen;};
FSFit fs_fit(const FSVector& f,const FSVector& w){
    auto row=[&](int n){EBRow r(107,Z(0));for(int i=0;i<=36;++i){int k=72-2*i;if(n==k)r[i]=-Z(1);if(n>=k)r[70+i]=f[n-k];}for(int i=0;i<=32;++i){int k=65-2*i;if(n>=k)r[37+i]=-w[n-k];}return r;};
    EBEchelon e(107);for(int n=0;n<192;++n)e.insert(row(n));auto kernel=e.kernel();FSFit fit;fit.rank=int(e.rows.size());fit.nullity=int(kernel.size());bool pass=!kernel.empty();
    for(auto& v:kernel){bool denominator=false;for(int i=70;i<107;++i)denominator|=v[i]!=Z(0);if(!denominator)pass=false;
        for(int n=192;n<FS_N;++n){Z sum;auto r=row(n);for(int j=0;j<107;++j)sum=sum+r[j]*v[j];if(sum!=Z(0))pass=false;++fit.heldout;}
        if(fit.chosen.empty()&&denominator)fit.chosen=v;
    }fit.success=pass;return fit;
}
int main(int argc,char** argv){try{
    require(argc==3,"usage: formal_series kernel3-study backmap-result");std::string root=argv[1];initialize_cover(read_json(root+"/inputs/p192-d7-prym.json"));Level2Input input;
    auto model=read_json(root+"/evidence/p192-d7-kernel3-plane-quartic.json"),twist=read_json(root+"/evidence/differential-twist-v2/result.json"),points=read_json(root+"/evidence/destination-relations-v2/result.json"),back=read_json(argv[2]);EBPoly f;
    for(auto& term:model.at("terms").arr){auto e=term.at("exponents");f.add({e.arr[0].integer(),e.arr[1].integer()},Z(mpz_class(term.at("coefficient").scalar)));}auto fx=f.partial(0),fy=f.partial(1);
    Z sqrt_s(mpz_class(back.at("sqrt_scalar").scalar));require(sqrt_s*sqrt_s==eb_z(read_t24(twist.at("common_scalar"))),"unchanged differential square root");std::array<std::array<Z,3>,3> differential{};
    for(int i=0;i<3;++i){auto line=read_line(twist.at("projective_differential").arr[i]);for(int j=0;j<3;++j)differential[i][j]=sqrt_s*eb_z(line[j]);}require(fs_det_z(differential)!=Z(0),"formal differential nonsingular");
    std::vector<int> indices;std::vector<std::array<Z,2>> seed;
    for(int k=int(points.at("points").arr.size())-1;k>=0&&seed.size()<3;--k){auto p=points.at("points").arr[k];std::array<Z,2> a{Z(mpz_class(p.at("x").scalar)),Z(mpz_class(p.at("y").scalar))};require(f.eval(a[0],a[1])==Z(0),"formal reference point curve");if(fy.eval(a[0],a[1])==Z(0))continue;bool okay=true;for(auto& b:seed)okay&=a[0]!=b[0];if(seed.size()==2){std::array<std::array<Z,3>,3> m{{{seed[0][0],seed[0][1],Z(1)},{seed[1][0],seed[1][1],Z(1)},{a[0],a[1],Z(1)}}};okay&=fs_det_z(m)!=Z(0);}if(okay){indices.push_back(k);seed.push_back(a);}}
    require(seed.size()==3,"regular rational divisor seed");Poly<Z> u{Z(1)},v;for(auto& a:seed)u=u*Poly<Z>{-a[0],Z(1)};for(int i=0;i<3;++i){Poly<Z> l{Z(1)};Z den(1);for(int j=0;j<3;++j)if(i!=j){l=l*Poly<Z>{-seed[j][0],Z(1)};den=den*(seed[i][0]-seed[j][0]);}v=v+l.scale(seed[i][1]/den);}
    FSVector w{},iw{},rev{};w[0]=iw[0]=rev[0]=Z(1);for(int i=0;i<7;++i)rev[14-2*i]=scalar(curve.at(i));for(int n=1;n<FS_N;++n){Z sum;for(int j=1;j<n;++j)sum=sum+w[j]*w[n-j];w[n]=(rev[n]-sum)/Z(2);sum=Z(0);for(int j=1;j<=n;++j)sum=sum+w[j]*iw[n-j];iw[n]=-sum;}require(fs_product(w,w)==rev,"formal source square root identity");
    FSDag dag;fs_graph=&dag;require(dag.constant(Z(0))==0,"DAG zero identity");FSCubic U,V,velocity;std::vector<std::pair<FSExpr,FSExpr>> integrate;for(int i=0;i<3;++i){U[i]=FSExpr::node(dag.input(u.at(i)));V[i]=FSExpr::node(dag.input(v.at(i)));velocity[i]=FSExpr::node(dag.input(Z(0)));for(int n=0;n<FS_N;++n)for(int j=0;j<3;++j){int shift=4-2*j;if(n>=shift)dag.nodes[velocity[i].id]->c[n]=dag.nodes[velocity[i].id]->c[n]-differential[i][j]*iw[n-shift];}}
    FSCubic X{FSExpr(0),FSExpr(1),FSExpr(0)},one{FSExpr(1),FSExpr(0),FSExpr(0)};std::array<FSCubic,3> canonical{X,V,one},S{};
    for(int i=0;i<3;++i){auto q=canonical[i];for(int j=0;j<3;++j){S[i][j]=fs_trace(q,U);q=fs_mul(q,X,U);}}auto det=fs_det(S);FSCubic H{};
    for(int j=0;j<3;++j){auto m=S;for(int i=0;i<3;++i)m[i][j]=velocity[i];H[j]=fs_det(m)/det;}
    auto dx=fs_mul(fs_curve(fy,X,V,U),H,U),dy=fs_neg(fs_mul(fs_curve(fx,X,V,U),H,U));FSCubic Ut{U[1],FSExpr(2)*U[2],FSExpr(3)},Vt{V[1],FSExpr(2)*V[2],FSExpr(0)};
    auto Ud=fs_neg(fs_mul(Ut,dx,U)),Vd=fs_add(dy,fs_neg(fs_mul(Vt,dx,U)));for(int j=0;j<3;++j){integrate.emplace_back(U[j],Ud[j]);integrate.emplace_back(V[j],Vd[j]);}
    std::array<FSExpr,3> bx,by;std::vector<FSExpr> zero_checks;
    for(int i=0;i<3;++i){bx[i]=FSExpr::node(dag.input(seed[i][0]));by[i]=FSExpr::node(dag.input(seed[i][1]));auto h=fs_at(H,bx[i]);integrate.emplace_back(bx[i],fs_eval(fy,bx[i],by[i])*h);integrate.emplace_back(by[i],-fs_eval(fx,bx[i],by[i])*h);zero_checks.push_back(fs_eval(f,bx[i],by[i]));zero_checks.push_back(fs_at(U,bx[i])+bx[i]*bx[i]*bx[i]);zero_checks.push_back(fs_at(V,bx[i])-by[i]);}
    zero_checks.push_back(U[2]+bx[0]+bx[1]+bx[2]);zero_checks.push_back(U[1]-bx[0]*bx[1]-bx[0]*bx[2]-bx[1]*bx[2]);zero_checks.push_back(U[0]+bx[0]*bx[1]*bx[2]);for(auto z:fs_curve(f,X,V,U))zero_checks.push_back(z);
    for(int i=0;i<3;++i){FSExpr sum;for(int j=0;j<3;++j)sum=sum+S[i][j]*H[j];zero_checks.push_back(sum-velocity[i]);}
    auto v2=fs_mul(V,V,U),v3=fs_mul(v2,V,U);auto tr=fs_trace(V,U),tr2=fs_trace(v2,U),tr3=fs_trace(v3,U);std::array<FSExpr,6> symmetric{-U[2],U[1],-U[0],tr,(tr*tr-tr2)/FSExpr(2),(tr*tr*tr-FSExpr(3)*tr*tr2+FSExpr(2)*tr3)/FSExpr(6)};
    std::cerr<<"formal DAG nodes="<<dag.nodes.size()<<" seed="<<indices[0]<<','<<indices[1]<<','<<indices[2]<<'\n';
    for(int n=0;n<FS_N;++n){dag.coefficient(n);if(n+1<FS_N)for(auto [variable,rhs]:integrate)dag.nodes[variable.id]->c[n+1]=dag.value(rhs)[n]/Z(n+1);if(n%32==31)std::cerr<<"formal coefficient "<<n<<" complete\n";}
    int checks=0;for(auto expr:zero_checks)for(auto& c:dag.value(expr)){require(c==Z(0),"formal branch/trace/quartic identity");++checks;}require(dag.value(det)[0]!=Z(0),"formal Abel seed determinant unit");
    std::array<std::array<Z,3>,3> singular{{{seed[0][0],seed[0][1],Z(1)},{seed[0][0],seed[0][1],Z(1)},{seed[2][0],seed[2][1],Z(1)}}};require(fs_det_z(singular)==Z(0),"singular seed control");require(fy.eval(seed[0][0],seed[0][1])!=Z(0),"changed branch first coefficient rejects");
    std::array<FSFit,6> fits;bool fitted=true;for(int i=0;i<6;++i){fits[i]=fs_fit(dag.value(symmetric[i]),w);fitted&=fits[i].success;std::cerr<<"fit "<<i<<" rank="<<fits[i].rank<<" nullity="<<fits[i].nullity<<" heldout="<<fits[i].success<<'\n';}
    std::cout<<"{\"scope\":\"P192-kernel3-rational-reference-formal-reconstruction\",\"standing\":\"producer-unverified\",\"precision\":"<<FS_N<<",\"training_coefficients\":192,\"heldout_coefficients\":32,\"seed_indices\":["<<indices[0]<<','<<indices[1]<<','<<indices[2]<<"],\"dag_nodes\":"<<dag.nodes.size()<<",\"native_identity_checks\":"<<checks<<",\"singular_seed_control\":true,\"changed_branch_control\":true,\"all_symmetric_fits_pass\":"<<(fitted?"true":"false")<<",\"global_correspondence_checked\":false,\"generic_isogeny_established\":false,\"differential\":[";
    for(int i=0;i<3;++i){if(i)std::cout<<',';fs_emit(std::cout,std::vector<Z>(differential[i].begin(),differential[i].end()));}std::cout<<"],\"fits\":[";
    for(int i=0;i<6;++i){if(i)std::cout<<',';auto& a=fits[i];std::cout<<"{\"index\":"<<i<<",\"rank\":"<<a.rank<<",\"nullity\":"<<a.nullity<<",\"heldout_checks\":"<<a.heldout<<",\"success\":"<<(a.success?"true":"false")<<",\"candidate_coefficients_A37_B33_C37\":";fs_emit(std::cout,a.chosen);std::cout<<",\"series\":";fs_emit(std::cout,dag.value(symmetric[i]));std::cout<<'}';}std::cout<<"],\"source_w_series\":";fs_emit(std::cout,w);std::cout<<"}\n";return fitted?0:2;
}catch(const std::exception& e){std::cerr<<"REJECT "<<e.what()<<'\n';return 1;}}
