-- Get all the expenses in a room
-- params: $1 =  the room id

SELECT 
    expense.id,
    expense.paid_by,
    expense.amount,
    expense.title,
    json_group_array(user_expense.user_id) as participants,
    expense.room_id,
    expense.description,
    expense.created_at
FROM expense 
LEFT JOIN user_expense ON expense.id = user_expense.expense_id
WHERE room_id = $1
