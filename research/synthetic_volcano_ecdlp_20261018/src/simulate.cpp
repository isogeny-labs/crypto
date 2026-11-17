// Fully SYNTHETIC volcano/ECDLP cost experiment. No EC arithmetic or DLP solving.
// Native C++17; all cryptographic-looking outputs are invented model values.
#include <algorithm>
#include <array>
#include <cmath>
#include <cstdint>
#include <filesystem>
#include <fstream>
#include <iomanip>
#include <iostream>
#include <map>
#include <numeric>
#include <queue>
#include <random>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

using namespace std;
namespace fs = std::filesystem;
static constexpr double PI = 3.14159265358979323846;
static constexpr array<double,3> WEIGHTS{0.0,0.1,1.0};
static const array<string,3> FAMILIES{"prime","binary","koblitz_origin"};
static const array<string,3> SCENARIOS{"null","planted_floor_advantage","planted_floor_penalty"};
static const array<string,3> METHODS{"rho","ic2_scan","ic3_pair_table"};
static uint64_t SEED = 20261018;
static int DEPTH = 4, TARGETS = 10, PILOTS = 3;

struct Cost {
    double g=0, n=0;
    double at(double w) const { return g+w*n; }
    Cost operator+(const Cost& b) const {return {g+b.g,n+b.n};}
    Cost operator*(double x) const {return {g*x,n*x};}
    Cost& operator+=(const Cost& b) {g+=b.g;n+=b.n;return *this;}
};
struct Stats {double lo,mean,hi,median,sd; int count;};
Stats stats(vector<double> a) {
    if(a.empty()) throw runtime_error("empty statistics");
    sort(a.begin(),a.end()); double mean=accumulate(a.begin(),a.end(),0.0)/a.size(), ss=0;
    for(double x:a) ss+=(x-mean)*(x-mean);
    return {a.front(),mean,a.back(),(a[(a.size()-1)/2]+a[a.size()/2])/2,
            a.size()>1?sqrt(ss/(a.size()-1)):0,int(a.size())};
}
uint64_t hash_key(const string& s) {
    uint64_t x=14695981039346656037ULL ^ SEED;
    for(unsigned char c:s) {x^=c;x*=1099511628211ULL;}
    x+=0x9e3779b97f4a7c15ULL;x=(x^(x>>30))*0xbf58476d1ce4e5b9ULL;
    x=(x^(x>>27))*0x94d049bb133111ebULL;return x^(x>>31);
}
string join_key(const string& f,int b,int v,int m,const string& tag) {
    return f+":"+to_string(b)+":"+to_string(v)+":"+to_string(m)+":"+tag;
}
double uniform01(mt19937_64& rng) {
    return (double(rng()>>11)+0.5)/9007199254740992.0;
}
double normal01(mt19937_64& rng) {
    return sqrt(-2*log(uniform01(rng)))*cos(2*PI*uniform01(rng));
}
uint64_t mod_pow(uint64_t a,uint64_t k,uint64_t n) {
    uint64_t out=1;
    while(k) {if(k&1) out=(__uint128_t(out)*a)%n;a=(__uint128_t(a)*a)%n;k>>=1;}
    return out;
}
bool prime(uint64_t n) {
    if(n<2)return false;
    for(uint64_t p:{2,3,5,7,11,13,17,19,23,29,31,37}) {if(n==p)return true;if(n%p==0)return false;}
    uint64_t d=n-1,s=0;while(!(d&1)){d>>=1;s++;}
    for(uint64_t a:{2ULL,325ULL,9375ULL,28178ULL,450775ULL,9780504ULL,1795265022ULL}) {
        if(a%n==0)continue;
        uint64_t x=mod_pow(a%n,d,n);if(x==1||x==n-1)continue;
        bool ok=false;for(uint64_t j=1;j<s;j++){x=(__uint128_t(x)*x)%n;if(x==n-1){ok=true;break;}}
        if(!ok)return false;
    }return true;
}
uint64_t prime_below(uint64_t n) {uint64_t p=(n-1)|1ULL;while(p>=n||!prime(p))p-=2;return p;}
uint64_t nominal_r(int bits) {
    static map<int,uint64_t> values;
    auto it=values.find(bits);if(it!=values.end())return it->second;
    return values[bits]=prime_below(1ULL<<(bits-2));
}
struct Vertex {int id,level,local,parent;};
struct Edge {int a,b,degree;bool horizontal;};
struct Graph {
    vector<Vertex> v;vector<Edge> e;vector<int> offsets,counts;
    Graph(int d) {
        for(int l=0;l<=d;l++) {
            int count=l==0?1:4*int(pow(3,l-1));offsets.push_back(v.size());counts.push_back(count);
            for(int j=0;j<count;j++) {
                int id=v.size(),parent=l==0?-1:offsets[l-1]+j%counts[l-1];
                v.push_back({id,l,j,parent});if(l)e.push_back({parent,id,3,false});
            }
        }
        // Illustrative cyclic shifts, NOT computed ideal classes/modular polynomials.
        for(int l=0;l<=d;l++) for(auto ds:vector<pair<int,int>>{{11,1},{23,5},{29,7}})
            for(int j=0;j<counts[l];j++)
                e.push_back({offsets[l]+j,offsets[l]+(j+ds.second)%counts[l],ds.first,true});
    }
    vector<vector<int>> adjacency(int overlays) const {
        vector<vector<int>> a(v.size());
        for(auto x:e) {
            int rank=x.degree==11?1:x.degree==23?2:3;
            if(x.horizontal&&rank>overlays)continue;
            if(x.a!=x.b){a[x.a].push_back(x.b);a[x.b].push_back(x.a);}
        }return a;
    }
    pair<double,int> distance_summary(int overlays) const {
        auto a=adjacency(overlays);double sum=0;uint64_t pairs=0;int diameter=0;
        for(int start=0;start<int(v.size());start++) {
            vector<int> dist(v.size(),-1);queue<int> q;q.push(start);dist[start]=0;
            while(!q.empty()) {int x=q.front();q.pop();for(int y:a[x])if(dist[y]<0){dist[y]=dist[x]+1;q.push(y);}}
            for(int end=start+1;end<int(v.size());end++) {
                if(dist[end]<0)throw runtime_error("disconnected graph");
                sum+=dist[end];pairs++;diameter=max(diameter,dist[end]);
            }
        }return {sum/pairs,diameter};
    }
    Cost construction() const {
        Cost c;for(auto x:e)c+=Cost{4.0*x.degree*x.degree,20.0*x.degree};return c;
    }
};
void require(bool b,const string& s) {if(!b)throw runtime_error("validation: "+s);}
int graph_tests(const Graph& g) {
    int checks=0;
    require(g.counts[0]==1,"one inert crater vertex");checks++;
    require(g.e.size()==(g.v.size()-1)+3*g.v.size(),"all overlay multiedges retained");checks++;
    vector<int> degree(g.v.size()),up(g.v.size());
    for(auto e:g.e) {
        if(!e.horizontal){degree[e.a]++;degree[e.b]++;up[e.b]++;require(g.v[e.b].level==g.v[e.a].level+1,"vertical adjacency");}
        else require(g.v[e.a].level==g.v[e.b].level,"horizontal stratum");
        checks++;
    }
    for(auto v:g.v) {
        require(degree[v.id]==(v.level==DEPTH?1:4),"3-volcano valency");
        require(up[v.id]==(v.level?1:0),"unique ascending edge");checks+=2;
    }
    for(int b:{30,40,50}) {auto r=prime_below(1ULL<<(b-2));require(prime(r),"nominal prime");
        for(int ell:{3,11,23,29})require(r%ell!=0,"subgroup/map coprimality");
        checks+=5;}
    require(stats({1,2,3}).mean==2&&stats({1,2,3}).lo==1&&stats({1,2,3}).hi==3,"summary arithmetic");checks++;
    require((Cost{2,3}+Cost{5,7}).at(.1)==8,"cost accounting");checks++;
    require(hash_key("replay")==hash_key("replay"),"stable seed");checks++;
    require(g.distance_summary(3).first<=g.distance_summary(0).first,"overlay cannot lengthen unweighted shortest paths");checks++;
    return checks;
}
struct Run {
    Cost curve,fb,relations,la,online,verify;
    double r=0,p=0,B=0,U=0,orbit=1,oracle=1,memory=0;
    uint64_t relation_attempts=0,descent_attempts=0;
    int relation_rows=0;
    Cost pre()const{return curve+fb+relations+la;}
    Cost tail()const{return online+verify;}
    Cost total()const{return pre()+tail();}
};
Run model(const Vertex& v,const string& family,int bits,int method,int scenario,const string& tag) {
    Run z;z.r=double(nominal_r(bits));double lg=log2(z.r);
    z.orbit=(family=="koblitz_origin"&&v.level==0)?bits:1;
    mt19937_64 rng(hash_key(join_key(family,bits,v.id,method,tag)));
    z.curve={2*lg,64};z.verify={2*lg,6*lg};
    if(method==0) {
        double sigma=sqrt(z.r/(2*z.orbit));
        double steps=ceil(sigma*sqrt(-2*log(uniform01(rng))));
        z.online={steps,steps*(4+(z.orbit>1?2*log2(z.orbit):0))};z.memory=4096;
        return z;
    }
    int m=method==1?2:3;z.B=2*ceil(cbrt(z.r)/2);z.U=ceil(z.B/(2*z.orbit));
    // Random-sum occupancy model. This is an assumption, not a measured yield.
    double lambda=pow(z.B,m)/(tgamma(m+1)*z.r);z.p=-expm1(-lambda);
    z.relation_rows=int(ceil(1.08*z.U)+8); // Full rank is assumed, never claimed checked.
    negative_binomial_distribution<uint64_t> nb(z.relation_rows,z.p);
    z.relation_attempts=nb(rng)+z.relation_rows;
    z.descent_attempts=uint64_t(ceil(log(uniform01(rng))/log1p(-z.p)));
    mt19937_64 vertex_rng(hash_key("vertex:"+family+":"+to_string(v.id)));
    double vertex_factor=exp(.12*normal01(vertex_rng)-.5*.12*.12);
    double trend=scenario==1?pow(.55,v.level):scenario==2?pow(1.7,v.level):1;
    z.oracle=vertex_factor*trend;
    z.fb={1.5*z.B,3*z.B};
    double pairs=z.B*(z.B-1)/2;
    if(m==3)z.fb+=Cost{pairs,3*pairs};
    // A scan sees fewer entries when it succeeds early. Counts are fractional expectations.
    double scan=z.B*(1-z.p/2)*z.oracle;
    z.relations={z.relation_attempts*scan,z.relation_attempts*(3*scan+2*lg)};
    z.la={0,16*(m+1)*z.U*z.U};
    z.online={z.descent_attempts*scan,z.descent_attempts*(3*scan+2*lg)};
    z.memory=48*z.B+16*(m+1)*z.U+(m==3?24*pairs:0);
    return z;
}
Cost path_build(const Vertex& v) {return {double(v.level)*12*3*3,double(v.level)*24};}
Cost path_eval(const Vertex& v,int points=1) {return {double(v.level)*3*3*points,double(v.level)*12*points};}
ofstream file(const fs::path& d,const string& name) {
    ofstream f(d/name);if(!f)throw runtime_error("cannot write "+(d/name).string());f<<setprecision(17);return f;
}
void write_cost(ostream& f,Cost c) {f<<c.g<<','<<c.n;}
void write_stats(ostream& f,const Stats& s) {f<<s.count<<','<<s.lo<<','<<s.mean<<','<<s.hi<<','<<s.median<<','<<s.sd;}
string stat_json(const Stats& s) {
    ostringstream f;f<<setprecision(17)<<"{\"n\":"<<s.count<<",\"min\":"<<s.lo<<",\"mean\":"<<s.mean
     <<",\"max\":"<<s.hi<<",\"median\":"<<s.median<<",\"sd\":"<<s.sd<<'}';return f.str();
}
void run(const fs::path& out) {
    if(fs::exists(out))throw runtime_error("output already exists; choose a new run directory");
    fs::create_directories(out);Graph graph(DEPTH);int tests=graph_tests(graph);
    const int n=graph.v.size();uint64_t raw_count=0,summary_count=0,strategy_count=0;
    auto edges=file(out,"edges.csv");edges<<"status,a,b,degree,kind,existence_status\n";
    for(auto e:graph.e)edges<<"SYNTHETIC,"<<e.a<<','<<e.b<<','<<e.degree<<','<<(e.horizontal?"horizontal_overlay":"vertical")<<",not_constructed\n";
    auto verts=file(out,"vertices.csv");verts<<"status,vertex,level,local_index,parent,relative_conductor,curve_coordinates\n";
    for(auto v:graph.v)verts<<"SYNTHETIC,"<<v.id<<','<<v.level<<','<<v.local<<','<<v.parent<<','<<int(pow(3,v.level))<<",not_constructed\n";
    auto topo=file(out,"topology_ablation.csv");topo<<"status,overlay_degrees,vertices,undirected_multiedges,mean_shortest_path,diameter,solver_effect\n";
    for(int overlay=0;overlay<=3;overlay++){auto d=graph.distance_summary(overlay);topo<<"SYNTHETIC,"<<overlay<<','<<n<<','<<(n-1)+overlay*n<<','<<d.first<<','<<d.second<<",none_by_assumption\n";}
    auto raw=file(out,"trials.csv");
    raw<<"status,family,field_log2,nominal_r,scenario,vertex,level,target,method,orbit_discount,fb_signed_points,fb_unknowns,assumed_success_p,assumed_full_rank_rows,relation_attempts,descent_attempts,oracle_multiplier,modeled_peak_bytes,curve_g,curve_native,fb_g,fb_native,relations_g,relations_native,la_g,la_native,online_g,online_native,verify_g,verify_native,batch_shared_g,batch_shared_native\n";
    auto cs=file(out,"curve_summary.csv");
    cs<<"status,family,field_log2,scenario,vertex,level,method,native_weight,accounting,n,min,mean,max,median,sd,modeled_peak_bytes\n";
    auto ls=file(out,"level_summary.csv");
    ls<<"status,family,field_log2,scenario,level,vertices,method,native_weight,accounting,n,min,mean,max,median,sd,estimand\n";
    auto ss=file(out,"strategy_summary.csv");
    ss<<"status,family,field_log2,scenario,method,strategy,selected_vertex,selected_level,native_weight,accounting,n,min,mean,max,median,sd,candidate_over_surface_rho,modeled_peak_bytes\n";
    auto st=file(out,"strategy_trials.csv");
    st<<"status,family,field_log2,scenario,method,strategy,selected_vertex,native_weight,accounting,target,modeled_work,surface_rho_work\n";
    auto pats=file(out,"patterns.csv");
    pats<<"status,family,field_log2,scenario,method,native_weight,accounting,n,floor_point_means_below_surface,level_mean_floor_over_surface,descriptive_only\n";
    auto pilots=file(out,"pilot_selection.csv");
    pilots<<"status,family,field_log2,scenario,method,native_weight,selection,candidates,pilot_targets,selected_vertex,selected_level,charged_setup_g,charged_setup_native,test_targets_used_for_selection\n";
    auto diag=file(out,"diagnostics.csv");
    diag<<"status,family,field_log2,scenario,method,vertex,level,mean_cold_w01,mean_batch10_w01,mean_curve_w01,mean_fb_w01,mean_relations_w01,mean_la_w01,mean_online_w01,mean_verify_w01,modeled_peak_bytes,fb_signed_points,fb_unknowns,assumed_success_p,oracle_multiplier,orbit_discount\n";
    auto compact=file(out,"dashboard_data.json");compact<<"{\"status\":\"FULLY SYNTHETIC — INVENTED DATA\",\"seed\":"<<SEED
      <<",\"targets\":"<<TARGETS<<",\"depth\":"<<DEPTH<<",\"vertices\":"<<n<<",\"level_counts\":[";
    for(int i=0;i<=DEPTH;i++){if(i)compact<<',';compact<<graph.counts[i];}
    compact<<"],\"levels\":[";bool first_level=true;
    vector<string> curve_json,strategy_json;
    for(const string& family:FAMILIES)for(int bits:{30,40,50})for(int scenario=0;scenario<3;scenario++) {
      vector<array<vector<Run>,3>> trial(n);
      vector<array<Run,3>> shared(n),pilot_pre(n);
      vector<array<vector<Run>,3>> pilot_runs(n);
      for(auto v:graph.v)for(int method=0;method<3;method++) {
        shared[v.id][method]=model(v,family,bits,method,scenario,"batch_shared");
        pilot_pre[v.id][method]=model(v,family,bits,method,scenario,"pilot_shared");
        for(int t=0;t<PILOTS;t++)pilot_runs[v.id][method].push_back(model(v,family,bits,method,scenario,"pilot_"+to_string(t)));
        for(int t=0;t<TARGETS;t++) {
          Run z=model(v,family,bits,method,scenario,"test_"+to_string(t));trial[v.id][method].push_back(z);
          raw<<"SYNTHETIC,"<<family<<','<<bits<<','<<uint64_t(z.r)<<','<<SCENARIOS[scenario]<<','<<v.id<<','<<v.level<<','<<t<<','<<METHODS[method]<<','
            <<z.orbit<<','<<z.B<<','<<z.U<<','<<z.p<<','<<z.relation_rows<<','<<z.relation_attempts<<','<<z.descent_attempts<<','<<z.oracle<<','<<z.memory<<',';
          for(Cost c:{z.curve,z.fb,z.relations,z.la,z.online,z.verify}){write_cost(raw,c);raw<<',';}
          write_cost(raw,shared[v.id][method].pre());raw<<'\n';raw_count++;
          require(z.total().g>=0&&isfinite(z.total().at(1)),"finite nonnegative total");
        }
        array<vector<double>,8> dvals;
        for(auto z:trial[v.id][method]) {
            Cost batch=shared[v.id][method].pre()*(1.0/TARGETS)+z.tail();
            array<double,8> a{z.total().at(.1),batch.at(.1),z.curve.at(.1),z.fb.at(.1),z.relations.at(.1),z.la.at(.1),z.online.at(.1),z.verify.at(.1)};
            for(int i=0;i<8;i++)dvals[i].push_back(a[i]);
        }
        const auto& z=trial[v.id][method][0];
        diag<<"SYNTHETIC,"<<family<<','<<bits<<','<<SCENARIOS[scenario]<<','<<METHODS[method]<<','<<v.id<<','<<v.level;
        for(auto a:dvals)diag<<','<<stats(a).mean;
        diag<<','<<z.memory<<','<<z.B<<','<<z.U<<','<<z.p<<','<<z.oracle<<','<<z.orbit<<'\n';
        ostringstream cj;cj<<setprecision(17)<<"{\"family\":\""<<family<<"\",\"bits\":"<<bits<<",\"scenario\":\""<<SCENARIOS[scenario]<<"\",\"method\":\""<<METHODS[method]
           <<"\",\"vertex\":"<<v.id<<",\"level\":"<<v.level<<",\"cold\":"<<stats(dvals[0]).mean<<",\"batch\":"<<stats(dvals[1]).mean
           <<",\"cold_stats\":"<<stat_json(stats(dvals[0]))<<",\"batch_stats\":"<<stat_json(stats(dvals[1]))
           <<",\"memory\":"<<z.memory<<",\"orbit\":"<<z.orbit<<'}';curve_json.push_back(cj.str());
        for(double w:WEIGHTS)for(int accounting=0;accounting<2;accounting++) {
            vector<double> vals;for(auto z:trial[v.id][method])vals.push_back((accounting?shared[v.id][method].pre()*(1.0/TARGETS)+z.tail():z.total()).at(w));
            cs<<"SYNTHETIC,"<<family<<','<<bits<<','<<SCENARIOS[scenario]<<','<<v.id<<','<<v.level<<','<<METHODS[method]<<','<<w<<','<<(accounting?"batch10_per_target":"cold")<<',';
            write_stats(cs,stats(vals));cs<<','<<z.memory<<'\n';summary_count++;
        }
      }
      for(int method=0;method<3;method++)for(double w:WEIGHTS)for(int accounting=0;accounting<2;accounting++) {
        vector<vector<double>> level_values;
        for(int l=0;l<=DEPTH;l++) {
            vector<double> vals(TARGETS,0);
            for(int j=0;j<graph.counts[l];j++) {
                int id=graph.offsets[l]+j;
                for(int t=0;t<TARGETS;t++) {
                    const auto& z=trial[id][method][t];
                    vals[t]+=(accounting?shared[id][method].pre()*(1.0/TARGETS)+z.tail():z.total()).at(w)/graph.counts[l];
                }
            }
            level_values.push_back(vals);auto s=stats(vals);
            ls<<"SYNTHETIC,"<<family<<','<<bits<<','<<SCENARIOS[scenario]<<','<<l<<','<<graph.counts[l]<<','<<METHODS[method]<<','<<w<<','<<(accounting?"batch10_per_target":"cold")<<',';
            write_stats(ls,s);ls<<",mean_over_vertices_then_summary_over_ten_targets\n";
            if(!first_level)compact<<',';
            first_level=false;
            compact<<"{\"family\":\""<<family<<"\",\"bits\":"<<bits<<",\"scenario\":\""<<SCENARIOS[scenario]<<"\",\"level\":"<<l
              <<",\"method\":\""<<METHODS[method]<<"\",\"weight\":"<<w<<",\"mode\":\""<<(accounting?"batch10_per_target":"cold")<<"\",\"stats\":"<<stat_json(s)<<'}';
        }
        int below=0;for(int t=0;t<TARGETS;t++)below+=level_values.back()[t]<level_values.front()[t];
        pats<<"SYNTHETIC,"<<family<<','<<bits<<','<<SCENARIOS[scenario]<<','<<METHODS[method]<<','<<w<<','<<(accounting?"batch10_per_target":"cold")<<','<<TARGETS<<','<<below<<','
          <<stats(level_values.back()).mean/stats(level_values.front()).mean<<",injected_model_not_empirical_frequency\n";
      }
      for(int method=0;method<3;method++)for(double w:WEIGHTS) {
        array<int,4> selected{0,graph.offsets[DEPTH],0,0};
        array<Cost,4> profile_cost{};
        for(int strategy=2;strategy<4;strategy++) {
            double best=numeric_limits<double>::infinity();int candidates=0;
            Cost charged=graph.construction(); // Enumeration plus explicit-map construction surrogate.
            for(auto v:graph.v) {
                if(strategy==3&&v.local!=0)continue; // One prespecified vertex per level.
                candidates++;Cost pilot_tail;
                for(auto z:pilot_runs[v.id][method])pilot_tail+=z.tail()+path_eval(v);
                charged+=pilot_pre[v.id][method].pre()+pilot_tail+path_eval(v);
                double score=(pilot_pre[v.id][method].pre()*(1.0/TARGETS)+pilot_tail*(1.0/PILOTS)).at(w);
                if(score<best){best=score;selected[strategy]=v.id;}
            }
            profile_cost[strategy]=charged;auto v=graph.v[selected[strategy]];
            string strategy_name=strategy==2?"profiled_all_vertices":"profiled_one_per_level";
            pilots<<"SYNTHETIC,"<<family<<','<<bits<<','<<SCENARIOS[scenario]<<','<<METHODS[method]<<','<<w<<','<<strategy_name<<','<<candidates<<','<<PILOTS<<','<<v.id<<','<<v.level<<',';
            write_cost(pilots,charged);pilots<<",0\n";
        }
        for(int strategy=0;strategy<4;strategy++)for(int accounting=0;accounting<2;accounting++) {
            string name=strategy==0?"direct_surface":strategy==1?"predeclared_floor":strategy==2?"profiled_all_vertices":"profiled_one_per_level";
            auto v=graph.v[selected[strategy]];vector<double> vals,base;
            double peak_memory=trial[v.id][method][0].memory;
            if(strategy>=2) {
                // Process candidates sequentially, retaining only the incumbent's data.
                for(auto candidate:graph.v) {
                    if(strategy==3&&candidate.local!=0)continue;
                    peak_memory=max(peak_memory,2*trial[candidate.id][method][0].memory);
                }
                peak_memory+=graph.v.size()*256+graph.e.size()*128;
            }
            for(int t=0;t<TARGETS;t++) {
                auto z=trial[v.id][method][t];Cost c;
                if(strategy<2) {
                    c=accounting?shared[v.id][method].pre()*(1.0/TARGETS)+z.tail():z.total();
                    if(strategy==1)c+=accounting?path_build(v)*(1.0/TARGETS)+path_eval(v)*(1.0/TARGETS)+path_eval(v):path_build(v)+path_eval(v,2);
                } else c=profile_cost[strategy]*(accounting?1.0/TARGETS:1.0)+z.tail()+path_eval(v);
                auto root=trial[0][0][t];double b=(accounting?shared[0][0].pre()*(1.0/TARGETS)+root.tail():root.total()).at(w);
                vals.push_back(c.at(w));base.push_back(b);
                st<<"SYNTHETIC,"<<family<<','<<bits<<','<<SCENARIOS[scenario]<<','<<METHODS[method]<<','<<name<<','<<v.id<<','<<w<<','<<(accounting?"batch10_per_target":"cold")<<','<<t<<','<<vals.back()<<','<<b<<'\n';
            }
            auto s=stats(vals);double ratio=s.mean/stats(base).mean;
            ss<<"SYNTHETIC,"<<family<<','<<bits<<','<<SCENARIOS[scenario]<<','<<METHODS[method]<<','<<name<<','<<v.id<<','<<v.level<<','<<w<<','<<(accounting?"batch10_per_target":"cold")<<',';
            write_stats(ss,s);ss<<','<<ratio<<','<<peak_memory<<'\n';strategy_count++;
            ostringstream sj;sj<<setprecision(17)<<"{\"family\":\""<<family<<"\",\"bits\":"<<bits<<",\"scenario\":\""<<SCENARIOS[scenario]<<"\",\"method\":\""<<METHODS[method]
              <<"\",\"strategy\":\""<<name<<"\",\"vertex\":"<<v.id<<",\"level\":"<<v.level<<",\"weight\":"<<w<<",\"mode\":\""<<(accounting?"batch10_per_target":"cold")
              <<"\",\"stats\":"<<stat_json(s)<<",\"ratio\":"<<ratio<<",\"memory\":"<<peak_memory<<'}';strategy_json.push_back(sj.str());
        }
      }
    }
    compact<<"],\"curves\":[";
    for(size_t i=0;i<curve_json.size();i++){if(i)compact<<',';compact<<curve_json[i];}
    compact<<"],\"strategies\":[";
    for(size_t i=0;i<strategy_json.size();i++){if(i)compact<<',';compact<<strategy_json[i];}
    compact<<"],\"graph_vertices\":[";
    for(size_t i=0;i<graph.v.size();i++){if(i)compact<<',';auto v=graph.v[i];compact<<"{\"id\":"<<v.id<<",\"level\":"<<v.level<<",\"local\":"<<v.local<<",\"parent\":"<<v.parent<<'}';}
    compact<<"],\"graph_edges\":[";
    for(size_t i=0;i<graph.e.size();i++){if(i)compact<<',';auto e=graph.e[i];compact<<"{\"a\":"<<e.a<<",\"b\":"<<e.b<<",\"degree\":"<<e.degree<<",\"horizontal\":"<<(e.horizontal?"true":"false")<<'}';}
    compact<<"]}\n";
    auto config=file(out,"config.json");
    config<<"{\n\"schema\":\"synthetic-volcano/v1\",\"status\":\"INVENTED DATA; NO CURVES CONSTRUCTED; NO ECDLP SOLVES\",\n\"seed\":"<<SEED<<",\"targets\":"<<TARGETS<<",\"pilot_targets\":"<<PILOTS<<",\"depth\":"<<DEPTH
      <<",\"vertical_degree\":3,\"crater\":\"inert; one vertex\",\"horizontal_labels\":[11,23,29],\"horizontal_shifts\":[1,5,7],\n"
      <<"\"field_log2\":[30,40,50],\"families\":[\"prime\",\"binary\",\"koblitz_origin\"],\n"
      <<"\"nominal_subgroup\":\"largest prime strictly below 2^(field_log2-2); NOT a certified curve subgroup\",\n"
      <<"\"scenarios\":{\"null\":1,\"planted_floor_advantage\":0.55,\"planted_floor_penalty\":1.7},\n"
      <<"\"scenario_factor_scope\":\"decomposition oracle scan cost multiplied by factor^level; rho unaffected\",\n"
      <<"\"cost_unit\":\"synthetic GAE = invented group count + weight * invented native count; NOT calibrated\",\n"
      <<"\"native_weights\":[0,0.1,1],\"rank_status\":\"assumed, never computed\",\"point_status\":\"symbolic target labels, no coordinates\",\n"
      <<"\"rho\":\"Rayleigh collision surrogate; E steps sqrt(pi*r/(4*orbit)); negation on\",\n"
      <<"\"factor_base\":\"B=2*ceil(cuberoot(r)/2) signed points; U=ceil(B/(2*orbit))\",\n"
      <<"\"success_probability\":\"1-exp(-B^m/(m!*r)); random-sum occupancy assumption\",\n"
      <<"\"relations_needed\":\"ceil(1.08*U)+8; independence/full rank assumed\",\n"
      <<"\"relation_attempts\":\"negative_binomial(successes=relations_needed,p)+relations_needed\",\n"
      <<"\"descent_attempts\":\"geometric(p)\",\n"
      <<"\"ic3_setup\":\"B*(B-1)/2 pair sums, 3 native units/pair, 24 modeled bytes/pair\",\n"
      <<"\"linear_algebra\":\"16*(m+1)*U^2 native units; synthetic sparse-quadratic cost\",\n"
      <<"\"curve_setup\":\"2*log2(r) group units + 64 native\",\n"
      <<"\"factor_base_setup\":\"1.5*B group units + 3*B native; plus IC3 pair table\",\n"
      <<"\"verification\":\"2*log2(r) group units + 6*log2(r) native; COST ONLY, not executed\",\n"
      <<"\"oracle_scan\":\"B*(1-p/2)*exp(0.12*Z-0.5*0.12^2)*scenario_factor^level group units/attempt; 3 times that + 2*log2(r) native\",\n"
      <<"\"rho_native\":\"steps*(4+2*log2(orbit) if orbit>1 else 4)\",\n"
      <<"\"edge_construction\":\"full graph: 4*degree^2 group + 20*degree native per multiedge\",\n"
      <<"\"prespecified_path_construction\":\"12*3^2 group + 24 native per hop\",\n"
      <<"\"point_map\":\"3*3 group + 12 native per vertical hop per point\",\n"
      <<"\"batch10\":\"one independent precomputation shared by ten targets; all ten target descents and costed verifications retained\",\n"
      <<"\"profile_selection\":\"3 separate pilot targets per candidate; rank by candidate precomputation/10 + mean pilot tail; charge all graph construction, candidate setups and pilots\",\n"
      <<"\"profile_memory\":\"sequential candidates; hold incumbent plus current candidate, conservatively two max-sized workspaces, plus graph storage\",\n"
      <<"\"koblitz_scope\":\"hypothetical Koblitz-origin surface only has assumed orbit=field degree; all descendants orbit=1; no actual degree-30/40/50 depth-4 Koblitz existence claim\",\n"
      <<"\"graph_scope\":\"complete abstract component for the specified labels, not the all-degree EC graph or a realized isogeny class\",\n"
      <<"\"paired_scenarios\":\"same pseudorandom samples in all three scenarios; only oracle multiplier changes\"\n}\n";
    auto val=file(out,"validation.json");
    val<<"{\"status\":\"SIMULATOR_CHECKS_PASSED\",\"cryptographic_correctness\":\"NOT TESTED; NO EC COMPUTATION\",\"graph_and_accounting_assertions\":"<<tests
      <<",\"raw_trial_rows\":"<<raw_count<<",\"curve_summary_rows\":"<<summary_count<<",\"strategy_summary_rows\":"<<strategy_count
      <<",\"vertices_per_component\":"<<n<<",\"multiedges_per_component\":"<<graph.e.size()<<",\"holdout_targets\":"<<TARGETS<<",\"separate_pilot_targets\":"<<PILOTS<<"}\n";
    cout<<"SYNTHETIC: "<<raw_count<<" raw rows; "<<n<<" abstract vertices per component; "<<tests<<" structural/accounting checks passed. No EC computations performed.\n";
}
int main(int argc,char** argv) {
    try {
        fs::path out="run-synthetic";bool self=false;
        for(int i=1;i<argc;i++) {
            string a=argv[i];
            if(a=="--self-test")self=true;
            else if(a=="--out"&&i+1<argc)out=argv[++i];
            else if(a=="--seed"&&i+1<argc)SEED=stoull(argv[++i]);
            else if(a=="--depth"&&i+1<argc)DEPTH=stoi(argv[++i]);
            else throw runtime_error("usage: simulate [--self-test] [--seed N] [--depth 1..5] [--out NEW_DIRECTORY]");
        }
        require(DEPTH>=1&&DEPTH<=5,"depth 1..5");
        if(self) {Graph g(DEPTH);cout<<graph_tests(g)<<" simulator assertions passed; cryptographic correctness not applicable.\n";}
        else run(out);
    }catch(const exception& e){cerr<<e.what()<<'\n';return 1;}return 0;
}
