use std::hash::Hash;

use eframe::{
    egui::{Area, Context, Frame, Id, InnerResponse, Layout, Order, Sense, Ui},
    emath::{Align2, Pos2, Vec2},
    epaint::{Color32, Rounding, Shape},
};
use serde::{Deserialize, Serialize};

use super::super::get_app_theme;

#[derive(Clone)]
pub struct ScreenPrompt {
    id: Id,
    prompt_frame: Frame,
    bg_overlay_color: Color32,
    outside_click_closes: bool,
    min_size: Vec2,
}

#[derive(Clone, Default, Debug, Copy, Deserialize, Serialize)]
pub struct State {
    is_shown: bool,
}

impl State {
    pub fn shown(&mut self, shown: bool) {
        self.is_shown = shown;
    }
}

impl ScreenPrompt {
    const PROMPT_BASE_ID: &'static str = "ui_prompt";

    pub fn new(name: impl Hash, min_size: Vec2) -> Self {
        Self {
            id: Id::new(Self::PROMPT_BASE_ID).with(name),
            prompt_frame: get_app_theme().prompt_frame,
            bg_overlay_color: Color32::from_black_alpha(200),
            outside_click_closes: false,
            min_size,
        }
    }

    pub fn outside_click_closes(self, outside_click_closes: bool) -> Self {
        Self {
            outside_click_closes,
            ..self
        }
    }

    pub fn set_shown(ctx: &Context, name: impl Hash, shown: bool) {
        ctx.memory()
            .data
            .get_persisted_mut_or_default::<State>(Id::new(Self::PROMPT_BASE_ID).with(name))
            .is_shown = shown;
    }
}

impl ScreenPrompt {
    pub fn show<R>(
        &mut self,
        ctx: &Context,
        add_contents: impl FnOnce(&mut Ui, &mut State) -> R,
    ) -> Option<InnerResponse<R>> {
        let state = ctx.memory().data.get_persisted::<State>(self.id);
        let mut state = state.unwrap_or_default();

        let res = if state.is_shown {
            let area_res = Area::new("prompt_bg")
                .fixed_pos(Pos2::ZERO)
                .show(ctx, |ui| {
                    let screen_rect = ctx.input().screen_rect;

                    ui.allocate_response(screen_rect.size(), Sense::click());

                    ui.painter().add(Shape::rect_filled(
                        screen_rect,
                        Rounding::none(),
                        self.bg_overlay_color,
                    ));

                    let prompt_area_res = Area::new("prompt_centered")
                        .anchor(Align2::CENTER_CENTER, Vec2::splat(0.0))
                        .order(Order::Foreground)
                        .show(ctx, |ui| {
                            let InnerResponse { inner, .. } = self.prompt_frame.show(ui, |ui| {
                                // let inner_rect = screen_rect.shrink(50.0);
                                ui.set_min_size(self.min_size);
                                ui.allocate_ui_with_layout(
                                    self.min_size,
                                    Layout::top_down(eframe::emath::Align::Center),
                                    |ui| add_contents(ui, &mut state),
                                )
                                .inner
                                // ui.allocate_ui_at_rect(inner_rect, |ui| {
                                //     ui.centered_and_justified(|ui| add_contents(ui, &mut state))
                                //         .inner
                                // })
                                // .inner
                            });

                            inner
                        });

                    if prompt_area_res.response.clicked_elsewhere() && self.outside_click_closes {
                        state.is_shown = false;
                    };

                    // ui.painter().debug_rect(
                    //     screen_rect,
                    //     Color32::YELLOW.linear_multiply(0.5),
                    //     "screen_rect",
                    // );

                    // ui.painter().debug_rect(
                    //     screen_rect.shrink(50.0),
                    //     Color32::GREEN.linear_multiply(0.5),
                    //     "screen_rect.shrink(50.0)",
                    // );

                    prompt_area_res.inner
                });
            Some(area_res)
        } else {
            None
        };

        ctx.memory().data.insert_persisted(self.id, state);

        res
    }
}
