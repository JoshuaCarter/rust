use super::{message_stream::*, trading::*, market::*, health::*};

impl From<NewOrderCall> for MessageStreamCall {
    fn from(x: NewOrderCall) -> Self {
        return Self { new: Some(x), ..Self::default() };
    }
}
impl From<NewOrderReply> for MessageStreamReply {
    fn from(x: NewOrderReply) -> Self {
        return Self { new: Some(x), ..Self::default() };
    }
}
impl From<CxlOrderCall> for MessageStreamCall {
    fn from(x: CxlOrderCall) -> Self {
        return Self { cxl: Some(x), ..Self::default() };
    }
}
impl From<CxlOrderReply> for MessageStreamReply {
    fn from(x: CxlOrderReply) -> Self {
        return Self { cxl: Some(x), ..Self::default() };
    }
}
impl From<BookUpdatesCall> for MessageStreamCall {
    fn from(x: BookUpdatesCall) -> Self {
        return Self { book: Some(x), ..Self::default() };
    }
}
impl From<BookUpdatesReply> for MessageStreamReply {
    fn from(x: BookUpdatesReply) -> Self {
        return Self { book: Some(x), ..Self::default() };
    }
}
impl From<PingMessage> for MessageStreamCall {
    fn from(x: PingMessage) -> Self {
        return Self { ping: Some(x), ..Self::default() };
    }
}
impl From<PingMessage> for MessageStreamReply {
    fn from(x: PingMessage) -> Self {
        return Self { ping: Some(x), ..Self::default() };
    }
}
