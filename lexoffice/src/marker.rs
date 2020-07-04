/// Marks a field as readonly. The user is not allowed to set
/// this field, but it may be sent by the server.
pub type ReadOnly<T> = Option<T>;
