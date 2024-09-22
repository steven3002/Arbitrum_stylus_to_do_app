#![cfg_attr(not(any(feature = "export-abi", test)), no_main)]
// written by Steven Hert, omo this code needs to be formated after the competition

extern crate alloc;
use stylus_sdk::{ alloy_primitives::U8, prelude::*, stylus_proc::entrypoint };
use stylus_sdk::{ console, msg };

sol_storage! {
 #[entrypoint]
 pub struct MetaDate{
    mapping(address => MyTodo) my_todo;
 }

 pub struct MyTodo{
    uint8 current_id;
    Todo[] todo_list;
 }

 pub struct Todo{
    uint8 id;
    bool completed;
    string todo;
 }
}

#[public]
impl MetaDate {
    pub fn get_user_todo(&self) -> Vec<String> {
        let mut list_d: Vec<String> = vec![];
        let sender_address = msg::sender();
        let todo = self.my_todo.get(sender_address);

        for index in 0..todo.todo_list.len() {
            let list_todo = todo.todo_list.get(index).unwrap();
            list_d.push(
                format!(
                    r#"{{"index":{},"status":{},"todo":"{}"}}"#,
                    list_todo.id.get(),
                    list_todo.completed.get(),
                    list_todo.todo.get_string()
                )
            );
        }
        return list_d;
    }

    pub fn add_todo(&mut self, todo: String) {
        let mut user = self.my_todo.setter(msg::sender());
        let user_todo_id = user.current_id.get();
        let mut users_todo_list = user.todo_list.grow();
        // let todo_test = format!("user {}: todo: {}", msg::sender(), todo);
        users_todo_list.id.set(user_todo_id);
        users_todo_list.completed.set(false);
        users_todo_list.todo.set_str(todo);
        let new_id = user_todo_id + U8::from(1);

        user.current_id.set(new_id);
    }

    pub fn mark_completed(&mut self, todo_index: u8) {
        let mut user = self.my_todo.setter(msg::sender());
        // let mut todo_list = user.todo_list.get();
        for index in 0..user.todo_list.len() {
            let mut todo = user.todo_list.get_mut(index).unwrap();
            if U8::from(todo_index) == todo.id.get() {
                todo.completed.set(true);
                return;
            }
        }
    }
    pub fn delete_todo(&mut self, todo_id: u8) {
        let mut user = self.my_todo.setter(msg::sender());
        let len = user.todo_list.len();

        let mut index_to_remove = None;
        for i in 0..len {
            let todo_item = user.todo_list.get(i).expect("Item exists");
            if todo_item.id.get() == U8::from(todo_id) {
                index_to_remove = Some(i);
                break;
            }
        }

        if let Some(index) = index_to_remove {
            for i in index..len - 1 {
                let next_id = user.todo_list
                    .get(i + 1)
                    .expect("Item exists")
                    .id.get();
                let next_completed = user.todo_list
                    .get(i + 1)
                    .expect("Item exists")
                    .completed.get();
                let next_todo = user.todo_list
                    .get(i + 1)
                    .expect("Item exists")
                    .todo.get_string();

                let mut old_item = user.todo_list.setter(i).expect("Setter exists");
                old_item.id.set(next_id);
                old_item.completed.set(next_completed);
                old_item.todo.set_str(&next_todo);
            }

            user.todo_list.truncate(len - 1);
        }
    }
}
