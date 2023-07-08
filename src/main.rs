use gloo::console;
use std::mem;
use yew::prelude::*;
mod bits;
use bits::Bits;
use itertools::Itertools;
use web_sys::HtmlInputElement;

enum Msg {
    FlipBit(usize),
    SetDataFromStrRadix(String, u32),
}

struct App {
    pub bits: Bits,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { bits: Bits::new() }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FlipBit(index) => {
                self.bits.flip_bit(index);
                true
            }
            Msg::SetDataFromStrRadix(input, radix) => {
                let src = input
                    .strip_prefix("0x")
                    .unwrap_or(input.strip_prefix("0b").unwrap_or(&input));
                self.bits.data = match src {
                    "" => 0,
                    src => match bits::DataType::from_str_radix(src, radix) {
                        Ok(parsed) => parsed,
                        Err(_) => self.bits.data,
                    },
                };
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <div class="bytes">
                    { (0..mem::size_of::<bits::DataType>() * 8).rev().map(|index| {
                        let bit_value = self.bits.get_bit(index);
                        let button_class = if bit_value { "button-on" } else { "button-off" };
                        html! {
                            <div class="button-item">
                                <label>{ format!("{:02}", index) }</label>
                                <button class={button_class} onclick={ctx.link().callback(move |_| Msg::FlipBit(index))}>
                                    { bit_value as i32 }
                                </button>
                            </div>
                        }
                    }).chunks(8).into_iter().map(|html_vec| {
                        html!{
                            <div class="byte">
                                {html_vec.collect::<Html>() }
                            </div>
                        }
                    }).chunks(2).into_iter().map(|html_vec| {
                        html!{
                            <div class="bytes">
                                {html_vec.collect::<Html>() }
                            </div>
                        }
                    })
                    .collect::<Html>() }
                </div>
                <div class="output-container">
                <div class="output-item">
                    <span>{"Binary: "}</span>
                    <input
                        type="text"
                        value={self.bits.to_radix_string(2)}
                        oninput={ctx.link().callback(|e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            Msg::SetDataFromStrRadix(input.value(), 2)
                        })}
                    />
                </div>
                <div class="output-item">
                    <span>{"Hex: "}</span>
                    <input
                        type="text"
                        value={self.bits.to_radix_string(16)}
                        oninput={ctx.link().callback(|e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            Msg::SetDataFromStrRadix(input.value(), 16)
                        })}
                    />
                </div>
                <div class="output-item">
                    <span>{"Decimal: "}</span>
                    <input
                        type="text"
                        value={self.bits.to_radix_string(10)}
                        oninput={ctx.link().callback(|e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            Msg::SetDataFromStrRadix(input.value(), 10)
                        })}
                    />
                </div>
            </div>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
