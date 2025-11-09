use dialoguer::{Input, Select, theme::ColorfulTheme};

fn main() {
    let mut todos: Vec<String> = Vec::new();

    loop {
        let options = &["add", "view", "remove"];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose your Action")
            .default(0)
            .items(&options[..])
            .interact()
            .unwrap();

        let action = options[selection];

        if action == "add" {
            println!();
            let todo: String = Input::new()
                .with_prompt("Input new Todo")
                .interact_text()
                .unwrap();

            todos.push(todo)
        }

        if action == "view" {
            todos.iter().for_each(|todo| println!("{}", todo));
        }

        if action == "remove" {
            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Choose your Action")
                .default(0)
                .items(&todos[..])
                .interact()
                .unwrap();

            todos.remove(selection);
        }
    }
}
