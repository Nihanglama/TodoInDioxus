use dioxus::{html::input_data::keyboard_types::Key, prelude::*};
pub type Todos = im::HashMap<u32, TodoItem>;

#[derive(Debug, PartialEq, Clone)]
pub struct TodoItem {
    pub id: u32,
    pub contents: String,
}
pub fn app(cx: Scope<()>) -> Element {
    let todos: &UseRef<im::HashMap<u32, TodoItem>> = use_ref(cx, || {
        let default_todos = im::HashMap::<u32, TodoItem>::default();
        default_todos
    });
    let new_todo_item: &UseRef<String> = use_ref(cx, String::new);
    let todo_id: &UseState<u32> = use_state(cx, || 0);

    cx.render(rsx! {
            section { class: "todo-app",
            style { include_str!("style.css") }
                div {
                    header { class: "header",
                        h1 {"Todo App"}
                        todo_input { todos: todos.clone(), new_todo_item: new_todo_item, todo_id: todo_id }
                    }
                    todo_list { todos: todos.clone() }
                }
            }
        })
}
#[derive(Props, PartialEq)]
pub struct TodoInputProps<'a> {
    todos: UseRef<im::HashMap<u32, TodoItem>>,
    new_todo_item: &'a UseRef<String>,
    todo_id: &'a UseState<u32>,
}
pub fn todo_input<'a>(cx: Scope<'a, TodoInputProps>) -> Element<'a> {
    cx.render(rsx! {
        input {
            class: "new-todo",
            placeholder: "Add Todo",
            value: "{cx.props.new_todo_item.read()}",
            autofocus: true,
            oninput: move |event| cx.props.new_todo_item.set(event.value.clone()),
            onkeydown: move |event| {
                if event.key() == Key::Enter && !cx.props.new_todo_item.read().is_empty() {
                    cx.props.todos.write().insert(
                        *cx.props.todo_id.get(),
                        TodoItem {
                            id: *cx.props.todo_id.get(),
                            contents: cx.props.new_todo_item.read().clone(),
                        },
                    );
                    cx.props.todo_id.set(cx.props.todo_id + 1);
                    cx.props.new_todo_item.set("".to_string());
                }
            }
        }
    })
}
#[derive(Props, PartialEq)]
pub struct TodoListProps {
    todos: UseRef<im::HashMap<u32, TodoItem>>,
}
pub fn todo_list<'a>(cx: Scope<'a, TodoListProps>) -> Element {
    cx.render(rsx! {ul { class: "todo-list",
    cx.props.todos.read().iter().map(|(id, _todo)| {
        rsx! { todo_entry { key: "{id}", id: *id, set_todos: &cx.props.todos } }
    })
    }
    })
}
#[derive(Props)]
pub struct TodoEntryProps<'a> {
    set_todos: &'a UseRef<Todos>,
    id: u32,
}
pub fn todo_entry<'a>(cx: Scope<'a, TodoEntryProps<'a>>) -> Element {
    let todos: std::cell::Ref<'_, im::HashMap<u32, TodoItem>> = cx.props.set_todos.read();
    let todo: &TodoItem = &todos[&cx.props.id];
    render!(li {
        div { class: "view",
            label { "{todo.contents}" }
            button {
                class: "remove",
                onclick: move |_| {
                    cx.props.set_todos.write().remove(&cx.props.id);
                },
                "Delete"
                
            }
        }
    })
}
