-- Shopping lists schema

CREATE TABLE IF NOT EXISTS shopping_lists (
    id TEXT PRIMARY KEY,
    week_start TEXT NOT NULL,
    name TEXT NOT NULL,
    list_type TEXT NOT NULL DEFAULT 'weekly' CHECK (list_type IN ('weekly', 'midweek', 'custom')),
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_shopping_lists_week ON shopping_lists(week_start);

CREATE TABLE IF NOT EXISTS shopping_list_items (
    id TEXT PRIMARY KEY,
    list_id TEXT NOT NULL,
    ingredient_id TEXT,
    name TEXT NOT NULL,
    quantity REAL NOT NULL DEFAULT 0,
    unit TEXT NOT NULL DEFAULT '',
    category TEXT NOT NULL DEFAULT 'Other',
    is_checked INTEGER NOT NULL DEFAULT 0,
    is_deleted INTEGER NOT NULL DEFAULT 0,
    deleted_at TEXT,
    moved_to_list_id TEXT,
    source_recipe_ids TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (list_id) REFERENCES shopping_lists(id) ON DELETE CASCADE,
    FOREIGN KEY (ingredient_id) REFERENCES ingredients(id) ON DELETE SET NULL,
    FOREIGN KEY (moved_to_list_id) REFERENCES shopping_lists(id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_shopping_list_items_list ON shopping_list_items(list_id);
CREATE INDEX IF NOT EXISTS idx_shopping_list_items_ingredient ON shopping_list_items(ingredient_id);
