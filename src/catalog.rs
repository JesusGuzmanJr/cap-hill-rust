use crate::DateTime;

pub enum Status {
    CheckedOut { until: DateTime },
    OnHold { until: DateTime },
    Available,
}

pub struct Entry {
    title: String,
    authors: Vec<String>,
    publisher: String,
    published: DateTime,
    status: Status,
    summary: String,
}
