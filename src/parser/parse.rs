// Copyright 2022 Greptime Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::parser::{lex, Expr};

pub fn parse(input: &str) -> Result<Expr, String> {
    match lex::lexer(input) {
        Err(e) => Err(e),
        Ok(lexer) => {
            let (res, errs) = crate::promql_y::parse(&lexer);
            for err in errs {
                println!("{:?}", err)
            }
            match res {
                Some(r) => r,
                None => Err("empty AST".into()),
            }
        }
    }
}
