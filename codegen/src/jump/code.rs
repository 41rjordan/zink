//! Table for the code section.

use crate::func::Func;
use indexmap::IndexMap;

/// Code section for EVM.
#[derive(Default, Debug)]
pub struct Code {
    offset: usize,
    /// Function table.
    funcs: IndexMap<Func, usize>,
}

impl Code {
    /// Create a new code section.
    pub fn new() -> Self {
        Self {
            offset: 0,
            funcs: Default::default(),
        }
    }

    /// Get the functions in the code section.
    pub fn funcs(&self) -> Vec<Func> {
        self.funcs.keys().cloned().collect()
    }

    /// Shift the code section.
    pub fn shift(&mut self, offset: u16) {
        tracing::debug!("shift code section by 0x{:x} bytes.", offset);
        let offset = offset as usize;
        self.offset += offset;
        self.funcs.values_mut().for_each(|pc| *pc += offset);
    }

    /// Add a function to the code section.
    pub fn try_add_func(&mut self, func: Func) {
        if self.funcs.contains_key(&func) {
            return;
        }

        let bytecode = func.bytecode();
        let len = bytecode.len();
        self.funcs.insert(func, self.offset);
        self.offset += len;
    }

    /// Get the current offset of the code section.
    pub fn offset(&self) -> usize {
        self.offset
    }

    /// Get the offset of a function.
    pub fn offset_of(&self, func: &Func) -> Option<u16> {
        self.funcs.get(func).and_then(|i| (*i).try_into().ok())
    }

    /// Get the bytecode of the code section.
    pub fn finish(&self) -> Vec<u8> {
        let mut code = Vec::new();
        for func in self.funcs.keys() {
            tracing::debug!("add function to code section: {:?}", func);
            code.extend(func.bytecode());
        }
        code
    }
}
