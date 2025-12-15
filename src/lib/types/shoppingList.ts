export interface ShoppingItem {
  id: string;
  name: string;
  quantity: number;
  unit: string;
  category: string;
  isOnHand: boolean;
  isManual: boolean; // true if manually added, false if from recipe
  sourceRecipeIds: string[]; // recipes this ingredient came from
  isDeleted?: boolean; // soft delete - item hidden but can be restored
  deletedAt?: string; // ISO date when soft deleted
  movedToListId?: string; // track if item was moved to another list (for visual indication)
  movedToListName?: string; // name of the list it was moved to
}

export interface QuickList {
  id: string;
  name: string;
  items: QuickListItem[];
}

export interface QuickListItem {
  id: string;
  name: string;
  quantity: number;
  unit: string;
  category: string;
}

export type ShoppingListType = 'weekly' | 'midweek' | 'custom';

export interface ShoppingList {
  id: string;
  name: string;
  type: ShoppingListType;
  items: ShoppingItem[];
}

export interface WeeklyShoppingLists {
  weekStart: string; // ISO date of Monday
  lists: ShoppingList[];
}

export const GROCERY_CATEGORIES = [
  'Produce',
  'Meat & Seafood',
  'Dairy & Eggs',
  'Bakery',
  'Pantry',
  'Frozen',
  'Beverages',
  'Snacks',
  'Other',
] as const;

export type GroceryCategory = typeof GROCERY_CATEGORIES[number];

// Unit conversion helpers for imperial/metric display
export interface UnitDisplay {
  imperial: string;
  metric: string;
}
