// src/pages/footer.rs

use yew::prelude::*;

struct State {}

pub struct Footer {
    state: State,
}

pub enum Msg {}

impl Component for Footer {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { state: State {} }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {

        html! {<div>
            <div class="footer">
                <table>
                <tr>
                <td width="50%">
                {"Text\n
                idk\n
                This is more Text"}
                </td>
                <td width="50%">
                {"Other Text
                Wow, such information
                So much text.
                "}
                </td>
              </tr>
                </table>
            </div>
        </div>}
    }
}