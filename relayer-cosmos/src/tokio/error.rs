use flex_error::define_error;

define_error! {
    #[derive(Clone)]
    Error {
        ChannelClosed
            | _ | { "unexpected closure of internal rust channels" },
    }
}