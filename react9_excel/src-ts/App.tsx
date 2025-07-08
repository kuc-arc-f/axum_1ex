import React, { useState, useEffect } from 'react';
import TodoList from './components/TodoList';
import TodoDialog from './components/TodoDialog';
import { itemsApi } from './client/api/items';

function App() {
  const [items, setItems] = useState<Item[]>([]);
  const [todos, setTodos] = useState([]);
  const [isDialogOpen, setIsDialogOpen] = useState(false);
  const [editingTodo, setEditingTodo] = useState(null);
  const [error, setError] = useState<string | null>(null);

  // アイテム一覧を取得
  const fetchItems = async () => {
    try {
      const data = await itemsApi.getAll();
      console.log(data);
      setItems(data);
      setTodos(data)
    } catch (err) {
      setError('アイテムの取得に失敗しました');
    } finally {
    }
  };

  useEffect(() => {
    fetchItems();
  }, []);

  const handleAdd = () => {
    setEditingTodo(null);
    setIsDialogOpen(true);
  };

  const handleEdit = (todo) => {
    setEditingTodo(todo);
    setIsDialogOpen(true);
  };

  const handleDelete = async(id) => {
    await itemsApi.delete(id)
    fetchItems();
    //setTodos(todos.filter(todo => todo.id !== id));
  };

  const handleSave = async (todo) => {
    console.log(todo);
    if (todo.id) {
      await itemsApi.update(todo.id , todo);
    } else {
      await itemsApi.create(todo);
    }
    setIsDialogOpen(false);
    await fetchItems();
  };

  const downloadTask = () =>{
    location.href = "/download"
  }

  return (
    <div className="container mx-auto p-4">
      <h1 className="text-2xl font-bold mb-4">Todo App</h1>
      <div class="flex flex-row">
        <div class="flex-1 text-start text-white m-1">
          <button
            onClick={handleAdd}
            className="bg-blue-500 text-white px-4 py-2 rounded mb-4"
          >
            Add Todo
          </button>
        </div>
        <div class="flex-1 text-center text-white m-1">2
          <button
          onClick={()=> downloadTask()}
          className="bg-blue-500 text-white px-4 py-2 rounded mb-4">
            downLoad
          </button>
        </div>
      </div>
      <TodoList todos={todos} onEdit={handleEdit} onDelete={handleDelete} />
      {isDialogOpen && (
        <TodoDialog
          todo={editingTodo}
          onSave={handleSave}
          onClose={() => setIsDialogOpen(false)}
        />
      )}
    </div>
  );
}

export default App;
