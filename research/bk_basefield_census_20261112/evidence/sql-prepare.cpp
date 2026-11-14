// SQL compilation on an empty schema only; never executes INSERT/check/SELECT.
#include <sqlite3.h>
#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
int main(int argc,char**argv){
 if(argc!=2)return 2;
 sqlite3*db=nullptr;if(sqlite3_open(":memory:",&db)!=SQLITE_OK)return 3;
 if(sqlite3_exec(db,"CREATE TABLE rows(id INTEGER PRIMARY KEY,payload TEXT);",nullptr,nullptr,nullptr)!=SQLITE_OK)return 4;
 std::ifstream in(argv[1]);std::string line,sql;
 while(std::getline(in,line)){auto p=line.find_first_not_of(" \t");if(p!=std::string::npos&&line.compare(p,2,"--")!=0)sql+=line+'\n';}
 const char*at=sql.c_str();int count=0;
 while(*at){sqlite3_stmt*stmt=nullptr;const char*tail=nullptr;if(sqlite3_prepare_v2(db,at,-1,&stmt,&tail)!=SQLITE_OK){std::cerr<<sqlite3_errmsg(db)<<'\n'<<at<<'\n';return 5;}
  if(stmt){std::string text=sqlite3_sql(stmt);auto p=text.find_first_not_of(" \t\n");if(p!=std::string::npos&&text.compare(p,6,"CREATE")==0){int rc=sqlite3_step(stmt);if(rc!=SQLITE_DONE&&rc!=SQLITE_ROW){std::cerr<<sqlite3_errmsg(db)<<'\n';return 6;}}sqlite3_finalize(stmt);count++;}
  at=tail;
 }
 sqlite3_close(db);std::cout<<"Compiled "<<count<<" SQL statements on empty schema; no census rows/checks/queries executed.\n";return 0;
}
