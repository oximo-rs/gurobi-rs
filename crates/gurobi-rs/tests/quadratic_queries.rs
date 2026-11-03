use gurobi_rs::prelude::*;
use gurobi_rs::{Error, QConstrTerms};

fn quadratic(mut terms: Vec<(Var, Var, f64)>) -> Vec<(u32, u32, f64)> {
    let mut result: Vec<_> = terms
        .drain(..)
        .map(|(x, y, c)| (x.id().min(y.id()), x.id().max(y.id()), c))
        .collect();
    result.sort_by_key(|&(x, y, _)| (x, y));
    result
}

fn linear(terms: Vec<(Var, f64)>) -> Vec<(u32, f64)> {
    let mut result: Vec<_> = terms.into_iter().map(|(x, c)| (x.id(), c)).collect();
    result.sort_by_key(|&(x, _)| x);
    result
}

#[test]
fn query_objective_and_constraint_terms() -> gurobi_rs::Result<()> {
    let mut model = Model::new("quadratic-queries")?;
    let x = model.add_var("x", Continuous, 0.0, 0.0, 10.0, [])?;
    let y = model.add_var("y", Continuous, 0.0, 0.0, 10.0, [])?;
    model.set_objective_sparse([], [(x, x, 2.0), (x, y, 1.0), (y, x, 3.0)], 0.0, Minimize)?;
    model.update()?;
    assert_eq!(
        quadratic(model.get_qobjective_terms()?),
        quadratic(vec![(x, x, 2.0), (x, y, 4.0)])
    );

    let row = model.add_qconstr_sparse(
        "row",
        [(x, 2.0), (x, 3.0), (y, -1.0)],
        [(x, x, 2.0), (x, y, 1.0), (y, x, 3.0)],
        4.0,
        ConstrSense::Less,
        9.0,
    )?;
    let only_quadratic =
        model.add_qconstr_sparse("q", [], [(y, y, 1.0)], 0.0, ConstrSense::Less, 9.0)?;
    model.update()?;
    let QConstrTerms {
        linear: l,
        quadratic: q,
    } = model.get_qconstr_terms(&row)?;
    assert_eq!(linear(l), linear(vec![(x, 5.0), (y, -1.0)]));
    assert_eq!(quadratic(q), quadratic(vec![(x, x, 2.0), (x, y, 4.0)]));
    assert_eq!(model.get_obj_attr(attr::QCRHS, &row)?, 5.0);
    assert!(model.get_qconstr_terms(&only_quadratic)?.linear.is_empty());

    model.remove(x)?;
    assert_eq!(model.get_qobjective_terms(), Err(Error::ModelUpdateNeeded));
    assert_eq!(model.get_qconstr_terms(&row), Err(Error::ModelUpdateNeeded));

    model.update()?;
    assert!(model.get_qobjective_terms()?.is_empty());

    let terms = model.get_qconstr_terms(&row)?;
    assert_eq!(linear(terms.linear), linear(vec![(y, -1.0)]));
    assert!(terms.quadratic.is_empty());

    Ok(())
}

#[test]
fn query_empty_objective_and_constraint_handle_states() -> gurobi_rs::Result<()> {
    let mut model = Model::new("query-states")?;
    model.set_param(param::UpdateMode, 0)?;
    assert!(model.get_qobjective_terms()?.is_empty());

    let x = model.add_var("x", Continuous, 0.0, 0.0, 1.0, [])?;
    assert_eq!(model.get_qobjective_terms(), Err(Error::ModelUpdateNeeded));

    model.update()?;
    model.add_qpterms([(x, x, 2.0)])?;
    assert!(model.get_qobjective_terms()?.is_empty());

    model.update()?;
    assert_eq!(
        quadratic(model.get_qobjective_terms()?),
        quadratic(vec![(x, x, 2.0)])
    );

    let row = model.add_qconstr_sparse("row", [], [(x, x, 1.0)], 0.0, ConstrSense::Less, 1.0)?;
    assert_eq!(
        model.get_qconstr_terms(&row),
        Err(Error::ModelObjectPending)
    );

    model.update()?;
    let mut other = Model::new("foreign")?;
    let y = other.add_var("y", Continuous, 0.0, 0.0, 1.0, [])?;
    other.update()?;
    let foreign =
        other.add_qconstr_sparse("foreign", [], [(y, y, 1.0)], 0.0, ConstrSense::Less, 1.0)?;
    other.update()?;
    assert_eq!(
        model.get_qconstr_terms(&foreign),
        Err(Error::ModelObjectMismatch)
    );

    model.remove(row)?;
    assert_eq!(
        model.get_qconstr_terms(&row),
        Err(Error::ModelObjectRemoved)
    );

    model.update()?;
    assert_eq!(
        model.get_qconstr_terms(&row),
        Err(Error::ModelObjectRemoved)
    );

    Ok(())
}
