use nu_plugin::{MsgPackSerializer, Plugin, PluginCommand, serve_plugin};

mod commands;
pub use commands::*;

pub struct {{ plugin_struct }};

impl {{ plugin_struct }} {
    fn new() -> Self {
        Self {}
    }
}

impl Plugin for {{ plugin_struct }} {
    fn commands(&self) -> Vec<Box<dyn PluginCommand<Plugin = Self>>> {
        vec![
            // Commands should be added here
            Box::new({{ command_struct }}),
        ]
    }
}

fn main() {
    serve_plugin(&{{ plugin_struct }}::new(), MsgPackSerializer);
}
