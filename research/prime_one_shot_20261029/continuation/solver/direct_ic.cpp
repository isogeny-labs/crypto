// Direct mixed-target IC correctness baseline on one fixed tiny curve.
// The PDP coefficient census remains exponential; no performance claim is made.
#define NORM_SOLVER_NO_MAIN
#include "norm_solver.cpp"
#define POINT_ORACLE_NO_MAIN
#include "point_oracle.cpp"
#include <array>
#include <filesystem>
#include <fstream>
#include <optional>
#include <random>

namespace direct {
using reference::Point;
constexpr int order=19;
enum class Phase { Setup, Generation, Relation, Verification };
struct Counters {
    std::uint64_t setup_curve_pairs=0,setup_membership_tests=0;
    std::array<std::uint64_t,4> point_add_calls{};
    std::uint64_t rng_words=0,attempts=0,identity_misses=0,pdp_misses=0;
    std::uint64_t pdp_calls=0,coefficient_pairs=0,root_field_candidates=0;
    std::uint64_t accepted_rows=0,batches=0,zero_denominator_batches=0;
    std::uint64_t matrix_modular_reductions=0,recovery_modular_reductions=0;
};
struct Arithmetic {
    std::uint64_t*reductions;
    int mod(long long x) const {++*reductions;x%=order;return static_cast<int>(x<0?x+order:x);}
    int inverse(int x)const{
        x=mod(x);if(!x)throw std::runtime_error("zero scalar inverse");
        int y=1;for(unsigned e=order-2;e;e>>=1,x=mod(x*x))if(e&1U)y=mod(y*x);
        return y;
    }
};
struct Kernel {std::vector<int>w;std::size_t rank=0;};
// This routine's entire input is the coefficient matrix C and arithmetic counter.
// Labels a,b, Q and any source scalar are deliberately absent from its API.
Kernel left_null(const std::vector<std::vector<int>>&C,Arithmetic field) {
    if(C.empty()||C[0].empty())throw std::runtime_error("empty coefficient matrix");
    const std::size_t rows=C.size(),cols=C[0].size();
    for(const auto&r:C)if(r.size()!=cols)throw std::runtime_error("ragged coefficient matrix");
    std::vector<std::vector<int>> T(cols,std::vector<int>(rows));
    for(std::size_t i=0;i<rows;++i)for(std::size_t j=0;j<cols;++j)T[j][i]=field.mod(C[i][j]);
    std::vector<int> pivot(rows,-1);std::size_t rank=0;
    for(std::size_t col=0;col<rows&&rank<cols;++col){
        std::size_t found=rank;while(found<cols&&T[found][col]==0)++found;
        if(found==cols)continue;
        std::swap(T[found],T[rank]);int inverse=field.inverse(T[rank][col]);
        for(std::size_t j=col;j<rows;++j)T[rank][j]=field.mod(T[rank][j]*inverse);
        for(std::size_t i=0;i<cols;++i)if(i!=rank&&T[i][col]!=0){
            int factor=T[i][col];for(std::size_t j=col;j<rows;++j)T[i][j]=field.mod(T[i][j]-factor*T[rank][j]);
        }
        pivot[col]=static_cast<int>(rank++);
    }
    std::size_t free_col=0;while(free_col<rows&&pivot[free_col]>=0)++free_col;
    if(free_col==rows)throw std::runtime_error("no nonzero left-null vector");
    std::vector<int>w(rows,0);w[free_col]=1;
    for(std::size_t col=0;col<rows;++col)if(pivot[col]>=0)w[col]=field.mod(-T[static_cast<std::size_t>(pivot[col])][free_col]);
    for(std::size_t j=0;j<cols;++j){int sum=0;for(std::size_t i=0;i<rows;++i)sum=field.mod(sum+w[i]*C[i][j]);if(sum)throw std::runtime_error("invalid left-null vector");}
    return {w,rank};
}
struct Recovery {std::optional<int>scalar;int alpha=0,beta=0;};
Recovery recover(const std::vector<int>&w,const std::vector<int>&a,const std::vector<int>&b,Arithmetic field) {
    if(w.size()!=a.size()||w.size()!=b.size())throw std::runtime_error("label dimension mismatch");
    int alpha=0,beta=0;
    for(std::size_t i=0;i<w.size();++i){alpha=field.mod(alpha+w[i]*a[i]);beta=field.mod(beta+w[i]*b[i]);}
    if(beta==0){if(alpha!=0)throw std::runtime_error("zero-denominator group relation is inconsistent");return {std::nullopt,alpha,beta};}
    return {field.mod(-alpha*field.inverse(beta)),alpha,beta};
}
struct RelationTrace {std::uint64_t batch,attempt;int a,b,rx,ry;std::vector<int>c;std::string model;};
struct KernelTrace {std::uint64_t batch;std::size_t rank;std::vector<int>w;int alpha,beta;bool retry;};
struct Output {int scalar=0;std::size_t final_rank=0;Counters counts;std::vector<RelationTrace>relations;std::vector<KernelTrace>kernels;};
class Solver {
    reference::Curve E{17,2,2};
    Point P{5,1,false},Q;
    Counters counts;
    std::vector<Point> base;
    std::vector<RelationTrace> relations;
    std::vector<KernelTrace> kernels;
    std::mt19937_64 rng;
    Point add(Point a,Point b,Phase phase){++counts.point_add_calls[static_cast<std::size_t>(phase)];return E.sum(a,b);}
    Point multiply(Point p,int k,Phase phase){
        Point out;
        while(k){if(k&1)out=add(out,p,phase);k>>=1;if(k)p=add(p,p,phase);}
        return out;
    }
    int random_scalar(){
        constexpr std::uint64_t bound=19;
        constexpr std::uint64_t threshold=(std::uint64_t(0)-bound)%bound;
        std::uint64_t word=0;do{word=rng();++counts.rng_words;}while(word<threshold);
        return static_cast<int>(word%bound);
    }
    void cold_setup(){
        if(Q.infinity||Q.x<0||Q.x>=E.p||Q.y<0||Q.y>=E.p||!E.contains(Q))throw std::invalid_argument("nonzero canonical rational Q required");
        std::vector<Point> points;
        for(int x=0;x<E.p;++x)for(int y=0;y<E.p;++y){++counts.setup_curve_pairs;if(E.contains({x,y,false}))points.push_back({x,y,false});}
        if(points.size()+1!=order||!coeff::prime(order))throw std::runtime_error("fixture curve order changed");
        for(Point p:points)if(p.y<=E.p/2){++counts.setup_membership_tests;if(E.powmod(p.x,8)==1)base.push_back(p);}
        if(base.size()!=3)throw std::runtime_error("fixture signed-base dimension changed");
        if(!multiply(P,order,Phase::Setup).infinity||!multiply(Q,order,Phase::Setup).infinity)throw std::runtime_error("subgroup membership failure");
    }
    std::vector<int> bind_model(const std::string&model,Point R){
        std::vector<int> row(base.size(),0);std::stringstream input(model);std::string token;Point sum;
        std::size_t factors=0;
        while(std::getline(input,token,';')){
            auto cut=token.find(':');if(cut==std::string::npos)throw std::runtime_error("bad PDP model");
            Point p{std::stoi(token.substr(0,cut)),std::stoi(token.substr(cut+1)),false};
            if(!E.contains(p)||p.x==R.x||E.powmod(p.x,8)!=1)throw std::runtime_error("invalid PDP point");
            auto it=std::find_if(base.begin(),base.end(),[&](Point b){return b.x==p.x;});
            if(it==base.end())throw std::runtime_error("PDP point absent from base");
            std::size_t index=static_cast<std::size_t>(it-base.begin());
            if(row[index]!=0)throw std::runtime_error("repeated PDP abscissa");
            if(p.y==it->y)row[index]=1;
            else if(p.y==E.residue(-it->y))row[index]=order-1;
            else throw std::runtime_error("PDP point sign mismatch");
            sum=add(sum,p,Phase::Relation);++factors;
        }
        if(factors!=3||!reference::equal(sum,R))throw std::runtime_error("PDP relation verification failed");
        return row;
    }
public:
    // Q and a development RNG seed are the complete target-dependent inputs.
    // In particular, this class never receives the expected discrete logarithm.
    Solver(Point target,std::uint64_t seed):Q(target),rng(seed){}
    Output solve(){
        cold_setup();
        for(unsigned batch=0;batch<64;++batch){
            ++counts.batches;std::vector<std::vector<int>> C;std::vector<int>a,b;
            while(C.size()<base.size()+1){
                if(counts.attempts>=4096)throw std::runtime_error("development attempt cap reached");
                ++counts.attempts;int aa=random_scalar(),bb=random_scalar();
                Point R=add(multiply(P,aa,Phase::Generation),multiply(Q,bb,Phase::Generation),Phase::Generation);
                if(R.infinity){++counts.identity_misses;continue;}
                // The PDP gets only the public curve/domain and R, never aa,bb or Q.
                auto solved=coeff::solve({{17},2,2,3,1,R.x,R.y});
                ++counts.pdp_calls;counts.coefficient_pairs+=solved.coefficient_pairs;counts.root_field_candidates+=solved.root_field_candidates;
                if(solved.models.empty()){++counts.pdp_misses;continue;}
                // The canonical choice depends on the point-model set only.
                C.push_back(bind_model(*solved.models.begin(),R));a.push_back(aa);b.push_back(bb);++counts.accepted_rows;
                relations.push_back({counts.batches,counts.attempts,aa,bb,R.x,R.y,C.back(),*solved.models.begin()});
            }
            Kernel kernel=left_null(C,{&counts.matrix_modular_reductions});
            auto recovered=recover(kernel.w,a,b,{&counts.recovery_modular_reductions});
            kernels.push_back({counts.batches,kernel.rank,kernel.w,recovered.alpha,recovered.beta,!recovered.scalar.has_value()});
            if(!recovered.scalar){++counts.zero_denominator_batches;continue;}
            if(!reference::equal(multiply(P,*recovered.scalar,Phase::Verification),Q))throw std::runtime_error("returned scalar failed group verification");
            if(counts.attempts!=counts.identity_misses+counts.pdp_misses+counts.accepted_rows||counts.pdp_calls!=counts.pdp_misses+counts.accepted_rows||counts.coefficient_pairs!=272*counts.pdp_calls)throw std::runtime_error("incomplete attempt accounting");
            return {*recovered.scalar,kernel.rank,counts,relations,kernels};
        }
        throw std::runtime_error("development batch cap reached");
    }
};
void unit_linear_checks(){
    std::uint64_t reductions=0;
    const std::vector<std::vector<std::vector<int>>> matrices={
        {{0,0,0},{0,0,0},{0,0,0},{0,0,0}},
        {{1,0,0},{2,0,0},{3,0,0},{4,0,0}},
        {{1,0,0},{0,1,0},{1,1,0},{2,3,0}},
        {{1,0,0},{0,1,0},{0,0,1},{1,1,1}}};
    for(std::size_t rank=0;rank<matrices.size();++rank){Kernel k=left_null(matrices[rank],{&reductions});if(k.rank!=rank)throw std::runtime_error("rank-free kernel control failed");}
    Kernel k=left_null(matrices[0],{&reductions});
    if(recover(k.w,{0,0,0,0},{0,0,0,0},{&reductions}).scalar)throw std::runtime_error("zero denominator accepted");
    auto scalar=recover(k.w,{5,0,0,0},{3,0,0,0},{&reductions});
    if(!scalar.scalar||*scalar.scalar!=11)throw std::runtime_error("scalar recovery control failed");
}
} // namespace direct
int main(int argc,char**argv){
    try{
        if(argc==5&&std::string(argv[1])=="--solve"){
            reference::Point Q{reference::read_int(argv[2]),reference::read_int(argv[3]),false};
            std::string seed_text(argv[4]);
            if(seed_text.empty()||seed_text.find_first_not_of("0123456789")!=std::string::npos)throw std::invalid_argument("decimal RNG seed required");
            std::uint64_t seed=std::stoull(seed_text);
            auto answer=direct::Solver(Q,seed).solve();const auto&c=answer.counts;
            std::cout<<"result\t"<<answer.scalar<<'\t'<<c.attempts<<'\t'<<c.pdp_calls<<'\t'<<c.coefficient_pairs<<'\t'<<c.accepted_rows<<'\t'<<c.batches<<'\t'<<c.zero_denominator_batches<<'\t'<<answer.final_rank<<"\tverified\n";
            return 0;
        }
        if((argc!=2&&argc!=3)||std::string(argv[1])!="--selftest")throw std::invalid_argument("usage: direct_ic --selftest [TRACE_DIR] | --solve Qx Qy Seed");
        std::ofstream relation_file,kernel_file;
        if(argc==3){
            std::filesystem::create_directories(argv[2]);
            relation_file.open(std::filesystem::path(argv[2])/"direct-relations.tsv");
            kernel_file.open(std::filesystem::path(argv[2])/"direct-kernels.tsv");
            if(!relation_file||!kernel_file)throw std::runtime_error("cannot open trace files");
            relation_file<<"Qx\tQy\tbatch\tattempt\ta\tb\tRx\tRy\tc0\tc1\tc2\tmodel\n";
            kernel_file<<"Qx\tQy\tbatch\tC_rank\tw0\tw1\tw2\tw3\talpha\tbeta\tretry\n";
        }
        direct::unit_linear_checks();
        std::cout<<"fixture_expected\tQx\tQy\trecovered\tB\tfinal_C_rank\tsetup_curve_pairs\tsetup_membership_tests\tattempts\tidentity_misses\tpdp_misses\tpdp_calls\tcoefficient_pairs\troot_field_candidates\taccepted_rows\tbatches\tzero_denominator_batches\tsetup_group_add_calls\tgeneration_group_add_calls\trelation_group_add_calls\tverification_group_add_calls\tmatrix_modular_reductions\trecovery_modular_reductions\trng_words\n";
        // Fixture generation is outside Solver. Only the resulting point enters it.
        reference::Curve E{17,2,2};reference::Point P{5,1,false},Q;
        for(int expected=1;expected<direct::order;++expected){
            Q=E.sum(Q,P);
            std::uint64_t seed=0x4e6f726d444c50ULL^(static_cast<std::uint64_t>(Q.x)<<32)^static_cast<std::uint64_t>(Q.y);
            auto result=direct::Solver(Q,seed).solve();
            if(result.scalar!=expected)throw std::runtime_error("fixture answer mismatch");
            if(relation_file.is_open()){
                for(const auto&r:result.relations){relation_file<<Q.x<<'\t'<<Q.y<<'\t'<<r.batch<<'\t'<<r.attempt<<'\t'<<r.a<<'\t'<<r.b<<'\t'<<r.rx<<'\t'<<r.ry;for(int c:r.c)relation_file<<'\t'<<c;relation_file<<'\t'<<r.model<<'\n';}
                for(const auto&k:result.kernels){kernel_file<<Q.x<<'\t'<<Q.y<<'\t'<<k.batch<<'\t'<<k.rank;for(int w:k.w)kernel_file<<'\t'<<w;kernel_file<<'\t'<<k.alpha<<'\t'<<k.beta<<'\t'<<(k.retry?1:0)<<'\n';}
            }
            const auto&c=result.counts;
            std::cout<<expected<<'\t'<<Q.x<<'\t'<<Q.y<<'\t'<<result.scalar<<"\t3\t"<<result.final_rank<<'\t'<<c.setup_curve_pairs<<'\t'<<c.setup_membership_tests<<'\t'<<c.attempts<<'\t'<<c.identity_misses<<'\t'<<c.pdp_misses<<'\t'<<c.pdp_calls<<'\t'<<c.coefficient_pairs<<'\t'<<c.root_field_candidates<<'\t'<<c.accepted_rows<<'\t'<<c.batches<<'\t'<<c.zero_denominator_batches<<'\t'<<c.point_add_calls[0]<<'\t'<<c.point_add_calls[1]<<'\t'<<c.point_add_calls[2]<<'\t'<<c.point_add_calls[3]<<'\t'<<c.matrix_modular_reductions<<'\t'<<c.recovery_modular_reductions<<'\t'<<c.rng_words<<'\n';
        }
        if(argc==3){relation_file.flush();kernel_file.flush();if(!relation_file||!kernel_file)throw std::runtime_error("incomplete trace output");}
        std::cerr<<"PASS direct_IC_targets=18 known_FB_logs=0 source_scalar_inputs=0 kernel_rank_controls=4 zero_denominator_control=PASS timings=not_collected\n";
        return 0;
    }catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 2;}
}
