use gurobi_rs::prelude::*;
mod utils;

fn main() -> gurobi_rs::Result<()> {
    create_model!(_g, m);
    add_var!(m, Binary, name: 0u8)?;
    Ok(())
}
