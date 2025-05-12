use nu_plugin::{MsgPackSerializer, Plugin, PluginCommand, serve_plugin};
{% if multi_command == "No" -%}
{%- if command_is_simple == "Yes" -%}
use nu_plugin::{EngineInterface, EvaluatedCall, SimplePluginCommand};
use nu_protocol::{Category, Example, LabeledError, Signature, SyntaxShape, Value};
{%- else -%}
use nu_plugin::{EngineInterface, EvaluatedCall};
use nu_protocol::{Category, Example, LabeledError, PipelineData, Signals, Signature, Type, Value};
{%- endif %}
{%- else %}
mod commands;
pub use commands::*;
{%- endif %}

pub struct {{ plugin_struct }};

impl Plugin for {{ plugin_struct }} {
    fn version(&self) -> String {
        // This automatically uses the version of your package from Cargo.toml as the plugin version
        // sent to Nushell
        env!("CARGO_PKG_VERSION").into()
    }

    fn commands(&self) -> Vec<Box<dyn PluginCommand<Plugin = Self>>> {
        vec![
            // Commands should be added here
            Box::new({{ command_struct }}),
        ]
    }
}

{% if multi_command == "No" -%}
pub struct {{ command_struct }};

{% if command_is_simple == "Yes" -%}
impl SimplePluginCommand for {{ command_struct }} {
    type Plugin = {{ plugin_struct }};

    fn name(&self) -> &str {
        "{{ command_name }}"
    }

    fn signature(&self) -> Signature {
        Signature::build(PluginCommand::name(self))
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
    // output against what was specified in the example. You can remove this test if the examples
    // can't be tested this way, but we recommend including it if possible.

    PluginTest::new("{{ plugin_name }}", {{ plugin_struct }}.into())?
        .test_command_examples(&{{ command_struct }})
}

{% endif -%}
fn main() {
    serve_plugin(&{{ plugin_struct }}, MsgPackSerializer);
}
