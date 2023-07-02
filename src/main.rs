use yew::prelude::*;
mod bits;
use bits::Bits;

enum Msg {
    FlipBit(usize),
    SetSize(usize),
}

struct App {
    pub bits: Bits,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            bits: Bits::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FlipBit(index) => {
                self.bits.flip_bit(index);
                true
            }
            Msg::SetSize(size) => {
                self.bits.set_size(size);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <div class="button-container">
                    { (0..self.bits.size).rev().map(|index| {
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
                    }).collect::<Html>() }
                </div>
                <div class="output-container">
                    <div class="output-item">
                        <span>{"Binary:"}</span>
                        <p>{self.bits.to_bit_string()}</p>
                    </div>
                    <div class="output-item">
                        <span>{"Hex:   "}</span>
                        <p>{self.bits.to_hex_string()}</p>
                    </div>
                    <div class="output-item">
                        <span>{"Decimal:"}</span>
                        <p>{self.bits.to_dec_string()}</p>
                    </div>
                </div>
            </div>
        }
    }
    
}

fn main() {
    yew::start_app::<App>();
}