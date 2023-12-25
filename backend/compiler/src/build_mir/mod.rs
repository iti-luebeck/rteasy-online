mod declaration;
mod expression;
mod step;

use crate::mir::*;
use crate::{symbols::Symbols, InternalError};

type Result<T> = std::result::Result<T, InternalError>;

pub fn build_mir<'s>(ast: ast::Ast<'s>, symbols: &Symbols<'s>) -> Result<Mir<'s>> {
    let mut statements = build_statements(ast.statements, symbols)?;
    if let Some(trailing_label) = ast.trailing_label {
        statements.push(Statement {
            label: Some(trailing_label.node),
            steps: Spanned {
                node: vec![Step {
                    id: StepId(0),
                    criteria: Vec::new(),
                    operation: Operation::Nop(Nop),
                    annotation: Annotation::new(false, false),
                    span: Span::dummy(),
                }],
                span: Span::dummy(),
            },
            span: trailing_label.span,
            span_semicolon: Span::dummy(),
            span_pipe: None,
        });
    }

    Ok(Mir {
        declarations: ast
            .declarations
            .into_iter()
            .filter(
                |declaration| {
                    if let ast::Declaration::Alias(_) = declaration {
                        false
                    } else {
                        true
                    }
                },
            )
            .map(|declaration| declaration::build(declaration, symbols))
            .collect::<Result<_>>()?,
        statements,
    })
}

fn build_statements<'s>(
    statements: Vec<ast::Statement<'s>>,
    symbols: &Symbols<'s>,
) -> Result<Vec<Statement<'s>>> {
    statements
        .into_iter()
        .map(|statement| {
            Ok(Statement {
                label: statement.label.map(|label| label.node),
                steps: Spanned {
                    node: step::build(
                        statement.operations.operations,
                        statement.operations.operations_post,
                        symbols,
                    )?,
                    span: statement.operations.span,
                },
                span: statement.span,
                span_semicolon: statement.span_semicolon,
                span_pipe: statement.operations.span_pipe,
            })
        })
        .collect()
}
