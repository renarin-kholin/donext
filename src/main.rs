use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use vizia::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct AppTask {
    title: String,
    is_done: bool,
    id: i32,
}
impl AppTask {
    fn new(title: String, id: i32) -> Self {
        Self {
            title,
            is_done: false,
            id,
        }
    }
    fn mark_done(&mut self) {
        self.is_done = true;
    }
}
impl Default for AppTask {
    fn default() -> Self {
        Self {
            title: "Currently not doing anything.".to_string(),
            is_done: false,
            id: 0,
        }
    }
}
#[derive(Debug)]
pub enum AppEvent {
    UpdateDoNextInput(String),
    AddTask,
    CancelTask,
    CompleteTask,
    DeleteTask(i32),
}

#[derive(Debug, Serialize, Deserialize)]
struct AppDataSerializable {
    donext_input: String,
    tasks: Vec<AppTask>,
    current_id: i32,
}

impl AppDataSerializable {
    fn path() -> PathBuf {
        dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("donext")
            .join("data.json")
    }

    fn load() -> Option<Self> {
        let path = Self::path();
        if path.exists() {
            let content = fs::read_to_string(&path).ok()?;
            serde_json::from_str(&content).ok()
        } else {
            None
        }
    }

    fn save(&self) {
        let path = Self::path();
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let content = serde_json::to_string_pretty(self).expect("Failed to serialize data");
        let _ = fs::write(path, content);
    }
}
struct AppData {
    pub donext_input: Signal<String>,
    pub tasks: Signal<Vec<AppTask>>,
    pub current_id: i32,
}
impl Model for AppData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _meta| {
            match app_event {
                AppEvent::UpdateDoNextInput(donext) => {
                    self.donext_input.update(|d| *d = donext.to_string());
                }
                AppEvent::AddTask => {
                    let current_input = self.donext_input.get();
                    self.tasks
                        .update(|t| t.push(AppTask::new(current_input, self.current_id + 1)));
                    self.current_id += 1;

                    self.donext_input.update(|d| *d = "".to_string());
                }
                AppEvent::CancelTask => {
                    self.tasks.update(|t| {
                        t.pop();
                    });
                }
                AppEvent::CompleteTask => {
                    self.tasks.update(|t| {
                        if let Some(last) = t.last_mut() {
                            last.mark_done();
                        }
                    });
                }
                AppEvent::DeleteTask(id) => {
                    self.tasks.update(|t| {
                        let index = t.iter().position(|ti| ti.id == *id);
                        if let Some(index) = index {
                            t.remove(index);
                        }
                    });
                }
            }
            let serializable = AppDataSerializable {
                donext_input: self.donext_input.get(),
                tasks: self.tasks.get(),
                current_id: self.current_id,
            };
            serializable.save();
        });
    }
}
fn main() -> Result<(), ApplicationError> {
    let (initial_input, initial_tasks, initial_id) = AppDataSerializable::load()
        .map(|d| (d.donext_input, d.tasks, d.current_id))
        .unwrap_or((String::new(), vec![], 0));

    let app = Application::new(move |cx| {
        cx.add_stylesheet(include_style!("src/style.css"))
            .expect("Failed to load css styles.");
        cx.emit(EnvironmentEvent::SetThemeMode(ThemeMode::DarkMode));

        let donext = Signal::new(initial_input.clone());
        let tasks: Signal<Vec<AppTask>> = Signal::new(initial_tasks.clone());
        AppData {
            donext_input: donext,
            tasks,
            current_id: initial_id,
        }
        .build(cx);

        VStack::new(cx, |cx| {
            VStack::new(cx, move |cx| {
                HStack::new(cx, |cx| {
                    Textbox::new(cx, donext)
                        .placeholder("What to do next")
                        .class("donext_textbox")
                        .on_edit(|cx, text| cx.emit(AppEvent::UpdateDoNextInput(text)))
                        .on_submit(|cx, _, from_enter| {
                            if from_enter {
                                cx.emit(AppEvent::AddTask)
                            }
                        });
                    Button::new(cx, |cx| Label::new(cx, "Next"))
                        .class("donext_button")
                        .on_press(|cx| cx.emit(AppEvent::AddTask))
                        .variant(ButtonVariant::Primary);
                })
                .class("donext_container");
                VStack::new(cx, |cx| {
                    Label::new(cx, "IN PROGRESS").class("currently_doing_label");
                    HStack::new(cx, |cx| {
                        let current_task = Memo::new(move |_| {
                            let tasks = tasks.read();
                            let last = tasks.last();
                            if last.is_some_and(|t| !t.is_done) {
                                last.unwrap().title.clone()
                            } else {
                                "Currently not doing anything".to_string()
                            }
                        });
                        Label::new(cx, current_task).class("currently_doing_value");
                        Button::new(cx, |cx| Label::new(cx, "Cancel"))
                            .class("donext_cancel_button")
                            .on_press(|cx| cx.emit(AppEvent::CancelTask))
                            .variant(ButtonVariant::Outline);
                        Button::new(cx, |cx| Label::new(cx, "Done"))
                            .variant(ButtonVariant::Secondary)
                            .on_press(|cx| cx.emit(AppEvent::CompleteTask))
                            .class("currently_doing_button");
                    })
                    .class("currently_doing_value_container");
                })
                .class("currently_doing_container");
                Divider::horizontal(cx);
                ScrollView::new(cx, move |cx| {
                    List::new(cx, tasks, move |cx, _index, item| {
                        let done_task = Memo::new(move |_| {
                            let item = item.get();
                            format!("Done {}", item.title)
                        });
                        if item.get().is_done {
                            HStack::new(cx, move |cx| {
                                Label::new(cx, done_task).class("done_task_item");
                                Button::new(cx, move |cx| Label::new(cx, "Delete")).on_press(
                                    move |cx| cx.emit(AppEvent::DeleteTask(item.get().id)),
                                );
                            })
                            .class("done_task_item_container");
                        }
                    })
                    .class("done_tasks_list");
                })
                .show_vertical_scrollbar(true)
                .size(Stretch(1.0))
                .class("done_tasks_scrollview");
            })
            .class("app");
        })
        .alignment(Alignment::TopCenter);
    })
    .min_inner_size(Some((500, 720)))
    .title("DoNext");
    app.run()
}
