I've added CRUD functionality for tasks with permission validation:
- **Create Task (`POST /task`)**: Checks if the user has `"create_task"` permission.
- **Edit Task (`PUT /task/:id`)**: Requires `"edit_task"` permission.
- **Delete Task (`DELETE /task/:id/:user_id`)**: Requires `"delete_task"` permission.

Let me know if you need additional refinements!
