// Simulation wiring only. This is not an AWS F2 shell or the FPC1 mailbox.
module crypto_kernels_top (
    input logic clk, rst_n, clear,
    input logic perm_start,
    input logic [1599:0] perm_state_in,
    output logic perm_ready, perm_busy, perm_done,
    output logic [1599:0] perm_state_out,
    input logic shake_start, shake_mode_256,
    input logic [31:0] shake_input_length, shake_output_length,
    output logic shake_ready, shake_busy, shake_done,
    input logic shake_in_valid,
    output logic shake_in_ready,
    input logic [7:0] shake_in_byte,
    output logic shake_out_valid,
    input logic shake_out_ready,
    output logic [7:0] shake_out_byte,
    input logic mul_start, mul_mode_dsa,
    input logic [31:0] mul_a, mul_b,
    output logic mul_ready, mul_busy, mul_done, mul_error,
    output logic [31:0] mul_result,
    input logic ntt_start, ntt_mode_dsa, ntt_inverse,
    output logic ntt_ready, ntt_busy, ntt_done, ntt_error,
    input logic ntt_in_valid,
    output logic ntt_in_ready,
    input logic [31:0] ntt_in_coeff,
    output logic ntt_out_valid,
    input logic ntt_out_ready,
    output logic [31:0] ntt_out_coeff
);
  keccak_f1600 permutation (
      .clk(clk), .rst_n(rst_n), .clear(clear), .start(perm_start),
      .state_in(perm_state_in), .ready(perm_ready), .busy(perm_busy),
      .done(perm_done), .state_out(perm_state_out)
  );
  shake_stream shake (
      .clk(clk), .rst_n(rst_n), .clear(clear), .start(shake_start),
      .mode_256(shake_mode_256), .input_length(shake_input_length),
      .output_length(shake_output_length), .ready(shake_ready), .busy(shake_busy),
      .done(shake_done), .in_valid(shake_in_valid), .in_ready(shake_in_ready),
      .in_byte(shake_in_byte), .out_valid(shake_out_valid),
      .out_ready(shake_out_ready), .out_byte(shake_out_byte)
  );
  lattice_modmul multiplier (
      .clk(clk), .rst_n(rst_n), .clear(clear), .start(mul_start),
      .mode_dsa(mul_mode_dsa), .a(mul_a), .b(mul_b), .ready(mul_ready),
      .busy(mul_busy), .done(mul_done), .error(mul_error), .result(mul_result)
  );
  lattice_ntt ntt (
      .clk(clk), .rst_n(rst_n), .clear(clear), .start(ntt_start),
      .mode_dsa(ntt_mode_dsa), .inverse(ntt_inverse), .ready(ntt_ready),
      .busy(ntt_busy), .done(ntt_done), .error(ntt_error),
      .in_valid(ntt_in_valid), .in_ready(ntt_in_ready), .in_coeff(ntt_in_coeff),
      .out_valid(ntt_out_valid), .out_ready(ntt_out_ready), .out_coeff(ntt_out_coeff)
  );
endmodule
