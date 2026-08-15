use gurobi_rs::constr::RangeExpr;
use gurobi_rs::prelude::*;

#[test]
fn add_constrs_uses_total_nonzeros_and_returns_distinct_handles() -> gurobi_rs::Result<()> {
    let mut env = Env::new("")?;
    env.set(param::OutputFlag, 0)?;
    let mut model = Model::with_env("batched-constrs", &env)?;

    let x = add_ctsvar!(model, name: "x")?;
    let y = add_ctsvar!(model, name: "y")?;
    let z = add_ctsvar!(model, name: "z")?;
    let constraints = [
        (&"c1", c!(x + y <= 1)),
        (&"c2", c!(x + 2.0 * y + 3.0 * z >= 0)),
    ];

    let constrs = model.add_constrs(constraints)?;
    assert_eq!(constrs.len(), 2);
    assert_ne!(constrs[0], constrs[1]);

    model.update()?;
    assert_eq!(model.get_attr(attr::NumConstrs)?, 2);
    assert_eq!(model.get_attr(attr::NumNZs)?, 5);
    assert_eq!(model.get_obj_attr(attr::ConstrName, &constrs[0])?, "c1");
    assert_eq!(model.get_obj_attr(attr::ConstrName, &constrs[1])?, "c2");
    Ok(())
}

#[test]
fn add_ranges_uses_total_nonzeros_and_returns_distinct_handles() -> gurobi_rs::Result<()> {
    let mut env = Env::new("")?;
    env.set(param::OutputFlag, 0)?;
    let mut model = Model::with_env("batched-ranges", &env)?;

    let x = add_ctsvar!(model, name: "x")?;
    let y = add_ctsvar!(model, name: "y")?;
    let z = add_ctsvar!(model, name: "z")?;
    let ranges = [
        (
            &"r1",
            RangeExpr {
                expr: x + y,
                lb: 0.0,
                ub: 2.0,
            },
        ),
        (
            &"r2",
            RangeExpr {
                expr: x + 2.0 * y + 3.0 * z,
                lb: 1.0,
                ub: 4.0,
            },
        ),
    ];

    let (range_vars, constrs) = model.add_ranges(ranges)?;
    assert_eq!(range_vars.len(), 2);
    assert_eq!(constrs.len(), 2);
    assert_ne!(range_vars[0], range_vars[1]);
    assert_ne!(constrs[0], constrs[1]);

    model.update()?;
    assert_eq!(model.get_attr(attr::NumVars)?, 5);
    assert_eq!(model.get_attr(attr::NumConstrs)?, 2);
    assert_eq!(model.get_obj_attr(attr::ConstrName, &constrs[0])?, "r1");
    assert_eq!(model.get_obj_attr(attr::ConstrName, &constrs[1])?, "r2");
    assert_eq!(model.get_coeff(&x, &constrs[1])?, 1.0);
    assert_eq!(model.get_coeff(&y, &constrs[1])?, 2.0);
    assert_eq!(model.get_coeff(&z, &constrs[1])?, 3.0);
    Ok(())
}
