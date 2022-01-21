# Bevy Toolbar

## Usage:
This is starter for your own game-specific tools, clone source and manually add it to `[workspace]` section in `Cargo.toml`.

Now add it to `src/main.rs` and files in `bevy_toolbar` folder to your needs.

```diff
use bevy::prelude::*;
+ use bevy_toolbar::*;

fn main() {
   App::new()
       .add_plugins(DefaultPlugins)
+      .add_plugins(ToolbarPlugins)
       .run();
}
```

When you open game, two panels should be rendered on top and the bottom. Top panel contains buttons which opens menus with tools, bottom panel show useful window informations like current title, vsync status, fps, window state and more.

## License
bevy_toolbar is relased as Public Domain library see [LICENSE](LICENSE).
