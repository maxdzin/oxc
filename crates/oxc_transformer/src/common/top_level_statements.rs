//! Utility transform to add statements to top of program.
//!
//! `TopLevelStatementsStore` contains a `Vec<Statement>`. It is stored on `TransformCtx`.
//!
//! `TopLevelStatements` transform inserts those statements at top of program.
//!
//! Other transforms can add statements to the store with `TopLevelStatementsStore::insert_statement`:
//!
//! ```rs
//! self.ctx.top_level_statements.insert_statement(stmt);
//! ```

use std::cell::RefCell;

use oxc_ast::ast::*;
use oxc_traverse::{Traverse, TraverseCtx};

use crate::TransformCtx;

/// Transform that inserts any statements which have been requested insertion via `TopLevelStatementsStore`
/// to top of the program.
///
/// Insertions are made after any existing `import` statements.
///
/// Must run after all other transforms.
pub struct TopLevelStatements<'a, 'ctx> {
    ctx: &'ctx TransformCtx<'a>,
}

impl<'a, 'ctx> TopLevelStatements<'a, 'ctx> {
    pub fn new(ctx: &'ctx TransformCtx<'a>) -> Self {
        Self { ctx }
    }
}

impl<'a, 'ctx> Traverse<'a> for TopLevelStatements<'a, 'ctx> {
    fn exit_program(&mut self, program: &mut Program<'a>, _ctx: &mut TraverseCtx<'a>) {
        let mut stmts = self.ctx.top_level_statements.stmts.borrow_mut();
        if stmts.is_empty() {
            return;
        }

        // Insert statements after any existing `import` statements
        let index = program
            .body
            .iter()
            .rposition(|stmt| matches!(stmt, Statement::ImportDeclaration(_)))
            .map_or(0, |i| i + 1);

        program.body.splice(index..index, stmts.drain(..));
    }
}

/// Store for statements to be added at top of program
pub struct TopLevelStatementsStore<'a> {
    stmts: RefCell<Vec<Statement<'a>>>,
}

impl<'a> TopLevelStatementsStore<'a> {
    pub fn new() -> Self {
        Self { stmts: RefCell::new(vec![]) }
    }
}

impl<'a> TopLevelStatementsStore<'a> {
    /// Add a statement to be inserted at top of program.
    pub fn insert_statement(&self, stmt: Statement<'a>) {
        self.stmts.borrow_mut().push(stmt);
    }
}