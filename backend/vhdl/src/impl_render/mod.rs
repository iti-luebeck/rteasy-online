mod memory_data;
mod sensitivity_list;

use self::memory_data::MemoryData;
use crate::{error::RenderError, render_as_rt::RenderAsRt, render_as_vhdl::RenderAsVhdl};
use crate::{
    BitRange, BusKind, Declarations, Expression, Ident, Lvalue, NextStateLogic, Operation,
    RegisterKind, Statement, Vhdl,
};
use indexmap::IndexSet;
use memory_file::MemoryFile;
use std::collections::HashMap;
use std::fmt::Write;
use temply::Template;

pub fn render(
    vhdl: &Vhdl,
    module_name: &str,
    memories: HashMap<Ident, MemoryFile>,
) -> Result<String, RenderError> {
    // Trim module name
    let module_name = module_name.trim();

    // Memories
    let memories = &self::memory_data::memories(memories, &vhdl.declarations)?;

    // Render
    let mut buffer = String::new();
    VhdlTemplate {
        module_name,
        statements: &vhdl.statements,
        criteria: &vhdl.signals.criteria,
        operations: &vhdl.signals.operations,
        declarations: &vhdl.declarations,
        memories,
    }
    .render(&mut buffer)
    .unwrap();
    Ok(buffer)
}

#[derive(Debug, Template)]
#[dedent]
#[template = "./impl_render/template.vhdl"]
struct VhdlTemplate<'a> {
    module_name: &'a str,
    statements: &'a [Statement],
    criteria: &'a IndexSet<Expression>,  // Index = CriterionId
    operations: &'a IndexSet<Operation>, // Index = OperationId

    declarations: &'a Declarations,
    memories: &'a HashMap<Ident, MemoryData>,
}

impl<'a> VhdlTemplate<'a> {
    fn any_port(&self) -> bool {
        self.declarations.buses.iter().any(|(_, _, kind)| *kind == BusKind::Input)
            || self.declarations.registers.iter().any(|(_, _, kind)| *kind == RegisterKind::Output)
    }

    fn ports_input(&self) -> impl Iterator<Item = (&'a Ident, BitRange, bool)> + '_ {
        let any_output =
            self.declarations.registers.iter().any(|(_, _, kind)| *kind == RegisterKind::Output);
        let inputs = self
            .declarations
            .buses
            .iter()
            .filter(|(_, _, kind)| *kind == BusKind::Input)
            .collect::<Vec<_>>();
        let len = inputs.len();

        inputs.into_iter().enumerate().map(move |(idx, (name, range, _))| {
            let is_last = !any_output && idx == len - 1;
            (name, *range, is_last)
        })
    }

    fn ports_output(&self) -> impl Iterator<Item = (&'a Ident, BitRange, bool)> + '_ {
        let outputs = self
            .declarations
            .registers
            .iter()
            .filter(|(_, _, kind)| *kind == RegisterKind::Output)
            .collect::<Vec<_>>();
        let len = outputs.len();

        outputs.into_iter().enumerate().map(move |(idx, (name, range, _))| {
            let is_last = idx == len - 1;
            (name, *range, is_last)
        })
    }

    fn operations(&self, clocked: bool) -> impl Iterator<Item = (usize, &Operation)> + '_ {
        self.operations.iter().enumerate().filter(move |(_, op)| op.is_clocked() == clocked)
    }

    fn operations_tmp_var(&self, clocked: bool) -> impl Iterator<Item = (usize, BitRange)> + '_ {
        self.operations.iter().enumerate().filter_map(move |(idx, op)| {
            if op.is_clocked() == clocked {
                match op {
                    Operation::Write(_) | Operation::Read(_) => None,
                    Operation::Assignment(assignment) => match assignment.lhs {
                        Lvalue::Register(_) | Lvalue::Bus(_) | Lvalue::RegisterArray(_) => None,
                        Lvalue::ConcatClocked(_) | Lvalue::ConcatUnclocked(_) => {
                            Some((idx, BitRange::Downto(assignment.rhs.extend_to.size() - 1, 0)))
                        }
                    },
                }
            } else {
                None
            }
        })
    }

    fn sensitivity_list_bus_mux(&self) -> String {
        let expressions = self.operations.iter().filter_map(|op| match op {
            Operation::Write(_) => None,
            Operation::Read(_) => None,
            Operation::Assignment(assignment) => {
                if op.is_clocked() {
                    None
                } else {
                    Some(&assignment.rhs)
                }
            }
        });
        let items = sensitivity_list::build(expressions);

        let mut buffer = "(c".to_string();
        for item in items {
            write!(&mut buffer, ", {}", RenderAsVhdl(&item)).unwrap();
        }
        buffer += ")";

        buffer
    }

    fn sensitivity_list_criteria_gen(&self) -> String {
        let expressions = self.criteria.iter();
        let mut items = sensitivity_list::build(expressions).into_iter();

        match items.next() {
            Some(item) => {
                let mut buffer = "(".to_string();
                write!(&mut buffer, "{}", RenderAsVhdl(&item)).unwrap();
                for item in items {
                    write!(&mut buffer, ", {}", RenderAsVhdl(&item)).unwrap();
                }
                buffer += ")";
                buffer
            }
            None => "(c) -- c is used here because vhdl simulators would otherwise \
                get stuck at an empty sensitivity list"
                .to_string(),
        }
    }
}
