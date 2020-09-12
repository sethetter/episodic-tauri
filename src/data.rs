#[derive(Clone)]
pub struct Show {
    pub tmdb_id: u32,
    pub name: String,
}

#[derive(Clone)]
pub struct WatchItem {
    pub name: String,
    pub show: Option<Show>,
    pub season: Option<u32>,
}
