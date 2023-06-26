use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod components;
use components::chat_area::ChatArea;
use components::type_area::TypeArea;

use crate::api::converse;
use crate::model::conversation::{Conversation, Message};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    let (conversation, set_conversation) = create_signal(cx, Conversation::new());

    let send = create_action(cx, move |new_message: &String| {
        let mut curr = conversation.get_untracked();
        let user_message = Message {
            text: new_message.clone(),
            user: true,
        };
        curr.messages.push(user_message);
        set_conversation(curr.clone());
        converse(cx, curr)
    });

    create_effect(cx, move |_| {
        log!("{:?}", send.input().get());
        if let Some(_) = send.input().get() {
            let mut curr = conversation.get_untracked();
            let model_message = Message {
                text: String::from("..."),
                user: false,
            };
            curr.messages.push(model_message);
            set_conversation(curr);
        }
    });

    create_effect(cx, move |_| {
        log!("{:?}", send.value().get());
        if let Some(Ok(response)) = send.value().get() {
            let mut curr = conversation.get_untracked();
            let last: &mut Message = curr.messages.last_mut().unwrap();
            last.text = response;
            set_conversation(curr);
        }
    });

    view! { cx,
        <ChatArea conversation/>
        <TypeArea send/>
    }
}