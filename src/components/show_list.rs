use yew::prelude::*;
use crate::data::Show;

pub struct ShowList {
    link: ComponentLink<Self>,
    shows: Vec<Show>,
}

#[derive(Clone, Properties)]
pub struct ShowListProps {
    pub shows: Vec<Show>,
}

impl Component for ShowList {
    type Message = ();
    type Properties = ShowListProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, shows: props.shows }
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
        let render_show = |i: &Show| -> Html {
            html! {
                <li>{ format!("{} ({})", &i.name, &i.tmdb_id) }</li>
            }
        };
        html! {
            <div class="w-1/2">
                <h1 class="text-lg">{ "Shows" }</h1>
                <hr class="my-2 mr-4"/>
                <ul class="shows">
                    { self.shows.iter().map(render_show).collect::<Html>() }
                </ul>
            </div>
        }

    }
}
