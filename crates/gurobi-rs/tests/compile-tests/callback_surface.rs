use gurobi_rs::callback::{CbResult, Where};
use gurobi_rs::Var;

fn callback(w: Where, vars: &[Var]) -> CbResult {
    if let Where::MultiObj(ctx) = w {
        let _ = ctx.obj_cnt()?;
        let _ = ctx.sol_cnt()?;
        let _ = ctx.status()?;
        let _ = ctx.obj_best()?;
        let _ = ctx.obj_bnd()?;
        let _ = ctx.mip_gap()?;
        let _ = ctx.iter_cnt()?;
        let _ = ctx.node_cnt()?;
        let _ = ctx.node_left()?;
        let _ = ctx.runtime()?;
        let _ = ctx.work()?;
        let _ = ctx.get_solution(vars.iter())?;
    }
    Ok(())
}

fn main() {
    let _ = callback;
}
