# nu_plugin_template

This is a starter plugin. It just lays out one way to make nushell plugins with nushell version 0.72.0

This template is intended to be used with [cargo-generate](https://github.com/cargo-generate/cargo-generate) in order to quickly
bootstrap nushell plugin projects.

## Usage:

```
> cargo generate --git https://github.com/fdncred/nu_plugin_template
> cd {{ project-name }}
> cargo build

# You only need to run this once per nushell session, or after updating the
# signature of the plugin.
> register ./target/debug/{{ plugin_name }}

> 'pas' | {{ plugin_name }} faux
Hello, faux! with value: pas
```

## Config values

- `plugin_name` - all nushell plugins are binaries with the name format
`nu_plugin_SOMETHING`. This is how nushell discovers them. You need to tell this
generator what that `SOMETHING` is. If you enter `random` as the plugin name,
your binary will be called `nu_plugin_random`, and you will run it by entering
`random`.

- `plugin_struct` - name of the struct that implements the `Plugin` trait from
`nu-plugin` crate.


[nushell]: https://www.nushell.sh/
[structured types]: https://www.nushell.sh/book/types_of_data.html

This is a nushell plugin template.

# Installing

[add the plugin]: https://www.nushell.sh/book/plugins.html#adding-a-plugin
[`register`]: https://www.nushell.sh/book/commands/register.html

To [add the plugin] permanently, just install it and call [`register`] on it:

## Using Cargo

```bash
cargo install nu_plugin_template
register ~/.cargo/bin/nu_plugin_template
```
## Running
```bash
'pas' | template faux
Hello, faux! with value: pas
```