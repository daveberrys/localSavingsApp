use blenderui::prelude::*;

struct Hello;

mod handler;
use handler::read_css_file;
use handler::savings_reader;

impl Render for Hello {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let savings = savings_reader::read_savings();

        rsx! {
            <style>{read_css_file::read("main.css")}</style>
            <body>
                <div class="column">
                    <span class="big">"Local Savings"</span>
                    <span>"The money in your account, that's what it's worth!"</span>

                    <hr styled />

                    <span>"Your current savings are; " {format!("${}", savings)}</span>

                    <Savings />
                </div>
            </body>
        }
    }
}


#[component]
fn Savings() {
    let (text, set_text) = use_state(String::new());

    rsx! {
        <TextField
            value={text()}
            onInput={move |e: &ChangeEvent| set_text(e.value.clone())}
        />

        <div class="row" style="margin-top: 8px; gap: 4px;">
            <Button
                onClick={move |_| {
                    savings_reader::adjust_savings(text().trim().parse::<f64>().unwrap_or(0.0) * -1.0);
                }}
            >
                "Decrease"
            </Button>
            <Button
                onClick={move |_| {
                    savings_reader::adjust_savings(text().trim().parse::<f64>().unwrap_or(0.0) * 1.0);
                }}
            >
                "Increase"
            </Button>
        </div>
    }
}


fn main() {
    Application::new().run(|cx| {
        cx.open_window(WindowOptions::default(), |_, cx| cx.new(|_| Hello)).unwrap();
    });
}
