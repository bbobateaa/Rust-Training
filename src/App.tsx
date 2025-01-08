import React, { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import './App.css'

interface TodoItem {
  item: string;
  completed: boolean;
}

const App: React.FC = () => {
  const [todos, setTodos] = useState<TodoItem[]>([]);
  const [newTodo, setNewTodo] = useState("");

  // Fetch todos from Rust backend
  useEffect(() => {
    invoke<TodoItem[]>("get_todos")
      .then((data) => setTodos(data))
      .catch((error) => console.error("Error fetching todos:", error));
  }, []);

  // Save todos to file
  async function saveTodos(todos: TodoItem[]) {
    try {
      // Convert todos array to a Map or any other format if needed
      const todosObject = todos.reduce((acc, todo) => {
        acc[todo.item] = todo.completed;
        return acc;
      }, {} as Record<string, boolean>);

      // Invoke the `save_todos` command in Rust
      await invoke("save_todos", { todos: todosObject });
      console.log("Todos saved successfully!");
      console.log("todos: ", todos)
    } catch (error) {
      console.error("Failed to save todos:", error);
    }
  }

  // Load todos from file
  async function loadTodos() {
    try {
      // Invoke the `load_todos` command in Rust
      const todos: TodoItem[] = await invoke("load_todos");
  
      if (todos.length === 0) {
        console.log("No todos found.");
        return;
      }
  
      setTodos(todos); // Update the state with the loaded todos
      console.log("Loaded todos:", todos);
    } catch (error) {
      console.error("Failed to load todos:", error);
    }
  }  

  // Add a new todo
  const addTodo = () => {
    if (newTodo.trim()) {
      invoke("add_todo", { item: newTodo })
        .then(() => {
          setTodos([...todos, { item: newTodo, completed: true }]);
          setNewTodo("");
        })
        .catch((error) => console.error("Error adding todo:", error));
    }
  };

  // Complete a todo
  const completeTodo = (item: string) => {
    invoke("complete_todo", { item })
      .then(() => {
        setTodos(
          todos.map((todo) =>
            todo.item === item ? { ...todo, completed: false } : todo
          )
        );
      })
      .catch((error) => console.error("Error completing todo:", error));
  };

  return (
    <div>
      <div className="container-1">
        <h1>Todo List</h1>
        <ul className="list-style">
          {todos.map((todo, index) => (
            <li key={index}>
              {todo.item} - {todo.completed ? "Active" : "Completed"}{" "}
              {todo.completed && (
                <button className="button-style-3" onClick={() => completeTodo(todo.item)}>Complete</button>
              )}
            </li>
          ))}
        </ul>
        <input
          className="search-style"
          type="text"
          value={newTodo}
          onChange={(e) => setNewTodo(e.target.value)}
          placeholder="Add a new todo"
        />
        <button className="button-style-1" onClick={addTodo}>Add Todo</button>
        <br />
        <div className="container-2">
          <button className="button-style-2" onClick={() => saveTodos(todos)}>Save Todos</button>
          <button className="button-style-2" onClick={loadTodos}>Load Todos</button>
        </div>
      </div>
    </div>
  );
};

export default App;
