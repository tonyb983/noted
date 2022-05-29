use eframe::{
    egui::{
        style::Margin, Context, Frame, Grid, Id, Key, Modifiers, PointerButton, RichText, Slider,
        SliderOrientation, Style, Ui, Widget, Window,
    },
    epaint::FontId,
    App,
};
use egui_hotkey::{BindVariant, Binding, Hotkey, HotkeyBinding};
use std::{
    hash::Hash,
    ops::{Deref, DerefMut},
    sync::Arc,
};

use super::{app::GuiApp, widgets::ToApp};

mod detail {
    use super::*;

    #[derive(Clone, Copy, Debug)]
    pub enum BetterBindVariant {
        Mouse(PointerButton),
        Key(Key),
    }

    impl From<PointerButton> for BetterBindVariant {
        fn from(b: PointerButton) -> Self {
            BetterBindVariant::Mouse(b)
        }
    }

    impl From<Key> for BetterBindVariant {
        fn from(b: Key) -> Self {
            BetterBindVariant::Key(b)
        }
    }

    impl PartialEq<BetterBindVariant> for BetterBindVariant {
        fn eq(&self, other: &BetterBindVariant) -> bool {
            match (self, other) {
                (BetterBindVariant::Mouse(a), BetterBindVariant::Mouse(b)) => a == b,
                (BetterBindVariant::Key(a), BetterBindVariant::Key(b)) => a == b,
                _ => false,
            }
        }
    }

    impl Eq for BetterBindVariant {}

    impl Hash for BetterBindVariant {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            match self {
                BetterBindVariant::Mouse(a) => match a {
                    PointerButton::Primary => 0u8.hash(state),
                    PointerButton::Secondary => 1u8.hash(state),
                    PointerButton::Middle => 2u8.hash(state),
                },
                BetterBindVariant::Key(a) => a.hash(state),
            }
        }
    }

    impl From<BindVariant> for BetterBindVariant {
        fn from(bv: BindVariant) -> Self {
            match bv {
                BindVariant::Mouse(mb) => BetterBindVariant::Mouse(mb),
                BindVariant::Keyboard(k) => BetterBindVariant::Key(k),
            }
        }
    }

    impl From<BetterBindVariant> for BindVariant {
        fn from(bv: BetterBindVariant) -> Self {
            match bv {
                BetterBindVariant::Mouse(mb) => BindVariant::Mouse(mb),
                BetterBindVariant::Key(k) => BindVariant::Keyboard(k),
            }
        }
    }

    #[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
    #[allow(clippy::struct_excessive_bools)]
    pub struct BetterModifiers {
        pub alt: bool,
        pub ctrl: bool,
        pub shift: bool,
        pub mac_cmd: bool,
        pub command: bool,
    }

    impl From<Modifiers> for BetterModifiers {
        fn from(m: Modifiers) -> Self {
            BetterModifiers {
                alt: m.alt,
                ctrl: m.ctrl,
                shift: m.shift,
                mac_cmd: m.mac_cmd,
                command: m.command,
            }
        }
    }

    impl From<BetterModifiers> for Modifiers {
        fn from(m: BetterModifiers) -> Self {
            Modifiers {
                alt: m.alt,
                ctrl: m.ctrl,
                shift: m.shift,
                mac_cmd: m.mac_cmd,
                command: m.command,
            }
        }
    }
}

use detail::{BetterBindVariant, BetterModifiers};

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct BetterBinding {
    variant: BetterBindVariant,
    modifiers: BetterModifiers,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MaybeBinding(Option<BetterBinding>);

impl Deref for MaybeBinding {
    type Target = Option<BetterBinding>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MaybeBinding {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl BetterBinding {
    pub fn new(bv: impl Into<BetterBindVariant>, m: impl Into<BetterModifiers>) -> Self {
        BetterBinding {
            variant: bv.into(),
            modifiers: m.into(),
        }
    }

    pub fn to_binding(self) -> Binding {
        Binding {
            variant: self.variant.into(),
            modifiers: self.modifiers.into(),
        }
    }

    pub fn with_ctrl(input: impl Into<BetterBindVariant>) -> Self {
        BetterBinding::new(
            input,
            BetterModifiers {
                alt: false,
                ctrl: true,
                shift: false,
                mac_cmd: false,
                command: false,
            },
        )
    }

    pub fn with_alt(input: impl Into<BetterBindVariant>) -> Self {
        BetterBinding::new(
            input,
            BetterModifiers {
                alt: true,
                ctrl: false,
                shift: false,
                mac_cmd: false,
                command: false,
            },
        )
    }

    pub fn with_ctrl_alt(input: impl Into<BetterBindVariant>) -> Self {
        BetterBinding::new(
            input,
            BetterModifiers {
                alt: true,
                ctrl: true,
                shift: false,
                mac_cmd: false,
                command: false,
            },
        )
    }
}

impl From<BetterBinding> for MaybeBinding {
    fn from(b: BetterBinding) -> Self {
        MaybeBinding(Some(b))
    }
}

impl From<Option<Binding>> for MaybeBinding {
    fn from(b: Option<Binding>) -> Self {
        match b {
            None => MaybeBinding(None),
            Some(b) => MaybeBinding(Some(b.into())),
        }
    }
}

impl HotkeyBinding for MaybeBinding {
    const ACCEPT_MOUSE: bool = true;

    const ACCEPT_KEYBOARD: bool = true;

    fn new(variant: BindVariant, modifiers: Modifiers) -> Self {
        Self(Some(BetterBinding::new(variant, modifiers)))
    }

    fn get(&self) -> Option<Binding> {
        self.map(BetterBinding::to_binding)
    }

    fn set(&mut self, variant: BindVariant, modifiers: Modifiers) {
        *self = Self(Some(BetterBinding::new(variant, modifiers)));
    }

    fn clear(&mut self) {
        *self = Self(None);
    }
}

impl From<Binding> for BetterBinding {
    fn from(binding: Binding) -> Self {
        Self {
            variant: binding.variant.into(),
            modifiers: binding.modifiers.into(),
        }
    }
}

impl From<BetterBinding> for Binding {
    fn from(binding: BetterBinding) -> Self {
        binding.to_binding()
    }
}

impl From<&BetterBinding> for Binding {
    fn from(binding: &BetterBinding) -> Self {
        binding.to_binding()
    }
}

pub struct Hotkeys {
    new_note: Option<Binding>,
    copy: Option<Binding>,
    cut: Option<Binding>,
    paste: Option<Binding>,
    delete: Option<Binding>,
    save: Option<Binding>,
    close_note_editor: Option<Binding>,
    toggle_settings: Option<Binding>,
    quit: Option<Binding>,
    undo: Option<Binding>,
    redo: Option<Binding>,
}

impl Default for Hotkeys {
    #[allow(clippy::too_many_lines)]
    fn default() -> Self {
        Self {
            new_note: Some(Binding {
                variant: BindVariant::Keyboard(Key::N),
                modifiers: Modifiers {
                    alt: false,
                    ctrl: true,
                    shift: false,
                    mac_cmd: false,
                    command: false,
                },
            }),
            copy: Some(Binding {
                variant: BindVariant::Keyboard(Key::C),
                modifiers: Modifiers {
                    alt: false,
                    ctrl: true,
                    shift: false,
                    mac_cmd: false,
                    command: false,
                },
            }),
            paste: Some(Binding {
                variant: BindVariant::Keyboard(Key::V),
                modifiers: Modifiers {
                    alt: false,
                    ctrl: true,
                    shift: false,
                    mac_cmd: false,
                    command: false,
                },
            }),
            cut: Some(Binding {
                variant: BindVariant::Keyboard(Key::X),
                modifiers: Modifiers {
                    alt: false,
                    ctrl: true,
                    shift: false,
                    mac_cmd: false,
                    command: false,
                },
            }),
            delete: Some(Binding {
                variant: BindVariant::Keyboard(Key::D),
                modifiers: Modifiers {
                    alt: false,
                    ctrl: true,
                    shift: false,
                    mac_cmd: false,
                    command: false,
                },
            }),
            save: Some(Binding {
                variant: BindVariant::Keyboard(Key::S),
                modifiers: Modifiers {
                    alt: false,
                    ctrl: true,
                    shift: false,
                    mac_cmd: false,
                    command: false,
                },
            }),
            close_note_editor: Some(Binding {
                variant: BindVariant::Keyboard(Key::W),
                modifiers: Modifiers {
                    alt: false,
                    ctrl: true,
                    shift: false,
                    mac_cmd: false,
                    command: false,
                },
            }),
            toggle_settings: Some(Binding {
                variant: BindVariant::Keyboard(Key::S),
                modifiers: Modifiers {
                    alt: true,
                    ctrl: true,
                    shift: false,
                    mac_cmd: false,
                    command: false,
                },
            }),
            quit: Some(Binding {
                variant: BindVariant::Keyboard(Key::Q),
                modifiers: Modifiers {
                    alt: false,
                    ctrl: true,
                    shift: false,
                    mac_cmd: false,
                    command: false,
                },
            }),
            undo: Some(Binding {
                variant: BindVariant::Keyboard(Key::Z),
                modifiers: Modifiers {
                    alt: false,
                    ctrl: true,
                    shift: false,
                    mac_cmd: false,
                    command: false,
                },
            }),
            redo: Some(Binding {
                variant: BindVariant::Keyboard(Key::Y),
                modifiers: Modifiers {
                    alt: false,
                    ctrl: true,
                    shift: false,
                    mac_cmd: false,
                    command: false,
                },
            }),
        }
    }
}

impl Hotkeys {
    pub fn validate_hotkeys(&mut self) {
        let mut is_valid = true;
        let mut bindings = std::collections::HashSet::<BetterBinding>::new();
        if let Some(cne) = self.close_note_editor {
            if !bindings.insert(cne.into()) {
                is_valid = false;
            }
        }
        if let Some(nne) = self.new_note {
            if !bindings.insert(nne.into()) {
                is_valid = false;
            }
        }
        if let Some(c) = self.copy {
            if !bindings.insert(c.into()) {
                is_valid = false;
            }
        }
        if let Some(p) = self.paste {
            if !bindings.insert(p.into()) {
                is_valid = false;
            }
        }
        if let Some(x) = self.cut {
            if !bindings.insert(x.into()) {
                is_valid = false;
            }
        }
        if let Some(d) = self.delete {
            if !bindings.insert(d.into()) {
                is_valid = false;
            }
        }
        if let Some(s) = self.save {
            if !bindings.insert(s.into()) {
                is_valid = false;
            }
        }
        if let Some(t) = self.toggle_settings {
            if !bindings.insert(t.into()) {
                is_valid = false;
            }
        }
        if let Some(q) = self.quit {
            if !bindings.insert(q.into()) {
                is_valid = false;
            }
        }
        if let Some(u) = self.undo {
            if !bindings.insert(u.into()) {
                is_valid = false;
            }
        }
        if let Some(r) = self.redo {
            if !bindings.insert(r.into()) {
                is_valid = false;
            }
        }

        if !is_valid {
            *self = Self::default();
        }
    }

    pub fn check_hotkeys(&self, ctx: &Context) -> HotkeyState {
        let new_note = self
            .new_note
            .map(|b| b.pressed(ctx.input()))
            .unwrap_or_default();
        let copy = self
            .copy
            .map(|b| b.pressed(ctx.input()))
            .unwrap_or_default();
        let cut = self.cut.map(|b| b.pressed(ctx.input())).unwrap_or_default();
        let paste = self
            .paste
            .map(|b| b.pressed(ctx.input()))
            .unwrap_or_default();
        let delete = self
            .delete
            .map(|b| b.pressed(ctx.input()))
            .unwrap_or_default();
        let save = self
            .save
            .map(|b| b.pressed(ctx.input()))
            .unwrap_or_default();
        let close_note_editor = self
            .close_note_editor
            .map(|b| b.pressed(ctx.input()))
            .unwrap_or_default();
        let toggle_settings = self
            .toggle_settings
            .map(|b| b.pressed(ctx.input()))
            .unwrap_or_default();
        let quit = self
            .quit
            .map(|b| b.pressed(ctx.input()))
            .unwrap_or_default();
        let undo = self
            .undo
            .map(|b| b.pressed(ctx.input()))
            .unwrap_or_default();
        let redo = self
            .redo
            .map(|b| b.pressed(ctx.input()))
            .unwrap_or_default();
        HotkeyState {
            new_note,
            copy,
            cut,
            paste,
            delete,
            save,
            close_note_editor,
            toggle_settings,
            quit,
            undo,
            redo,
        }
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(clippy::struct_excessive_bools)]
pub struct HotkeyState {
    pub new_note: bool,
    pub copy: bool,
    pub cut: bool,
    pub paste: bool,
    pub delete: bool,
    pub save: bool,
    pub close_note_editor: bool,
    pub toggle_settings: bool,
    pub quit: bool,
    pub undo: bool,
    pub redo: bool,
}

pub struct HotkeyEditor;

impl HotkeyEditor {
    pub fn render(ui: &mut Ui, hotkeys: &mut Hotkeys) {
        const LABEL_SIZE: f32 = 17.5;
        const SPACING: f32 = 9.0;
        let id = Id::new("hotkey_editor");

        ui.set_width(100.0);

        Frame {
            fill: super::get_app_theme().colors.darker_gray,
            inner_margin: Margin::same(6.0),
            rounding: super::get_app_theme().rounding.big,
            ..Frame::default()
        }
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Hotkey Editor (Click to Edit)");
            });
        });

        ui.indent(id.with("body"), |ui| {
            Grid::new(id.with("body_grid"))
                .num_columns(2)
                .spacing((25., 10.))
                .show(ui, |ui| {
                    ui.heading("Action");
                    ui.heading("Hotkey");
                    ui.end_row();

                    // ui.end_row();

                    ui.label(RichText::new("New Note").font(FontId::proportional(LABEL_SIZE)));
                    Hotkey::new(&mut hotkeys.new_note).ui(ui);
                    ui.end_row();

                    ui.label(RichText::new("Copy").font(FontId::proportional(LABEL_SIZE)));
                    Hotkey::new(&mut hotkeys.copy).ui(ui);
                    ui.end_row();

                    ui.label(RichText::new("Paste").font(FontId::proportional(LABEL_SIZE)));
                    Hotkey::new(&mut hotkeys.paste).ui(ui);
                    ui.end_row();

                    ui.label(RichText::new("Cut").font(FontId::proportional(LABEL_SIZE)));
                    Hotkey::new(&mut hotkeys.cut).ui(ui);
                    ui.end_row();

                    ui.label(RichText::new("Delete").font(FontId::proportional(LABEL_SIZE)));
                    Hotkey::new(&mut hotkeys.delete).ui(ui);
                    ui.end_row();

                    ui.label(RichText::new("Save").font(FontId::proportional(LABEL_SIZE)));
                    Hotkey::new(&mut hotkeys.save).ui(ui);
                    ui.end_row();

                    ui.label(
                        RichText::new("Close Note Editor").font(FontId::proportional(LABEL_SIZE)),
                    );
                    Hotkey::new(&mut hotkeys.close_note_editor).ui(ui);
                    ui.end_row();

                    ui.label(
                        RichText::new("Toggle Settings").font(FontId::proportional(LABEL_SIZE)),
                    );
                    Hotkey::new(&mut hotkeys.toggle_settings).ui(ui);
                    ui.end_row();

                    ui.label(RichText::new("Close App").font(FontId::proportional(LABEL_SIZE)));
                    Hotkey::new(&mut hotkeys.quit).ui(ui);
                    ui.end_row();
                });
        });
    }
}

pub type HotkeyAction<'app> = Box<dyn Fn(&mut GuiApp) + 'app>;

use std::collections::HashMap;

#[derive(Default)]
pub struct HotkeyMap<'app>(HashMap<BetterBinding, HotkeyAction<'app>>);

impl<'app> HotkeyMap<'app> {
    pub fn standard_hotkeys() -> Self {
        let mut map = HashMap::new();

        map.insert(
            BetterBinding::with_ctrl(Key::S),
            box default_save_action as _,
        );
        map.insert(
            BetterBinding::with_ctrl(Key::D),
            box default_delete_action as _,
        );
        map.insert(
            BetterBinding::with_ctrl(Key::N),
            box default_new_note_action as _,
        );

        Self(map)
    }
}

fn default_save_action(app: &mut GuiApp) {
    app.send_app_message(ToApp::SaveRequested);
}

fn default_delete_action(app: &mut GuiApp) {
    app.send_app_message(ToApp::DeleteActiveNote);
}

fn default_new_note_action(app: &mut GuiApp) {
    app.send_app_message(ToApp::CreateNewNote);
}
