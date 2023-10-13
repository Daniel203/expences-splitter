-- Get all the user that are subscribed to a room
-- params: $1 =  the room id

SELECT 
    user.id,
    user.username,
    '' as password,
    user.created_at
FROM user_room 
JOIN user ON user_room.user_id = user.id
WHERE room_id = $1
