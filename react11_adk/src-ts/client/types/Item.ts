export interface Item {
  id: number;
  title: string;
  content: string;
  content_type: string;
  public_type: string;
  food_orange: boolean;
  food_apple: boolean;
  food_banana: boolean;
  food_melon: boolean;
  food_grape: boolean;
  category_food: boolean;
  category_drink: boolean;
  created_at: string;
  updated_at: string;
}

export interface NewItem {
  title: string;
  content: string;
  content_type: string;
  public_type: string;
  food_orange: boolean;
  food_apple: boolean;
  food_banana: boolean;
  food_melon: boolean;
  food_grape: boolean;
  category_food: boolean;
  category_drink: boolean;
}