//! Index-based upload entry points.

use std::ffi::CStr;

use gurobi_rs::prelude::*;

fn environment() -> gurobi_rs::Result<Env> {
    let mut env = Env::new("")?;
    env.set(param::OutputFlag, 0)?;
    Ok(env)
}

fn columns(model: &mut Model, n: usize) -> gurobi_rs::Result<Vec<Var>> {
    (0..n)
        .map(|i| model.add_var(&format!("x{i}"), Continuous, 0.0, -10.0, 10.0, []))
        .collect()
}

#[test]
fn csr_rows_match_the_supplied_layout() -> gurobi_rs::Result<()> {
    let env = environment()?;
    let mut model = Model::with_env("csr-rows", &env)?;
    let x = columns(&mut model, 3)?;
    let names: [&CStr; 3] = [c"first", c"empty", c"last"];
    let constrs = model.add_constrs_csr(
        &[0, 2, 2],
        &[2, 0, 1],
        &[3.0, -1.0, 5.0],
        &[ConstrSense::Less, ConstrSense::Equal, ConstrSense::Greater],
        &[4.0, 0.0, -2.0],
        &names,
    )?;
    model.update()?;

    assert_eq!(constrs.len(), 3);
    assert_eq!(model.get_coeff(&x[2], &constrs[0])?, 3.0);
    assert_eq!(model.get_coeff(&x[0], &constrs[0])?, -1.0);
    assert_eq!(model.get_coeff(&x[1], &constrs[2])?, 5.0);
    assert_eq!(model.get_coeff(&x[0], &constrs[1])?, 0.0);
    assert_eq!(model.get_obj_attr(attr::ConstrName, &constrs[1])?, "empty");
    assert_eq!(model.get_obj_attr(attr::RHS, &constrs[2])?, -2.0);
    assert_eq!(
        model.get_obj_attr(attr::Sense, &constrs[0])?,
        ConstrSense::Less
    );
    Ok(())
}

#[test]
fn csr_ranges_return_slack_columns() -> gurobi_rs::Result<()> {
    let env = environment()?;
    let mut model = Model::with_env("csr-ranges", &env)?;
    let x = columns(&mut model, 2)?;
    let (slacks, constrs) = model.add_ranges_csr(
        &[0, 1],
        &[0, 0, 1],
        &[1.0, 2.0, 3.0],
        &[-1.0, 0.0],
        &[1.0, 4.0],
        &[],
    )?;
    model.update()?;

    assert_eq!((slacks.len(), constrs.len()), (2, 2));
    assert_eq!(model.get_coeff(&x[1], &constrs[1])?, 3.0);
    assert_eq!(model.get_attr(attr::NumVars)?, 4);
    assert_eq!(model.var_index(&slacks[1])?, 3);
    Ok(())
}

#[cfg(feature = "gurobi13")]
#[test]
fn raw_columns_quadratic_and_general_rows_use_native_indices() -> gurobi_rs::Result<()> {
    let env = environment()?;
    let mut model = Model::with_env("raw-columns", &env)?;
    let x = columns(&mut model, 2)?;
    let aux = model.add_vars_raw(
        &[],
        &[0.0, f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY],
        &[f64::INFINITY; 4],
        &[Continuous; 4],
        &[c"abs", c"min", c"max", c"nl"],
    )?;
    let index = |v: &Var| model.var_index(v);
    let (x0, x1) = (index(&x[0])?, index(&x[1])?);
    let [a, lo, hi, nl] = [
        index(&aux[0])?,
        index(&aux[1])?,
        index(&aux[2])?,
        index(&aux[3])?,
    ];
    assert_eq!([a, lo, hi, nl], [2, 3, 4, 5]);

    let q = model.add_qconstr_raw(
        c"q",
        &[x0],
        &[1.0],
        &[x0, x1],
        &[x0, x1],
        &[1.0, 1.0],
        ConstrSense::Less,
        9.0,
    )?;
    model.add_genconstr_abs_raw(c"abs_def", a, x0)?;
    model.add_genconstr_min_raw(c"min_def", lo, &[x0, x1], f64::INFINITY)?;
    model.add_genconstr_max_raw(c"max_def", hi, &[x0, x1], f64::NEG_INFINITY)?;
    model.add_genconstr_nl_raw(
        c"nl_def",
        nl,
        &[
            gurobi_rs::Opcode::Exp as i32,
            gurobi_rs::Opcode::Variable as i32,
        ],
        &[-1.0, f64::from(x1)],
        &[-1, 0],
    )?;
    model.update()?;

    assert_eq!(model.get_attr(attr::NumVars)?, 6);
    assert_eq!(model.get_attr(attr::NumGenConstrs)?, 4);
    let terms = model.get_qconstr_terms(&q)?;
    assert_eq!(terms.quadratic.len(), 2);
    Ok(())
}

#[test]
fn raw_uploads_reject_invalid_layouts_before_native_calls() -> gurobi_rs::Result<()> {
    let env = environment()?;
    let mut model = Model::with_env("raw-invalid", &env)?;
    columns(&mut model, 2)?;
    let less = [ConstrSense::Less];

    // Column outside the model, non-monotone starts, and length mismatches.
    assert!(
        model
            .add_constrs_csr(&[0], &[2], &[1.0], &less, &[0.0], &[])
            .is_err()
    );
    assert!(
        model
            .add_constrs_csr(&[0], &[-1], &[1.0], &less, &[0.0], &[])
            .is_err()
    );
    assert!(
        model
            .add_constrs_csr(&[0, 2], &[0], &[1.0], &[less[0], less[0]], &[0.0, 0.0], &[])
            .is_err()
    );
    assert!(
        model
            .add_constrs_csr(&[1, 0], &[0], &[1.0], &[less[0], less[0]], &[0.0, 0.0], &[])
            .is_err()
    );
    assert!(
        model
            .add_constrs_csr(&[0], &[0], &[], &less, &[0.0], &[])
            .is_err()
    );
    assert!(
        model
            .add_constrs_csr(&[0], &[0], &[1.0], &less, &[0.0], &[c"a", c"b"])
            .is_err()
    );
    assert!(
        model
            .add_ranges_csr(&[0], &[0], &[1.0], &[0.0], &[], &[])
            .is_err()
    );
    assert!(
        model
            .add_qconstr_raw(c"q", &[5], &[1.0], &[], &[], &[], less[0], 0.0)
            .is_err()
    );
    assert!(model.add_genconstr_abs_raw(c"a", 7, 0).is_err());
    assert!(
        model
            .add_vars_raw(&[], &[0.0], &[], &[Continuous], &[])
            .is_err()
    );
    assert!(
        model
            .add_vars_raw(&[1.0, 2.0], &[0.0], &[1.0], &[Continuous], &[])
            .is_err()
    );
    // Entries for an empty batch, and a column repeated within a row.
    assert!(
        model
            .add_constrs_csr(&[], &[0], &[1.0], &[], &[], &[])
            .is_err()
    );
    assert!(
        model
            .add_constrs_csr(&[0], &[1, 0, 1], &[1.0; 3], &less, &[0.0], &[])
            .is_err()
    );
    assert!(
        model
            .add_ranges_csr(&[0], &[0, 0], &[1.0; 2], &[0.0], &[1.0], &[])
            .is_err()
    );

    // Empty batches are no-ops.
    assert!(
        model
            .add_constrs_csr(&[], &[], &[], &[], &[], &[])?
            .is_empty()
    );
    assert!(
        model
            .add_ranges_csr(&[], &[], &[], &[], &[], &[])?
            .0
            .is_empty()
    );
    model.update()?;
    assert_eq!(model.get_attr(attr::NumConstrs)?, 0);
    Ok(())
}

#[test]
fn long_rows_detect_repeats_with_the_marker_array() -> gurobi_rs::Result<()> {
    let env = environment()?;
    let mut model = Model::with_env("raw-long-rows", &env)?;
    columns(&mut model, 40)?;
    let mut ind: Vec<i32> = (0..40).collect();
    let less = [ConstrSense::Less; 2];
    // Rows may share columns; a column repeated inside one row is rejected.
    let constrs = model.add_constrs_csr(&[0, 20], &ind, &[1.0; 40], &less, &[1.0; 2], &[])?;
    assert_eq!(constrs.len(), 2);
    let both: Vec<i32> = ind.iter().chain(&ind).copied().collect();
    let constrs = model.add_constrs_csr(&[0, 40], &both, &[1.0; 80], &less, &[1.0; 2], &[])?;
    assert_eq!(constrs.len(), 2);
    ind[39] = 3;
    assert!(
        model
            .add_constrs_csr(&[0], &ind, &[1.0; 40], &less[..1], &[1.0], &[])
            .is_err()
    );
    model.update()?;
    assert_eq!(model.get_attr(attr::NumConstrs)?, 4);
    Ok(())
}

#[test]
fn objective_coefficients_are_optional() -> gurobi_rs::Result<()> {
    let env = environment()?;
    let mut model = Model::with_env("raw-obj", &env)?;
    let x = model.add_vars_raw(&[2.5, -1.0], &[0.0; 2], &[1.0; 2], &[Continuous; 2], &[])?;
    let y = model.add_vars_raw(&[], &[0.0], &[1.0], &[Binary], &[c"y"])?;
    model.update()?;
    assert_eq!(model.get_obj_attr(attr::Obj, &x[0])?, 2.5);
    assert_eq!(model.get_obj_attr(attr::Obj, &x[1])?, -1.0);
    assert_eq!(model.get_obj_attr(attr::Obj, &y[0])?, 0.0);
    assert_eq!(model.get_obj_attr(attr::VType, &y[0])?, Binary);
    Ok(())
}

#[test]
fn csc_columns_join_existing_rows() -> gurobi_rs::Result<()> {
    let env = environment()?;
    let mut model = Model::with_env("raw-csc", &env)?;
    columns(&mut model, 1)?;
    let rows = model.add_constrs_csr(
        &[0, 1],
        &[0, 0],
        &[1.0, 1.0],
        &[ConstrSense::Less, ConstrSense::Greater],
        &[1.0, 0.0],
        &[],
    )?;
    let (r0, r1) = (model.constr_index(&rows[0])?, model.constr_index(&rows[1])?);
    let new = model.add_vars_csc(
        &[0, 1],
        &[r1, r0, r1],
        &[2.0, 3.0, 4.0],
        &[],
        &[0.0; 2],
        &[1.0; 2],
        &[Continuous; 2],
        &[c"a", c"b"],
    )?;
    model.update()?;
    assert_eq!(model.get_coeff(&new[0], &rows[0])?, 0.0);
    assert_eq!(model.get_coeff(&new[0], &rows[1])?, 2.0);
    assert_eq!(model.get_coeff(&new[1], &rows[0])?, 3.0);
    assert_eq!(model.get_coeff(&new[1], &rows[1])?, 4.0);

    let (lb, ub, vtype) = ([0.0], [1.0], [Continuous]);
    // A row outside the model, and a row repeated within a column.
    assert!(
        model
            .add_vars_csc(&[0], &[2], &[1.0], &[], &lb, &ub, &vtype, &[])
            .is_err()
    );
    assert!(
        model
            .add_vars_csc(&[0], &[r0, r0], &[1.0; 2], &[], &lb, &ub, &vtype, &[])
            .is_err()
    );
    Ok(())
}

#[test]
fn lazy_mode_rejects_columns_without_an_index() -> gurobi_rs::Result<()> {
    let mut env = environment()?;
    env.set(param::UpdateMode, 0)?;
    let mut model = Model::with_env("raw-lazy", &env)?;
    columns(&mut model, 1)?;
    model.update()?;
    let pending = columns(&mut model, 1)?;
    let less = [ConstrSense::Less];

    assert!(
        model
            .add_constrs_csr(&[0], &[1], &[1.0], &less, &[0.0], &[])
            .is_err()
    );
    assert!(model.add_genconstr_abs_raw(c"a", 1, 0).is_err());
    model.add_constrs_csr(&[0], &[0], &[1.0], &less, &[0.0], &[])?;
    model.update()?;
    assert_eq!(model.get_attr(attr::NumVars)?, 2);
    assert_eq!(model.get_attr(attr::NumConstrs)?, 1);
    assert_eq!(model.var_index(&pending[0])?, 1);
    model.add_constrs_csr(&[0], &[1], &[1.0], &less, &[0.0], &[])?;
    Ok(())
}

#[test]
fn failed_native_adds_discard_pending_changes_on_both_sides() -> gurobi_rs::Result<()> {
    let env = environment()?;
    let mut model = Model::with_env("raw-rollback", &env)?;
    let x = columns(&mut model, 3)?;
    model.update()?;

    model.remove(x[2])?;
    let extra = columns(&mut model, 1)?;
    let row = model.add_constrs_csr(&[0], &[0], &[1.0], &[ConstrSense::Less], &[1.0], &[])?;

    let failed = model.add_vars_raw(&[], &[f64::NAN], &[f64::NAN], &[Continuous], &[]);
    assert!(matches!(failed, Err(gurobi_rs::Error::FromAPI(..))));

    model.update()?;
    assert_eq!(model.get_attr(attr::NumVars)?, 3);
    assert_eq!(model.get_attr(attr::NumConstrs)?, 0);
    assert_eq!(model.get_vars()?.len(), 3);
    assert!(model.get_constrs()?.is_empty());
    assert_eq!(model.var_index(&x[2])?, 2);
    assert_eq!(model.get_obj_attr(attr::VarName, &x[2])?, "x2");
    assert_eq!(
        model.var_index(&extra[0]),
        Err(gurobi_rs::Error::ModelObjectRemoved)
    );
    assert_eq!(
        model.constr_index(&row[0]),
        Err(gurobi_rs::Error::ModelObjectRemoved)
    );

    let again = columns(&mut model, 1)?;
    model.update()?;
    assert_eq!(model.var_index(&again[0])?, 3);
    Ok(())
}

#[cfg(feature = "gurobi13")]
#[test]
fn nonlinear_trees_are_validated_before_native_calls() -> gurobi_rs::Result<()> {
    use gurobi_rs::Opcode::{Exp, Plus, Variable};
    let env = environment()?;
    let mut model = Model::with_env("raw-nl", &env)?;
    columns(&mut model, 3)?;
    let (plus, exp, var) = (Plus as i32, Exp as i32, Variable as i32);

    let rejected: [(Vec<i32>, Vec<f64>, Vec<i32>); 7] = [
        // Empty tree.
        (vec![], vec![], vec![]),
        // Node 0 is not the root.
        (vec![var], vec![0.0], vec![0]),
        // A parent that is not an earlier node.
        (
            vec![plus, exp, var, var],
            vec![-1.0, -1.0, 0.0, 1.0],
            vec![-1, 2, 0, 2],
        ),
        // Unknown opcode.
        (vec![99], vec![0.0], vec![-1]),
        // Variable nodes outside the columns or not integral.
        (vec![var], vec![3.0], vec![-1]),
        (vec![var], vec![0.5], vec![-1]),
        (vec![var], vec![f64::NAN], vec![-1]),
    ];
    for (opcode, data, parent) in rejected {
        assert!(
            model
                .add_genconstr_nl_raw(c"nl", 2, &opcode, &data, &parent)
                .is_err(),
            "{opcode:?} {data:?} {parent:?}"
        );
    }

    // x2 = x0 + exp(x1)
    model.add_genconstr_nl_raw(
        c"nl",
        2,
        &[plus, var, exp, var],
        &[-1.0, 0.0, -1.0, 1.0],
        &[-1, 0, 0, 2],
    )?;
    model.update()?;
    assert_eq!(model.get_attr(attr::NumGenConstrs)?, 1);
    Ok(())
}
