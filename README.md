# Bevy Toolbar

Toolbar plugin for [Bevy Engine](https://github.com/bevyengine/bevy), created to provide set of tools and utils which can be expanded for your own needs.

Crate itself is designed to be installed manually, fork this repo and fetch source, add `bevy_toolbar` to your workspace in `Cargo.toml`, then start adding new features related to your game.

### Installation
This branch target main branch of bevy, before installation use `git checkout 0.6`.

Remember to add `ToolbarPlugins` after `DefaultPlugins` because toolbar will not show up and most likely crash your game on startup.

```rs
use bevy::prelude::*;
use bevy_toolbar::ToolbarPlugins;

/*
This will create empty window with two panels,
one on top and second on the bottom.

Click "Settings" button to enable selected tools.
*/
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ToolbarPlugins)
        .run();
}
```

## Contributing
Feel free to open Pull Request(s) with new tools or integrations with other plugins.

## License
bevy_toolbar is released as Public Domain library see [LICENSE](LICENSE).
