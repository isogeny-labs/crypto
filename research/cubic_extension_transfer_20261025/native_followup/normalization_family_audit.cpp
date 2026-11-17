#define main obstruction_reference_main
#include "obstruction.cpp"
#undef main
int main(int argc,char**argv){try{
 require(argc==2,"usage: normalization_family_audit FOLLOWUP_DIR");std::string d=argv[1];auto input=read_json(d+"/results/obstruction-native.json");json rows=json::array();bool all=true;
 for(auto&row:input["cases"]){int p=row["p"];field_start(p,18);{
  F z=roots({F(p==109?103:152),F(1),F(0),F(1)})[0];auto ab=parse_curve(row["source_curve"],z);EC E{ab.first,ab.second};auto n=normal3(E);auto old=action_matrices(E,n);int section_matches=0,matrix_matches=0;
  for(int c=1;c<9;c++){F u=n.points[c].x,v=n.points[c].y,m=(F(3)*u*u+E.a)/(F(2)*v);require(m*m==F(3)*u,"marked point is not flex three torsion");section_matches+=n.sections[c][2]==-F(1)/(F(2)*v);
   M3 N{V3{m*u-v,m*u*u-F(3)*u*v,F(3)*v*v-u*m*v},V3{-m,F(2)*v-m*u,m*v},V3{F(-1),-u,-v}};for(auto&rr:N)for(auto&x:rr)x=x/(F(2)*v);bool same=true;for(int i=0;i<3;i++)for(int j=0;j<3;j++)same&=N[i][j]==old[c][i][j];matrix_matches+=same;
   auto cube=matmul(matmul(N,N),N);for(int i=0;i<3;i++)for(int j=0;j<3;j++)require(cube[i][j]==F(i==j),"explicit lift does not have order three");require(det(N)==F(1),"explicit lift determinant");
  }bool pass=section_matches==8&&matrix_matches==8;all&=pass;rows.push_back({{"p",p},{"label",row["label"]},{"normalized_sections_matching",section_matches},{"translation_matrices_matching",matrix_matches},{"status",pass?"passed":"mismatch"}});
 }field_stop();}
 write_json(d+"/results/normalization-family-audit.json",{{"status",all?"passed":"mismatch"},{"curves",rows},{"curve_count",rows.size()},{"nonzero_actions_tested",8*rows.size()},{"scope","Closed formula versus independently sampled action matrices on saved fixtures. Universal polynomial identities are separate certificates; this does not establish the neighbor obstruction relation."}});return all?0:3;
 }catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 2;}}
