-- Manual shopping items schema

CREATE TABLE IF NOT EXISTS manual_shopping_items (
    id TEXT PRIMARY KEY,
    week_start TEXT NOT NULL,
    name TEXT NOT NULL,
    quantity REAL NOT NULL DEFAULT 1,
    unit TEXT NOT NULL DEFAULT '',
    category TEXT NOT NULL DEFAULT 'Other',
    is_checked INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_manual_items_week ON manual_shopping_items(week_start);
