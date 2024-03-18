let plugin_name = variable::get("project-name");

if plugin_name.is_empty() {
    plugin_name = variable::prompt("What will this plugin be named?").to_snake_case();
}

if plugin_name.starts_with("nu_plugin_") {
    plugin_name.replace("nu_plugin_", "");
}

if plugin_name.is_empty() {
    abort("Plugin name must not be empty.");
}

variable::set("plugin_name", plugin_name);
variable::set("project-name", "nu_plugin_" + plugin_name);

print(`Creating a new plugin named "${variable::get("plugin_name")}"`);
print(`Your plugin crate will be named "${variable::get("project-name")}".`);
print("");
print("Note that the MIT license is used by default, to reflect the majority of");
print("Nushell projects. You can change this manually if you'd like to.");
print("");
print("!!! IMPORTANT !!!");
print("You must run cargo generate with --force, or it will rename your project to");
print("something that is non-standard for Nushell plugins and this will fail.");
print("");
print("If you see a message after this about renaming your project, please abort and");
print("try again with --force.");
print("");
