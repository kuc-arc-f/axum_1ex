import React, { useState, useEffect } from 'react'
import { Item } from './types'
import { api } from './api'
import ItemForm from './components/ItemForm'
import Modal from './components/Modal'

export default function App() {
  const [items, setItems] = useState<Item[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const [isModalOpen, setIsModalOpen] = useState(false)
  const [editingItem, setEditingItem] = useState<Item | null>(null)

  useEffect(() => {
    loadItems()
  }, [])

  const loadItems = async () => {
    try {
      setLoading(true)
      const fetchedItems = await api.getItems()
      setItems(fetchedItems)
      setError(null)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'エラーが発生しました')
    } finally {
      setLoading(false)
    }
  }

  const handleCreateItem = async (itemData: Omit<Item, 'id' | 'created_at' | 'updated_at'>) => {
    try {
      itemData.food_orange = 0;
      itemData.food_apple = 0;
      itemData.food_banana = 0;
      itemData.food_melon = 0;
      itemData.food_grape = 0;
      itemData.category_food = 0;
      itemData.category_drink = 0;
      itemData.category_gadget = 0;
      itemData.category_sport = 0;
      itemData.category_government = 0;
      itemData.category_internet = 0;
      itemData.category_smartphone = 0;
      console.log(itemData);

      await api.createItem(itemData)
      await loadItems()
      setIsModalOpen(false)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'アイテムの作成に失敗しました')
    }
  }

  const handleUpdateItem = async (itemData: Omit<Item, 'id' | 'created_at' | 'updated_at'>) => {
    if (!editingItem?.id) return
    
    try {
      await api.updateItem(editingItem.id, itemData)
      await loadItems()
      setIsModalOpen(false)
      setEditingItem(null)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'アイテムの更新に失敗しました')
    }
  }

  const handleDeleteItem = async (id: number) => {
    if (!confirm('このアイテムを削除しますか？')) return
    
    try {
      await api.deleteItem(id)
      await loadItems()
    } catch (err) {
      setError(err instanceof Error ? err.message : 'アイテムの削除に失敗しました')
    }
  }

  const openCreateModal = () => {
    setEditingItem(null)
    setIsModalOpen(true)
  }

  const openEditModal = (item: Item) => {
    setEditingItem(item)
    setIsModalOpen(true)
  }

  const closeModal = () => {
    setIsModalOpen(false)
    setEditingItem(null)
  }

  if (loading) {
    return (
      <div className="min-h-screen bg-gray-50 flex items-center justify-center">
        <div className="text-xl">読み込み中...</div>
      </div>
    )
  }

  return (
    <div className="min-h-screen bg-gray-50 py-8">
      <div className="max-w-6xl mx-auto px-4">
        <div className="mb-8">
          <h1 className="text-3xl font-bold text-gray-900 mb-4">CRUD アプリ</h1>
          
          {error && (
            <div className="mb-4 p-4 bg-red-100 border border-red-400 text-red-700 rounded">
              {error}
            </div>
          )}
          
          <button
            onClick={openCreateModal}
            className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500"
          >
            新しいアイテムを作成
          </button>
        </div>

        <div className="bg-white shadow rounded-lg overflow-hidden">
          <table className="min-w-full divide-y divide-gray-200">
            <thead className="bg-gray-50">
              <tr>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  タイトル
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  コンテンツ
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  公開設定
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  国
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  操作
                </th>
              </tr>
            </thead>
            <tbody className="bg-white divide-y divide-gray-200">
              {items.map((item) => (
                <tr key={item.id}>
                  <td className="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">
                    {item.title}
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                    {item.content}
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                    {item.public_type === 'public' ? '公開' : '非公開'}
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                    {item.country_jp}
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm font-medium space-x-2">
                    <button
                      onClick={() => openEditModal(item)}
                      className="text-indigo-600 hover:text-indigo-900"
                    >
                      編集
                    </button>
                    <button
                      onClick={() => handleDeleteItem(item.id!)}
                      className="text-red-600 hover:text-red-900"
                    >
                      削除
                    </button>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
          
          {items.length === 0 && (
            <div className="text-center py-8 text-gray-500">
              アイテムがありません
            </div>
          )}
        </div>

        <Modal
          isOpen={isModalOpen}
          onClose={closeModal}
          title={editingItem ? 'アイテムを編集' : '新しいアイテムを作成'}
        >
          <ItemForm
            item={editingItem || undefined}
            onSubmit={editingItem ? handleUpdateItem : handleCreateItem}
            onCancel={closeModal}
          />
        </Modal>
      </div>
    </div>
  )
}