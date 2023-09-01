use super::model::Movement;

pub fn register_movement(movement: Movement) -> String {
    return format!("New movement registered: {:?}", movement)
}
