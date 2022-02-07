use druid::widget::prelude::*;
use druid::widget::{Controller, Flex, Label};
use druid::{
    AppLauncher, Data, Lens, TimerToken, UnitPoint, WidgetExt, WindowDesc
};
use druid::keyboard_types::Key;

use std::ops::Add;
use std::sync::Arc;
use std::time::{Duration, Instant};

#[derive(Data, Clone, Lens)]
struct AppData {
    message: String,
    countdown: u64,
    deadline: Arc<Instant>
}

struct DelayController {
    timer_id: TimerToken
}

impl DelayController {
    fn new() -> DelayController {
        DelayController { timer_id: TimerToken::INVALID }
    }
}

impl<W: Widget<AppData>> Controller<AppData, W> for DelayController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut AppData,
        env: &Env
    ) {
        match event {
            Event::WindowConnected => {
                ctx.request_focus();
                let deadline = Duration::from_millis(250);
                self.timer_id = ctx.request_timer(deadline);
            },
            Event::Timer(id) => {
                if *id == self.timer_id {
                    let now = Instant::now();
                    if now < *data.deadline {
                        let deadline = Duration::from_millis(250);
                        self.timer_id = ctx.request_timer(deadline);
                        data.countdown = (*data.deadline - now).as_secs();
                    } else {
                        std::process::exit(0);
                    }
                }
            },
            Event::KeyDown(k) => {
                if k.key == Key::Escape {
                    std::process::exit(1)
                }
            },
            _ => {}
        }
        child.event(ctx, event, data, env);
    }
}

pub fn delay(delay_in_s: u64, message: Option<String>) {
    let main_window = WindowDesc::new(build_root_widget)
        .title("delay")
        .window_size((400.0, 400.0));

    AppLauncher::with_window(main_window)
        .launch(AppData {
            message: message
                .unwrap_or_else(|| "Program execution delayed".to_string()),
            countdown: delay_in_s,
            deadline: Arc::new(
                Instant::now().add(Duration::from_secs(delay_in_s))
            )
        })
        .expect("Failed to launch GUI");
}

fn build_root_widget() -> impl Widget<AppData> {
    let label_message = Label::new(|data: &AppData, _: &Env| {
        data.message.clone()
    }).with_text_size(32.0);
    let label_countdown = Label::new(|data: &AppData, _: &Env| {
        format!("{}", data.countdown)
    }).with_text_size(50.0);
    let label_hint = Label::new("(Press ESC to abort)")
        .with_text_size(20.0);

    Flex::column()
        .with_child(label_message)
        .with_child(label_countdown)
        .with_child(label_hint)
        .align_vertical(UnitPoint::CENTER)
        .controller(DelayController::new())
}
