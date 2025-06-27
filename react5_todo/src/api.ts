import { Item } from './types'

const API_BASE = '/api'

export const api = {
  async getItems(): Promise<Item[]> {
    const response = await fetch(`${API_BASE}/list`)
    if (!response.ok) throw new Error('Failed to fetch items')
    return response.json()
  },

  async getItem(id: number): Promise<Item> {
    const response = await fetch(`${API_BASE}/items/${id}`)
    if (!response.ok) throw new Error('Failed to fetch item')
    return response.json()
  },

  async createItem(item: Omit<Item, 'id' | 'created_at' | 'updated_at'>): Promise<Item> {
    const response = await fetch(`${API_BASE}/create`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(item),
    })
    if (!response.ok) throw new Error('Failed to create item')
    return response.json()
  },

  async updateItem(id: number, item: Omit<Item, 'id' | 'created_at' | 'updated_at'>): Promise<Item> {
    const response = await fetch(`${API_BASE}/items/${id}`, {
      method: 'PUT',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(item),
    })
    if (!response.ok) throw new Error('Failed to update item')
    return response.json()
  },

  async deleteItem(id: number): Promise<void> {
    //const response = await fetch(`${API_BASE}/items/${id}`, {
    //  method: 'DELETE',
    //})
    const item = {id: id}
    const response = await fetch(`${API_BASE}/delete`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(item),
    })
    if (!response.ok) throw new Error('Failed to delete item')
  },
}