/* Upstream iofiles.c lines 1-19. */
/* This file is part of msolve.
 *
 * msolve is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 2 of the License, or
 * (at your option) any later version.
 *
 * msolve is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with msolve.  If not, see <https://www.gnu.org/licenses/>
 *
 * Authors:
 * Jérémy Berthomieu
 * Christian Eder
 * Mohab Safey El Din */

/* Upstream iofiles.c lines 21-63. */
static inline void store_exponent(const char *term, data_gens_ff_t *gens, int32_t pos)
{
    len_t i, j, k;

    len_t op = 0;
    char *var = NULL;
    char *ev  = NULL;
    for (i = 0; i < strlen(term)+1; ++i) {
        if (term[i] == '*' || i == strlen(term)) {
            j = op;
            while (j < i && term[j] != '^') {
                ++j;
            }
            if (term[j-1] == ',') {
                --j;
            }
            while(term[op] == ' ' || term[op] == '+' || term[op] == '-') {
                ++op;
            }
            var = realloc(var, sizeof(char)*(j-op+1));
            memcpy(var, term+op, j-op);
            var[j-op] = '\0';
            if (term[j] == '^') {
                ev = realloc(ev, (sizeof(char)*(i-j)));
                ev = memcpy(ev, term+(j+1), i-j-1);
                ev[i-j-1] = '\0';
            } else {
                ev = realloc(ev, (sizeof(char)*2));
                ev[0] = '1';
                ev[1] ='\0';
            }
            for (k = 0; k < gens->nvars; ++k) {
                if (strcmp(gens->vnames[k], var) == 0) {
                    ((gens->exps) + pos)[k] = strtol(ev, NULL, 10);
                    break;
                }
            }
            op = i+1;
        }
    }
    free(var);
    free(ev);
}

/* Upstream iofiles.c lines 474-501. */
static inline nelts_t get_number_of_terms(const char *line)
{
  const char add_splicer        = '+';
  const char minus_splicer      = '-';
  const char whitespace_splicer = ' ';
  char *tmp = NULL;
  nelts_t nterms  = 1;
  /** remove useless whitespaces at the beginning */
  int i = 0;
  while (strncmp(&whitespace_splicer, line+i, 1) == 0)
    i++;
  /** check if first non-whitespace char is "-", set term counter -1 in this case */
  if (strncmp(&minus_splicer, line+i, 1) == 0)
    nterms--;
  /** now count terms */
  tmp = strchr(line, add_splicer);
  while (tmp != NULL) {
    nterms++;
    tmp = strchr(tmp+1, add_splicer);
  }
  tmp = strchr(line, minus_splicer);
  while (tmp != NULL) {
    nterms++;
    tmp = strchr(tmp+1, minus_splicer);
  }
  //MS que faire si dans la ligne on a 0*x+0*y ?
  return nterms;
}

/* Upstream iofiles.c lines 575-653. */
static inline void get_term(const char *line, char **prev_pos,
                            char **term, size_t *term_size)
{
  /** note that maximal term length we handle */
  const char add_splicer    = '+';
  const char minus_splicer  = '-';

  char *start_pos;
  char *curr_pos_add    = strchr(*prev_pos, add_splicer);
  char *curr_pos_minus  = strchr(*prev_pos, minus_splicer);
  if (curr_pos_minus == line)
    curr_pos_minus  = strchr(*prev_pos+1, minus_splicer);

  if (*prev_pos != line)
    start_pos = *prev_pos - 1;
  else
    start_pos = *prev_pos;

  if (curr_pos_add != NULL && curr_pos_minus != NULL) {
    size_t term_diff_add   = (size_t)(curr_pos_add - start_pos);
    size_t term_diff_minus = (size_t)(curr_pos_minus - start_pos);
    /** if "-" is the first char in the line, we have to adjust the
      * if minus is nearer */
    if (term_diff_add > term_diff_minus) {
      if( term_diff_minus > *term_size){
        fprintf(stderr, "Too large input integers, exit...\n");
        exit(1);
      }
      memcpy(*term, start_pos, term_diff_minus);
      (*term)[term_diff_minus]  = '\0';
      *prev_pos                 = curr_pos_minus+1;
      return;
    /** if plus is nearer */
    } else {
      if(term_diff_add > *term_size){
        fprintf(stderr, "Too large input integers, exit...\n");
        exit(1);
      }
      memcpy(*term, start_pos, term_diff_add);
      (*term)[term_diff_add]  = '\0';
      *prev_pos               = curr_pos_add+1;
      return;
    }
  } else {
    if (curr_pos_add != NULL) {
      size_t term_diff_add   = (size_t)(curr_pos_add - start_pos);
      if(term_diff_add > *term_size){
        fprintf(stderr, "Too large input integers, exit...\n");
        exit(1);
      }
      memcpy(*term, start_pos, term_diff_add);
      (*term)[term_diff_add]  = '\0';
      *prev_pos               = curr_pos_add+1;
      return;
    }
    if (curr_pos_minus != NULL) {
      size_t term_diff_minus = (size_t)(curr_pos_minus - start_pos);
      if(term_diff_minus > *term_size){
        fprintf(stderr, "Too large input integers, exit...\n");
        exit(1);
      }
      memcpy(*term, start_pos, term_diff_minus);
      (*term)[term_diff_minus]  = '\0';
      *prev_pos                 = curr_pos_minus+1;
      return;
    }
    if (curr_pos_add == NULL && curr_pos_minus == NULL) {
      size_t prev_idx  = (size_t)(start_pos - line);
      size_t term_diff = strlen(line) + 1 - prev_idx;
      if(term_diff > *term_size){
        fprintf(stderr, "Too large input integers, exit...\n");
        exit(1);
      }
      memcpy(*term, start_pos, term_diff);
      (*term)[term_diff]  = '\0';
      return;
    }
  }
}

/* Upstream iofiles.c lines 656-712. */
/*assumes that coeffs in file fit in word size */
static int get_coefficient_ff_and_term_from_line(char *line, int32_t nterms,
                                          int32_t field_char,
                                          data_gens_ff_t *gens, int32_t pos){
  char *prev_pos = NULL;
  size_t term_size = 50000;
  char *term  = (char *)malloc(term_size * sizeof(char));
  int64_t cf_tmp  = 0; /** temp for coefficient value, possibly coeff is negative. */

  prev_pos = line;
  get_term(line, &prev_pos, &term, &term_size);
  if(term != NULL){
    int32_t iv_tmp  = (int32_t)strtol(term, NULL, 10);
    if (iv_tmp == 0) {
      switch (term[0]) {
          case '0':
            iv_tmp = 0;
            break;
          case '-':
            iv_tmp = -1;
            break;
          default:
            iv_tmp = 1;
            break;
      }
    }
    while (iv_tmp < 0) {
      iv_tmp  +=  field_char; //MS change int -> long int
    }
    gens->cfs[pos]  = (int32_t)iv_tmp;
    store_exponent(term, gens, pos*gens->nvars);
    for(int j = 1; j < nterms; j++){
      get_term(line, &prev_pos, &term, &term_size);
      if (term != NULL) {
        cf_tmp  = (int64_t)strtol(term, NULL, 10);

        if (cf_tmp == 0) {
          if (term[0] == '-') {
            cf_tmp = -1;
          } else {
            cf_tmp = 1;
          }
        }
        while (cf_tmp < 0) {
          cf_tmp  += field_char;
        }
        gens->cfs[pos+j] = (int32_t)(cf_tmp % field_char);
        store_exponent(term, gens, (pos+j)*gens->nvars);
      }
      //      store_exponent(term, basis, ht);
    }
    free(term);
    return 0;
  }
  free(term);
  return 1;
}

