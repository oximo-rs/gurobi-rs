//! Index-based upload entry points for callers that construct native buffers directly.
//!
//! These methods bypass handle lookups, name conversion, and duplicate merging.
//! Callers must provide native indices (obtained from [`Model::var_index`] and
//! [`Model::constr_index`]), NUL-terminated names, and sparse vectors with no
//! duplicate entries.
//!
//! Indices must match Gurobi's current ordering. In immediate update mode
//! (`UpdateMode=1`), this includes objects added since the last update. In lazy
//! update mode (`UpdateMode=0`), objects added since the last update are rejected.
//!
//! All inputs are validated before invoking the native API.
//! Failed add operation causes Gurobi to discard all modifications
//! made since the last update.

use std::ffi::CStr;
use std::ptr::null;

use super::Model;
use crate::ffi;
use crate::prelude::*;
use crate::{Error, Result};

// `ConstrSense` and `VarType` slices are passed to Gurobi without copying.
const _: () = assert!(size_of::<ConstrSense>() == size_of::<ffi::c_char>());
const _: () = assert!(size_of::<VarType>() == size_of::<ffi::c_char>());

/// Groups up to this length are checked for repeats pairwise.
const PAIRWISE_REPEAT_LIMIT: usize = 16;

/// Signature shared by `GRBaddgenconstrMin` and `GRBaddgenconstrMax`.
type ExtremumFn = unsafe extern "C" fn(
    *mut ffi::GRBmodel,
    *const ffi::c_char,
    ffi::c_int,
    ffi::c_int,
    *const ffi::c_int,
    f64,
) -> ffi::c_int;

fn invalid(message: String) -> Error {
    Error::AlgebraicError(message)
}

/// Name pointers for `n` objects, or `None` when no names are given.
fn name_pointers(names: &[&CStr], n: usize) -> Result<Option<Vec<*const ffi::c_char>>> {
    match names.len() {
        0 => Ok(None),
        len if len == n => Ok(Some(names.iter().map(|name| name.as_ptr()).collect())),
        len => Err(invalid(format!("expected {n} names, got {len}"))),
    }
}

fn names_ptr(names: Option<&Vec<*const ffi::c_char>>) -> *const *const ffi::c_char {
    names.map_or(null(), |names| names.as_ptr())
}

fn as_ptr_or_null<T>(values: &[T]) -> *const T {
    if values.is_empty() {
        null()
    } else {
        values.as_ptr()
    }
}

/// Check that `index` addresses one of `count` native objects.
fn check_index(index: i32, count: usize, what: &str) -> Result<()> {
    if usize::try_from(index).is_ok_and(|index| index < count) {
        Ok(())
    } else {
        Err(invalid(format!(
            "{what} index {index} is outside 0..{count}"
        )))
    }
}

/// Check that `values` is empty (Gurobi default) or holds one entry per object.
fn check_optional_len<T>(values: &[T], n: usize, what: &str) -> Result<()> {
    if values.is_empty() || values.len() == n {
        Ok(())
    } else {
        Err(invalid(format!(
            "expected {n} {what}, got {}",
            values.len()
        )))
    }
}

/// Validate a compressed sparse layout of `ngroups` rows (or columns) whose
/// entries index `width` native objects of kind `what`.
fn check_compressed(
    ngroups: usize,
    beg: &[i32],
    ind: &[i32],
    val: &[f64],
    width: usize,
    what: &str,
) -> Result<()> {
    if beg.len() != ngroups {
        return Err(invalid(format!(
            "expected {ngroups} starts, got {}",
            beg.len()
        )));
    }
    if ind.len() != val.len() {
        return Err(invalid(
            "index and value slices must have the same length".into(),
        ));
    }
    if ngroups == 0 && !ind.is_empty() {
        return Err(invalid(format!(
            "{} entries given for an empty batch",
            ind.len()
        )));
    }

    crate::util::checked_count(ngroups)?;
    crate::util::checked_count(ind.len())?;
    let mut previous = 0;
    for (group, &start) in beg.iter().enumerate() {
        let in_range = usize::try_from(start).is_ok_and(|start| start <= ind.len());
        if !in_range || start < previous || (group == 0 && start != 0) {
            return Err(invalid(format!(
                "start {start} of group {group} is not a nondecreasing offset into {} entries",
                ind.len()
            )));
        }
        previous = start;
    }

    for &index in ind {
        check_index(index, width, what)?;
    }

    check_no_repeats(beg, ind, width, what)
}

/// Reject an index repeated within one group.
/// Expects validated starts and indices.
fn check_no_repeats(beg: &[i32], ind: &[i32], width: usize, what: &str) -> Result<()> {
    let mut seen_in: Vec<u32> = Vec::new();
    for (group, &start) in beg.iter().enumerate() {
        let end = beg.get(group + 1).map_or(ind.len(), |&end| end as usize);
        let entries = &ind[start as usize..end];
        let repeat = if entries.len() <= PAIRWISE_REPEAT_LIMIT {
            entries
                .iter()
                .enumerate()
                .find(|&(k, index)| entries[..k].contains(index))
                .map(|(_, &index)| index)
        } else {
            if seen_in.is_empty() {
                seen_in = vec![u32::MAX; width];
            }

            let tag = group as u32;
            entries.iter().copied().find(|&index| {
                let slot = &mut seen_in[index as usize];
                std::mem::replace(slot, tag) == tag
            })
        };

        if let Some(index) = repeat {
            return Err(invalid(format!(
                "{what} index {index} repeats within group {group}"
            )));
        }
    }

    Ok(())
}

/// Validate an expression tree for `GRBaddgenconstrNL` over `columns` variables.
#[cfg(feature = "gurobi13")]
fn check_nl_tree(opcode: &[i32], data: &[f64], parent: &[i32], columns: usize) -> Result<()> {
    if opcode.len() != data.len() || opcode.len() != parent.len() {
        return Err(invalid(
            "opcode, data and parent slices must have the same length".into(),
        ));
    }

    if opcode.is_empty() {
        return Err(invalid("an expression tree needs at least one node".into()));
    }

    crate::util::checked_count(opcode.len())?;
    let max_opcode = crate::Opcode::SignPow as i32;
    for (node, ((&op, &value), &up)) in opcode.iter().zip(data).zip(parent).enumerate() {
        if !(0..=max_opcode).contains(&op) {
            return Err(invalid(format!("node {node} has unknown opcode {op}")));
        }
        let parent_ok = if node == 0 {
            up == -1
        } else {
            usize::try_from(up).is_ok_and(|up| up < node)
        };

        if !parent_ok {
            return Err(invalid(format!(
                "node {node} has parent {up}; node 0 must be the root (-1) and every \
                 other parent must be an earlier node"
            )));
        }
        if op == crate::Opcode::Variable as i32 {
            let column_ok = value.fract() == 0.0 && value >= 0.0 && value < columns as f64;
            if !column_ok {
                return Err(invalid(format!(
                    "node {node} refers to column {value}, which is not an index in 0..{columns}"
                )));
            }
        }
    }

    Ok(())
}

impl Model {
    /// Number of native columns that raw indices may refer to.
    fn raw_column_count(&self) -> usize {
        self.vars.indexed_len()
    }

    /// Number of native linear rows that raw indices may refer to.
    fn raw_row_count(&self) -> usize {
        self.constrs.indexed_len()
    }

    fn check_raw_column(&self, index: i32) -> Result<()> {
        check_index(index, self.raw_column_count(), "column")
    }

    /// Add linear rows given in compressed sparse row form with one native call.
    ///
    /// # Errors
    /// - [`Error::AlgebraicError`] for inconsistent lengths, invalid row starts,
    ///   repeated columns, or column indices outside the indexed columns.
    /// - [`Error::FromAPI`] if a Gurobi API error occurs.
    pub fn add_constrs_csr(
        &mut self,
        beg: &[i32],
        ind: &[i32],
        val: &[f64],
        sense: &[ConstrSense],
        rhs: &[f64],
        names: &[&CStr],
    ) -> Result<Vec<Constr>> {
        let nrows = sense.len();
        if rhs.len() != nrows {
            return Err(invalid(format!(
                "expected {nrows} right-hand sides, got {}",
                rhs.len()
            )));
        }

        check_compressed(nrows, beg, ind, val, self.raw_column_count(), "column")?;
        if nrows == 0 {
            return Ok(Vec::new());
        }

        let names = name_pointers(names, nrows)?;
        let lazy = self.update_mode_lazy()?;
        self.check_add_apicall(unsafe {
            ffi::GRBaddconstrs(
                self.ptr,
                Self::sparse_count(nrows)?,
                Self::sparse_count(ind.len())?,
                beg.as_ptr(),
                as_ptr_or_null(ind),
                as_ptr_or_null(val),

                // `ConstrSense` is `repr(u8)`.
                sense.as_ptr().cast::<ffi::c_char>(),
                rhs.as_ptr(),
                names_ptr(names.as_ref()),
            )
        })?;

        Ok((0..nrows).map(|_| self.constrs.add_new(lazy)).collect())
    }

    /// Add range rows `lower[i] <= sum(val * x[ind]) <= upper[i]` in CSR form,
    /// returning the generated range variables and the rows.
    ///
    /// The layout and name rules match [`Model::add_constrs_csr`].
    /// Gurobi names each range variable `Rg` followed by its row's name.
    ///
    /// # Errors
    /// - [`Error::AlgebraicError`] for inconsistent lengths, invalid row starts,
    ///   repeated columns, or column indices outside the indexed columns.
    /// - [`Error::FromAPI`] if a Gurobi API error occurs.
    pub fn add_ranges_csr(
        &mut self,
        beg: &[i32],
        ind: &[i32],
        val: &[f64],
        lower: &[f64],
        upper: &[f64],
        names: &[&CStr],
    ) -> Result<(Vec<Var>, Vec<Constr>)> {
        let nrows = lower.len();
        if upper.len() != nrows {
            return Err(invalid(format!(
                "expected {nrows} upper bounds, got {}",
                upper.len()
            )));
        }

        check_compressed(nrows, beg, ind, val, self.raw_column_count(), "column")?;
        if nrows == 0 {
            return Ok((Vec::new(), Vec::new()));
        }

        let names = name_pointers(names, nrows)?;
        let lazy = self.update_mode_lazy()?;
        self.check_add_apicall(unsafe {
            ffi::GRBaddrangeconstrs(
                self.ptr,
                Self::sparse_count(nrows)?,
                Self::sparse_count(ind.len())?,
                beg.as_ptr(),
                as_ptr_or_null(ind),
                as_ptr_or_null(val),
                lower.as_ptr(),
                upper.as_ptr(),
                names_ptr(names.as_ref()),
            )
        })?;

        let vars = (0..nrows).map(|_| self.vars.add_new(lazy)).collect();
        let constrs = (0..nrows).map(|_| self.constrs.add_new(lazy)).collect();

        Ok((vars, constrs))
    }

    /// Add columns without constraint coefficients in one native call.
    ///
    /// `lb`, `ub` and `vtype` have one entry per column.
    /// `obj` and `names` may instead be empty.
    ///
    /// # Errors
    /// - [`Error::AlgebraicError`] for inconsistent lengths.
    /// - [`Error::FromAPI`] if a Gurobi API error occurs.
    pub fn add_vars_raw(
        &mut self,
        obj: &[f64],
        lb: &[f64],
        ub: &[f64],
        vtype: &[VarType],
        names: &[&CStr],
    ) -> Result<Vec<Var>> {
        let n = lb.len();
        self.add_vars_native(n, None, obj, lb, ub, vtype, names)
    }

    /// Add columns with their constraint coefficients, given in compressed
    /// sparse column form, in one native call.
    ///
    /// # Errors
    /// - [`Error::AlgebraicError`] for inconsistent lengths, invalid column
    ///   starts, repeated rows, or row indices outside the indexed rows.
    /// - [`Error::FromAPI`] if a Gurobi API error occurs.
    #[allow(clippy::too_many_arguments)]
    pub fn add_vars_csc(
        &mut self,
        vbeg: &[i32],
        vind: &[i32],
        vval: &[f64],
        obj: &[f64],
        lb: &[f64],
        ub: &[f64],
        vtype: &[VarType],
        names: &[&CStr],
    ) -> Result<Vec<Var>> {
        let n = lb.len();
        check_compressed(n, vbeg, vind, vval, self.raw_row_count(), "row")?;
        self.add_vars_native(n, Some((vbeg, vind, vval)), obj, lb, ub, vtype, names)
    }

    #[allow(clippy::too_many_arguments)]
    fn add_vars_native(
        &mut self,
        n: usize,
        columns: Option<(&[i32], &[i32], &[f64])>,
        obj: &[f64],
        lb: &[f64],
        ub: &[f64],
        vtype: &[VarType],
        names: &[&CStr],
    ) -> Result<Vec<Var>> {
        if ub.len() != n || vtype.len() != n {
            return Err(invalid(
                "bound and type slices must have the same length".into(),
            ));
        }

        check_optional_len(obj, n, "objective coefficients")?;
        if n == 0 {
            return Ok(Vec::new());
        }

        let names = name_pointers(names, n)?;
        let (vbeg, vind, vval) = columns.unwrap_or((&[], &[], &[]));
        let lazy = self.update_mode_lazy()?;
        self.check_add_apicall(unsafe {
            ffi::GRBaddvars(
                self.ptr,
                Self::sparse_count(n)?,
                Self::sparse_count(vind.len())?,
                as_ptr_or_null(vbeg),
                as_ptr_or_null(vind),
                as_ptr_or_null(vval),

                // A null obj equals all-zero coef.
                as_ptr_or_null(obj),
                lb.as_ptr(),
                ub.as_ptr(),

                // `VarType` is `repr(u8)`.
                vtype.as_ptr().cast::<ffi::c_char>(),
                names_ptr(names.as_ref()),
            )
        })?;

        Ok((0..n).map(|_| self.vars.add_new(lazy)).collect())
    }

    /// Add `sum(lval * x[lind]) + sum(qval * x[qrow] * x[qcol]) sense rhs`.
    ///
    /// Repeated linear columns and repeated quadratic pairs are summed by Gurobi.
    ///
    /// # Errors
    /// - [`Error::AlgebraicError`] for inconsistent lengths or column indices
    ///   outside the indexed columns.
    /// - [`Error::FromAPI`] if a Gurobi API error occurs.
    #[allow(clippy::too_many_arguments)]
    pub fn add_qconstr_raw(
        &mut self,
        name: &CStr,
        lind: &[i32],
        lval: &[f64],
        qrow: &[i32],
        qcol: &[i32],
        qval: &[f64],
        sense: ConstrSense,
        rhs: f64,
    ) -> Result<QConstr> {
        if lind.len() != lval.len() || qrow.len() != qval.len() || qcol.len() != qval.len() {
            return Err(invalid(
                "index and value slices must have the same length".into(),
            ));
        }
        for &index in lind.iter().chain(qrow).chain(qcol) {
            self.check_raw_column(index)?;
        }

        let lazy = self.update_mode_lazy()?;
        self.check_add_apicall(unsafe {
            ffi::GRBaddqconstr(
                self.ptr,
                Self::sparse_count(lind.len())?,
                as_ptr_or_null(lind),
                as_ptr_or_null(lval),
                Self::sparse_count(qval.len())?,
                as_ptr_or_null(qrow),
                as_ptr_or_null(qcol),
                as_ptr_or_null(qval),
                sense as u8 as ffi::c_char,
                rhs,
                name.as_ptr(),
            )
        })?;

        Ok(self.qconstrs.add_new(lazy))
    }

    /// Add a nonlinear general constraint `x[resvar] = f(tree)`
    /// whose `Variable` nodes already hold native column indices in `data`.
    ///
    /// The tree uses the encoding of [`Model::add_genconstr_nl`].
    ///
    /// # Errors
    /// - [`Error::AlgebraicError`] for inconsistent lengths, an empty or
    ///   malformed tree, unknown opcodes, or column indices outside the indexed
    ///   columns.
    /// - [`Error::FromAPI`] if a Gurobi API error occurs.
    #[cfg(feature = "gurobi13")]
    pub fn add_genconstr_nl_raw(
        &mut self,
        name: &CStr,
        resvar: i32,
        opcode: &[i32],
        data: &[f64],
        parent: &[i32],
    ) -> Result<GenConstr> {
        self.check_raw_column(resvar)?;
        check_nl_tree(opcode, data, parent, self.raw_column_count())?;
        let lazy = self.update_mode_lazy()?;
        self.check_add_apicall(unsafe {
            ffi::GRBaddgenconstrNL(
                self.ptr,
                name.as_ptr(),
                resvar,
                Self::sparse_count(opcode.len())?,
                opcode.as_ptr(),
                data.as_ptr(),
                parent.as_ptr(),
            )
        })?;

        Ok(self.genconstrs.add_new(lazy))
    }

    /// Add `x[resvar] = min(x[args], constant)` by column index.
    ///
    /// # Errors
    /// - [`Error::AlgebraicError`] for column indices outside the indexed columns.
    /// - [`Error::FromAPI`] if a Gurobi API error occurs.
    pub fn add_genconstr_min_raw(
        &mut self,
        name: &CStr,
        resvar: i32,
        args: &[i32],
        constant: f64,
    ) -> Result<GenConstr> {
        self.add_extremum_raw(name, resvar, args, constant, ffi::GRBaddgenconstrMin)
    }

    /// Add `x[resvar] = max(x[args], constant)` by column index.
    ///
    /// # Errors
    /// - [`Error::AlgebraicError`] for column indices outside the indexed columns.
    /// - [`Error::FromAPI`] if a Gurobi API error occurs.
    pub fn add_genconstr_max_raw(
        &mut self,
        name: &CStr,
        resvar: i32,
        args: &[i32],
        constant: f64,
    ) -> Result<GenConstr> {
        self.add_extremum_raw(name, resvar, args, constant, ffi::GRBaddgenconstrMax)
    }

    fn add_extremum_raw(
        &mut self,
        name: &CStr,
        resvar: i32,
        args: &[i32],
        constant: f64,
        call: ExtremumFn,
    ) -> Result<GenConstr> {
        self.check_raw_column(resvar)?;
        for &arg in args {
            self.check_raw_column(arg)?;
        }

        let lazy = self.update_mode_lazy()?;
        self.check_add_apicall(unsafe {
            call(
                self.ptr,
                name.as_ptr(),
                resvar,
                Self::sparse_count(args.len())?,
                as_ptr_or_null(args),
                constant,
            )
        })?;

        Ok(self.genconstrs.add_new(lazy))
    }

    /// Add `x[resvar] = |x[arg]|` by column index.
    ///
    /// # Errors
    /// - [`Error::AlgebraicError`] for column indices outside the indexed columns.
    /// - [`Error::FromAPI`] if a Gurobi API error occurs.
    pub fn add_genconstr_abs_raw(
        &mut self,
        name: &CStr,
        resvar: i32,
        arg: i32,
    ) -> Result<GenConstr> {
        self.check_raw_column(resvar)?;
        self.check_raw_column(arg)?;
        let lazy = self.update_mode_lazy()?;
        self.check_add_apicall(unsafe {
            ffi::GRBaddgenconstrAbs(self.ptr, name.as_ptr(), resvar, arg)
        })?;

        Ok(self.genconstrs.add_new(lazy))
    }
}
