variable::set("plugin_struct", variable::get("plugin_name").to_pascal_case() + "Plugin");
variable::set("command_struct", variable::get("command_name").to_pascal_case());
variable::set("command_module", variable::get("command_name").to_snake_case());
