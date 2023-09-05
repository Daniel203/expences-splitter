#[derive(Debug, Clone)]
pub struct Room {
    pub id: i64,
    pub room_name: String,
    pub max_participants: i64,
    pub owner: i64,
}
