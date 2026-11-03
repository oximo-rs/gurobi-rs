use gurobi_rs::prelude::*;

fn environment() -> gurobi_rs::Result<Env> {
    let mut env = Env::new("")?;
    env.set(param::OutputFlag, 0)?;
    Ok(env)
}

fn assert_objective(
    model: &mut Model,
    linear: &[(Var, f64)],
    constant: f64,
    sense: ModelSense,
    quadratic_count: i32,
    value: f64,
) -> gurobi_rs::Result<()> {
    model.optimize()?;
    assert_eq!(model.get_attr(attr::Status)?, Status::Optimal);
    assert_eq!(model.get_attr(attr::NumQNZs)?, quadratic_count);

    for &(variable, coefficient) in linear {
        assert_eq!(model.get_obj_attr(attr::Obj, &variable)?, coefficient);
    }
    assert_eq!(model.get_attr(attr::ObjCon)?, constant);
    assert_eq!(model.get_attr(attr::ModelSense)? as i32, sense as i32);
    assert!((model.get_attr(attr::ObjVal)? - value).abs() < 1e-10);

    Ok(())
}

#[test]
fn incremental_quadratic_terms_are_additive_and_preserve_other_objective_parts()
-> gurobi_rs::Result<()> {
    let env = environment()?;
    let mut model = Model::with_env("incremental-objective", &env)?;
    let x = model.add_var("x", Continuous, 0.0, 2.0, 2.0, [])?;
    let y = model.add_var("y", Continuous, 0.0, 3.0, 3.0, [])?;
    let linear = [(x, 4.0), (y, -2.0)];
    model.set_objective_sparse(linear, [(x, x, 2.0), (x, y, 1.0)], 7.0, Maximize)?;
    assert_objective(&mut model, &linear, 7.0, Maximize, 2, 23.0)?;

    model.add_qpterms([
        (x, x, 0.5),
        (x, x, 0.5),
        (y, y, 2.0),
        (x, y, 3.0),
        (y, x, -1.0),
    ])?;
    assert_objective(&mut model, &linear, 7.0, Maximize, 3, 57.0)?;

    model.add_qpterms([(x, x, -3.0), (x, y, -4.0), (y, x, 1.0), (y, y, -0.5)])?;
    assert_objective(&mut model, &linear, 7.0, Maximize, 1, 22.5)?;
    Ok(())
}

#[test]
fn clearing_quadratic_terms_preserves_other_objective_parts() -> gurobi_rs::Result<()> {
    let env = environment()?;
    let mut model = Model::with_env("clear-objective", &env)?;
    let x = model.add_var("x", Continuous, 0.0, 2.0, 2.0, [])?;
    let y = model.add_var("y", Continuous, 0.0, 3.0, 3.0, [])?;
    let linear = [(x, 4.0), (y, -2.0)];
    model.set_objective_sparse(linear, [(x, x, 2.0), (x, y, 1.0)], 7.0, Maximize)?;
    model.del_qpterms()?;
    assert_objective(&mut model, &linear, 7.0, Maximize, 0, 9.0)?;

    model.del_qpterms()?;
    assert_objective(&mut model, &linear, 7.0, Maximize, 0, 9.0)?;
    Ok(())
}

#[test]
fn invalid_incremental_handles_and_empty_input_preserve_the_objective() -> gurobi_rs::Result<()> {
    let env = environment()?;
    let mut model = Model::with_env("incremental-owner", &env)?;
    let x = model.add_var("x", Continuous, 0.0, 2.0, 2.0, [])?;
    let removed = model.add_var("removed", Continuous, 0.0, 0.0, 0.0, [])?;
    let mut other = Model::with_env("incremental-foreign", &env)?;
    let foreign = other.add_var("foreign", Continuous, 0.0, 0.0, 0.0, [])?;
    other.update()?;
    model.set_objective_sparse([(x, 3.0)], [(x, x, 2.0)], 7.0, Maximize)?;

    for pair in [(x, foreign, 1.0), (foreign, x, 1.0)] {
        assert_eq!(
            model.add_qpterms([(x, x, 5.0), pair]),
            Err(gurobi_rs::Error::ModelObjectMismatch)
        );
        assert_objective(&mut model, &[(x, 3.0)], 7.0, Maximize, 1, 21.0)?;
    }

    model.remove(removed)?;
    for pair in [(x, removed, 1.0), (removed, x, 1.0)] {
        assert_eq!(
            model.add_qpterms([(x, x, 5.0), pair]),
            Err(gurobi_rs::Error::ModelObjectRemoved)
        );
        assert_objective(&mut model, &[(x, 3.0)], 7.0, Maximize, 1, 21.0)?;
    }

    model.add_qpterms([])?;
    assert_objective(&mut model, &[(x, 3.0)], 7.0, Maximize, 1, 21.0)?;
    Ok(())
}

#[test]
fn incremental_quadratic_terms_update_pending_variables_first() -> gurobi_rs::Result<()> {
    let env = environment()?;
    let mut model = Model::with_env("incremental-pending", &env)?;
    model.set_param(param::UpdateMode, 0)?;
    let x = model.add_var("x", Continuous, 3.0, 2.0, 2.0, [])?;
    model.add_qpterms([(x, x, 2.0)])?;
    assert_eq!(model.get_attr(attr::NumVars)?, 1);
    assert_objective(&mut model, &[(x, 3.0)], 0.0, Minimize, 1, 14.0)?;
    Ok(())
}

#[test]
fn sparse_objectives_add_duplicates_and_clear_previous_terms() -> gurobi_rs::Result<()> {
    let env = environment()?;
    let mut model = Model::with_env("sparse-objective", &env)?;

    let x = model.add_var("x", Continuous, 0.0, 1.0, 1.0, [])?;
    let y = model.add_var("y", Continuous, 0.0, 2.0, 2.0, [])?;
    model.set_objective_sparse(
        [(x, 1.0), (x, 3.0)],
        [(x, x, 1.0), (x, x, 1.0), (x, y, 1.0), (y, x, 2.0)],
        -5.0,
        Minimize,
    )?;
    model.optimize()?;
    assert_eq!(model.get_attr(attr::NumQNZs)?, 2);
    assert!((model.get_attr(attr::ObjVal)? - 7.0).abs() < 1e-10);

    model.set_objective_sparse([(y, 0.5)], [], 9.0, Maximize)?;
    model.optimize()?;
    assert_eq!(model.get_attr(attr::NumQNZs)?, 0);
    assert_eq!(model.get_obj_attr(attr::Obj, &x)?, 0.0);
    assert_eq!(model.get_attr(attr::ObjCon)?, 9.0);
    assert!((model.get_attr(attr::ObjVal)? - 10.0).abs() < 1e-10);
    assert!(matches!(
        model.get_attr(attr::ModelSense)?,
        ModelSense::Maximize
    ));

    Ok(())
}

#[test]
fn sparse_rows_preserve_senses_constants_and_duplicate_pairs() -> gurobi_rs::Result<()> {
    let env = environment()?;
    let mut model = Model::with_env("sparse-rows", &env)?;

    let x = model.add_var("x", Continuous, 0.0, 1.0, 1.0, [])?;
    let y = model.add_var("y", Continuous, 0.0, 2.0, 2.0, [])?;
    model.update()?;
    for (i, sense, rhs) in [
        (0, ConstrSense::Less, 10.0),
        (1, ConstrSense::Greater, 6.0),
        (2, ConstrSense::Equal, 7.0),
    ] {
        let row = model.add_qconstr_sparse(
            &format!("q{i}"),
            [(x, 4.0)],
            [(x, x, 1.0), (x, x, 1.0), (x, y, 1.0), (y, x, 2.0)],
            -5.0,
            sense,
            rhs,
        )?;
        model.update()?;
        assert_eq!(model.get_obj_attr(attr::QCSense, &row)?, sense as i8);
        assert_eq!(model.get_obj_attr(attr::QCRHS, &row)?, rhs + 5.0);
    }
    model.set_param(param::NonConvex, 2)?;
    model.optimize()?;
    assert_eq!(model.get_attr(attr::Status)?, Status::Optimal);

    Ok(())
}

#[test]
fn invalid_sparse_handles_do_not_replace_the_objective_or_add_rows() -> gurobi_rs::Result<()> {
    let env = environment()?;
    let mut model = Model::with_env("owner", &env)?;

    let x = model.add_var("x", Continuous, 0.0, 0.0, 1.0, [])?;
    let y = model.add_var("y", Continuous, 0.0, 0.0, 1.0, [])?;
    let mut other = Model::with_env("foreign", &env)?;
    let foreign = other.add_var("foreign", Continuous, 0.0, 0.0, 1.0, [])?;
    other.update()?;
    model.set_objective_sparse([(y, 2.0)], [(y, y, 3.0)], 7.0, Minimize)?;
    assert_eq!(
        model.set_objective_sparse([(foreign, 0.0)], [], 0.0, Maximize),
        Err(gurobi_rs::Error::ModelObjectMismatch)
    );

    for pair in [(x, foreign, 0.0), (foreign, x, 0.0)] {
        assert_eq!(
            model.set_objective_sparse([], [pair], 0.0, Maximize),
            Err(gurobi_rs::Error::ModelObjectMismatch)
        );
        assert_eq!(
            model.add_qconstr_sparse("invalid", [], [pair], 0.0, ConstrSense::Less, 1.0),
            Err(gurobi_rs::Error::ModelObjectMismatch)
        );
    }
    assert_eq!(model.get_attr(attr::ObjCon)?, 7.0);
    assert_eq!(model.get_attr(attr::NumQConstrs)?, 0);
    assert_eq!(model.get_attr(attr::NumQNZs)?, 1);

    model.remove(x)?;
    model.update()?;
    assert_eq!(
        model.set_objective_sparse([(x, 1.0)], [], 0.0, Minimize),
        Err(gurobi_rs::Error::ModelObjectRemoved)
    );

    model.set_objective_sparse([(y, 4.0)], [], 2.0, Minimize)?;
    model.update()?;
    assert_eq!(model.get_obj_attr(attr::Obj, &y)?, 4.0);

    Ok(())
}

#[test]
fn sparse_pending_handles_follow_existing_build_rules() -> gurobi_rs::Result<()> {
    let env = environment()?;
    let mut model = Model::with_env("pending", &env)?;

    model.set_param(param::UpdateMode, 0)?;
    let x = model.add_var("x", Continuous, 0.0, 0.0, 1.0, [])?;
    assert_eq!(
        model.add_qconstr_sparse("pending", [], [(x, x, 1.0)], 0.0, ConstrSense::Less, 1.0),
        Err(gurobi_rs::Error::ModelObjectPending)
    );

    model.set_objective_sparse([], [(x, x, 1.0)], 3.0, Minimize)?;
    assert_eq!(model.get_attr(attr::NumVars)?, 1);
    assert!(
        model
            .add_qconstr_sparse("bad\0name", [], [(x, x, 1.0)], 0.0, ConstrSense::Less, 1.0)
            .is_err()
    );

    Ok(())
}
