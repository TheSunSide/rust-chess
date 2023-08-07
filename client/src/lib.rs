use dioxus::{html::input_data::keyboard_types::Key, prelude::*};
use log::info;
#[derive(PartialEq)]
pub enum FilterState {
    All,
    Active,
    Completed,
}

pub type Todos = im_rc::HashMap<u32, TodoItem>;

#[derive(Debug, PartialEq, Clone)]
pub struct TodoItem {
    pub id: u32,
    pub checked: bool,
    pub contents: String,
}

pub fn app(cx: Scope<()>) -> Element {
    cx.render(rsx! {
        section { class: "whole",
            style { include_str!("../src/style.css") }
            div {
                h1 { class: "centered", "My application" }
                div { class: "board",
                    (0..8).map(|i| {
                        rsx! {
                            div {
                                (0..8).map(|j| {
                                    if (i+j) % 2 == 0 {
                                        rsx! {
                                            div { class: "square black",
                                                onclick: move |_| {
                                                    info!("Clicked on square {i},{j}");
                                                }
                                         }
                                        }
                                    } else {
                                        rsx! {
                                            div { class: "square" }
                                        }
                                    }

                                })
                            }
                        }
                    })
                }
            }
        }
        footer { class: "info", p { "A footer" } }
    })
}

#[derive(Props)]
pub struct TodoEntryProps<'a> {
    set_todos: &'a UseRef<Todos>,
    id: u32,
}

pub fn todo_entry<'a>(cx: Scope<'a, TodoEntryProps<'a>>) -> Element {
    let editing = use_state(cx, || false);

    let todos = cx.props.set_todos.read();
    let todo = &todos[&cx.props.id];
    let is_checked = todo.checked;
    let completed = if is_checked { "completed" } else { "" };
    let is_editing = (**editing).then_some("editing").unwrap_or_default();

    render!(
        li {
            class: "{completed} {is_editing}",
            onclick: move |_| {
                if !is_checked {
                    editing.set(true)
                }
            },
            onfocusout: move |_| editing.set(false),
            div { class: "view",
                input {
                    class: "toggle",
                    r#type: "checkbox",
                    id: "cbg-{todo.id}",
                    checked: "{is_checked}",
                    onchange: move |evt| {
                        cx.props.set_todos.write()[&cx.props.id].checked = evt.value.parse().unwrap();
                    }
                }
                label { r#for: "cbg-{todo.id}", pointer_events: "none", "{todo.contents}" }
            }
            if **editing {
            rsx!{
                input {
                    class: "edit",
                    value: "{todo.contents}",
                    oninput: move |evt| cx.props.set_todos.write()[&cx.props.id].contents = evt.value.clone(),
                    autofocus: "true",
                    onkeydown: move |evt| {
                        match evt.key().to_string().as_str() {
                            "Enter" | "Escape" | "Tab" => editing.set(false),
                            _ => {}
                        }
                    },
                }
            }
        }
        }
    )
}
