# nu_plugin_template

This is a starter plugin. It just lays out one way to make nushell plugins with nushell version 0.72.0

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