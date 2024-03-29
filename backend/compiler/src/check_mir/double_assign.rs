use super::sim::{sim, Result, SimState};
use crate::mir::*;
use crate::symbols::{Symbol, Symbols};
use crate::{CompilerError, CompilerErrorKind, InternalError, SymbolType};
use std::collections::{HashMap, HashSet};

pub fn check(
    symbols: &Symbols<'_>,
    mir: &Mir<'_>,
    error_sink: &mut impl FnMut(CompilerError),
) -> Result {
    for statement in &mir.statements {
        let state = State::new(symbols);
        sim(statement, state, error_sink)?;
    }

    Ok(())
}

#[derive(Debug, Clone)]
struct State<'s> {
    symbols: &'s Symbols<'s>,
    assigned: HashMap<AssignTarget<'s>, Vec<AssignInfo>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct AssignTarget<'s> {
    name: Ident<'s>,
    type_: SymbolType,
}

#[derive(Debug, Clone)]
struct AssignInfo {
    range: Option<BitRange>,
}

impl<'s> State<'s> {
    fn new(symbols: &'s Symbols<'s>) -> Self {
        Self { symbols, assigned: HashMap::new() }
    }

    fn insert(&mut self, name: Ident<'s>, type_: SymbolType, range: Option<BitRange>) {
        self.assigned.entry(AssignTarget { name, type_ }).or_default().push(AssignInfo { range });
    }
}

impl<'s> SimState<'s> for State<'s> {
    fn condition(&mut self, _: &Expression<'s>) -> Result {
        Ok(())
    }
    fn nop(&mut self, _: &Nop) -> Result {
        Ok(())
    }
    fn goto(&mut self, _: &Goto<'s>) -> Result {
        Ok(())
    }
    fn write(&mut self, write: &Write<'s>) -> Result {
        self.insert(write.ident, SymbolType::Memory, None);
        Ok(())
    }

    fn read(&mut self, read: &Read<'s>) -> Result {
        match self.symbols.symbol(read.ident) {
            Some(Symbol::Memory { data_register, .. }) => {
                self.insert(data_register, SymbolType::Register, None);
                Ok(())
            }
            _ => Err(InternalError(format!("missing memory: {}", read.ident.0))),
        }
    }

    fn assignment(&mut self, assignment: &Assignment<'s>) -> Result {
        match &assignment.lhs {
            Lvalue::Register(reg) => {
                self.insert(reg.ident, SymbolType::Register, reg.range);
            }
            Lvalue::Bus(bus) => {
                self.insert(bus.ident, SymbolType::Bus, bus.range);
            }
            Lvalue::RegisterArray(reg_array) => {
                self.insert(reg_array.ident, SymbolType::RegisterArray, None);
            }
            Lvalue::ConcatClocked(concat) => {
                for part in &concat.parts {
                    match part {
                        ConcatPartLvalueClocked::Register(reg, _) => {
                            self.insert(reg.ident, SymbolType::Register, reg.range);
                        }
                        ConcatPartLvalueClocked::RegisterArray(reg_array, _) => {
                            self.insert(reg_array.ident, SymbolType::RegisterArray, None);
                        }
                    }
                }
            }
            Lvalue::ConcatUnclocked(concat) => {
                for part in &concat.parts {
                    match part {
                        ConcatPartLvalueUnclocked::Bus(bus, _) => {
                            self.insert(bus.ident, SymbolType::Bus, bus.range);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn assert(&mut self, _: &Assert<'_>) -> Result {
        Ok(())
    }

    fn finish(self, statement: &Statement<'s>, error_sink: &mut impl FnMut(CompilerError)) {
        for (target, infos) in self.assigned {
            if has_conflict(infos) {
                error_sink(CompilerError::new(
                    CompilerErrorKind::DoubleAssign(target.type_, target.name.0.to_string()),
                    statement.steps.span,
                ));
            }
        }
    }
}

fn has_conflict(infos: Vec<AssignInfo>) -> bool {
    let mut bits_assigned = HashSet::new();
    let mut all_bits_assigned = false;

    for info in infos {
        if all_bits_assigned {
            return true;
        }

        match info.range {
            Some(range) => {
                for bit in range.bits() {
                    let is_new = bits_assigned.insert(bit);
                    if !is_new {
                        return true;
                    }
                }
            }
            None => {
                if !bits_assigned.is_empty() {
                    return true;
                }

                all_bits_assigned = true;
            }
        }
    }

    false
}
