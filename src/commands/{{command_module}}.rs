{% if command_is_simple == "Yes" -%}
use nu_plugin::{EngineInterface, EvaluatedCall, SimplePluginCommand};
use nu_protocol::{Category, Example, LabeledError, Signature, SyntaxShape, Value};
{%- else -%}
use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
use nu_protocol::{Category, Example, LabeledError, PipelineData, Signals, Signature, Type, Value};
{%- endif %}

use crate::{{ plugin_struct }};

pub struct {{ command_struct }};

{% if command_is_simple == "Yes" -%}
impl SimplePluginCommand for {{ command_struct }} {
    type Plugin = {{ plugin_struct }};

    fn name(&self) -> &str {
        "{{ command_name }}"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .required("name", SyntaxShape::String, "(FIXME) A demo parameter - your name")
            .switch("shout", "(FIXME) Yell it instead", None)
            .category(Category::Experimental)
    }

    fn description(&self) -> &str {
        "(FIXME) help text for {{ command_name }}"
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                example: "{{ command_name }} Ellie",
                description: "Say hello to Ellie",
                result: Some(Value::test_string("Hello, Ellie. How are you today?")),
            },
            Example {
                example: "{{ command_name }} --shout Ellie",
                description: "Shout hello to Ellie",
                result: Some(Value::test_string("HELLO, ELLIE. HOW ARE YOU TODAY?")),
            },
        ]
    }

    fn run(
        &self,
        _plugin: &{{ plugin_struct }},
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: &Value,
    ) -> Result<Value, LabeledError> {
        let name: String = call.req(0)?;
        let mut greeting = format!("Hello, {name}. How are you today?");
        if call.has_flag("shout")? {
            greeting = greeting.to_uppercase();
        }
        Ok(Value::string(greeting, call.head))
    }
}
{%- else -%}
impl PluginCommand for {{ command_struct }} {
    type Plugin = {{ plugin_struct }};

    fn name(&self) -> &str {
        "{{ command_name }}"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .switch("shout", "(FIXME) Yell it instead", None)
            .input_output_type(Type::List(Type::String.into()), Type::List(Type::String.into()))
            .category(Category::Experimental)
    }

    fn description(&self) -> &str {
        "(FIXME) help text for {{ command_name }}"
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                example: "[ Ellie ] | {{ command_name }}",
                description: "Say hello to Ellie",
                result: Some(Value::test_list(vec![
                    Value::test_string("Hello, Ellie. How are you today?")
                ])),
            },
            Example {
                example: "[ Ellie ] | {{ command_name }} --shout",
                description: "Shout hello to Ellie",
                result: Some(Value::test_list(vec![
                    Value::test_string("HELLO, ELLIE. HOW ARE YOU TODAY?")
                ])),
            },
        ]
    }

    fn run(
        &self,
        _plugin: &{{ plugin_struct }},
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        input: PipelineData,
    ) -> Result<PipelineData, LabeledError> {
        let span = call.head;
        let shout = call.has_flag("shout")?;
        Ok(input.map(move |name| {
            match name.as_str() {
                Ok(name) => {
                    let mut greeting = format!("Hello, {name}. How are you today?");
                    if shout {
                        greeting = greeting.to_uppercase();
                    }
                    Value::string(greeting, span)
                }
                Err(err) => Value::error(err, span),
            }
        }, &Signals::empty())?)
    }
}
{%- endif %}

#[test]
fn test_examples() -> Result<(), nu_protocol::ShellError> {
    use nu_plugin_test_support::PluginTest;

    // This will automatically run the examples specified in your command and compare their actual
    // output against what was specified in the example.
    //
    // We recommend you add this test to any other commands you create, or remove it if the examples
    // can't be tested this way.

    PluginTest::new("{{ plugin_name }}", {{ plugin_struct }}.into())?
        .test_command_examples(&{{ command_struct }})
}
