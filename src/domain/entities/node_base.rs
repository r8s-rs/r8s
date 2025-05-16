pub trait NodeBase {
    fn get_type(&self) -> &'static str;

    fn is_trigger(&self) -> bool;
}
