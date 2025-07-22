use crate::{app::commands::autocomplete, app::HistoryRecords};
use leptos::{ev::KeyboardEvent, html::Input, prelude::*};
use std::cmp::Ordering;

//pub fn use_event_listener<Ev, El, M, F>(target: El, event: Ev, handler: F) -> impl Fn() + Clone + Send + Sync

pub fn keyboard_handler(
    input_ref: NodeRef<Input>,
    history: RwSignal<HistoryRecords>,
    history_index: ReadSignal<u8>,
    set_history_index: WriteSignal<u8>,
    //submitter: WriteSignal<usize>,
) -> impl FnMut(KeyboardEvent) + 'static {
    move |ev: KeyboardEvent| {
        //leptos::logging::log!("keyboard event: {:?}", ev);
        let index = history_index.get_untracked().into();
        let input = input_ref.get().unwrap();

        match ev.key().as_str() {
            //Previous command in history
            "ArrowUp" => {
                ev.prevent_default();
                history.with_untracked(|HistoryRecords(his)| {
                    let his = his.lock().unwrap();
                    if index < his.len() {
                        let c = format!("{}", his[index].0);
                        input.set_value(&c);
                        set_history_index.update(move |history_index| *history_index += 1);
                    }
                });
            }

            //Next command in history
            "ArrowDown" => match index.cmp(&1) {
                Ordering::Greater => {
                    history.with_untracked(|HistoryRecords(his)| {
                        let his = his.lock().unwrap();
                        let c = format!("{}", his[index - 2].0);
                        //hist.0[index - 2].user_input
                        input.set_value(&c);
                    });
                    set_history_index.update(move |history_index| *history_index -= 1);
                }
                Ordering::Equal => {
                    input.set_value("");
                    set_history_index.update(move |history_index| *history_index -= 1);
                }
                Ordering::Less => (),
            },

            //Autocomplete
            "Tab" => {
                ev.prevent_default();
                input.set_value(autocomplete(&input.value()));
            }
            _ => {}
        }

        //Ctrl
        if ev.ctrl_key() || ev.meta_key() {
            // Clear
            match &ev.key()[..] {
                "l" | "L" => {
                    ev.prevent_default();
                    //submitter.update(|prompts| {
                    //    *prompts = 0;
                    //});
                    //submitter.update(|prompts| {
                    //    *prompts += 1;
                    //});
                }
                // Can add Ctrl + P / N for history,
                // but will interfere with new window shortcut
                _ => {}
            }
        }
    }
}
