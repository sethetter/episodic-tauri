use yew::prelude::*;
use crate::{
    components::{
        show_list::ShowList,
        watch_list::WatchList,
    },
    data::{Show, WatchItem},
};

pub struct App {
    link: ComponentLink<Self>,
    value: i64,
    shows: Vec<Show>,
    watch_list: Vec<WatchItem>,
}

pub enum Msg {
    AddOne,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let show = Show { tmdb_id: 0, name: "Righteous Gemstones".into() };

        let shows = vec![show.clone()];
        let watch_list = vec![
            WatchItem {
                name: "Righteous Gemstones (S01E01)".into(),
                show: Some(show.clone()),
                season: Some(1),
            },
        ];

        Self { link, value: 0, shows, watch_list }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.value += 1
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
                <ShowList shows={&self.shows} />
                <WatchList items={&self.watch_list} />
            </div>
        }
    }
}
