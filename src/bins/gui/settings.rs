// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::path::{Path, PathBuf};

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

pub enum ValueType {
    Path,
    String,
    Bool,
    Number,
    Choice(Vec<String>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AppSettingKind {
    DefaultDatabase,
    LoadDefaultOnStart,
    DefaultSavePath,
    AutosaveEnabled,
    AutosaveInterval,
}

impl AppSettingKind {
    pub fn get_value_type(self) -> ValueType {
        match self {
            AppSettingKind::DefaultDatabase | AppSettingKind::DefaultSavePath => ValueType::Path,
            AppSettingKind::LoadDefaultOnStart | AppSettingKind::AutosaveEnabled => ValueType::Bool,
            AppSettingKind::AutosaveInterval => ValueType::Number,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AppSettings {
    /// The default database location
    pub default_database: PathBuf,
    /// The default location to save new databases
    pub default_save_path: PathBuf,
    /// How often should we autosave (in SECONDS)
    pub autosave_interval: u64,
    /// Whether the default Database should be loaded on start
    pub load_default_on_start: bool,
    pub autosave_enabled: bool,
}

impl AppSettings {
    pub fn new() -> crate::Result<Self> {
        let dirs = match Self::project_dirs() {
            Some(dirs) => dirs,
            None => {
                return Err(crate::Error::Io(std::io::Error::other(
                    "Unable to load project directories from system",
                )));
            }
        };

        let data_dir = dirs.data_dir();
        let default_save_path = data_dir.to_path_buf();
        let default_database = default_save_path.join("default.noted");

        let config = AppSettings {
            default_database,
            default_save_path,
            autosave_interval: 30,
            load_default_on_start: true,
            autosave_enabled: true,
        };

        Ok(config)
    }

    pub fn load_or_create() -> crate::Result<Self> {
        if let Ok(config) = Self::load_default() {
            return Ok(config);
        }

        let dirs = match Self::project_dirs() {
            Some(dirs) => dirs,
            None => {
                return Err(crate::Error::Io(std::io::Error::other(
                    "Unable to load project directories from system",
                )));
            }
        };

        std::fs::create_dir_all(dirs.cache_dir());
        std::fs::create_dir_all(dirs.config_dir());
        std::fs::create_dir_all(dirs.data_dir());
        std::fs::create_dir_all(dirs.data_local_dir());

        let config = Self::new()?;

        config.save_default()?;

        Ok(config)
    }

    pub fn load<P: AsRef<Path>>(path: P) -> crate::Result<Self> {
        let path = path.as_ref();
        let file = std::fs::File::open(path)?;
        let settings: Self = serde_json::from_reader(file)?;
        Ok(settings)
    }

    pub fn load_default() -> crate::Result<Self> {
        let path = Self::default_config_file()?;
        Self::load(path)
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> crate::Result {
        let path = path.as_ref();
        let file = std::fs::File::create(path)?;
        serde_json::to_writer_pretty(file, &self)?;
        Ok(())
    }

    pub fn save_default(&self) -> crate::Result {
        let path = Self::default_config_file()?;
        self.save(path)
    }

    pub fn get_setting(&self, kind: AppSettingKind) -> String {
        match kind {
            AppSettingKind::DefaultDatabase => self.default_database.display().to_string(),
            AppSettingKind::LoadDefaultOnStart => self.load_default_on_start.to_string(),
            AppSettingKind::DefaultSavePath => self.default_save_path.display().to_string(),
            AppSettingKind::AutosaveInterval => self.autosave_interval.to_string(),
            AppSettingKind::AutosaveEnabled => self.autosave_enabled.to_string(),
        }
    }

    pub fn set_setting(&mut self, key: AppSettingKind, value: &str) -> Option<()> {
        match key {
            AppSettingKind::DefaultDatabase => {
                self.default_database = PathBuf::try_from(value).ok()?;
                Some(())
            }
            AppSettingKind::DefaultSavePath => {
                self.default_save_path = PathBuf::try_from(value).ok()?;
                Some(())
            }
            AppSettingKind::AutosaveInterval => {
                self.autosave_interval = value.parse().ok()?;
                Some(())
            }
            AppSettingKind::LoadDefaultOnStart => {
                self.load_default_on_start = value.parse().ok()?;
                Some(())
            }
            AppSettingKind::AutosaveEnabled => {
                self.autosave_enabled = value.parse().ok()?;
                Some(())
            }
        }
    }

    pub fn default_config_file() -> crate::Result<PathBuf> {
        let dirs = match Self::project_dirs() {
            Some(dirs) => dirs,
            None => {
                return Err(crate::Error::Io(std::io::Error::other(
                    "Unable to load project directories from system",
                )));
            }
        };
        let config_path = dirs.config_dir().join(Self::DEFAULT_FILENAME);
        Ok(config_path)
    }

    const DEFAULT_FILENAME: &'static str = "noted_config.json";

    fn project_dirs() -> Option<&'static ProjectDirs> {
        use once_cell::sync::OnceCell;
        static DIRS: OnceCell<Option<ProjectDirs>> = OnceCell::new();
        DIRS.get_or_init(|| directories::ProjectDirs::from("rs", "imtony", "Noted"))
            .as_ref()
    }
}

fn settings_base_id() -> &'static egui::Id {
    use once_cell::sync::OnceCell;
    static ID: OnceCell<egui::Id> = OnceCell::new();
    ID.get_or_init(|| egui::Id::new("settings_section"))
}

pub struct AppSettingsUi;

impl AppSettingsUi {
    /// Render the [`AppSettings`] ui. Returns true if app settings have been changed.
    #[allow(clippy::similar_names)]
    pub fn render(ui: &mut egui::Ui, settings: &mut AppSettings) -> bool {
        let mut has_changed = false;
        let id = egui::Id::new("settings_section").with("appsettings");

        egui::Frame {
            fill: super::get_app_theme().colors.darker_gray,
            inner_margin: egui::style::Margin::same(6.0),
            rounding: super::get_app_theme().rounding.big,
            ..egui::Frame::default()
        }
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Application Settings");
            });
        });

        ui.separator();
        ui.indent(id.with("body"), |ui| {
            let ldos_res = ui.checkbox(
                &mut settings.load_default_on_start,
                "Load default database on application start",
            );
            if ldos_res.changed {
                has_changed = true;
            }

            let dd_res = ui.add_enabled(
                settings.load_default_on_start,
                egui::Label::new(settings.default_database.display().to_string())
                    .sense(egui::Sense::click()),
            );
            if dd_res.clicked() {
                if let Some(path) = rfd::FileDialog::new()
                    .set_title("Select Default Database")
                    .add_filter("Noted Database", &["db", "fdb", "noted", "data"])
                    .set_directory(settings.default_save_path.clone())
                    .pick_file()
                {
                    if settings.default_database != path {
                        settings.default_database = path;
                        has_changed = true;
                    }
                }
            }

            let dsp_res = ui.add(
                egui::Label::new(settings.default_save_path.display().to_string())
                    .sense(egui::Sense::click()),
            );
            if dsp_res.clicked() {
                if let Some(path) = rfd::FileDialog::new()
                    .set_title("Select Default Save Location")
                    .set_directory(settings.default_save_path.clone())
                    .pick_folder()
                {
                    if settings.default_save_path != path {
                        settings.default_save_path = path;
                        has_changed = true;
                    }
                }
            }

            let ase_res = ui.checkbox(
                &mut settings.autosave_enabled,
                "Enable Autosave of Current Database",
            );
            if ase_res.changed() {
                has_changed = true;
            }

            // ui.add_enabled(
            //     settings.autosave_enabled,
            //     egui::Label::new("Autosave Interval (In Seconds)"),
            // );
            let asi_res = ui.add_enabled(
                settings.autosave_enabled,
                egui::Slider::new(&mut settings.autosave_interval, 1u64..=300u64)
                    .integer()
                    .show_value(true)
                    .suffix("sec")
                    .orientation(egui::SliderOrientation::Horizontal)
                    .step_by(1.0)
                    .text("Autosave Interval (in seconds)"),
            );
            if asi_res.changed() {
                has_changed = true;
            }
        });

        has_changed
    }
}
