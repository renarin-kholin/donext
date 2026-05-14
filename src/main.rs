use vizia::prelude::*;
#[derive(Clone, Debug, PartialEq, Eq)]
struct AppTask {
    title: String,
    is_done: bool,
}
impl AppTask {
    fn new(title: String) -> Self {
        Self {
            title,
            is_done: false,
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
        }
    }
}
#[derive(Debug)]
pub enum AppEvent {
    UpdateDoNextInput(String),
    AddTask,
    CancelTask,
    CompleteTask,
}
struct AppData {
    pub donext_input: Signal<String>,
    pub tasks: Signal<Vec<AppTask>>,
}
impl Model for AppData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, meta| {
            println!("Event emitted: {:?}", app_event);
            match app_event {
                AppEvent::UpdateDoNextInput(donext) => {
                    self.donext_input.update(|d| *d = donext.to_string());
                }
                AppEvent::AddTask => {
                    let current_input = self.donext_input.get();
                    self.tasks.update(|t| t.push(AppTask::new(current_input)));

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
            }
        });
    }
}
fn main() -> Result<(), ApplicationError> {
    let app = Application::new(|cx| {
        cx.add_stylesheet(include_style!("src/style.css"))
            .expect("Failed to load css styles.");
        cx.emit(EnvironmentEvent::SetThemeMode(ThemeMode::DarkMode));

        let donext = Signal::new(String::from(""));
        let tasks: Signal<Vec<AppTask>> = Signal::new(vec![]);
        AppData {
            donext_input: donext,
            tasks,
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
                    List::new(cx, tasks, move |cx, index, item| {
                        let done_task = Memo::new(move |_| {
                            let item = item.get();
                            format!("Done {}", item.title)
                        });
                        if item.get().is_done {
                            Label::new(cx, done_task).class("done_task_item");
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
