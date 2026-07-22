use gurobi_rs::callback::{Callback, CbResult, Where};
use gurobi_rs::prelude::*;

struct ErrorCb {}
struct PanicCb {}

impl Callback for ErrorCb {
    fn callback(&mut self, _: Where) -> CbResult {
        anyhow::bail!("ruh roh")
    }
}

impl Callback for PanicCb {
    fn callback(&mut self, _: Where) -> CbResult {
        panic!("ruh roh")
    }
}

fn run(mut cb: impl Callback) -> gurobi_rs::Result<()> {
    let mut m = Model::new("")?;
    add_ctsvar!(m)?;
    let result = m.optimize_with_callback(&mut cb);
    match result {
        Err(gurobi_rs::Error::FromAPI(_, 10011)) => {}
        Err(e) => panic!("unexpected error: {}", e),
        Ok(()) => panic!("expected error"),
    }
    m.optimize()?;
    Ok(())
}

#[test]
fn cb_panics() -> gurobi_rs::Result<()> {
    run(PanicCb {})
}

#[test]
fn cb_errors() -> gurobi_rs::Result<()> {
    run(ErrorCb {})
}
