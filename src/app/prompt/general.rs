use crate::app::commands::run_command;
use crate::app::{HistoryRecord, HistoryRecords, UserCommand};

pub struct RunEnv<F: Fn()> {
    //pub(crate) state: Signal<ColorMode>,
    pub(crate) next_theme: F,
    pub(crate) history: HistoryRecords,
}

pub async fn run<F>(cmd: String, args: Vec<String>, env: &RunEnv<F>) -> HistoryRecord
where
    F: Fn(),
{
    fn next_id() -> u32 {
        use std::sync::atomic::{AtomicU32, Ordering};
        static COUNTER: AtomicU32 = AtomicU32::new(0);
        COUNTER.fetch_add(1, Ordering::SeqCst)
    }

    let r = general_commands(&cmd, &args, &env.next_theme, &env.history).await;

    let cmd = UserCommand {
        id: next_id(),
        cmd,
        args,
    };
    //let result = general_commands(user_input, state, next, his).await;
    HistoryRecord(cmd, r)
}

pub async fn general_commands<F: Fn()>(
    cmd: &str,
    args: &Vec<String>,
    //state: &ColorMode,
    next_theme: &F, //impl Fn() + Sized,
    //set_output: WriteSignal<String>,
    //submitter: WriteSignal<usize>,
    //updater: WriteSignal<VecDeque<CommandLine>>,
    HistoryRecords(his): &HistoryRecords,
) -> Result<String, String> {
    //let value = value.trim().replace("<", "‹").replace(">", "›");
    //let (cmd, args) = value.split_once(char::is_whitespace).unwrap_or((&value, ""));

    match cmd {
        "clear" => {
            //submitter.update(|prompts| {
            //    *prompts = 0;
            //});
            Ok(String::new())
        }
        "history" => {
            use itertools::Itertools;
            //let his = history.lock().unwrap();
            let r = his.iter().zip(1..).map(|(h, i)| format!("{i} {}", h.0)).join("\n");
            Ok(r) //set_output.set(hist);
        }
        "theme" | "t" | "wal" => {
            next_theme();
            let new_theme = ""; // state.get_untracked();
            let out = format!(r#"Theme changed to: <b class="grn">{new_theme}</b>"#);
            Ok(out) //set_output.set();
        }
        _ => run_command(cmd, args).await,
    }

    /*updater.update(|hist| {
        if !value.is_empty() && hist.front().filter(|h| h.line == value).is_none() {
            hist.push_front(CommandLine {
                line: value,
                result: "".into(), //Ok(String::new()),
            });
            while hist.len() > 20 {
                hist.pop_back();
            }
        }
    });*/

    // Clears if max limit is reached
    //submitter.update(|prompts| {
    //    if *prompts == u8::MAX {
    //        *prompts = 0;
    //    }
    //});

    //submitter.update(|prompts| {
    //    *prompts += 1;
    //});
}
