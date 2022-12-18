mod template;
use nu_plugin::{serve_plugin, EvaluatedCall, LabeledError, MsgPackSerializer, Plugin};
use nu_protocol::{Category, Signature, Spanned, SyntaxShape, Value};

struct Template;

impl Template {
    fn new() -> Self {
        Self {}
    }
}

impl Plugin for Template {
    fn signature(&self) -> Vec<Signature> {
        vec![Signature::build("template")
            .usage("View template")
            .required("path", SyntaxShape::String, "path to template file")
            .category(Category::Experimental)]
    }

    fn run(
        &mut self,
        name: &str,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        assert_eq!(name, "template");
        let param: Option<Spanned<String>> = call.opt(0)?;

        let ret_val = match input {
            Value::String { val, span } => {
                crate::template::template_do_something(param, val, *span)?
            }
            v => {
                return Err(LabeledError {
                    label: "Expected something from pipeline".into(),
                    msg: format!("requires some input, got {}", v.get_type()),
                    span: Some(call.head),
                });
            }
        };

        Ok(ret_val)
    }
}

fn main() {
    serve_plugin(&mut Template::new(), MsgPackSerializer);
}
