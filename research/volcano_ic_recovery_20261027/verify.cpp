#include "../photo_volcano_restart_20261017/source/json.hpp"
#include "independent.hpp"
#include <algorithm>
#include <array>
#include <fstream>
#include <iostream>
#include <set>
#include <string>
#include <vector>
using J=nlohmann::json;using U=std::uint64_t;using W=__uint128_t;using I=__int128_t;
static void check(bool ok,const char* s){if(!ok)throw std::runtime_error(s);}
static U number(const J& j){if(j.is_number_unsigned())return j.get<U>();check(j.is_string(),"integer representation");auto s=j.get<std::string>();check(!s.empty()&&std::all_of(s.begin(),s.end(),[](char c){return c>='0'&&c<='9';}),"integer syntax");std::size_t used=0;U n=std::stoull(s,&used);check(used==s.size()&&std::to_string(n)==s,"noncanonical integer");return n;}
static independent::Point point(const J& j){if(j.is_null())return {};check(j.is_array()&&j.size()==2,"point encoding");return {number(j[0]),number(j[1]),false};}
static J load(const char* path){std::ifstream f(path);check(bool(f),"input open");J j;f>>j;return j;}
class Elimination {
    U r;
    U times(U x,U y)const{return U(W(x)*y%r);}
    U minus(U x,U y)const{return U((W(x)+r-y)%r);}
    U inverse(U x)const{U a=r,b=x;I t=0,u=1;while(b){U q=a/b,rem=a%b;a=b;b=rem;I z=t-I(q)*u;t=u;u=z;}check(a==1,"singular scalar pivot");t%=I(r);return U(t<0?t+r:t);}
public:
    using Row=std::array<U,13>;
    explicit Elimination(U modulus):r(modulus){check(r>3&&r<(U(1)<<62),"scalar range");}
    U signed_value(int c)const{U n=U(c<0?-c:c)%r;return c<0&&n?r-n:n;}
    std::pair<std::size_t,std::array<U,12>> solve(std::vector<Row> rows)const {
        std::size_t pivot_row=0;std::array<int,12> pivot_columns{};
        for(int col=0;col<12&&pivot_row<rows.size();++col){std::size_t at=pivot_row;while(at<rows.size()&&!rows[at][col])++at;if(at==rows.size())continue;
            std::swap(rows[at],rows[pivot_row]);U z=inverse(rows[pivot_row][col]);for(auto& x:rows[pivot_row])x=times(x,z);
            for(std::size_t i=0;i<rows.size();++i)if(i!=pivot_row){U c=rows[i][col];for(int j=0;j<13;++j)rows[i][j]=minus(rows[i][j],times(c,rows[pivot_row][j]));}
            pivot_columns[pivot_row]=col;++pivot_row;
        }
        for(std::size_t i=pivot_row;i<rows.size();++i){for(int j=0;j<12;++j)check(rows[i][j]==0,"elimination remainder");check(rows[i][12]==0,"inconsistent relation system");}
        std::array<U,12> logs{};for(std::size_t i=0;i<pivot_row;++i)logs[pivot_columns[i]]=rows[i][12];return {pivot_row,logs};
    }
    U dot(const std::array<int,12>& c,const std::array<U,12>& logs)const{U out=0;for(int i=0;i<12;++i)out=U((W(out)+times(signed_value(c[i]),logs[i]))%r);return out;}
};
int main(int argc,char** argv){try{
    check(argc==4,"usage: verify PUBLIC_INPUT FROZEN_CONFIG PRIVATE_RECORDS_JSONL");J input=load(argv[1]),config=load(argv[2]);check(input.at("schema")=="volcano.ic.public-instance/v1","schema");
    U p=number(input.at("p")),r=number(input.at("r"));independent::Curve curve(p,number(input.at("a")),number(input.at("b")));Elimination eliminate(r);
    auto P=point(input.at("P")),Q=point(input.at("Q"));check(!P.infinity&&!Q.infinity&&curve.on(P)&&curve.on(Q)&&curve.times(r,P).infinity&&curve.times(r,Q).infinity,"public subgroup input");
    check(input.at("factor_base").size()==12,"factor-base size");std::array<independent::Point,12> base{};
    std::set<U> xs;for(int i=0;i<12;++i){base[i]=point(input.at("factor_base")[i]);check(!base[i].infinity&&curve.on(base[i])&&curve.times(r,base[i]).infinity&&xs.insert(base[i].x).second,"base point");}
    std::ifstream stream(argv[3]);check(bool(stream),"private journal open");std::string line;bool identity=false,target=false,claimed_logs=false;int radius=0;U attempts=0,relations=0;std::vector<Elimination::Row> rows;std::size_t rank=0;
    auto coefficients=[&](const J& j){check(j.is_array()&&j.size()==12,"coefficient arity");std::array<int,12> c{};for(int i=0;i<12;++i){check(j[i].is_number_integer()&&j[i]>=-radius&&j[i]<=radius,"coefficient outside frozen box");c[i]=j[i].get<int>();}return c;};
    auto sum=[&](const std::array<int,12>& c){independent::Point out;for(int i=0;i<12;++i)out=curve.plus(out,curve.signed_times(c[i],base[i]));return out;};
    while(std::getline(stream,line)){if(line.empty())continue;auto row=J::parse(line);auto kind=row.at("kind").get<std::string>();
        if(kind=="identity"){check(!identity&&row.at("input")==input&&row.at("config")==config,"mixed or duplicate input/config identity");U raw=number(config.at("radius"));check(raw>=1&&raw<=31,"radius bounds");radius=int(raw);identity=true;}
        else if(kind=="setup"){check(identity&&!target,"setup ordering");++attempts;check(number(row.at("trial"))==attempts,"setup sequence");U t=number(row.at("t"));check(t>0&&t<r&&point(row.at("R"))==curve.times(t,P),"setup RHS");
            std::size_t before=rank;if(row.at("found").get<bool>()){auto c=coefficients(row.at("coefficients"));check(sum(c)==curve.times(t,P),"independent relation equation failed");Elimination::Row v{};for(int i=0;i<12;++i)v[i]=eliminate.signed_value(c[i]);v[12]=t;rows.push_back(v);++relations;rank=eliminate.solve(rows).first;}
            check(number(row.at("rank"))==rank&&row.at("rank_increased").get<bool>()==(rank>before),"rank metadata mismatch");}
        else if(kind=="factor_base_logs_private"){check(identity&&!claimed_logs&&rank==12,"log stage ordering/rank");auto logs=eliminate.solve(rows).second;check(row.at("logs").size()==12,"log arity");for(int i=0;i<12;++i)check(number(row.at("logs")[i])==logs[i]&&curve.times(logs[i],P)==base[i],"factor-base logs not independently recovered");claimed_logs=true;}
        else if(kind=="target_recovery_private"){check(identity&&!target&&claimed_logs&&rank==12,"target stage ordering/rank");auto c=coefficients(row.at("coefficients"));auto logs=eliminate.solve(rows).second;U k=number(row.at("k"));check(k<r&&k==eliminate.dot(c,logs)&&sum(c)==Q&&curve.times(k,P)==Q,"independent target recovery failed");target=true;}
        else if(kind=="failure")throw std::runtime_error("journal reports a failed/censored run");
        else check(kind=="phase"||kind=="progress","unrecognized record type");
    }
    check(identity&&target&&rank==12,"incomplete recovery journal");
    std::cout<<J({{"passed",true},{"instance_id",input.at("instance_id")},{"setup_trials",attempts},{"independently_checked_relations",relations},{"independent_rank",rank},{"factor_base_logs_checked",12},{"target_equations_checked",1},{"method","separate affine/Euclidean arithmetic and fresh batch Gaussian elimination"},{"parameter_primality_proved_by_this_checker",false},{"timing_or_counter_validation",false},{"original_full_graph_requirement_complete",false}}).dump()<<'\n';return 0;
}catch(const std::exception& e){std::cerr<<"INDEPENDENT_REPLAY_ERROR "<<e.what()<<'\n';return 1;}}
