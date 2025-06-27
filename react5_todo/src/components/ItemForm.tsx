import React, { useState } from 'react'
import { Item } from '../types'

interface ItemFormProps {
  item?: Item
  onSubmit: (item: Omit<Item, 'id' | 'created_at' | 'updated_at'>) => void
  onCancel: () => void
}

const initialItem: Omit<Item, 'id' | 'created_at' | 'updated_at'> = {
  title: '',
  content: '',
  content_type: '',
  public_type: 'public',
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
  prefecture_en: ''
}

export default function ItemForm({ item, onSubmit, onCancel }: ItemFormProps) {
  const [formData, setFormData] = useState(() => ({
    ...initialItem,
    ...item
  }))
  const [errors, setErrors] = useState<{ [key: string]: string }>({})

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault()
    
    const newErrors: { [key: string]: string } = {}
    if (!formData.title.trim()) {
      newErrors.title = 'タイトルは必須です'
    }
    
    if (Object.keys(newErrors).length > 0) {
      setErrors(newErrors)
      return
    }
    
    setErrors({})
    onSubmit(formData)
  }

  const handleChange = (field: keyof typeof formData, value: any) => {
    setFormData(prev => ({ ...prev, [field]: value }))
    if (errors[field]) {
      setErrors(prev => ({ ...prev, [field]: '' }))
    }
  }

  return (
    <form onSubmit={handleSubmit} className="space-y-6">
      <div>
        <label className="block text-sm font-medium text-gray-700 mb-1">
          タイトル *
        </label>
        <input
          type="text"
          value={formData.title}
          onChange={(e) => handleChange('title', e.target.value)}
          className={`w-full px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 ${
            errors.title ? 'border-red-500' : 'border-gray-300'
          }`}
        />
        {errors.title && (
          <p className="mt-1 text-sm text-red-600">{errors.title}</p>
        )}
      </div>

      <div>
        <label className="block text-sm font-medium text-gray-700 mb-1">
          コンテンツ
        </label>
        <input
          type="text"
          value={formData.content}
          onChange={(e) => handleChange('content', e.target.value)}
          className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
      </div>

      <div>
        <label className="block text-sm font-medium text-gray-700 mb-1">
          コンテンツタイプ
        </label>
        <input
          type="text"
          value={formData.content_type}
          onChange={(e) => handleChange('content_type', e.target.value)}
          className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
      </div>

      <div>
        <label className="block text-sm font-medium text-gray-700 mb-2">
          公開設定
        </label>
        <div className="space-x-4">
          <label className="inline-flex items-center">
            <input
              type="radio"
              value="public"
              checked={formData.public_type === 'public'}
              onChange={(e) => handleChange('public_type', e.target.value)}
              className="mr-2"
            />
            公開
          </label>
          <label className="inline-flex items-center">
            <input
              type="radio"
              value="private"
              checked={formData.public_type === 'private'}
              onChange={(e) => handleChange('public_type', e.target.value)}
              className="mr-2"
            />
            非公開
          </label>
        </div>
      </div>

      <div>
        <label className="block text-sm font-medium text-gray-700 mb-2">
          フルーツ
        </label>
        <div className="grid grid-cols-2 gap-2">
          {[
            { key: 'food_orange' as const, label: 'オレンジ' },
            { key: 'food_apple' as const, label: 'りんご' },
            { key: 'food_banana' as const, label: 'バナナ' },
            { key: 'food_melon' as const, label: 'メロン' },
            { key: 'food_grape' as const, label: 'ぶどう' }
          ].map(({ key, label }) => (
            <label key={key} className="inline-flex items-center">
              <input
                type="checkbox"
                checked={formData[key]}
                onChange={(e) => handleChange(key, e.target.checked)}
                className="mr-2"
              />
              {label}
            </label>
          ))}
        </div>
      </div>

      <div>
        <label className="block text-sm font-medium text-gray-700 mb-2">
          カテゴリー
        </label>
        <div className="grid grid-cols-2 gap-2">
          {[
            { key: 'category_food' as const, label: '食べ物' },
            { key: 'category_drink' as const, label: '飲み物' },
            { key: 'category_gadget' as const, label: 'ガジェット' },
            { key: 'category_sport' as const, label: 'スポーツ' },
            { key: 'category_government' as const, label: '政府' },
            { key: 'category_internet' as const, label: 'インターネット' },
            { key: 'category_smartphone' as const, label: 'スマートフォン' }
          ].map(({ key, label }) => (
            <label key={key} className="inline-flex items-center">
              <input
                type="checkbox"
                checked={formData[key]}
                onChange={(e) => handleChange(key, e.target.checked)}
                className="mr-2"
              />
              {label}
            </label>
          ))}
        </div>
      </div>

      <div className="grid grid-cols-2 gap-4">
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-1">
            国（日本語）
          </label>
          <input
            type="text"
            value={formData.country_jp}
            onChange={(e) => handleChange('country_jp', e.target.value)}
            className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
        </div>
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-1">
            国（英語）
          </label>
          <input
            type="text"
            value={formData.country_en}
            onChange={(e) => handleChange('country_en', e.target.value)}
            className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
        </div>
      </div>

      <div className="grid grid-cols-2 gap-4">
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-1">
            都道府県（日本語）
          </label>
          <input
            type="text"
            value={formData.prefecture_jp}
            onChange={(e) => handleChange('prefecture_jp', e.target.value)}
            className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
        </div>
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-1">
            都道府県（英語）
          </label>
          <input
            type="text"
            value={formData.prefecture_en}
            onChange={(e) => handleChange('prefecture_en', e.target.value)}
            className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
        </div>
      </div>

      <div className="flex space-x-3 pt-4">
        <button
          type="submit"
          className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500"
        >
          保存
        </button>
        <button
          type="button"
          onClick={onCancel}
          className="px-4 py-2 bg-gray-300 text-gray-700 rounded-md hover:bg-gray-400 focus:outline-none focus:ring-2 focus:ring-gray-500"
        >
          キャンセル
        </button>
      </div>
    </form>
  )
}