use leptos::ev::{SubmitEvent, MouseEvent};
use leptos::html::Input;
use leptos::{component, IntoView};
use leptos::*;


fn main() {
    leptos::mount_to_body(|cx| view!{ cx, <App/> })
    
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (toggled, set_toggle) = create_signal(cx, false);

    provide_context(cx, set_toggle);

    view! {cx, 
        <div>
            <p>"Toggled? " {toggled}</p>
            <ButtonD />

            <ControlledInput />
            <UncontrolledInput />
            <NumbericInput />


        </div>
    }
}

#[component]
fn ButtonD(
    cx: Scope
) -> impl IntoView 
{
    // use_context searches up the context tree, hoping to
    // find a 'WriteSignal<bool>'
    // in this case, I .expect() because I know I provided it
    let setter = use_context::<WriteSignal<bool>>(cx)
        .expect("to have found the setter provided");

    view! { cx, 
        <button
            on:click=move |_| setter.update(|value| *value = !*value)
        >
            "Toggle"
        </button> 
    }
}

#[component]
fn ButtonA<F>(
    cx: Scope,
    on_click: F,
) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static 
{
    view! { cx,
        <button
            on:click=on_click
        >
            "Toggle"
        </button>
    }
}

#[component]
fn ProgressBar<F>(
    cx: Scope,
    #[prop(default = 100)]
    max: u16,
    progress: F
) -> impl IntoView
where 
    F: Fn() -> i32 + 'static
{
    view! { cx, 
        <progress 
            max=max
            value = progress
        />
    }
}

#[component]
fn ControlledInput (
    cx: Scope
) -> impl IntoView {
    let (name, set_name) = create_signal(cx, "Controlled".to_string());

    view! {cx,
        <input type="text"
            on:input=move |ev| {
                set_name(event_target_value(&ev));
           }
           prop:value=name
        />
        <p>"Name is: "{name}</p>
    }
}

#[component]
fn UncontrolledInput (
    cx: Scope
) -> impl IntoView {
    let (name, set_name) = create_signal(cx, "Uncontrolled".to_string());
    let input_element: NodeRef<Input> = create_node_ref(cx);

    let on_submit = move |ev: SubmitEvent| {
        // stop the page from reloading
        ev.prevent_default();

        //extract value from input
        let value = input_element()
            // event handlers can only fire after the view 
            // is mounted to the DOM, so the 'NodeRef' will be 'Some'
            .expect("<input> to exist")
            // 'NodeRef' implements 'Deref' for the DOM element type
            // this means we can call 'HtmlInputElement::value()'
            // to get the current value of the input
            .value();
        set_name(value);
    };

    view! { cx,
        <form on:submit=on_submit>
            <input type="text"
                value=name
                node_ref=input_element
            />
            <input type="submit" value="Submit"/>
        </form>
        <p>"Name is: " {name}</p>
    }
}

#[component]
fn ControlFlow (
    cx: Scope
) -> impl IntoView {
    let (value, set_value) = create_signal(cx, 0);
    let is_odd = move || value() & 1 == 1;
    let message = move || is_odd().then(|| "Ding ding ding!");

    view! { cx,
        <p>
        {message}
        </p>
    }
}

#[component]
fn NumbericInput(cx: Scope) -> impl IntoView {
    let (value, set_value) = create_signal(cx, Ok(0));

    // when input changes, try to parse a number from the input
    let on_input = move |ev| set_value(event_target_value(&ev).parse::<i32>());

    view! { cx,
        <label>
            "Type a number (or not!)"
            <input type="number" on:input=on_input/>
            <ErrorBoundary
                // the fallback receives a signal containing current errors
                fallback=|cx, errors| view! { cx,
                    <div class="error">
                        <p>"Not a number! Errors: "</p>
                        // we can render a list of errors as strings, if we'd like
                        <ul>
                            {move || errors.get()
                                .into_iter()
                                .map(|(_, e)| view! { cx, <li>{e.to_string()}</li>})
                                .collect_view(cx)
                            }
                        </ul>
                    </div>
                }
            >
            <p>
                "You entered "
                <strong>{value}</strong>
            </p>
            </ErrorBoundary>
        </label>
    }
}