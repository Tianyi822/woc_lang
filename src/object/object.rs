pub enum OBJType {
    NullObj,
    IntegerObj,
    BooleanObj,
}

/// This trait like the Object in Java, it has two methods:
/// - obj_type() -> ObjectType: return the type of the object
/// - inspect() -> String: return the string representation of the object
pub trait Object {
    fn obj_type() -> OBJType;
    fn inspect(&self) -> String;
}
