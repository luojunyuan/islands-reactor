#![windows_subsystem = "windows"]

use islands_reactor::*;
use islands_reactor_examples::run;

fn main() -> Result<()> {
    run("Tooltip placement", app)
}

fn app(_: &mut RenderCx) -> Element {
    vstack((
        button("Top")
            .tooltip_with(Tooltip::text("Anchored above").placement(TooltipPlacement::Top)),
        button("Bottom")
            .tooltip_with(Tooltip::text("Anchored below").placement(TooltipPlacement::Bottom)),
        button("Left")
            .tooltip_with(Tooltip::text("Anchored left").placement(TooltipPlacement::Left)),
        button("Right")
            .tooltip_with(Tooltip::text("Anchored right").placement(TooltipPlacement::Right)),
        button("Mouse")
            .tooltip_with(Tooltip::text("Follows the pointer").placement(TooltipPlacement::Mouse)),
    ))
    .spacing(8.0)
    .padding(16.0)
    .into()
}
