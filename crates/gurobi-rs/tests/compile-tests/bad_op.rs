use gurobi_rs::prelude::*;
mod utils;

fn main() -> gurobi_rs::Result<()> {
    create_model!(_g, m, x, y, z);
    c!(x + y + 2 * z + 1);
    c!(2 * z);
    c!(2 / z);
    Ok(())
}
