import React from 'react';

function TodoList({ todos, onEdit, onDelete }) {
  return (
    <table className="table-auto w-full">
      <thead>
        <tr>
          <th className="px-4 py-2">Title</th>
          <th className="px-4 py-2">Content</th>
          <th className="px-4 py-2">Completed</th>
          <th className="px-4 py-2">Actions</th>
        </tr>
      </thead>
      <tbody>
        {todos.map(todo => (
          <tr key={todo.id}>
            <td className="border px-4 py-2">{todo.title}</td>
            <td className="border px-4 py-2">{todo.content}</td>
            <td className="border px-4 py-2">{todo.completed ? 'Yes' : 'No'}</td>
            <td className="border px-4 py-2">
              <button onClick={() => onEdit(todo)} className="bg-green-500 text-white px-2 py-1 rounded mr-2">Edit</button>
              <button onClick={() => onDelete(todo.id)} className="bg-red-500 text-white px-2 py-1 rounded">Delete</button>
            </td>
          </tr>
        ))}
      </tbody>
    </table>
  );
}

export default TodoList;