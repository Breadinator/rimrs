#![warn(clippy::pedantic)]

use eframe::{
    egui::{
        self,
        CentralPanel,
        TopBottomPanel,
    },
    CreationContext,
    App,
};

pub mod panels;
pub mod helpers;

#[derive(Default)]
pub struct RimRs;

impl RimRs {
    #[must_use]
    #[allow(unused_variables)]
    pub fn new(cc: &CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl App for RimRs {
    #[allow(unused_variables)]
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        TopBottomPanel::top("paths_panel").show(ctx, panels::paths);
        TopBottomPanel::bottom("hint_panel").show(ctx, panels::hint);
        CentralPanel::default().show(ctx, panels::mods);
    }
}

