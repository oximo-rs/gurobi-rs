use gurobi_rs::prelude::*;
mod utils;

fn main() -> gurobi_rs::Result<()> {
    create_model!(_g, m, x, y, z);
    c!(x + y == );
    c!(==);
    c!(x);
    c!(>=3);
    Ok(())
}
