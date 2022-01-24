use druid::widget::prelude::*;
use druid::widget::{Flex, Label, TextBox};
use druid::{
    AppLauncher, Data, KeyEvent,
    Lens, TimerToken, UnitPoint, WidgetExt, WindowDesc
};
use druid::keyboard_types::Key;

use std::ops::Add;
use std::process;
use std::sync::Arc;
use std::time::{Duration, Instant};

#[derive(Data, Clone, Lens)]
struct AppData {
    message: String,
    countdown: u64,
    deadline: Arc<Instant>
}

struct DelayWidget<T: Widget<AppData>> {
    child: T,
    timer_id: TimerToken
}

impl<T: Widget<AppData>> DelayWidget<T> {
    fn new(child: T) -> DelayWidget<T> {
        DelayWidget
        {
            child,
            timer_id: TimerToken::INVALID
        }
    }
}

impl<T: Widget<AppData>> Widget<AppData> for DelayWidget<T> {
    fn event(
        &mut self,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut AppData,
        _env: &Env
    ) {
        match event {
            Event::WindowConnected => {
                ctx.request_paint();
                ctx.request_focus();
                let deadline = Duration::from_millis(250);
                self.timer_id = ctx.request_timer(deadline);
            },
            Event::Timer(id) => {
                if *id == self.timer_id {
                    let now = Instant::now();
                    if now < *data.deadline {
                        ctx.request_paint();
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
        self.child.event(ctx, event, data, _env);
    }

    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &AppData,
        env: &Env,
    ) {
        match event {
            LifeCycle::WidgetAdded => {
                ctx.register_for_focus();
            },
            _ => {}
        };
        self.child.lifecycle(ctx, event, data, env);
    }

    fn update(
        &mut self,
        ctx: &mut UpdateCtx,
        old_data: &AppData,
        data: &AppData,
        env: &Env
    ) {
        self.child.update(ctx, old_data, data, env);
    }

    fn layout(&mut self,
        layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &AppData,
        env: &Env,
    ) -> Size {
        self.child.layout(layout_ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppData, env: &Env) {
        self.child.paint(ctx, data, env);
    }
}

pub fn delay(delay_in_s: u64, message: Option<String>, cmd: &str, args: &Vec<String>) {
    let main_window = WindowDesc::new(build_root_widget)
        .title("delay")
        .window_size((400.0, 400.0));

    AppLauncher::with_window(main_window)
        .launch(AppData {
            message: message
                .unwrap_or("Program execution delayed".to_string()),
            countdown: delay_in_s,
            deadline: Arc::new(
                Instant::now().add(Duration::from_secs(delay_in_s))
            )
        })
        .expect("Failed to launch GUI");
}

fn build_root_widget() -> impl Widget<AppData> {
    let label = Label::dynamic(|data: &AppData, _: &Env| {
        format!("{}\n{}\n(Press ESC to abort)", data.message, data.countdown)
    })
    .with_text_size(32.0);
    let root = DelayWidget::new(label);

    Flex::column()
        .with_child(root)
        .align_vertical(UnitPoint::CENTER)
}
