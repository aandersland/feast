-- Quick lists schema

CREATE TABLE IF NOT EXISTS quick_lists (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS quick_list_items (
    id TEXT PRIMARY KEY,
    quick_list_id TEXT NOT NULL,
    name TEXT NOT NULL,
    quantity REAL NOT NULL DEFAULT 1,
    unit TEXT NOT NULL DEFAULT '',
    category TEXT NOT NULL DEFAULT 'Other',
    FOREIGN KEY (quick_list_id) REFERENCES quick_lists(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_quick_list_items_list ON quick_list_items(quick_list_id);
