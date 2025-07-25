use leptos::{component, logging::log, prelude::*, view, IntoView};
use std::sync::Arc;

use crate::app::{commands::get_prompt, HistoryRecords};
#[component]
pub fn History(history: RwSignal<HistoryRecords>) -> impl IntoView {
    let each_record =
        move || history.with(|HistoryRecords(his)| his.iter().map(Arc::clone).collect::<Vec<_>>().into_iter());
    view! {
        <div>
        <For
            each=each_record
            key=|rec| rec.0.id
            children=move |rec| {
                let user_input = format!("{}",rec.0);//user_input.clone();
                let result = result_string(&rec.1);
                //leptos::logging::log!("{} ...{:?}", user_input, output.len());
                view! {
                    <div>
                        <span class="inline">{get_prompt}</span>
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
