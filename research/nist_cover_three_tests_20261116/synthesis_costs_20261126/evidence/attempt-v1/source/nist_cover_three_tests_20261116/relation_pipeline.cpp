#include "public_classes_arithmetic.hpp"
#include <chrono>
#include <ctime>

struct RPStage {std::string name;long long wall_us,cpu_us;};
struct RPTimer {
    std::chrono::steady_clock::time_point wall=std::chrono::steady_clock::now();
    std::clock_t cpu=std::clock();
    std::vector<RPStage> stages;
    void mark(const std::string& name){
        auto now=std::chrono::steady_clock::now();auto ticks=std::clock();
        stages.push_back({name,std::chrono::duration_cast<std::chrono::microseconds>(now-wall).count(),
            static_cast<long long>(1000000.0*(ticks-cpu)/CLOCKS_PER_SEC)});
        wall=now;cpu=ticks;
    }
};
void rp_point(std::ostream& out,const EC& p){
    out<<"{\"infinity\":"<<(p.zero?"true":"false");
    if(!p.zero)out<<",\"x\":\""<<scalar(p.x).x<<"\",\"y\":\""<<scalar(p.y).x<<'"';
    out<<'}';
}
void rp_pool(std::ostream& out,const std::vector<BasePoint>& points,const std::vector<Fiber>& fibers){
    out<<"{\"points\":[";
    for(size_t i=0;i<points.size();++i){if(i)out<<',';out<<"{\"x\":\""<<points[i].x<<"\",\"y\":\""<<points[i].y.x<<"\"}";}
    out<<"],\"split_fibers\":[";
    for(size_t i=0;i<fibers.size();++i){if(i)out<<',';out<<"{\"x\":"<<fibers[i].x<<",\"point_indices\":[";
        for(size_t j=0;j<fibers[i].support.size();++j){out<<(j?",":"")<<fibers[i].support[j];}out<<"]}";}
    out<<"]}";
}
int main(int argc,char** argv){try{
    RPTimer timer;
    require(argc==5,"usage: relation_pipeline kernel3-study backmap.json public-classes.json output.json");
    std::string root=argv[1];initialize_cover(read_json(root+"/inputs/p192-d7-prym.json"));Level2Input level;
    auto model=read_json(root+"/evidence/p192-d7-kernel3-plane-quartic.json");
    auto predecessor=read_json(root+"/evidence/destination-relations-v2/result.json");
    auto map_input=read_json(argv[2]),public_input=read_json(argv[3]);
    require(model.at("kernel").integer()==3,"actual kernel3 input");BRMap map(model,map_input);
    mpz_class order("6277101735386680763835789423176059013767194773182842284081");
    timer.mark("input_and_field_setup");
    std::vector<BasePoint> points;std::vector<Fiber> fibers;Poly<Z> variable{Z(0),Z(1)};
    auto polynomial=[&](int x){std::vector<Z> c(5);for(auto& [e,a]:map.curve.terms)c[size_t(e[1])]=c[size_t(e[1])]+a*zpow(Z(x),e[0]);return Poly<Z>(c);};
    for(int x=0;x<256;++x){
        auto f=polynomial(x);require(f.degree()==4,"degree4 vertical chart");
        auto rational=std::get<0>(xgcd(f,powmod(variable,prime,f)-variable));std::vector<Z> roots;
        split_linear_part(rational,roots);std::sort(roots.begin(),roots.end(),[](const Z& a,const Z& b){return a.x<b.x;});
        require(int(roots.size())==rational.degree(),"complete rational root list");Fiber fiber{x,{}};
        for(auto& y:roots){require(f.eval(y)==Z(0),"fresh point equation");fiber.support.push_back(int(points.size()));points.push_back({x,y});}
        if(roots.size()==4){Poly<Z> product{f.c.back()};for(auto y:roots)product=product*Poly<Z>{-y,Z(1)};
            require(product==f,"full simple vertical fiber factorization");fibers.push_back(fiber);}
    }
    require(points.size()>=85&&fibers.size()>=2,"fixed pool and relation coverage");
    require(points.size()==predecessor.at("points").arr.size()&&fibers.size()==predecessor.at("split_fibers").arr.size(),"predecessor complete counts");
    for(size_t i=0;i<points.size();++i){auto& p=predecessor.at("points").arr[i];require(points[i].x==p.at("x").integer()&&points[i].y.x==mpz_class(p.at("y").scalar),"complete ordered point pool consistency");}
    for(size_t i=0;i<fibers.size();++i){auto& f=predecessor.at("split_fibers").arr[i];require(fibers[i].x==f.at("x").integer(),"ordered fiber x consistency");
        for(int j=0;j<4;++j)require(fibers[i].support[size_t(j)]==f.at("point_indices").arr[size_t(j)].integer(),"complete fiber support consistency");}
    timer.mark("collect_all_256_vertical_candidates");
    std::ostringstream pool;rp_pool(pool,points,fibers);auto fresh=Parser{pool.str()}.value();QuarticKM km(model,fresh);
    timer.mark("target_KM_setup");
    std::vector<Class> target_classes;
    for(size_t i=1;i<fibers.size();++i){std::map<int,int> divisor;for(int j:fibers[0].support)divisor[j]=2;for(int j:fibers[i].support)++divisor[j];
        auto c=km.effective(divisor);require(km.zero(c),"actual target Jacobian principal relation");target_classes.push_back(c);}
    timer.mark("all_target_Jacobian_relation_checks");
    std::vector<BRImage> images;std::vector<EC> base;std::vector<std::array<int,2>> projection;std::map<std::string,int> index;
    int local=0,infinity=0;F b(Poly<Z>{Z(mpz_class("2455155546008943817740293915197451784769108058161191238065"))});
    for(auto& p:points){auto image=map.image(Z(p.x),p.y);images.push_back(image);if(image.local)++local;auto q=image.point;
        if(q.zero){++infinity;projection.push_back({-1,0});continue;}
        require(q.y*q.y==q.x*q.x*q.x-F(3)*q.x+b,"projected exact P192 equation");require(times(q,order).zero,"projected subgroup");
        int sign=1;if(scalar(q.y).x>prime-scalar(q.y).x){q=q.neg();sign=-1;}
        auto key=scalar(q.x).x.get_str()+":"+scalar(q.y).x.get_str();auto [it,inserted]=index.emplace(key,int(base.size()));if(inserted)base.push_back(q);
        projection.push_back({it->second,sign});}
    timer.mark("all_point_norms_subgroup_checks_and_deduplication");
    std::vector<std::vector<mpz_class>> rows;int zero=0;
    for(size_t i=1;i<fibers.size();++i){std::vector<mpz_class> row(base.size());
        for(int j:fibers[i].support)if(projection[size_t(j)][0]>=0)row[size_t(projection[size_t(j)][0])]+=projection[size_t(j)][1];
        for(int j:fibers[0].support)if(projection[size_t(j)][0]>=0)row[size_t(projection[size_t(j)][0])]-=projection[size_t(j)][1];
        EC sum;bool nonzero=false;for(size_t j=0;j<row.size();++j)if(row[j]!=0){nonzero=true;mpz_class n=row[j];EC q=base[j];if(n<0){n=-n;q=q.neg();}sum=sum.add(times(q,n));}
        require(sum.zero,"transported elliptic relation");if(!nonzero)++zero;rows.push_back(row);}
    int rank=br_rank(rows,order);timer.mark("project_rows_check_sums_and_rank");
    rejects([&]{auto corrupted=rows.at(0);auto it=std::find_if(corrupted.begin(),corrupted.end(),[](const mpz_class& n){return n!=0;});require(it!=corrupted.end(),"nonempty row control");++*it;
        EC sum;for(size_t j=0;j<corrupted.size();++j){mpz_class n=corrupted[j];EC q=base[j];if(n<0){n=-n;q=q.neg();}sum=sum.add(times(q,n));}require(sum.zero,"corrupted row sum");},"actual coefficient mutation");
    timer.mark("producer_negative_control");
    std::ofstream out(argv[4],std::ios::out|std::ios::trunc);require(bool(out),"open new pipeline result");
    out<<"{\"standing\":\"producer-unverified\",\"scope\":\"P192-kernel3-fixed-vertical-relation-functional-pipeline\",\"candidate_lines\":256,\"factor_base_points\":"<<points.size()<<",\"split_fibers\":"<<fibers.size()<<",\"relations\":"<<rows.size()<<",\"projected_base_size\":"<<base.size()<<",\"rank_mod_r\":"<<rank<<",\"rank_deficiency\":"<<int(base.size())-rank<<",\"zero_projected_rows\":"<<zero<<",\"local_map_points\":"<<local<<",\"infinity_images\":"<<infinity<<",\"target_KM_zero_checks\":"<<target_classes.size()<<",\"pool\":"<<pool.str()<<",\"images\":[";
    for(size_t i=0;i<images.size();++i){if(i)out<<',';out<<"{\"point\":";rp_point(out,images[i].point);out<<",\"projected_index\":"<<projection[i][0]<<",\"sign\":"<<projection[i][1]<<'}';}
    out<<"],\"projected_base\":[";for(size_t i=0;i<base.size();++i){if(i)out<<',';rp_point(out,base[i]);}
    out<<"],\"rows\":[";for(size_t i=0;i<rows.size();++i){if(i)out<<',';out<<'[';bool first=true;for(size_t j=0;j<base.size();++j)if(rows[i][j]!=0){if(!first)out<<',';first=false;out<<'['<<j<<",\""<<rows[i][j]<<"\"]";}out<<']';}
    out<<"],\"public_composition_companions\":[";auto& companions=public_input.at("public_class_spaces").arr;
    for(size_t i=0;i<companions.size();++i){if(i)out<<',';out<<"{\"point\":";rp_point(out,pc_read_point(companions[i].at("source_point")));out<<",\"norm\":";rp_point(out,pc_read_point(companions[i].at("norm_from_closed_divisor_pair")));out<<'}';}
    out<<"],\"stages\":[";for(size_t i=0;i<timer.stages.size();++i){if(i)out<<',';auto& s=timer.stages[i];out<<"{\"name\":\""<<s.name<<"\",\"wall_us\":"<<s.wall_us<<",\"cpu_us\":"<<s.cpu_us<<'}';}
    out<<"],\"cost_scope\":\"local development diagnostics; excludes predecessor map construction and fresh public fiber construction; not admitted performance\"}\n";
    require(bool(out),"write pipeline result");std::cerr<<"pipeline complete: "<<rows.size()<<" rows, rank "<<rank<<'\n';
}catch(const std::exception& e){std::cerr<<"REJECT "<<e.what()<<'\n';return 1;}}
