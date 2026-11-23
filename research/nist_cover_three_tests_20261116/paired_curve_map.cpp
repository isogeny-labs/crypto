#include "quartic_descent_arithmetic.hpp"
Matrix3<T24> read_pair_matrix(const Json& j){require(j.arr.size()==3,"pair matrix shape");Matrix3<T24> m{};for(int i=0;i<3;++i)m[i]=read_line(j.arr[i]);return m;}
std::map<int,Line> base_lines(const Json& descent,const Matrix3<T24>& b){std::map<int,Line> out;for(auto& row:descent.at("labelled_bitangents").arr)out.emplace(row.at("label").integer(),row_times(read_line(row.at("line")),b));return out;}
int swap_halves(int index){return ((index&7)<<3)|(index>>3);}
bool all_line_matches(const std::map<int,Line>& a,const std::map<int,Line>& b,const std::map<int,int>& labels,const Matrix3<T24>& m){for(auto& [label,line]:a)if(normalize_line(row_times(b.at(labels.at(label)),m))!=normalize_line(line))return false;return true;}
int main(int argc,char** argv){try{
    require(argc==3,"usage: paired_curve_map original-study kernel2-study");std::string root=argv[1],second=argv[2];initialize_cover(read_json(root+"/inputs/p192-d7-prym.json"));Level2Input input;
    require(bool(std::ifstream(second+"/evidence/descent-v1/post-call-source-manifest.sha256")),"kernel2 descent completed");
    auto da=read_json(root+"/evidence/descent-v1/result.json"),db=read_json(second+"/evidence/descent-v1/result.json");auto ba=read_pair_matrix(da.at("change_of_coordinates")),bb=read_pair_matrix(db.at("change_of_coordinates"));
    auto ra=read_json(root+"/evidence/riemann-v1/result.json"),rb=read_json(second+"/evidence/riemann-v1/result.json");auto fa=normalized_form(transform_form(read_form(ra.at("quartic")),ba)),fb=normalized_form(transform_form(read_form(rb.at("quartic")),bb));
    auto la=base_lines(da,ba),lb=base_lines(db,bb);std::array<int,64> action{};
    for(int code=0;code<64;++code){int mask=input.masks[code],image=0;for(int i=1;i<7;++i)if((mask>>i)&1)image^=1<<(i%2?i+1:i-1);if(__builtin_popcount(unsigned(mask))%2)image^=1;image=canonical_subset(image);int found=-1;for(int j=0;j<64;++j)if(input.masks[j]==image){require(found<0,"unique source2 action");found=j;}require(found>=0,"source2 involution image");action[code]=found;}
    for(int code=0;code<64;++code)require(action[action[code]]==code,"all64 source2 involution checks");
    std::map<int,int> labels;for(auto& [label,line]:la){int target=swap_halves(action[swap_halves(label)^input.delta]^input.delta);require(theta_parity(target)==1,"odd characteristic preserved");labels.emplace(label,target);}
    std::array<int,4> frame{63,38,52,41};Matrix3<T24> source_frame{},target_frame{};for(int i=0;i<3;++i){source_frame[i]=la.at(frame[i]);target_frame[i]=lb.at(labels.at(frame[i]));}
    auto a=frame_coordinates(la.at(frame[3]),source_frame),b=frame_coordinates(lb.at(labels.at(frame[3])),target_frame);Matrix3<T24> point_map{};
    for(int j=0;j<3;++j){Line rhs{};for(int i=0;i<3;++i){require(a[i]!=T24(0)&&b[i]!=T24(0),"pair projective frame");rhs[i]=a[i]/b[i]*source_frame[i][j];}auto col=solve3(target_frame,rhs);for(int i=0;i<3;++i)point_map[i][j]=col[i];}
    T24 first;for(auto& row:point_map)for(auto& v:row)if(first==T24(0)&&v!=T24(0))first=v;require(first!=T24(0)&&determinant3(point_map)!=T24(0),"pair nonsingular point map");for(auto& row:point_map)for(auto& v:row){v=v/first;require(v.a.degree()<=0&&v.a.at(0).a.degree()<=0,"pair point map in original field");}
    require(all_line_matches(la,lb,labels,point_map),"all28 paired bitangent correspondences");require(normalized_form(transform_form(fb,point_map)).c==fa.c,"full paired quartic equation identity");
    Matrix3<T24> inverse{};for(int j=0;j<3;++j){Line unit{};unit[j]=T24(1);auto column=solve3(point_map,unit);for(int i=0;i<3;++i)inverse[i][j]=column[i];}require(multiply3(point_map,inverse)==identity3()&&multiply3(inverse,point_map)==identity3(),"inverse paired point map");
    rejects([&]{auto bad=point_map;bad[0][0]=bad[0][0]+T24(1);require(all_line_matches(la,lb,labels,bad),"corrupt paired point map");},"corrupt paired point map");
    std::cout<<"{\"scope\":\"P192-d7-kernel1-to-kernel2-candidate-curve-isomorphism\",\"source2_involution_checks\":64,\"labelled_bitangent_matches\":28,\"full_curve_equation_identity\":true,\"map_degree\":1,\"original_field\":true,\"standing\":\"producer-unverified\",\"source_divisor_isogeny_constructed\":false,\"point_map\":";emit_matrix(point_map);std::cout<<",\"inverse_map\":";emit_matrix(inverse);std::cout<<",\"label_map\":[";bool first_label=true;for(auto& [i,j]:labels){if(!first_label)std::cout<<',';first_label=false;std::cout<<'['<<i<<','<<j<<']';}std::cout<<"]}\n";
}catch(const std::exception& e){std::cerr<<"REJECT "<<e.what()<<'\n';return 1;}}
