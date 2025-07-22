mod banner;
mod commands;
mod history;
mod prompt;

use leptos::{component, logging::log, prelude::*, view, IntoView};
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use history::History;
use prompt::CommandLine;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct UserCommand {
    pub(crate) id: u32,
    pub cmd: String,
    pub args: Vec<String>,
}

impl std::fmt::Display for UserCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.cmd)?;
        let mut iter = self.args.iter();
        if let Some(a) = iter.next() {
            write!(f, " {a}")?;
            while let Some(a) = iter.next() {
                write!(f, ", {a}")?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct HistoryRecord(pub UserCommand, pub Result<String, String>);

#[derive(Debug, Clone, Default)]
pub struct HistoryRecords(pub Arc<Mutex<VecDeque<HistoryRecord>>>);

#[component]
pub fn App() -> impl IntoView {
    let history = RwSignal::new(HistoryRecords::default());
    let auto_cmds = RwSignal::new((0, 0));

    view! {
        <History history />
        <CommandLine history auto_cmds />
    }
}
