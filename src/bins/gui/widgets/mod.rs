// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod list;
mod panels;
mod screen_prompt;
mod screens;
mod toaster;

use eframe::{
    egui::{style::WidgetVisuals, Response, Ui},
    emath::{self, remap, Rect, Vec2},
    epaint::Shape,
};

pub fn combobox_icon_fn(ui: &Ui, rect: Rect, visuals: &WidgetVisuals, _is_open: bool) {
    let rect = Rect::from_center_size(
        rect.center(),
        Vec2::new(rect.width() * 0.6, rect.height() * 0.4),
    );

    ui.painter().add(Shape::convex_polygon(
        vec![rect.left_top(), rect.right_top(), rect.center_bottom()],
        visuals.fg_stroke.color,
        visuals.fg_stroke,
    ));
}

#[allow(dead_code)]
pub fn collapsing_state_icon_fn(ui: &mut Ui, openness: f32, response: &Response) {
    use std::f32::consts::TAU;

    let visuals = ui.style().interact(response);
    let stroke = visuals.fg_stroke;

    let rect = response.rect;

    // Draw a pointy triangle arrow:
    let rect = Rect::from_center_size(
        rect.center(),
        Vec2::new(rect.width() * 0.6, rect.height() * 0.4),
    );
    let rect = rect.expand(visuals.expansion);
    let mut points = vec![rect.left_top(), rect.right_top(), rect.center_bottom()];
    let rotation = emath::Rot2::from_angle(remap(openness, 0.0..=1.0, -TAU / 4.0..=0.0));
    for p in &mut points {
        *p = rect.center() + rotation * (*p - rect.center());
    }

    ui.painter()
        .add(Shape::convex_polygon(points, stroke.color, stroke));
}
