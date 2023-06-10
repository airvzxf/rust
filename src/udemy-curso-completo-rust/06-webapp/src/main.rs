use wasm_bindgen::{JsCast, UnwrapThrowExt};
use wasm_bindgen_futures::spawn_local;
use web_sys::{console, Event, EventTarget, HtmlInputElement, InputEvent};
use yew::{
    function_component, html, use_state, virtual_dom::VNode, Callback, Html, MouseEvent,
    Properties, Renderer, SubmitEvent, UseStateHandle,
};

use crate::youtube::search_youtube_video;
mod env;
mod youtube;

fn main() {
    println!("Hello, world!");
    Renderer::<App>::new().render();
}

#[derive(Clone)]
struct Video {
    name: String,
    id: String,
}

#[function_component(App)]
fn app() -> Html {
    let video: UseStateHandle<Option<Video>> = use_state(|| None);
    let on_search: Callback<String> = {
        let video: UseStateHandle<Option<Video>> = video.clone();
        Callback::from(move |text_to_search: String| {
            let video: UseStateHandle<Option<Video>> = video.clone();
            spawn_local(async move {
                match search_youtube_video(text_to_search).await {
                    Ok(video_item) => video.set(Some(Video {
                        id: video_item.id.video_id,
                        name: video_item.snippet.title,
                    })),
                    Err(error) => {
                        console::log_1(&error.to_string().into());
                    }
                };
            });
        })
    };

    let video_section: VNode = match (*video).clone() {
        Some(video) => html! {
            <VideoPlayer name={video.name} id={video.id}/>
        },
        None => html! {},
    };

    html! {
        <main>
            <h1>
                {"YouTube search"}
            </h1>
            <VideoControls on_search={on_search} />
            {video_section}
        </main>
    }
}

#[derive(Properties, PartialEq)]
struct VideoControlsProps {
    on_search: Callback<String>,
}

#[function_component(VideoControls)]
fn video_controls(props: &VideoControlsProps) -> Html {
    let text_to_search: UseStateHandle<String> = use_state(|| String::new());

    let handle_input: Callback<InputEvent> = {
        let text_to_search: UseStateHandle<String> = text_to_search.clone();
        Callback::from(move |input_event: InputEvent| {
            let input_text: String = get_value_from_input_event(input_event);
            console::log_1(&input_text.clone().into());
            text_to_search.set(input_text);
        })
    };

    let on_submit: Callback<SubmitEvent> = Callback::from(|event: SubmitEvent| {
        event.prevent_default();
    });

    let on_search_button: Callback<MouseEvent> = {
        let on_search: Callback<String> = props.on_search.clone();
        Callback::from(move |_| {
            on_search.emit(text_to_search.to_string());
        })
    };

    html! {
        <form onsubmit={on_submit}>
            <input type="text" placeholder="Add some word" oninput={handle_input} />
            <button onclick={on_search_button}>{"Let's search!"}</button>
        </form>
    }
}

#[derive(Properties, PartialEq)]
struct VideoPlayerProps {
    id: String,
    name: String,
}

#[function_component(VideoPlayer)]
fn video_player(props: &VideoPlayerProps) -> Html {
    let _youtube_url: String = format!("https://www.youtube-nocookie.com/embed/{}", props.id);
    html! {
        <div>
            <iframe width="560" height="315" src={_youtube_url} title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen=true></iframe>
        </div>
    }
}

fn get_value_from_input_event(e: InputEvent) -> String {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target: EventTarget = event.target().unwrap_throw();
    let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();

    target.value()
}
