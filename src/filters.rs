#[derive(PartialEq, Debug)]
pub struct Filters {
    pub intel: bool,
    pub demangle: bool,
    pub directives: bool,
    pub comments: bool,
    pub labels: bool
}
