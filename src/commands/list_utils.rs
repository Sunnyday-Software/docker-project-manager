use crate::{CommandRegistry, Value};
use crate::context::Context;

/// Register list utility commands
pub fn register_list_commands(registry: &mut CommandRegistry) {
  registry.register_closure_with_help(
        "list", 
        "Create a list from arguments",
        "(list element1 element2 ...)",
        "  (list 1 2 3)       ; Creates [1, 2, 3]\n  (list \"a\" \"b\")     ; Creates [\"a\", \"b\"]",
        |args, _ctx| {
            Ok(Value::List(args))
        }
    );

  registry.register_closure_with_help(
        "list-first", 
        "Get first element of a list",
        "(first list)",
        "  (first (list 1 2 3))  ; Returns 1\n  (first (list \"a\" \"b\")) ; Returns \"a\"",
        |args, _ctx| {
            if args.len() != 1 {
                return Err("first expects exactly one argument".to_string());
            }

            match &args[0] {
                Value::List(list) => {
                    if list.is_empty() {
                        Ok(Value::Nil)
                    } else {
                        Ok(list[0].clone())
                    }
                }
                _ => Err("first expects a list argument".to_string()),
            }
        }
    );

  registry.register_closure_with_help(
        "list-rest", 
        "Get all but first element of a list",
        "(rest list)",
        "  (rest (list 1 2 3))   ; Returns [2, 3]\n  (rest (list \"a\" \"b\")) ; Returns [\"b\"]",
        |args, _ctx| {
            if args.len() != 1 {
                return Err("rest expects exactly one argument".to_string());
            }

            match &args[0] {
                Value::List(list) => {
                    if list.len() <= 1 {
                        Ok(Value::List(Vec::new()))
                    } else {
                        Ok(Value::List(list[1..].to_vec()))
                    }
                }
                _ => Err("rest expects a list argument".to_string()),
            }
        }
    );
}
