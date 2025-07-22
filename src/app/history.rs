use leptos::{component, logging::log, prelude::*, view, IntoView};

use crate::app::{commands::get_prompt, HistoryRecord, HistoryRecords};

#[component]
pub fn History(history: RwSignal<HistoryRecords>) -> impl IntoView {
    let each_record = move || {
        history.with(|HistoryRecords(his)| {
            let his = his.lock().unwrap();
            his.iter()
                .map(|HistoryRecord(cmd, res)| (cmd.id, format!("{cmd}"), result_string(res)))
                .collect::<Vec<_>>()
                .into_iter()
        })
    };
    view! {
        <div>
        <For
            each=each_record
            key=|k| k.0 //user_input.clone() //.line.as_str()
            children=move |(_i, user_input, result)| {
                //let user_input = format!("{}",his.0);//user_input.clone();
                //let output = result_string(his);
                //leptos::logging::log!("{} ...{:?}", user_input, output.len());
                view! {
                    <div>
                        <p class="inline">{get_prompt}</p>
                        <input
                            inert
                            value=user_input
                            class="inp"
                            type="text"
                            maxlength=38
                            spellcheck="false"
                        />
                    </div>
                    <pre> <div class="output" inner_html=result /> </pre>
                 }
            }
        />
        </div>
    }
}

fn result_string(hr: &Result<String, String>) -> String {
    match hr.clone() {
        Ok(s) => s,
        Err(s) => s,
    }
}
