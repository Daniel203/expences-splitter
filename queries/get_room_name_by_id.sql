-- return the room name from the id
-- input: $1 = the room id

SELECT 
    room.room_name
FROM room
WHERE room.id = $1
