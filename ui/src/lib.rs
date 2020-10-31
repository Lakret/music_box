use wasm_bindgen::prelude::*;
use web_sys::Window;
use yew::prelude::*;

struct Model {
  link: ComponentLink<Self>,
  value: i64,
}

enum Msg {
  AddOne,
  GetState,
}

#[wasm_bindgen(inline_js = "export function send_init() { return external.invoke('init'); }")]
extern "C" {
  fn send_init() -> String;
}

impl Component for Model {
  type Message = Msg;
  type Properties = ();
  fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
    Self { link, value: 0 }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::AddOne => self.value += 1,
      Msg::GetState => {
        unsafe { send_init() };
        // TODO: synchronize state between frontend and backend
      }
    }
    true
  }

  fn change(&mut self, _props: Self::Properties) -> ShouldRender {
    // Should only return "true" if new properties are different to
    // previously received properties.
    // This component has no properties so we will always return "false".
    false
  }

  fn view(&self) -> Html {
    html! {
        <div>
            <button onclick=self.link.callback(|_| Msg::GetState)>{ "Get state" }</button>
            <button onclick=self.link.callback(|_| Msg::AddOne)>{ "Minus one" }</button>
            <p>{ self.value }</p>
        </div>
    }
  }
}

#[wasm_bindgen(start)]
pub fn run_app() {
  App::<Model>::new().mount_to_body();
}
