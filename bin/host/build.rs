use sp1_helper::build_program;

fn main() {
    build_program(&format!("../client-eth"));
    build_program(&format!("../client-op"));
    build_program(&format!("../client-linea"));
}
