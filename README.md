# nu_plugin_template

This template is intended to be used with [cargo-generate](https://github.com/cargo-generate/cargo-generate) in order to quickly bootstrap nushell plugin projects.

You must run `cargo generate` with `--force`.

## Usage:

```
> cargo generate --force --git https://github.com/nushell/nu_plugin_template
🤷   What will this plugin be named?: foo
Creating a new plugin named "foo"
Your plugin crate will be named "nu_plugin_foo".

Note that the MIT license is used by default, to reflect the majority of                  
Nushell projects. You can change this manually if you'd like to.
                                                        
!!! IMPORTANT !!!                            
You must run cargo generate with --force, or it will rename your project to
something that is non-standard for Nushell plugins and this will fail.
                                                        
If you see a message after this about renaming your project, please abort and
try again with --force.     
                                                        
🔧   Destination: /var/home/devyn/Projects/nushell/nu_plugin_foo ...
🔧   project-name: nu_plugin_foo ...
🔧   Generating template ...          
🤷   What should your first command be called? (spaces are okay): foo
✔ 🤷   Do you intend to create more than one command / subcommand? · No 
✔ 🤷   Would you like a simple command? Say no if you would like to use streaming. · Yes
🤷   What is your GitHub username? (Leave blank if you don't want to publish to GitHub) [default: ]: 
🔧   Moving generated files into: `/var/home/devyn/Projects/nushell/nu_plugin_foo`...
🔧   Initializing a fresh Git repository      
✨   Done! New project created /var/home/devyn/Projects/nushell/nu_plugin_foo
> cd nu_plugin_foo
> cargo build
> plugin add target/debug/nu_plugin_foo
> plugin use foo
> foo Ellie
Hello, Ellie. How are you today?
```

## Config values

- `plugin_name` - all nushell plugins are binaries with the name format
`nu_plugin_SOMETHING`. This is how nushell discovers them. You need to tell this
generator what that `SOMETHING` is. If you enter `random` as the plugin name,
your binary will be called `nu_plugin_random`.

- `command_name` - the name of your first/only command. This can be any valid nushell command name,
and can contain spaces. For example, if you're creating a format plugin for `FORMAT` files, you
might choose to go with `from FORMAT` or `to FORMAT`.

- `multi_commmand` - set to `Yes` if you expect that you'll be creating more than one command, in
which case we'll create a `commands` module for you and put the command in there. Set to `No` if you
would rather just have everything in `src/main.rs`.

- `command_is_simple` - set to `Yes` if you want a `SimplePluginCommand` with no streaming support,
or `No` if you want `PluginCommand` with a streaming example.

- `github_username` - we'll use this to set the repository field in `Cargo.toml` if you set it.

