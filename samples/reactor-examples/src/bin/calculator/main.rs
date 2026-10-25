#![windows_subsystem = "windows"]

use islands_reactor::*;
use islands_reactor_examples::run;

fn main() -> Result<()> {
    run("Calculator", app)
}

fn app(cx: &mut RenderCx) -> Element {
    let (display, set_display) = cx.use_state(String::from("0"));
    let set_for_numbers = set_display.clone();
    let current_for_numbers = display.clone();
    let number = move |digit: &'static str| {
        let set_display = set_for_numbers.clone();
        let current = current_for_numbers.clone();
        button(digit).on_click(move || {
            let next = if current == "0" {
                digit.to_owned()
            } else {
                format!("{current}{digit}")
            };
            set_display.call(next);
        })
    };
    let set_for_operators = set_display.clone();
    let current_for_operators = display.clone();
    let operator = move |op: &'static str| {
        let set_display = set_for_operators.clone();
        let current = current_for_operators.clone();
        button(op).on_click(move || {
            let next = if current.ends_with(['+', '-', '*', '/']) {
                format!("{}{}", &current[..current.len() - 1], op)
            } else {
                format!("{current}{op}")
            };
            set_display.call(next);
        })
    };
    let set_equals = set_display.clone();
    let expression = display.clone();
    let set_clear = set_display;

    border(
        vstack((
            text_block(display).font_size(28.0).bold(),
            hstack((number("7"), number("8"), number("9"), operator("/"))).spacing(4.0),
            hstack((number("4"), number("5"), number("6"), operator("*"))).spacing(4.0),
            hstack((number("1"), number("2"), number("3"), operator("-"))).spacing(4.0),
            hstack((
                number("0"),
                button("C").on_click(move || set_clear.call(String::from("0"))),
                button("=").accent().on_click(move || {
                    if let Some(value) = evaluate(&expression) {
                        set_equals.call(value.to_string());
                    }
                }),
                operator("+"),
            ))
            .spacing(4.0),
        ))
        .spacing(8.0)
        .width(280.0),
    )
    .padding(16.0)
    .into()
}

fn evaluate(expression: &str) -> Option<f64> {
    for (index, op) in expression.char_indices().skip(1) {
        if matches!(op, '+' | '-' | '*' | '/') {
            let left = expression[..index].parse::<f64>().ok()?;
            let right = expression[index + op.len_utf8()..].parse::<f64>().ok()?;
            return match op {
                '+' => Some(left + right),
                '-' => Some(left - right),
                '*' => Some(left * right),
                '/' if right != 0.0 => Some(left / right),
                _ => None,
            };
        }
    }
    None
}
