<img src="https://github.com/ryanmcgrath/subatomic/blob/trunk/assets/icon1024@2x.png?raw=true" width="160" />

# Subatomic
This is an experimental site browser/wrapper, built for macOS - which means menus, key commands and so on work as expected. The demo is built for Protonmail, since I was curious about the level of effort to turn it into a desktop client. It's built on [appkit-rs](https://github.com/ryanmcgrath/appkit-rs/), and if you wanted to, you could swap the icon and URL for whatever web property you want.

Feel free to browse the code to see how it's built in 100% Rust (via the Objective-C runtime, mind you). I likely won't update this beyond what you see here, as there's other things I want to spend my time on... but this might be of interest to some people, so I've open sourced it. If nothing else, it shows how to build, package, and sign a complete Rust macOS application - one which properly implements things like `NSMenu`.

## Why Protonmail?
It actually covers a pretty decent surface of components to test and get working. Originally I wanted to use Twitter for this, but notifications would be a pain to get working as they don't offer support for Safari with them.

Also, I'm too lazy to stop and design a new icon.

## Some features
- Implements better notifications for incoming email.
- Implements file downloading, which (apparently) is still not part of `WKWebView` in 2020? Full disclosure, this just links against a private setup and handles the `NSSavePanel` formalities.
- Showcases file uploading, with `NSOpenPanel`.
- Shoves in a dark mode theme.
- Has a snazzy Mail.app-esque icon, which for all intents and purposes is open source and you can do whatever you want with it.

## To build and run
- Ensure you have Xcode and such tooling installed.
- Install [cargo bundle](https://github.com/burtonageo/cargo-bundle)
- Ensure you have [appkit-rs](https://github.com/ryanmcgrath/appkit-rs/) cloned and set correctly in the `Cargo.toml`
- Edit the `debug` or `release` scripts to specify your Apple Developer Profile
- Run `PROFILE="" debug` or `PROFILE="" release`, and ignore the slew of warnings that come from this being ultra experimental. `PROFILE` is your Apple Developer certificate for building.

## License
Do what you want with it. Anything Protonmail is copyright them.

## Questions or concerns...
Hit me up on [Twitter](https://twitter.com/ryanmcgrath/) or [email me](mailto:ryan@rymc.io).
