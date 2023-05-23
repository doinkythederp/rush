# Installing plugins

By default, Rush will not search for or load any plugins. This chapter will guide you through obtaining plugins and configuring Rush to load them when it starts up.

## Obtaining a plugin

Plugins are 3rd party software and it's up to the author to decide where to distribute them. Check with the plugin's author to find out where to obtain it from. Make sure your plugin file has a `.wasm` extension.

If you'd like to try out an example plugin, you can download the [Welcome Message](https://github.com/doinkythederp/rush/raw/plugins/config/plugins/welcome_message.wasm) ([source](https://github.com/doinkythederp/rush/tree/plugins/examples/plugins/welcome-message)) plugin, which displays a message when you start Rush to let you know that you installed it correctly.

Rush also provides a simple [PATH Autocomplete](https://github.com/doinkythederp/rush/raw/plugins/config/plugins/path_autocomplete.wasm) ([source](https://github.com/doinkythederp/rush/tree/plugins/examples/plugins/path-autocomplete)) plugin that suggests your available commands based on what you've typed so far.

## Configuring Rush to load plugins

Once your plugin is downloaded, place it in a directory near your Rush configuration. In this guide, we'll create one called `plugins` and insert the Welcome Message example plugin.

```txt
your-config-directory/
├── config.rush
└── plugins/
    └── welcome_message.wasm
```

Rush doesn't know to search the `plugins` directory for plugins yet, though. We can add the following line to `config.rush` to remedy this; Rush will now load every `.wasm` file in the plugins directory on startup. Rush will recursively search for plugins, so you can optionally make subdirectories to further organize.

```diff
  # Method 1: Plugin auto-discovery (recommended)
  show-errors: true
  ...etc...
+ plugin: ./plugins
```

Alternatively, you could specify a specific plugin file to disable the automatic discovery of new plugins. Only the plugins you specify will be loaded.

```diff
  # Method 2: Specific files
  show-errors: true
  ...etc...
+ plugin: ./plugins/welcome_message.wasm
```

## Adding more plugins to Rush

> **Note:**
> If you configured Rush to automatically discover plugins, this may be unnecessary.

Adding more `plugin` directives to your config file allows you to load plugins from more than one location. For example, we could load both the PATH Autocomplete plugin and the Welcome Message plugin using this technique.

```diff
  show-errors: true
  ...etc...
+ plugin: ./plugins/welcome_message.wasm
+ plugin: ./plugins/path_autocomplete.wasm
```
