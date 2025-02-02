use re_viewer_context::ViewerContext;
use re_viewport::{SpaceInfoCollection, ViewportBlueprint};

/// Show the Blueprint section of the left panel based on the current [`ViewportBlueprint`]
pub fn blueprint_panel_ui(
    blueprint: &mut ViewportBlueprint<'_>,
    ctx: &mut ViewerContext<'_>,
    ui: &mut egui::Ui,
    spaces_info: &SpaceInfoCollection,
) {
    ctx.re_ui.panel_content(ui, |_, ui| {
        ctx.re_ui.panel_title_bar_with_buttons(
            ui,
            "Blueprint",
            Some("The Blueprint is where you can configure the Rerun Viewer"),
            |ui| {
                blueprint.add_new_spaceview_button_ui(ctx, ui, spaces_info);
                reset_button_ui(blueprint, ctx, ui, spaces_info);
            },
        );

        blueprint.tree_ui(ctx, ui);
    });
}

fn reset_button_ui(
    blueprint: &mut ViewportBlueprint<'_>,
    ctx: &mut ViewerContext<'_>,
    ui: &mut egui::Ui,
    spaces_info: &SpaceInfoCollection,
) {
    if ctx
        .re_ui
        .small_icon_button(ui, &re_ui::icons::RESET)
        .on_hover_text("Re-populate Viewport with automatically chosen Space Views")
        .clicked()
    {
        blueprint.reset(ctx, spaces_info);
    }
}
