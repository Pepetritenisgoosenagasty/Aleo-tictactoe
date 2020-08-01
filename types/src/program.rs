//! A typed Leo program consists of import, circuit, and function definitions.
//! Each defined type consists of typed statements and expressions.

use crate::{Circuit, Function, Identifier, Import, InputVariable, TestFunction};
use leo_ast::files::File;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A simple program with statement expressions, program arguments and program returns.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Program {
    pub name: String,
    pub expected_input: Vec<InputVariable>,
    pub imports: Vec<Import>,
    pub circuits: HashMap<Identifier, Circuit>,
    pub functions: HashMap<Identifier, Function>,
    pub tests: HashMap<Identifier, TestFunction>,
}

const MAIN_FUNCTION_NAME: &str = "main";

impl<'ast> Program {
    //! Logic to convert from an abstract syntax tree (ast) representation to a Leo program.
    pub fn from(file: File<'ast>, name: String) -> Self {
        // Compiled ast -> aleo program representation
        let imports = file
            .imports
            .into_iter()
            .map(|import| Import::from(import))
            .collect::<Vec<Import>>();

        let mut circuits = HashMap::new();
        let mut functions = HashMap::new();
        let mut tests = HashMap::new();
        let mut expected_input = vec![];

        file.circuits.into_iter().for_each(|circuit| {
            circuits.insert(Identifier::from(circuit.identifier.clone()), Circuit::from(circuit));
        });
        file.functions.into_iter().for_each(|function_def| {
            let function = Function::from(function_def);
            if function.function_name.name.eq(MAIN_FUNCTION_NAME) {
                expected_input = function.input.clone();
            }
            functions.insert(function.function_name.clone(), function);
        });
        file.tests.into_iter().for_each(|test_def| {
            let test = TestFunction::from(test_def);
            tests.insert(test.0.function_name.clone(), test);
        });

        Self {
            name,
            expected_input,
            imports,
            circuits,
            functions,
            tests,
        }
    }
}

impl Program {
    pub fn new(name: String) -> Self {
        Self {
            name,
            expected_input: vec![],
            imports: vec![],
            circuits: HashMap::new(),
            functions: HashMap::new(),
            tests: HashMap::new(),
        }
    }

    pub fn get_name(&self) -> String {
        self.name.to_string()
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }
}
