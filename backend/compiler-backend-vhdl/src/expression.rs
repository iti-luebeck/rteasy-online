use super::concat::generate_concat_expr;
use crate::gen_ident;
use compiler::mir;
use vhdl::*;

pub fn generate_expression<'s>(
    expression: &mir::Expression<'s>,
    declarations: &Declarations,
    ctx_size: usize,
) -> Expression {
    let (kind, extend_to) = match expression {
        mir::Expression::Atom(atom) => {
            (ExpressionKind::Atom(generate_atom(atom, declarations)), Extend::Zero(ctx_size))
        }
        mir::Expression::BinaryTerm(binary_term) => {
            let ctx_size_inner = binary_term.ctx_size.calc(ctx_size);
            let kind = ExpressionKind::BinaryTerm(Box::new(BinaryTerm {
                lhs: generate_expression(&binary_term.lhs, declarations, ctx_size_inner),
                rhs: generate_expression(&binary_term.rhs, declarations, ctx_size_inner),
                operator: binary_term.operator,
            }));
            (kind, Extend::Zero(ctx_size))
        }
        mir::Expression::UnaryTerm(unary_term) => {
            let ctx_size_inner = unary_term.ctx_size.calc(ctx_size);
            let kind = ExpressionKind::UnaryTerm(Box::new(UnaryTerm {
                expression: generate_expression(
                    &unary_term.expression,
                    declarations,
                    ctx_size_inner,
                ),
                operator: unary_term.operator,
            }));
            let extend_to = match unary_term.operator {
                UnaryOperator::Sxt => Extend::Sign(ctx_size),
                _ => Extend::Zero(ctx_size),
            };
            (kind, extend_to)
        }
    };

    Expression { kind, extend_to }
}

pub fn generate_atom<'s>(atom: &mir::Atom<'s>, declarations: &Declarations) -> Atom {
    match atom {
        mir::Atom::Concat(concat) => Atom::Concat(generate_concat_expr(concat, declarations)),
        mir::Atom::Register(reg) => Atom::Register(generate_register(reg, declarations)),
        mir::Atom::Bus(bus) => Atom::Bus(generate_bus(bus, declarations)),
        mir::Atom::RegisterArray(reg_array) => {
            Atom::RegisterArray(generate_register_array(reg_array, declarations))
        }
        mir::Atom::Number(number) => Atom::Number(generate_number(&number)),
    }
}

pub fn generate_register<'s>(reg: &mir::Register<'s>, declarations: &Declarations) -> Register {
    let ident = gen_ident(reg.ident);

    let range_declaration =
        declarations.registers.iter().find(|(name, _, _)| ident == *name).unwrap().1;

    Register {
        ident,
        range: generate_bit_range(reg.range.map(|s| s), range_declaration),
        kind: reg.kind,
    }
}

pub fn generate_bus<'s>(bus: &mir::Bus<'s>, declarations: &Declarations) -> Bus {
    let ident = gen_ident(bus.ident);

    let range_declaration =
        declarations.buses.iter().find(|(name, _, _)| ident == *name).unwrap().1;

    Bus {
        ident,
        range: generate_bit_range(bus.range.map(|s| s), range_declaration),
        kind: bus.kind,
    }
}

pub fn generate_register_array<'s>(
    reg_array: &mir::RegisterArray<'s>,
    declarations: &Declarations,
) -> RegisterArray {
    let ident = gen_ident(reg_array.ident);

    let range_declaration =
        declarations.register_arrays.iter().find(|(name, _, _)| ident == *name).unwrap().1;

    RegisterArray {
        ident,
        index: Box::new(generate_expression(
            &reg_array.index,
            declarations,
            reg_array.index_ctx_size,
        )),
        range: generate_bit_range(reg_array.range.map(|s| s), range_declaration),
    }
}

pub fn generate_number<'s>(number: &mir::Number) -> Number {
    Number { value: number.value.clone(), kind: DebugInfo(number.kind) }
}

fn generate_bit_range(
    range: Option<mir::BitRange>,
    range_declaration: BitRange,
) -> Option<BitRange> {
    // Full range (= None) if range is None
    let range = range?;

    // Map range
    let range = match range_declaration {
        BitRange::Downto(_, _) => BitRange::Downto(range.0, range.1),
        BitRange::To(_, _) => BitRange::To(range.0, range.1),
    };

    // Full range (= None) if range is equals range_declaration
    if range == range_declaration {
        return None;
    }

    Some(range)
}
