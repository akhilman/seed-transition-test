use futures::future::TryFutureExt;
use futures::prelude::*;
use gloo_timers::future::TimeoutFuture;
use seed::{prelude::*, *};

// Model

struct Model {
    count: usize,
}

impl Default for Model {
    fn default() -> Self {
        Self { count: 0 }
    }
}

// Update

#[derive(Clone)]
enum Msg {
    Tick,
    Error(String),
}

async fn wait_tick() -> Result<Msg, Msg> {
    TimeoutFuture::new(500).await;
    Ok(Msg::Tick)
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Tick => {
            if model.count < usize::max_value() {
                model.count += 1
            } else {
                model.count = 0
            }
        }
        Msg::Error(err) => log!(err),
    };
    orders.perform_cmd(wait_tick());
}

// View

fn point(key: usize, pos: f32) -> Node<Msg> {
    let x = pos * 640.0;
    let y = (pos * 6.0).sin() * 50.0 + 50.0 + 20.0;
    g![
        circle![
            attrs![
                At::Cx => 15.0 + x,
                At::Cy => 15.0 + y,
                At::R => 30,
                At::Stroke => "blue",
                At::Fill => "none",
            ],
            style![
                St::Transition => "all 0.2s ease-in-out",
            ],
        ],
        text![
            attrs![
            At::X => 15.0 + x,
            At::Y => 15.0 + y,
            At::TextAnchor => "middle",
            At::DominantBaseline => "middle",
            ],
            format!("{}", key),
        ]
    ]
}

fn view(model: &Model) -> impl View<Msg> {
    let outer_style = style! {
        St::Display => "flex";
        St::FlexDirection => "column";
        St::TextAlign => "center";
    };
    div![
        outer_style,
        h1!["Animation"],
        div![
            style! {
                St::Color => if model.count > 4 {"purple" }else{ "gray"};
                St::Border => "2px solid #004422";
                St::Padding => unit!(20,px);
            },
            h3![format!("{}", model.count)],
            svg![
                attrs![At::Width => 640, At::Height => 240],
                (0..6)
                    .map(|num| {
                        let key = 6 - num + (model.count / 10);
                        let pos = (num as f32 / 6.0) + ((model.count % 10) as f32 / 10.0 / 6.0);
                        point(key, pos)
                    })
                    .collect::<Vec<_>>()
            ]
        ],
    ]
}

fn after_mount(_: Url, orders: &mut impl Orders<Msg>) -> AfterMount<Model> {
    orders.perform_cmd(wait_tick());
    AfterMount::default()
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view)
        .after_mount(after_mount)
        .build_and_start();
}
