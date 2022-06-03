use yew::prelude::*;

struct Model {
	value: i32,
}

#[function_component(App)]
fn app() -> Html {
	let state = use_state(|| Model { value: 0 });
	let inc = {
		let state = state.clone();

		// This is a closure that takes a reference to the state
		Callback::from(move |_| {
			state.set(Model {
				value: state.value + 1,
			})
		})
	};

	html! {
			<div>
				<button {inc}>{"Increment"} </button>
				<p> {"Count: "} {state.value} </p>
			</div>
	}
}

fn main() {
	yew::start_app::<App>();
}
