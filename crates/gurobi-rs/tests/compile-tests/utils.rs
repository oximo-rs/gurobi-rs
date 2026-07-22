#[macro_export]
macro_rules! create_model {
  ($gag:ident, $model:ident $(,$var:ident)*) => {
    // `gag` suppresses the Gurobi license banner on stdout, but only builds on Unix.
    #[cfg(unix)]
    let $gag = gag::Gag::stdout().unwrap();
    #[cfg(not(unix))]
    let $gag = ();
    let mut $model = Model::new("test")?;
    $(
      let $var = add_binvar!($model)?;
    )*
  }
}
