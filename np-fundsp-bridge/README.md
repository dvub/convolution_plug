# nih-plug FunDSP Bridge

This is a small project which aims to bridge nih-plug and FunDSP. This project currently has 2 main features:

1. A processor struct, which wraps a FunDSP graph and provides convenient block processing
2. A `ParamNode` which allows the use of nih-plug parameters directly in a graph.

### How to use this

1. Add a new `PluginDspProcesor` field to your plugin struct.
2. Use `PluginDspProcessor`'s `default()` in the plugin's `Default` impl.
3. When the plugin is initialized, make sure to update the `PluginDspProcessor`'s `graph` field (either directly or through the setter)
4. In your plugin's `process()`, call `PluginDspProcessor::process()`, passing in nih-plug's `Buffer`.

### Contribution

Contributions are very welcome! New features, fixes, documentation, are all appreciated.

### Todos

- possibly make `graph` more generic, allowing a user to pass in and use a `Net` instead of a static graph.
- Allow passing in other nih-plug arguments to `process()`, such as aux inputs, etc.
- testing
