use crate::compiling::compile::prelude::*;

/// Compile the body of a closure function.
impl Compile<(ast::ExprClosure, &[CompileMetaCapture])> for Compiler<'_> {
    fn compile(
        &mut self,
        (expr_closure, captures): (ast::ExprClosure, &[CompileMetaCapture]),
    ) -> CompileResult<()> {
        let span = expr_closure.span();
        log::trace!("ExprClosure => {:?}", self.source.source(span));

        let count = {
            for (arg, _) in expr_closure.args.as_slice() {
                let span = arg.span();

                match arg {
                    ast::FnArg::SelfValue(s) => {
                        return Err(CompileError::new(s, CompileErrorKind::UnsupportedSelf))
                    }
                    ast::FnArg::Ident(ident) => {
                        let ident = ident.resolve(&self.storage, &*self.source)?;
                        self.scopes.new_var(ident.as_ref(), span)?;
                    }
                    ast::FnArg::Ignore(..) => {
                        // Ignore incoming variable.
                        let _ = self.scopes.decl_anon(span)?;
                    }
                }
            }

            if !captures.is_empty() {
                self.asm.push(Inst::PushTuple, span);

                for capture in captures {
                    self.scopes.new_var(&capture.ident, span)?;
                }
            }

            self.scopes.total_var_count(span)?
        };

        self.compile((&*expr_closure.body, Needs::Value))?;

        if count != 0 {
            self.asm.push(Inst::Clean { count }, span);
        }

        self.asm.push(Inst::Return, span);

        self.scopes.pop_last(span)?;
        Ok(())
    }
}

/// Compile a closure expression.
impl Compile<(&ast::ExprClosure, Needs)> for Compiler<'_> {
    fn compile(&mut self, (expr_closure, needs): (&ast::ExprClosure, Needs)) -> CompileResult<()> {
        let span = expr_closure.span();
        log::trace!("ExprClosure => {:?}", self.source.source(span));

        if !needs.value() {
            self.warnings.not_used(self.source_id, span, self.context());
            return Ok(());
        }

        let item = self.query.item_for(expr_closure)?.clone();
        let hash = Hash::type_hash(&item.item);

        let meta = self
            .query
            .query_meta_with(span, None, &item, Default::default())?
            .ok_or_else(|| {
                CompileError::new(
                    span,
                    CompileErrorKind::MissingType {
                        item: item.item.clone(),
                    },
                )
            })?;

        let captures = match &meta.kind {
            CompileMetaKind::Closure { captures, .. } => captures,
            _ => {
                return Err(CompileError::expected_meta(span, meta, "a closure"));
            }
        };

        log::trace!("captures: {} => {:?}", item.item, captures);

        if captures.is_empty() {
            // NB: if closure doesn't capture the environment it acts like a regular
            // function. No need to store and load the environment.
            self.asm.push_with_comment(
                Inst::LoadFn { hash },
                span,
                format!("closure `{}`", item.item),
            );
        } else {
            // Construct a closure environment.
            for capture in &**captures {
                let var =
                    self.scopes
                        .get_var(&capture.ident, self.source_id, self.visitor, span)?;
                var.copy(&mut self.asm, span, format!("capture `{}`", capture.ident));
            }

            self.asm.push_with_comment(
                Inst::Closure {
                    hash,
                    count: captures.len(),
                },
                span,
                format!("closure `{}`", item.item),
            );
        }

        Ok(())
    }
}
