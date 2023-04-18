## To-Do list
- [x] Test the different ways errors propagate using anyhow and thiserror
- [x] Decide between the thiserror and anyhow approach → chose anyhow
- [ ] Refactor error handling according to the previous task
- [ ] Document existing functions according to the documentation guidelines
- [ ] Redesign commands for an MVP to make the app simpler and more minimalistic (in the future I can decide more features, but, for now, minimalistic is the way to go)
- [ ] Implement the "create" command

## List Groups
- **no_group** (group of lists with no group)
- **all** (all lists)
- **active** (list group that is currently being used)
- **custom_group_name** (all lists in a custom group created by the user)

## Commands 
### create
- **create** *list_name*: creates a new list in active list group
- **create** *list_name* *group_name*: creates a new list in a specific group using group name
- **create** *list_name* **--number** *group_number*: creates a new list in a specific group 
- **create group** *group**: creates a new group

### show
- **show**: shows all items in active list
- **show --name**: shows only the name of the active list
- **show group**: shows the name of all lists in active group
- **show group --items**: shows the name and items of all lists in a group

### set
- **set** **--number** *list_number*: set active list by number (when moving to a new group first list in the group is automatically set)
- **set** *list_name* : set active list (when moving to a new group first list in the group is automatically set)

### push
- **push** *item*: add item to the end of the active list
- **push** *item* **--number** *list_number*: add item to the end of a list (list must belong to the active group)
- **push** *item* *list_name*: add item to the end of a list (list must belong to the active group)

### pop
- **pop** *item*: delete item from the end of the active list
- **pop** *item* *list_name*: delete item from the end of a list (list must belong to the active group)
- **pop** *item* **--number** *list_number*: delete item from the end of a list (list must belong to the active group)

### add
- **add** *item* *position*: add item to a specified position off the active list
- **add** *item* *position* *list_name*: add item to a specified position of a specified list (list must belong to the active group)
- **add** *item* *position* **--number** *list_number*: add item to a specified position of a specified list (list must belong to the active group)

### del
- **del** *position*: delete item in a specified position of the active list
- **del** *position* *list_name*: delete item to a specified position in a specified list (list must belong to the active group)
- **del** *position* **--number** *list_number*: delete item to a specified position in a specified list (list must belong to the active group)

### swap
- **swap** *item_name* *item_name*: switch places of two items in active list
- **swap** **--number** *item_number* *item_number*: switch places of two items in active list
- **swap** *item_name* *item_name* *list_name*: switch places of two items in a specified list (list must belong to the active group)
- **swap** **--number** *item_number* *item_number* *list_number*: switch places of two items in a specified list (list must belong to the active group)

### reorder
- **reorder** *destination* *item_name_array*: move one or more items to a specified position in active list (with multiple item you only choose the destination of the first item, the others follow the sequence specified in the command)
- **reorder** *destination* **--number** *item_number_array*: move one or more items to a specified position in active list (with multiple item you only choose the destination of the first item, the others follow the sequence specified in the command)

### enter
- **enter** *group_name*: enter a list group
- **enter** **--number** *group_number*: enter a list group

### leave
- **leave**: leave group (go to *no group*)

### take
- **take** *list_name*: take list from the active group to *no group*
- **take** **--number** *list_name*: take list from the active group to *no group*
- **take** *list_name* *group_name*: take list from a specified group to *no group*
- **take** **--number** *list_number* *group_number*: take list from a specified group to *no group*

### put
- **put** *list_name* *group_name*: put list from *no group* in a group
- **put** **--number** *list_number* *group_number*: put list from *no group* in a group

### takeput
- **takeput** *list_name* *group_name*: take list from a group and put it in a specified group
- **takeput** **--number** *list_number* *group_number*: take list from a group and put it in a specified group

### group
- **group** **list_name_array**: group lists
- **group** **--number** **list_number_array**: group lists using list numbers

### ungroup
- **ungroup**: ungroup active list group
- **ungroup** *group_name*: ungroup specified list group
- **ungroup** **--number** *group_number*: ungroup a specified list group using the group number


## Documentation Guidelines
### Functions
- Short Description
- Return Values
    - Ok()
    - Err()
- Panic Scenarios
- Examples

