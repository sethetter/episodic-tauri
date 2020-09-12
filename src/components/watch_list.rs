use yew::prelude::*;
use crate::data::WatchItem;

pub struct WatchList {
    link: ComponentLink<Self>,
    items: Vec<WatchItem>,
}

#[derive(Clone, Properties)]
pub struct WatchListProps {
    pub items: Vec<WatchItem>,
}

impl Component for WatchList {
    type Message = ();
    type Properties = WatchListProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, items: props.items }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        let render_item = |i: &WatchItem| -> Html {
            html! {
                <li>{ format!("{}", &i.name) }</li>
            }
        };
        html! {
            <div>
                <ul class="watch-list">
                    { self.items.iter().map(render_item).collect::<Html>() }
                </ul>
            </div>
        }

    }
}
