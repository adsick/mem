pub struct Read {
    link: String,

    status: ReadStatus

    // priority?
}

pub enum ReadStatus{
    Todo,
    Read
}