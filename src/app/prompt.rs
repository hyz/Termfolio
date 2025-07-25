mod general;
mod keyboard;
mod themes;

use super::commands::get_prompt;
use crate::app::{HistoryRecord, HistoryRecords};
use keyboard::keyboard_handler;
use leptos::{
    component,
    ev::SubmitEvent,
    html::{Form, Input},
    leptos_dom::helpers,
    logging::log,
    prelude::*,
    view, IntoView,
};
use leptos_use::use_event_listener;
use std::{sync::Arc, time::Duration};
use themes::theme_changer;
use thiserror::Error;

const AUTO_CMDS: &str = "about\nlinks\nhelp\n";

type UserInput = (String, Vec<String>);

async fn command_entered<F: Fn()>(
    (cmd, args): UserInput,
    history: RwSignal<HistoryRecords>,
    auto_cmds: RwSignal<(usize, usize)>,
    next_theme: F,
) -> Result<(), Error> {
    //log!("{cmd} {args:?} #{}",line!());
    let env = general::RunEnv {
        history: history.get_untracked(),
        //state,
        next_theme,
    };
    let HistoryRecord(cmd, res) = general::run(cmd, args, &env).await;

    log!("{} {cmd} {:?}", cmd.id, res.as_ref().map(String::len));

    history.update(|HistoryRecords(his)| {
        //let mut his = his.lock().unwrap();
        //let his = Arc::get_mut(his).unwrap();
        his.push_back(Arc::new(HistoryRecord(cmd, res)));
        while his.len() > 20 {
            his.pop_front();
        }
    });

    auto_cmds.notify();

    let (i, j) = auto_cmds.get_untracked();
    if let Some(s) = AUTO_CMDS.get(i..j) {
        if s.ends_with('\n') {
            log!("{i} {j} {s:?}");
            auto_cmds.set((j, j));
        }
    }
    Ok(())
}

#[component]
pub fn CommandLine(
    history: RwSignal<HistoryRecords>,
    auto_cmds: RwSignal<(usize, usize)>,
    //children: Children,
) -> impl IntoView {
    let (cmdargs, cmdargs_tx) = signal(("".into(), vec![]));
    let (state, next) = theme_changer();
    let cats = LocalResource::new(move || command_entered(cmdargs.get(), history, auto_cmds, next.clone()));
    //cats.await;

    //Output and history index signals
    let (history_index, set_history_index) = signal(0);

    //Form and input elements
    let form_ref = NodeRef::<Form>::new();
    let input_ref = NodeRef::<Input>::new();

    Effect::new(move |_o| {
        let (i, j) = auto_cmds.get();
        let cmd_line = AUTO_CMDS.get(i..j).filter(|s| !s.is_empty());
        match cmd_line {
            Some(cl) if cl.ends_with('\n') => {
                let form = form_ref.get_untracked().unwrap();
                let _ = form.request_submit().unwrap();
                return;
            }
            Some(cl) => {
                let input = input_ref.get_untracked().unwrap();
                input.set_value(cl.trim());
            }
            None if i >= AUTO_CMDS.len() => return,
            None => {}
        }
        helpers::set_timeout(move || auto_cmds.set((i, j + 1)), Duration::from_millis(300));
    });
    //On submit
    //let (state, next) = theme_changer();
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let input = input_ref.get().unwrap();
        let line = input.value();

        let char_map = |c: char| match c {
            '<' => '‹',
            '>' => '›',
            c => c,
        };
        let (cmd, args) = line.split_once(char::is_whitespace).unwrap_or((&line, ""));
        let cmd = cmd.trim().chars().map(char_map).collect();
        let args = args.trim().chars().map(char_map).collect();
        //let value = user_input.replace("<", "‹").replace(">", "›");
        //let (cmd, args) = value.split_once(char::is_whitespace).unwrap_or((&value, ""));
        cmdargs_tx.set((cmd, vec![args]));

        ////Theme changer
        //let next = next.clone();
        //leptos::task::spawn_local(async move {
        //    let user_input = user_input.trim();
        //    // check if duplicated with last history
        //    if history.with_untracked(|his| Some(user_input) == his.back().map(|l| l.user_input.as_str())) {
        //        log!("duplicated: {user_input}");
        //        return;
        //    }

        //    let his = history.read_untracked(); //.get_untracked();
        //    let result = general_commands(user_input, state, next, his).await;
        //    log!("{user_input}: ...{:?}", result.as_ref().map(|x| x.len()));

        //    history.update(|his| {
        //        his.push_back(UserCommand {
        //            user_input: user_input.into(),
        //            args: vec![],
        //            result,
        //        });
        //        while his.len() > 20 {
        //            his.pop_front();
        //        }
        //    });
        //    let (i, j) = auto_cmds.get_untracked();
        //    if AUTO_CMDS.get(i..j).filter(|cl| cl.ends_with('\n')).is_some() {
        //        auto_cmds.set((j, j));
        //    }
        //});

        //form.set_inert(true);
        //input.set_inert(true);
        input.set_value("");
        _ = input.focus();
    };

    // Event listener for Up and Down arrow keys, Tab and Ctrl/Command + L
    _ = use_event_listener(document(), leptos::ev::keydown, {
        let mut kb_handler = keyboard_handler(input_ref, history, history_index, set_history_index);
        move |ev| {
            let input = input_ref.get().unwrap();
            input.focus().unwrap();
            (kb_handler)(ev)
        }
    });

    // Focus on the new prompt on mount

    //input_ref.on_load(move |input| { log!("{}", input.node_name()); });
    Effect::new(move || {
        cats.with(|_his| {
            use web_sys::{ScrollBehavior, ScrollToOptions};
            let document = document();
            let body = document.body().unwrap();
            let options = ScrollToOptions::new();
            options.set_behavior(ScrollBehavior::Instant);
            options.set_top(body.scroll_height() as f64);
            window().scroll_to_with_scroll_to_options(&options);
        });
    });

    view! {
        <div id="observer-target" />
        <div id="floating-element">
            <span class="inline underscore">{get_prompt}</span>
            <form id="prompt-form" on:submit=on_submit node_ref=form_ref>
                <input
                    node_ref=input_ref
                    id="prompt-form-input"
                    autocomplete="off"
                    class="inp"
                    type="text"
                    maxlength=38
                    spellcheck="false"
                />
            </form>
        </div>
        <script>{HOOK_SCRIPT}</script>
    }
}

const HOOK_SCRIPT: &str = r#"(function(){
  const observer = new IntersectionObserver((entries) => {
    entries.forEach(entry => {
      const floatingElement = document.getElementById('floating-element');
      if (!entry.isIntersecting) {
        floatingElement.classList.add('sticky');
      } else {
        floatingElement.classList.remove('sticky');
        window.scrollTo(0, document.body.scrollHeight);
      }
    });
  }, { threshold: 0 });
  observer.observe(document.getElementById('observer-target'));
})();"#;

// log!("#{} {}", line!(), file!());
