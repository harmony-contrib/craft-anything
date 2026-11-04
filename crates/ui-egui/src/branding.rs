//! Identity of the application hosting the UI; engine and file-format names stay stable.

/// Set [`crate::CraftApp::branding`] before showing the interface.
#[derive(Clone, Copy, Debug)]
pub struct Branding {
    pub name: &'static str,
    pub icon_png: &'static [u8],
    pub attribution: &'static str,
}

impl Branding {
    pub(crate) fn update_title(self, ctx: &egui::Context) {
        let id = egui::Id::unique(("host-title", ctx.viewport_id()));
        if ctx.data(|data| data.get_temp::<&'static str>(id)) != Some(self.name) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Title(self.name.into()));
            ctx.data_mut(|data| data.insert_temp(id, self.name));
        }
    }
}

impl crate::CraftApp {
    /// The host's name, or the original application name when no branding was supplied.
    pub fn display_name(&self) -> &'static str {
        self.branding.map_or("Craft Anything", |branding| branding.name)
    }

    /// The host's PNG, loaded by `setup_context`, or the default application artwork.
    pub fn brand_icon(&self, size: f32) -> egui::Image<'static> {
        match self.branding {
            Some(branding) => {
                // A host can change the icon without reusing a previous image-loader cache entry.
                let uri = format!("bytes://host-icon-{:p}-{}.png", branding.icon_png.as_ptr(), branding.icon_png.len());
                egui::Image::from_bytes(uri, branding.icon_png).fit_to_exact_size(egui::Vec2::splat(size))
            }
            None => crate::app_icon(size),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui_kittest::kittest::Queryable as _;

    #[test]
    fn host_identity_reaches_the_header_about_window_and_image_loader() {
        let dir = tempfile::tempdir().unwrap();
        let engine = std::sync::Arc::new(craft_anything_engine::Engine::new(craft_anything_engine::Session::isolated(dir.path())));
        let mut app = crate::demo::app(engine);
        assert_eq!(app.display_name(), "Craft Anything");
        let image = image::RgbaImage::from_pixel(2, 2, image::Rgba([32, 64, 128, 255]));
        let mut png = std::io::Cursor::new(Vec::new());
        image.write_to(&mut png, image::ImageFormat::Png).unwrap();
        let png = Box::leak(png.into_inner().into_boxed_slice());
        app.branding =
            Some(Branding { name: "Embedded App", icon_png: png, attribution: "Based on Craft Anything; original contributors retain their credits." });
        app.settings.first_run_done = true;

        let ctx = egui::Context::default();
        crate::CraftApp::setup_context(&ctx);
        let output = ctx.run(Default::default(), |ctx| app.branding.unwrap().update_title(ctx));
        assert!(
            output
                .viewport_output
                .values()
                .any(|viewport| viewport.commands.iter().any(|command| { matches!(command, egui::ViewportCommand::Title(title) if title == "Embedded App") }))
        );
        assert!(matches!(app.brand_icon(24.0).load_for_size(&ctx, egui::Vec2::splat(24.0)), Ok(egui::load::TexturePoll::Ready { .. })));

        let mut harness = egui_kittest::Harness::builder().with_size(egui::vec2(1280.0, 820.0)).build_eframe(move |cc| {
            crate::CraftApp::setup_context(&cc.egui_ctx);
            app
        });
        harness.run_steps(3);
        harness.get_by_label("Embedded App");
        harness.state_mut().state.modal = Some(crate::state::Modal::About);
        harness.run_steps(3);
        harness.get_by_label("Based on Craft Anything; original contributors retain their credits.");
    }
}
