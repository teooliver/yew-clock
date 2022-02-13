use gloo::{
    console::{self},
    timers::callback::Interval,
};

use yew::{html, Component, Context, Html};

pub enum Msg {
    StartClock,
    StopClock,
    UpdateTime,
}

pub struct Clock {
    time: String,
    standalone: Option<Interval>,
    is_running: bool,
}

impl Clock {
    fn get_current_time() -> String {
        let date = js_sys::Date::new_0();
        String::from(date.to_locale_time_string("en-US"))
    }
}

impl Component for Clock {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();
        link.send_message(Msg::StartClock);

        Self {
            time: Clock::get_current_time(),
            standalone: None,
            is_running: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::StartClock => {
                let handle = {
                    let link = ctx.link().clone();
                    Interval::new(1, move || link.send_message(Msg::UpdateTime))
                };

                self.standalone = Some(handle);
                self.is_running = true;

                console::log!("Start Clock");
            }

            Msg::UpdateTime => {
                self.time = Clock::get_current_time();
            }

            Msg::StopClock => {
                if let Some(timer) = self.standalone.take() {
                    drop(timer);
                }
                self.is_running = false;
                console::log!("Stop Clock");
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
            <div id="clock">
                <div id="time" class="time">
                    { &self.time }
                </div>
                <div>
                    <button disabled={self.is_running} onclick={ctx.link().callback(|_| Msg::StartClock)} class="cancel-btn">
                        { "Start Clock" }
                    </button>
                    <button disabled={!self.is_running} onclick={ctx.link().callback(|_| Msg::StopClock)} class="cancel-btn">
                        { "Stop Clock" }
                    </button>
                </div>
            </div>
            <hr class="hr" />
            </>
        }
    }
}
