import React, { useState, useEffect } from 'react';

function TodoDialog({ todo, onSave, onClose }) {
  const [formData, setFormData] = useState({
    title: '',
    content: '',
    completed: false,
    content_type: '',
    is_public: false,
    food_orange: false,
    food_apple: false,
    food_banana: false,
    food_melon: false,
    food_grape: false,
    pub_date1: null,
    pub_date2: null,
    pub_date3: null,
    pub_date4: null,
    pub_date5: null,
    pub_date6: null,
    qty1: '',
    qty2: '',
    qty3: '',
    qty4: '',
    qty5: '',
    qty6: '',
  });

  useEffect(() => {
    if (todo) {
      setFormData(todo);
    }
  }, [todo]);

  const handleChange = (e) => {
    const { name, value, type, checked } = e.target;
    setFormData(prev => ({ ...prev, [name]: type === 'checkbox' ? checked : value }));
  };

  const handleSubmit = (e) => {
    e.preventDefault();
    onSave(formData);
  };

  return (
    <div className="fixed inset-0 bg-gray-600 bg-opacity-50 overflow-y-auto h-full w-full">
      <div className="relative top-20 mx-auto p-5 border w-1/2 shadow-lg rounded-md bg-white">
        <form onSubmit={handleSubmit}>
          <h2 className="text-xl mb-4">{todo ? 'Edit' : 'Add'} Todo</h2>
          <div className="grid grid-cols-2 gap-4">
            <div>
              <label>Title</label>
              <input type="text" name="title" value={formData.title} onChange={handleChange} className="w-full p-2 border" />
            </div>
            <div>
              <label>Content</label>
              <input type="text" name="content" value={formData.content} onChange={handleChange} className="w-full p-2 border" />
            </div>
            <div>
              <label>Content Type</label>
              <input type="text" name="content_type" value={formData.content_type} onChange={handleChange} className="w-full p-2 border" />
            </div>
            <div className="flex items-center">
              <input type="checkbox" name="completed" checked={formData.completed} onChange={handleChange} className="mr-2" />
              <label>Completed</label>
            </div>
            <div className="flex items-center">
              <input type="radio" name="is_public" value="true" checked={formData.is_public === true} onChange={() => setFormData(prev => ({...prev, is_public: true}))} className="mr-2" />
              <label className="mr-4">Public</label>
              <input type="radio" name="is_public" value="false" checked={formData.is_public === false} onChange={() => setFormData(prev => ({...prev, is_public: false}))} className="mr-2" />
              <label>Private</label>
            </div>
            <div>
              <label>Foods</label>
              <div className="flex flex-wrap">
                <div className="flex items-center mr-4"><input type="checkbox" name="food_orange" checked={formData.food_orange} onChange={handleChange} className="mr-2" /> Orange</div>
                <div className="flex items-center mr-4"><input type="checkbox" name="food_apple" checked={formData.food_apple} onChange={handleChange} className="mr-2" /> Apple</div>
                <div className="flex items-center mr-4"><input type="checkbox" name="food_banana" checked={formData.food_banana} onChange={handleChange} className="mr-2" /> Banana</div>
                <div className="flex items-center mr-4"><input type="checkbox" name="food_melon" checked={formData.food_melon} onChange={handleChange} className="mr-2" /> Melon</div>
                <div className="flex items-center"><input type="checkbox" name="food_grape" checked={formData.food_grape} onChange={handleChange} className="mr-2" /> Grape</div>
              </div>
            </div>
            <div><label>Date 1</label><input type="date" name="pub_date1" value={formData.pub_date1} onChange={handleChange} className="w-full p-2 border" /></div>
            <div><label>Qty 1</label><input type="text" name="qty1" value={formData.qty1} onChange={handleChange} className="w-full p-2 border" /></div>
            <div><label>Date 2</label><input type="date" name="pub_date2" value={formData.pub_date2} onChange={handleChange} className="w-full p-2 border" /></div>
            <div><label>Qty 2</label><input type="text" name="qty2" value={formData.qty2} onChange={handleChange} className="w-full p-2 border" /></div>
            <div><label>Date 3</label><input type="date" name="pub_date3" value={formData.pub_date3} onChange={handleChange} className="w-full p-2 border" /></div>
            <div><label>Qty 3</label><input type="text" name="qty3" value={formData.qty3} onChange={handleChange} className="w-full p-2 border" /></div>
            <div><label>Date 4</label><input type="date" name="pub_date4" value={formData.pub_date4} onChange={handleChange} className="w-full p-2 border" /></div>
            <div><label>Qty 4</label><input type="text" name="qty4" value={formData.qty4} onChange={handleChange} className="w-full p-2 border" /></div>
            <div><label>Date 5</label><input type="date" name="pub_date5" value={formData.pub_date5} onChange={handleChange} className="w-full p-2 border" /></div>
            <div><label>Qty 5</label><input type="text" name="qty5" value={formData.qty5} onChange={handleChange} className="w-full p-2 border" /></div>
            <div><label>Date 6</label><input type="date" name="pub_date6" value={formData.pub_date6} onChange={handleChange} className="w-full p-2 border" /></div>
            <div><label>Qty 6</label><input type="text" name="qty6" value={formData.qty6} onChange={handleChange} className="w-full p-2 border" /></div>
          </div>
          <div className="flex justify-end mt-4">
            <button type="button" onClick={onClose} className="bg-gray-500 text-white px-4 py-2 rounded mr-2">Cancel</button>
            <button type="submit" className="bg-blue-500 text-white px-4 py-2 rounded">Save</button>
          </div>
        </form>
      </div>
    </div>
  );
}

export default TodoDialog;