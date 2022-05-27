// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use eframe::egui::{Context, InnerResponse, Ui, WidgetText, Window};

pub struct SimplePrompt;

impl SimplePrompt {
    pub fn show<R>(
        ctx: &Context,
        title: impl Into<WidgetText>,
        contents: impl FnOnce(&mut Ui) -> R,
    ) -> Option<InnerResponse<Option<R>>> {
        Window::new(title)
            .collapsible(false)
            .resizable(false)
            .show(ctx, contents)
    }

    pub fn show_with_window<R>(
        ctx: &Context,
        title: impl Into<WidgetText>,
        window: impl Fn(&mut Window<'_>),
        contents: impl FnOnce(&mut Ui) -> R,
    ) -> Option<InnerResponse<Option<R>>> {
        Window::new(title).mutate(window).show(ctx, contents)
    }
}
