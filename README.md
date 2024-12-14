# Bevy 2D Shooter

This is a 2d top-down shooter written in [Rust](https://www.rust-lang.org/) using the [Bevy](https://bevyengine.org/) game engine.

Some of the code was learned from [another project](https://github.com/bones-ai/bevy-2d-shooter).

## What's the difference?
- Better code organization
- Handle physical movement in Fixed Update
- Complete game lifecycle - main menu, game over screen
- Use bevy_button_released_plugin for better button interaction
- Smoother movement and zooming of the camera
- Pop texts when damaging
- Health bar with gradual changing background
- Game config file serialized and deserialized with serde and toml

## Credits

- Original project - [https://github.com/bones-ai/bevy-2d-shooter](https://github.com/bones-ai/bevy-2d-shooter)
- Game assets - [https://0x72.itch.io/dungeontileset-ii](https://0x72.itch.io/dungeontileset-ii)
- Monogram Font - [https://datagoblin.itch.io/monogram](https://datagoblin.itch.io/monogram)

## Controls

- `WASD`or`↑←↓→` for movement
- Left mouse button to shoot
- Mouse wheel to change camera zoom