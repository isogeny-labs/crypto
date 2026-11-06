// Keccak-f[1600], FIPS 202: one complete round per clock, 24 rounds.
// Lane (x,y) occupies state[64*(x+5*y) +: 64], least-significant byte first.
module keccak_f1600 (
    input  logic          clk,
    input  logic          rst_n,
    input  logic          clear,
    input  logic          start,
    input  logic [1599:0] state_in,
    output logic          ready,
    output logic          busy,
    output logic          done,
    output logic [1599:0] state_out
);
  logic [1599:0] state_q, next_state;
  logic [4:0] round_q;

  function automatic logic [63:0] rol(input logic [63:0] value, input int shift);
    return shift == 0 ? value : (value << shift) | (value >> (64 - shift));
  endfunction

  function automatic int rho(input int lane);
    case (lane)
       0: return  0;  1: return  1;  2: return 62;  3: return 28;  4: return 27;
       5: return 36;  6: return 44;  7: return  6;  8: return 55;  9: return 20;
      10: return  3; 11: return 10; 12: return 43; 13: return 25; 14: return 39;
      15: return 41; 16: return 45; 17: return 15; 18: return 21; 19: return  8;
      20: return 18; 21: return  2; 22: return 61; 23: return 56; 24: return 14;
      default: return 0;
    endcase
  endfunction

  function automatic logic [63:0] rc(input logic [4:0] round_index);
    case (round_index)
       0: return 64'h0000000000000001;  1: return 64'h0000000000008082;
       2: return 64'h800000000000808a;  3: return 64'h8000000080008000;
       4: return 64'h000000000000808b;  5: return 64'h0000000080000001;
       6: return 64'h8000000080008081;  7: return 64'h8000000000008009;
       8: return 64'h000000000000008a;  9: return 64'h0000000000000088;
      10: return 64'h0000000080008009; 11: return 64'h000000008000000a;
      12: return 64'h000000008000808b; 13: return 64'h800000000000008b;
      14: return 64'h8000000000008089; 15: return 64'h8000000000008003;
      16: return 64'h8000000000008002; 17: return 64'h8000000000000080;
      18: return 64'h000000000000800a; 19: return 64'h800000008000000a;
      20: return 64'h8000000080008081; 21: return 64'h8000000000008080;
      22: return 64'h0000000080000001; 23: return 64'h8000000080008008;
      default: return 64'b0;
    endcase
  endfunction

  function automatic logic [1599:0] keccak_round(
      input logic [1599:0] current, input logic [4:0] round_index);
    logic [63:0] a [0:24], b [0:24], c [0:4], d [0:4];
    logic [1599:0] result;
    for (int y = 0; y < 5; y++)
      for (int x = 0; x < 5; x++)
        a[x + 5*y] = current[64*(x + 5*y) +: 64];
    for (int x = 0; x < 5; x++)
      c[x] = a[x] ^ a[x+5] ^ a[x+10] ^ a[x+15] ^ a[x+20];
    for (int x = 0; x < 5; x++)
      d[x] = c[(x+4)%5] ^ rol(c[(x+1)%5], 1);
    for (int y = 0; y < 5; y++)
      for (int x = 0; x < 5; x++)
        b[y + 5*((2*x + 3*y)%5)] = rol(a[x + 5*y] ^ d[x], rho(x + 5*y));
    for (int y = 0; y < 5; y++)
      for (int x = 0; x < 5; x++)
        result[64*(x + 5*y) +: 64] =
            b[x + 5*y] ^ ((~b[(x+1)%5 + 5*y]) & b[(x+2)%5 + 5*y]);
    result[63:0] = result[63:0] ^ rc(round_index);
    return result;
  endfunction

  assign ready = rst_n && !clear && !busy;
  assign next_state = keccak_round(state_q, round_q);

  always_ff @(posedge clk or negedge rst_n) begin
    if (!rst_n) begin
      state_q <= '0;
      state_out <= '0;
      round_q <= '0;
      busy <= 1'b0;
      done <= 1'b0;
    end else if (clear) begin
      state_q <= '0;
      state_out <= '0;
      round_q <= '0;
      busy <= 1'b0;
      done <= 1'b0;
    end else begin
      done <= 1'b0;
      if (busy) begin
        state_q <= next_state;
        if (round_q == 5'd23) begin
          state_out <= next_state;
          state_q <= '0;
          round_q <= '0;
          busy <= 1'b0;
          done <= 1'b1;
        end else round_q <= round_q + 5'd1;
      end else if (start) begin
        state_q <= state_in;
        state_out <= '0;
        round_q <= '0;
        busy <= 1'b1;
      end
    end
  end
endmodule
