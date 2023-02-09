mod {{ plugin_name }};
use nu_plugin::{serve_plugin, EvaluatedCall, LabeledError, MsgPackSerializer, Plugin};
use nu_protocol::{Category, PluginExample, PluginSignature, Spanned, SyntaxShape, Value};

struct {{ plugin_struct }};

impl {{ plugin_struct }} {
    fn new() -> Self {
        Self {}
    }
}

impl Plugin for {{ plugin_struct }} {
    fn signature(&self) -> Vec<PluginSignature> {
        vec![PluginSignature::build("{{ plugin_name }}")
            .usage("View {{ plugin_name }} results")
            .required("path", SyntaxShape::String, "path to {{ plugin_name }} input file")
            .category(Category::Experimental)
            .plugin_examples(vec![PluginExample {
                description: "This is the example descripion".into(),
                example: "some pipeline involving {{ plugin_name }}".into(),
                result: None,
            }])]
    }

    fn run(
        &mut self,
        name: &str,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        assert_eq!(name, "{{ plugin_name }}");
        let param: Option<Spanned<String>> = call.opt(0)?;

        let ret_val = match input {
            Value::String { val, span } => {
                crate::{{ plugin_name }}::{{ plugin_name }}_do_something(param, val, *span)?
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
    serve_plugin(&mut {{ plugin_struct }}::new(), MsgPackSerializer);
}
