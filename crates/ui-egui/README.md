# Embedding Craft Anything

`CraftApp` implements `eframe::App` and also exposes `show(&mut egui::Ui)` for a
host that owns the frame. Call `CraftApp::setup_context(&cc.egui_ctx)` once to
install fonts and image loaders, then construct the app with the host's shared
`Arc<Engine>` and `Arc<AiHub>`.

An embedding host can set its identity before the first frame:

```rust,ignore
let mut app = CraftApp::new(engine, hub);
app.branding = Some(Branding {
    name: "Modune",
    icon_png: include_bytes!("assets/icon.png"),
    attribution: "Modune uses Craft Anything. MIT OR Apache-2.0; upstream contributors retain their copyrights.",
});
```

The identity is used in the title bar, first-run screen, window title and About
dialog. Set the initial native viewport title and icon in the host's
`eframe::NativeOptions` too. Disable this crate's default features to omit the
upstream app artwork; leave `branding` unset to retain the upstream display name.
Project formats, tool identifiers and configuration names remain stable.

For `aarch64-unknown-linux-ohos`, the root consumer must use the same
`[patch.crates-io]` entries as this workspace: egui 0.37 at
`42760ab8470178ea3a955a194dad0a3ecadfb26f` from `richerfu/egui`, and winit at
`fe8cbb2e689b519b9282bc29b84c8ea6bc9df313` from `richerfu/winit`. Cargo does not
inherit patches from git dependencies. The host owns the OpenHarmony ability,
window, application data directories and native services.

Desktop dialogs, clipboard image reads, accessibility integration and the Linux
desktop keyring are excluded on OHOS. The host can open granted paths through
`CraftApp::open_path`, supply pictures through `attach` or `add_pictures`, and save
`report::markdown` through the engine's `core_report_save` tool. Desktop-only UI
actions report when the corresponding service is unavailable. Text
clipboard and URL handling remain with eframe/winit. API keys use the existing
owner-only file fallback when no system keychain is available.

Font setup reads `HarmonyOS_Sans_SC.ttf`, `HarmonyOS_Sans_TC.ttf` and
`HarmonyOS_Sans.ttf` from `/system/fonts`, with a 40 MiB limit per face.
Compilation checks do not verify permissions, rendering, fonts or external
program availability on a device.
