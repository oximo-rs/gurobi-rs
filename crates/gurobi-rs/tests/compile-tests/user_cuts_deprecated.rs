#![deny(deprecated)]

use gurobi_rs::callback::*;
use gurobi_rs::prelude::*;

fn callback(w: Where) -> CbResult {
    match w {
        Where::MIPSol(ctx) => {
            ctx.add_cut(c!(0 == 0))?;
        }
        Where::MIPNode(ctx) => {
            ctx.add_cut(c!(0 == 0))?;
        }
        _ => {}
    }
    Ok(())
}

fn main() {}
