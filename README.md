# Todo List App

This is a simple console-based Todo List App written in Rust.

## About

This project serves as an introductory Rust application, demonstrating basic Rust concepts and functionalities such as:

- Structs and methods (`struct Todo` and associated methods like `new`, `add_task`, `mark_complete`, `delete_task`, and `print_tasks`).
- Error handling using `Result` and `expect`.
- Handling user input/output using `std::io`.
- Looping and branching with `loop` and `match` statements.
- Parsing input and performing actions based on user choices.
- String manipulation (`format`, `trim`, `to_string`).
- 
## Usage

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/todo-list.git
   ```

2. Navigate into the project directory:
   ```bash
   cd todo-list
   ```

3. Build and run the application:
   ```bash
   cargo run
   ```

4. Follow the on-screen instructions to interact with the Todo List:
   - To add a task, enter `1`, followed by the task description.
   - To mark a task as complete, enter `2`, followed by the task number.
   - To delete a task, enter `3`, followed by the task number.
   - To exit the application, enter `4`.

## Features

- Add tasks to the list.
- Mark tasks as complete.
- Delete tasks from the list.
- Simple and intuitive user interface.

## Dependencies

This application uses the standard library `std::io` for user input/output.

## Code Structure

- `main.rs`: Contains the main logic of the Todo List App.
- `README.md`: Documentation for the Todo List App.

## Contributing

Contributions are welcome! If you find any issues or have suggestions for improvements, feel free to open an issue or submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
