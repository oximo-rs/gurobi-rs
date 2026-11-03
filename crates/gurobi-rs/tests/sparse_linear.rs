use gurobi_rs::prelude::*;

fn environment() -> gurobi_rs::Result<Env> {
    let mut env = Env::new("")?;
    env.set(param::OutputFlag, 0)?;
    Ok(env)
}

#[test]
fn add_constrs_sparse_batches_rows_with_constants_and_duplicates() -> gurobi_rs::Result<()> {
    let env = environment()?;
    let mut model = Model::with_env("sparse-constrs", &env)?;

    let x = model.add_var("x", Continuous, 0.0, -10.0, 10.0, [])?;
    let y = model.add_var("y", Continuous, 0.0, -10.0, 10.0, [])?;
    let z = model.add_var("z", Continuous, 0.0, -10.0, 10.0, [])?;

    let constrs = model.add_constrs_sparse([
        (
            &"c1",
            vec![(x, 1.0), (y, 2.0), (x, 3.0)],
            4.0,
            ConstrSense::Less,
            10.0,
        ),
        (
            &"c2",
            vec![(y, -1.0), (z, 2.0)],
            -1.0,
            ConstrSense::Greater,
            0.0,
        ),
        (
            &"c3",
            Vec::<(Var, f64)>::new(),
            2.0,
            ConstrSense::Equal,
            2.0,
        ),
    ])?;
    assert_eq!(constrs.len(), 3);
    assert!(constrs.windows(2).all(|pair| pair[0] != pair[1]));

    model.update()?;
    assert_eq!(model.get_attr(attr::NumConstrs)?, 3);
    assert_eq!(model.get_attr(attr::NumNZs)?, 4);
    assert_eq!(model.get_obj_attr(attr::ConstrName, &constrs[0])?, "c1");
    assert_eq!(model.get_obj_attr(attr::ConstrName, &constrs[1])?, "c2");
    assert_eq!(model.get_obj_attr(attr::ConstrName, &constrs[2])?, "c3");

    assert_eq!(model.get_obj_attr(attr::RHS, &constrs[0])?, 6.0);
    assert_eq!(model.get_obj_attr(attr::RHS, &constrs[1])?, 1.0);
    assert_eq!(model.get_obj_attr(attr::RHS, &constrs[2])?, 0.0);
    assert_eq!(
        model.get_obj_attr(attr::Sense, &constrs[0])? as i8,
        ConstrSense::Less as i8
    );
    assert_eq!(
        model.get_obj_attr(attr::Sense, &constrs[1])? as i8,
        ConstrSense::Greater as i8
    );
    assert_eq!(
        model.get_obj_attr(attr::Sense, &constrs[2])? as i8,
        ConstrSense::Equal as i8
    );

    assert_eq!(model.get_coeff(&x, &constrs[0])?, 4.0);
    assert_eq!(model.get_coeff(&y, &constrs[0])?, 2.0);
    assert_eq!(model.get_coeff(&y, &constrs[1])?, -1.0);
    assert_eq!(model.get_coeff(&z, &constrs[1])?, 2.0);

    Ok(())
}

#[test]
fn add_ranges_sparse_batches_rows_and_returns_distinct_handles() -> gurobi_rs::Result<()> {
    let env = environment()?;
    let mut model = Model::with_env("sparse-ranges", &env)?;

    let x = model.add_var("x", Continuous, 0.0, -10.0, 10.0, [])?;
    let y = model.add_var("y", Continuous, 0.0, -10.0, 10.0, [])?;

    let (range_vars, constrs) = model.add_ranges_sparse([
        (&"r1", vec![(x, 1.0), (y, 2.0)], 3.0, 0.0, 5.0),
        (&"r2", vec![(y, 1.0)], -2.0, 1.0, 4.0),
    ])?;
    assert_eq!(range_vars.len(), 2);
    assert_eq!(constrs.len(), 2);
    assert_ne!(range_vars[0], range_vars[1]);
    assert_ne!(constrs[0], constrs[1]);

    model.update()?;
    assert_eq!(model.get_attr(attr::NumVars)?, 4);
    assert_eq!(model.get_attr(attr::NumConstrs)?, 2);
    assert_eq!(model.get_obj_attr(attr::ConstrName, &constrs[0])?, "r1");
    assert_eq!(model.get_obj_attr(attr::ConstrName, &constrs[1])?, "r2");
    assert_eq!(model.get_coeff(&x, &constrs[0])?, 1.0);
    assert_eq!(model.get_coeff(&y, &constrs[0])?, 2.0);
    assert_eq!(model.get_coeff(&y, &constrs[1])?, 1.0);

    Ok(())
}

#[test]
fn add_ranges_sparse_applies_shifted_bounds() -> gurobi_rs::Result<()> {
    let env = environment()?;
    let mut model = Model::with_env("sparse-range-bounds", &env)?;

    let x = model.add_var("x", Continuous, 0.0, -10.0, 10.0, [])?;
    model.add_ranges_sparse([(&"r", vec![(x, 1.0)], 5.0, 0.0, 1.0)])?;

    model.set_objective(x, Maximize)?;
    model.optimize()?;
    assert_eq!(model.get_attr(attr::Status)?, Status::Optimal);
    assert!((model.get_attr(attr::ObjVal)? - (-4.0)).abs() < 1e-10);

    Ok(())
}

#[test]
fn sparse_row_batches_accept_empty_iterators() -> gurobi_rs::Result<()> {
    let env = environment()?;
    let mut model = Model::with_env("sparse-empty", &env)?;

    let constrs =
        model.add_constrs_sparse(Vec::<(&&str, Vec<(Var, f64)>, f64, ConstrSense, f64)>::new())?;
    assert!(constrs.is_empty());

    let (vars, constrs) =
        model.add_ranges_sparse(Vec::<(&&str, Vec<(Var, f64)>, f64, f64, f64)>::new())?;
    assert!(vars.is_empty());
    assert!(constrs.is_empty());

    model.update()?;
    assert_eq!(model.get_attr(attr::NumConstrs)?, 0);
    assert_eq!(model.get_attr(attr::NumVars)?, 0);

    Ok(())
}

#[test]
fn sparse_rows_accept_zero_nonzero_rows() -> gurobi_rs::Result<()> {
    let env = environment()?;
    let mut model = Model::with_env("sparse-zero-rows", &env)?;

    let constrs = model.add_constrs_sparse([(
        &"empty",
        Vec::<(Var, f64)>::new(),
        0.0,
        ConstrSense::Less,
        1.0,
    )])?;
    assert_eq!(constrs.len(), 1);

    model.update()?;
    assert_eq!(model.get_attr(attr::NumConstrs)?, 1);
    assert_eq!(model.get_attr(attr::NumNZs)?, 0);

    let (range_vars, range_constrs) =
        model.add_ranges_sparse([(&"range", Vec::<(Var, f64)>::new(), 0.0, 0.0, 1.0)])?;
    assert_eq!(range_vars.len(), 1);
    assert_eq!(range_constrs.len(), 1);

    model.update()?;
    assert_eq!(model.get_attr(attr::NumVars)?, 1);
    assert_eq!(model.get_attr(attr::NumConstrs)?, 2);

    Ok(())
}

#[test]
fn sparse_rows_validate_handles_and_names_before_mutating() -> gurobi_rs::Result<()> {
    let env = environment()?;
    let mut model = Model::with_env("sparse-linear-errors", &env)?;

    let x = model.add_var("x", Continuous, 0.0, 0.0, 1.0, [])?;
    let mut other = Model::with_env("sparse-linear-foreign", &env)?;
    let foreign = other.add_var("foreign", Continuous, 0.0, 0.0, 1.0, [])?;
    other.update()?;
    model.update()?;

    assert_eq!(
        model.add_constrs_sparse([(
            &"foreign",
            vec![(x, 1.0), (foreign, 1.0)],
            0.0,
            ConstrSense::Less,
            1.0,
        )]),
        Err(gurobi_rs::Error::ModelObjectMismatch)
    );
    assert!(matches!(
        model.add_constrs_sparse([(
            &"invalid\0name",
            vec![(x, 1.0)],
            0.0,
            ConstrSense::Less,
            1.0,
        )]),
        Err(gurobi_rs::Error::NulError(_))
    ));
    assert_eq!(
        model.add_ranges_sparse([(&"foreign", vec![(foreign, 1.0)], 0.0, 0.0, 1.0,)]),
        Err(gurobi_rs::Error::ModelObjectMismatch)
    );
    assert_eq!(model.get_attr(attr::NumConstrs)?, 0);
    assert_eq!(model.get_attr(attr::NumNZs)?, 0);

    model.remove(x)?;
    model.update()?;
    assert_eq!(
        model.add_constrs_sparse([(&"removed", vec![(x, 1.0)], 0.0, ConstrSense::Less, 1.0,)]),
        Err(gurobi_rs::Error::ModelObjectRemoved)
    );

    model.set_param(param::UpdateMode, 0)?;
    let pending = model.add_var("pending", Continuous, 0.0, 0.0, 1.0, [])?;
    assert_eq!(
        model.add_constrs_sparse([(
            &"pending",
            vec![(pending, 1.0)],
            0.0,
            ConstrSense::Less,
            1.0,
        )]),
        Err(gurobi_rs::Error::ModelObjectPending)
    );

    Ok(())
}
