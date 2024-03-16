{% if command_is_simple == "Yes" -%}
use nu_plugin::{EngineInterface, EvaluatedCall, LabeledError, SimplePluginCommand};
use nu_protocol::{Category, PluginExample, PluginSignature, SyntaxShape, Value};
{%- else -%}
use nu_plugin::{EngineInterface, EvaluatedCall, LabeledError, PluginCommand};
use nu_protocol::{
    Category, PipelineData, PluginExample, PluginSignature, Type, Value,
};
{%- endif %}

use crate::{{ plugin_struct }};

pub struct {{ command_struct }};

{% if command_is_simple == "Yes" %}
impl SimplePluginCommand for {{ command_struct }} {
    type Plugin = {{ plugin_struct }};

    fn signature(&self) -> PluginSignature {
        PluginSignature::build("{{ command_name }}")
            .required("name", SyntaxShape::String, "(FIXME) A demo parameter - your name")
            .switch("shout", "(FIXME) Yell it instead", None)
            .plugin_examples(vec![
                PluginExample {
                    example: "{{ command_name }} Ferris".into(),
                    description: "Say hello to Ferris".into(),
                    result: Some(Value::test_string("Hello, Ferris. How are you today?")),
                },
                PluginExample {
                    example: "{{ command_name }} --shout Ferris".into(),
                    description: "Shout hello to Ferris".into(),
                    result: Some(Value::test_string("HELLO, FERRIS. HOW ARE YOU TODAY?")),
                },
            ])
            .category(Category::Experimental)
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
{% else %}
impl PluginCommand for {{ command_struct }} {
    type Plugin = {{ plugin_struct }};

    fn signature(&self) -> PluginSignature {
        PluginSignature::build("{{ command_name }}")
            .switch("shout", "(FIXME) Yell it instead", None)
            .input_output_type(Type::List(Type::String.into()), Type::List(Type::String.into()))
            .plugin_examples(vec![
                PluginExample {
                    example: "[ Ferris ] | {{ command_name }}".into(),
                    description: "Say hello to Ferris".into(),
                    result: Some(Value::test_list(vec![
                        Value::test_string("Hello, Ferris. How are you today?")
                    ])),
                },
                PluginExample {
                    example: "[ Ferris ] | {{ command_name }} --shout".into(),
                    description: "Shout hello to Ferris".into(),
                    result: Some(Value::test_list(vec![
                        Value::test_string("HELLO, FERRIS. HOW ARE YOU TODAY?")
                    ])),
                },
            ])
            .category(Category::Experimental)
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
        }, None)?)
    }
}
{% endif %}
