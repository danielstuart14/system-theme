# System Theme [![Latest Version]][crates.io] [![docs]][docs.rs]

[Latest Version]: https://img.shields.io/crates/v/system-theme.svg
[crates.io]: https://crates.io/crates/system-theme
[docs]: https://docs.rs/system-theme/badge.svg
[docs.rs]: https://docs.rs/system-theme

A cross-platform Rust library to help build native-looking applications.

It provides access (when supported by the platform) to:

- System theme kind (Windows, macOS, GTK, or Qt)
- System theme scheme (light or dark)
- System contrast level (normal or high)
- System accent color

It also includes predefined theme palettes designed to match native applications.

## Roadmap

🚧 **Platforms**\
&nbsp;&nbsp;&nbsp;&nbsp;🟢 Windows\
&nbsp;&nbsp;&nbsp;&nbsp;🟢 macOS\
&nbsp;&nbsp;&nbsp;&nbsp;🚧 XDG (Linux, FreeBSD, etc.)\
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;🟢 GTK (e.g. GNOME)\
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;🟢 Qt (e.g. KDE)\
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;🚧 Cosmic (detected as GTK theme kind)\
&nbsp;&nbsp;&nbsp;&nbsp;🔴 Web\
&nbsp;&nbsp;&nbsp;&nbsp;🔴 Android\
&nbsp;&nbsp;&nbsp;&nbsp;🔴 iOS

🚧 **Theming**\
&nbsp;&nbsp;&nbsp;&nbsp;🟢 Default color palette (Fluent, Aqua, Adwaita, Breeze)\
&nbsp;&nbsp;&nbsp;&nbsp;🔴 Custom color palette (user-defined system themes)\
&nbsp;&nbsp;&nbsp;&nbsp;🔴 Subscription to theme changes (async)

🚧 **Integrations**\
&nbsp;&nbsp;&nbsp;&nbsp;🚧 Iced (feature: `iced`)\
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;🟢 Palette\
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;🔴 Widget Themes

## License

System theme is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
