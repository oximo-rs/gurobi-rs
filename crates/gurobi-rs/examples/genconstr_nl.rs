//! Gurobi 13 nonlinear expression-tree example.
//!
//! Solves:
//!   minimize y
//!   subject to y = exp(x) + x^2,   x in [-2, 2],   y free
//!
//! Encodes the right-hand side as a single nonlinear expression tree passed to
//! `GRBaddgenconstrNL` via [`Model::add_genconstr_nl`].
//!
//! Tree (parent index in brackets):
//!
//!   node 0  PLUS         parent = -1   <-- root, equals `y`
//!   node 1  EXP          parent = 0
//!   node 2  VARIABLE x   parent = 1
//!   node 3  SQUARE       parent = 0
//!   node 4  VARIABLE x   parent = 3

use gurobi_rs::Opcode;
use gurobi_rs::prelude::*;

fn main() -> gurobi_rs::Result<()> {
    let mut model = Model::new("nl_expr_tree")?;

    // x is added first => its column index is 0.
    let x = add_ctsvar!(model, name: "x", bounds: -2.0..2.0)?;
    let y = add_ctsvar!(model, name: "y", bounds: -INFINITY..INFINITY)?;
    model.update()?;

    model.set_objective(1.0 * y, Minimize)?;

    let x_idx = 0.0_f64; // first variable added

    let opcode: Vec<i32> = vec![
        Opcode::Plus as i32,
        Opcode::Exp as i32,
        Opcode::Variable as i32,
        Opcode::Square as i32,
        Opcode::Variable as i32,
    ];
    let data: Vec<f64> = vec![0.0, 0.0, x_idx, 0.0, x_idx];
    let parent: Vec<i32> = vec![-1, 0, 1, 0, 3];

    model.add_genconstr_nl("nl_rhs", y, &opcode, &data, &parent)?;

    model.set_param(param::FuncNonlinear, 1)?;
    model.set_param(param::NonConvex, 2)?;

    model.optimize()?;

    let status = model.status()?;
    println!("status = {status:?}");

    let x_val = model.get_obj_attr(attr::X, &x)?;
    let y_val = model.get_obj_attr(attr::X, &y)?;
    let obj = model.get_attr(attr::ObjVal)?;
    println!("x = {x_val:.6}");
    println!("y = {y_val:.6}");
    println!("obj = {obj}");

    let expected = x_val.exp() + x_val * x_val;
    println!(
        "exp(x) + x^2 = {expected:.6}   residual = {:.2e}",
        (y_val - expected).abs()
    );

    Ok(())
}
