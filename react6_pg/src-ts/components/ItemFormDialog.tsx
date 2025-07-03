import React, { useState, useEffect, useRef } from 'react';

const initialFormData = {
  title: '',
  content: '',
  content_type: '',
  public_type: '公開',
  food_orange: false,
  food_apple: false,
  food_banana: false,
  food_melon: false,
  food_grape: false,
  category_food: false,
  category_drink: false,
  category_gadget: false,
  category_sport: false,
  category_government: false,
  category_internet: false,
  category_smartphone: false,
  country_jp: '',
  country_en: '',
  prefecture_jp: '',
  prefecture_en: '',
  post_no_jp: '',
  post_no_en: '',
  address_1_jp: '',
  address_1_en: '',
  address_2_jp: '',
  address_2_en: '',
  address_other_jp: '',
  address_other_en: '',
  pub_date1: '',
  pub_date2: '',
  pub_date3: '',
  pub_date4: '',
  pub_date5: '',
  pub_date6: '',
  qty1: '',
  qty2: '',
  qty3: '',
  qty4: '',
  qty5: '',
  qty6: '',
};

function ItemFormDialog({ isOpen, onClose, onSave, initialData }) {
  const [formData, setFormData] = useState(initialFormData);
  const dialogRef = useRef(null);

  useEffect(() => {
    if (initialData) {
      setFormData({ ...initialFormData, ...initialData });
    } else {
      setFormData(initialFormData);
    }
  }, [initialData]);

  useEffect(() => {
    const dialogNode = dialogRef.current;
    if (isOpen && dialogNode && !dialogNode.open) {
      dialogNode.showModal();
    } else if (!isOpen && dialogNode && dialogNode.open) {
      dialogNode.close();
    }
  }, [isOpen]);


  const handleChange = (e) => {
    const { name, value, type, checked } = e.target;
    setFormData(prev => ({
      ...prev,
      [name]: type === 'checkbox' ? checked : value
    }));
  };

  const handleSubmit = (e) => {
    e.preventDefault();
    onSave(formData);
  };

  const handleDialogClose = () => {
    if (dialogRef.current && dialogRef.current.open) {
      dialogRef.current.close();
    }
    onClose(); // Notify parent about closing
  };

  // Prevent closing dialog on form submit or clicking inside
  const stopPropagation = (e) => e.stopPropagation();

  // Close dialog on backdrop click
  const handleBackdropClick = (e) => {
    if (dialogRef.current && e.target === dialogRef.current) {
        handleDialogClose();
    }
  };

  const foodOptions = [
    { name: 'food_orange', label: 'オレンジ' },
    { name: 'food_apple', label: 'リンゴ' },
    { name: 'food_banana', label: 'バナナ' },
    { name: 'food_melon', label: 'メロン' },
    { name: 'food_grape', label: 'ぶどう' },
  ];

  const categoryOptions = [
    { name: 'category_food', label: '食べ物' },
    { name: 'category_drink', label: '飲み物' },
    { name: 'category_gadget', label: 'ガジェット' },
    { name: 'category_sport', label: 'スポーツ' },
    { name: 'category_government', label: '行政' },
    { name: 'category_internet', label: 'インターネット' },
    { name: 'category_smartphone', label: 'スマートフォン' },
  ];

  if (!isOpen) return null; // Or render nothing if managed by dialog.showModal()

  return (
    <dialog
        ref={dialogRef}
        className="p-0 rounded-lg shadow-xl w-full max-w-2xl backdrop:bg-black backdrop:bg-opacity-50"
        onClick={handleBackdropClick} // Close on backdrop click
        onClose={onClose} // Handles Escape key
    >
      <div className="bg-white p-6 rounded-lg" onClick={stopPropagation}> {/* Prevent backdrop click through content */}
        <h2 className="text-2xl font-semibold mb-6 text-gray-800">
          {initialData && initialData.id ? 'アイテム編集' : 'アイテム追加'}
        </h2>
        <form onSubmit={handleSubmit} className="space-y-6">
          {/* Title */}
          <div>
            <label htmlFor="title" className="block text-sm font-medium text-gray-700 mb-1">タイトル</label>
            <input
              type="text"
              name="title"
              id="title"
              value={formData.title}
              onChange={handleChange}
              className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm"
              required
            />
          </div>

          {/* Content */}
          <div>
            <label htmlFor="content" className="block text-sm font-medium text-gray-700 mb-1">内容</label>
            <input
              type="text"
              name="content"
              id="content"
              value={formData.content}
              onChange={handleChange}
              className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm"
            />
          </div>

          {/* Content Type */}
          <div>
            <label htmlFor="content_type" className="block text-sm font-medium text-gray-700 mb-1">コンテンツタイプ</label>
            <input
              type="text"
              name="content_type"
              id="content_type"
              value={formData.content_type}
              onChange={handleChange}
              className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm"
            />
          </div>

          {/* Public Type (Radio) */}
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">公開設定</label>
            <div className="mt-2 space-x-4">
              <label className="inline-flex items-center">
                <input
                  type="radio"
                  name="public_type"
                  value="公開"
                  checked={formData.public_type === '公開'}
                  onChange={handleChange}
                  className="form-radio h-4 w-4 text-indigo-600 border-gray-300 focus:ring-indigo-500"
                />
                <span className="ml-2 text-sm text-gray-700">公開</span>
              </label>
              <label className="inline-flex items-center">
                <input
                  type="radio"
                  name="public_type"
                  value="非公開"
                  checked={formData.public_type === '非公開'}
                  onChange={handleChange}
                  className="form-radio h-4 w-4 text-indigo-600 border-gray-300 focus:ring-indigo-500"
                />
                <span className="ml-2 text-sm text-gray-700">非公開</span>
              </label>
            </div>
          </div>

          {/* Food Checkboxes */}
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">好きな食べ物</label>
            <div className="mt-2 grid grid-cols-2 sm:grid-cols-3 gap-2">
              {foodOptions.map(food => (
                <label key={food.name} className="inline-flex items-center">
                  <input
                    type="checkbox"
                    name={food.name}
                    checked={!!formData[food.name]} // Ensure boolean for controlled component
                    onChange={handleChange}
                    className="form-checkbox h-4 w-4 text-indigo-600 border-gray-300 rounded focus:ring-indigo-500"
                  />
                  <span className="ml-2 text-sm text-gray-700">{food.label}</span>
                </label>
              ))}
            </div>
          </div>

          {/* Category Checkboxes */}
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">カテゴリ</label>
            <div className="mt-2 grid grid-cols-2 sm:grid-cols-3 gap-2">
              {categoryOptions.map(category => (
                <label key={category.name} className="inline-flex items-center">
                  <input
                    type="checkbox"
                    name={category.name}
                    checked={!!formData[category.name]}
                    onChange={handleChange}
                    className="form-checkbox h-4 w-4 text-indigo-600 border-gray-300 rounded focus:ring-indigo-500"
                  />
                  <span className="ml-2 text-sm text-gray-700">{category.label}</span>
                </label>
              ))}
            </div>
          </div>

          {/* Country Inputs */}
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div>
              <label htmlFor="country_jp" className="block text-sm font-medium text-gray-700 mb-1">国 (日本語)</label>
              <input type="text" name="country_jp" id="country_jp" value={formData.country_jp} onChange={handleChange} className="mt-1 block w-full input-style" />
            </div>
            <div>
              <label htmlFor="country_en" className="block text-sm font-medium text-gray-700 mb-1">国 (英語)</label>
              <input type="text" name="country_en" id="country_en" value={formData.country_en} onChange={handleChange} className="mt-1 block w-full input-style" />
            </div>
          </div>
          
          {/* Prefecture Inputs */}
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div>
              <label htmlFor="prefecture_jp" className="block text-sm font-medium text-gray-700 mb-1">都道府県 (日本語)</label>
              <input type="text" name="prefecture_jp" id="prefecture_jp" value={formData.prefecture_jp} onChange={handleChange} className="mt-1 block w-full input-style" />
            </div>
            <div>
              <label htmlFor="prefecture_en" className="block text-sm font-medium text-gray-700 mb-1">都道府県 (英語)</label>
              <input type="text" name="prefecture_en" id="prefecture_en" value={formData.prefecture_en} onChange={handleChange} className="mt-1 block w-full input-style" />
            </div>
          </div>

          {/* Post Number Inputs */}
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div>
              <label htmlFor="post_no_jp" className="block text-sm font-medium text-gray-700 mb-1">郵便番号 (日本語)</label>
              <input type="text" name="post_no_jp" id="post_no_jp" value={formData.post_no_jp} onChange={handleChange} className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm" />
            </div>
            <div>
              <label htmlFor="post_no_en" className="block text-sm font-medium text-gray-700 mb-1">郵便番号 (英語)</label>
              <input type="text" name="post_no_en" id="post_no_en" value={formData.post_no_en} onChange={handleChange} className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm" />
            </div>
          </div>

          {/* Address 1 Inputs */}
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div>
              <label htmlFor="address_1_jp" className="block text-sm font-medium text-gray-700 mb-1">住所1 (日本語)</label>
              <input type="text" name="address_1_jp" id="address_1_jp" value={formData.address_1_jp} onChange={handleChange} className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm" />
            </div>
            <div>
              <label htmlFor="address_1_en" className="block text-sm font-medium text-gray-700 mb-1">住所1 (英語)</label>
              <input type="text" name="address_1_en" id="address_1_en" value={formData.address_1_en} onChange={handleChange} className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm" />
            </div>
          </div>

          {/* Address 2 Inputs */}
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div>
              <label htmlFor="address_2_jp" className="block text-sm font-medium text-gray-700 mb-1">住所2 (日本語)</label>
              <input type="text" name="address_2_jp" id="address_2_jp" value={formData.address_2_jp} onChange={handleChange} className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm" />
            </div>
            <div>
              <label htmlFor="address_2_en" className="block text-sm font-medium text-gray-700 mb-1">住所2 (英語)</label>
              <input type="text" name="address_2_en" id="address_2_en" value={formData.address_2_en} onChange={handleChange} className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm" />
            </div>
          </div>

          {/* Address Other Inputs */}
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div>
              <label htmlFor="address_other_jp" className="block text-sm font-medium text-gray-700 mb-1">その他住所 (日本語)</label>
              <input type="text" name="address_other_jp" id="address_other_jp" value={formData.address_other_jp} onChange={handleChange} className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm" />
            </div>
            <div>
              <label htmlFor="address_other_en" className="block text-sm font-medium text-gray-700 mb-1">その他住所 (英語)</label>
              <input type="text" name="address_other_en" id="address_other_en" value={formData.address_other_en} onChange={handleChange} className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm" />
            </div>
          </div>

          {/* Publication Date Inputs */}
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div>
              <label htmlFor="pub_date1" className="block text-sm font-medium text-gray-700 mb-1">公開日1</label>
              <input type="date" name="pub_date1" id="pub_date1" value={formData.pub_date1} onChange={handleChange} className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm" />
            </div>
            <div>
              <label htmlFor="pub_date2" className="block text-sm font-medium text-gray-700 mb-1">公開日2</label>
              <input type="date" name="pub_date2" id="pub_date2" value={formData.pub_date2} onChange={handleChange} className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm" />
            </div>
          </div>

          {/* Additional Publication Date Inputs */}
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div>
              <label htmlFor="pub_date3" className="block text-sm font-medium text-gray-700 mb-1">公開日3</label>
              <input type="date" name="pub_date3" id="pub_date3" value={formData.pub_date3} onChange={handleChange} className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm" />
            </div>
            <div>
              <label htmlFor="pub_date4" className="block text-sm font-medium text-gray-700 mb-1">公開日4</label>
              <input type="date" name="pub_date4" id="pub_date4" value={formData.pub_date4} onChange={handleChange} className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm" />
            </div>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div>
              <label htmlFor="pub_date5" className="block text-sm font-medium text-gray-700 mb-1">公開日5</label>
              <input type="date" name="pub_date5" id="pub_date5" value={formData.pub_date5} onChange={handleChange} className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm" />
            </div>
            <div>
              <label htmlFor="pub_date6" className="block text-sm font-medium text-gray-700 mb-1">公開日6</label>
              <input type="date" name="pub_date6" id="pub_date6" value={formData.pub_date6} onChange={handleChange} className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm" />
            </div>
          </div>

          {/* Quantity Inputs */}
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div>
              <label htmlFor="qty1" className="block text-sm font-medium text-gray-700 mb-1">数量1</label>
              <input type="text" name="qty1" id="qty1" value={formData.qty1} onChange={handleChange} className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm" />
            </div>
            <div>
              <label htmlFor="qty2" className="block text-sm font-medium text-gray-700 mb-1">数量2</label>
              <input type="text" name="qty2" id="qty2" value={formData.qty2} onChange={handleChange} className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm" />
            </div>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div>
              <label htmlFor="qty3" className="block text-sm font-medium text-gray-700 mb-1">数量3</label>
              <input type="text" name="qty3" id="qty3" value={formData.qty3} onChange={handleChange} className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm" />
            </div>
            <div>
              <label htmlFor="qty4" className="block text-sm font-medium text-gray-700 mb-1">数量4</label>
              <input type="text" name="qty4" id="qty4" value={formData.qty4} onChange={handleChange} className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm" />
            </div>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div>
              <label htmlFor="qty5" className="block text-sm font-medium text-gray-700 mb-1">数量5</label>
              <input type="text" name="qty5" id="qty5" value={formData.qty5} onChange={handleChange} className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm" />
            </div>
            <div>
              <label htmlFor="qty6" className="block text-sm font-medium text-gray-700 mb-1">数量6</label>
              <input type="text" name="qty6" id="qty6" value={formData.qty6} onChange={handleChange} className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm" />
            </div>
          </div>
          
          {/* Buttons */}
          <div className="flex justify-end space-x-3 pt-4">
            <button
              type="button"
              onClick={handleDialogClose}
              className="px-4 py-2 text-sm font-medium text-gray-700 bg-gray-100 hover:bg-gray-200 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
            >
              キャンセル
            </button>
            <button
              type="submit"
              className="px-4 py-2 text-sm font-medium text-white bg-indigo-600 hover:bg-indigo-700 border border-transparent rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
            >
              保存
            </button>
          </div>
        </form>
      </div>
    </dialog>
  );
}

// グローバルなinputスタイルを定義 (Tailwindの@applyを使っても良いが、ここではCSS-in-JS風に)
// `tailwind.config.js` の `plugins` で `@tailwindcss/forms` を追加するとより良いフォームスタイルが得られます
// ここでは手動でスタイルを適用しています。
const inputStyle = "px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm";
// `ItemFormDialog.js`内で上記のスタイルを適用するために、classNameに`input-style`を適用
// 例: className={`mt-1 block w-full ${inputStyle}`} のようにするか、
// src/index.css に .input-style { @apply px-3 py-2 ...; } を定義します。
// 今回は直接クラスを埋め込んでいるので、この定数は不要です。

export default ItemFormDialog;