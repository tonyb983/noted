// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Plans for the `Toaster`:
//! `Toast` will represent a single "toast message" or notification. `ToastKind` will determine how it is styled. `Toaster` will
//! expose very few public methods, for adding a new toast, clearing all toasts, and ...? Ideal workflow would be:
//! ```ignore
//!
//! ```
//!

pub enum ToastKind {
    Info,
    Warning,
    Error,
    Custom,
}

pub struct Toast {
    pub id: egui::Id,
    pub kind: ToastKind,
    pub text: String,
    pub duration: f32,
}

pub struct Toaster {
    anchor: egui::Align2,
}

impl Toaster {
    const TOAST_ID_BASE: &'static str = "noted_toast";
}

fn tester() {}
