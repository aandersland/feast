---
date: 2025-12-15
status: draft
target_command: create_plan
---

# Backend Schema & Commands (Phases 1-2)

## Goal

Build the complete SQLite database schema and Rust Tauri commands to support all Feast app features: Recipes, Meal Planning, Shopping Lists, and Quick Lists. This enables the existing mocked frontend to connect to a real persistent backend.

## Background

Feast is a Tauri v2 desktop app with a complete Svelte 5 frontend using mock data. The backend currently only has a basic `items` table with CRUD operations. The frontend has 40+ components across 5 feature areas, all reading from in-memory Svelte stores with hardcoded mock data. Tauri IPC wrappers exist but are unused.

**Current backend state:**
- 1 table (`items`) - not used by the app
- 4 commands (`greet`, `get_items`, `create_item`, `delete_item`)
- Migration infrastructure and connection pooling in place
- `AppError` enum pattern for error handling

## Requirements

### Must Have

#### Database Schema (9 tables)
- `recipes` - id, name, description, prep_time, cook_time, servings, image_path, source_url, notes, created_at, updated_at
- `ingredients` - id, name, category, default_unit (normalized names for aggregation)
- `recipe_ingredients` - id, recipe_id, ingredient_id, quantity, unit, notes (FK with CASCADE)
- `meal_plans` - id, date, meal_type, recipe_id, servings, created_at (unique constraint on date+meal_type+recipe_id)
- `shopping_lists` - id, week_start, name, created_at
- `shopping_list_items` - id, list_id, ingredient_id, quantity, unit, is_checked, is_deleted, moved_to_list_id
- `quick_lists` - id, name, created_at, updated_at
- `quick_list_items` - id, quick_list_id, name, category, default_quantity, default_unit
- `manual_shopping_items` - id, week_start, name, category, quantity, unit, is_checked, created_at

#### Tauri Commands (~30 commands across 6 modules)

**Recipes Module:**
- `get_recipes` - List all recipes with optional filtering
- `get_recipe` - Get single recipe by ID with ingredients
- `create_recipe` - Create recipe with ingredients (transaction)
- `update_recipe` - Update recipe and ingredients (transaction)
- `delete_recipe` - Delete recipe (cascades to ingredients)

**Ingredients Module:**
- `get_ingredients` - List all ingredients
- `create_ingredient` - Create new ingredient
- `get_or_create_ingredient` - Find existing or create (for recipe creation)

**Meal Plans Module:**
- `get_meal_plans` - Get meal plans for date range
- `create_meal_plan` - Add recipe to meal plan
- `update_meal_plan` - Update servings
- `delete_meal_plan` - Remove from meal plan

**Shopping Lists Module:**
- `get_shopping_lists` - Get lists for a week
- `create_shopping_list` - Create new list
- `delete_shopping_list` - Delete list
- `add_shopping_item` - Add item to list
- `update_shopping_item` - Update item (check, quantity)
- `move_shopping_item` - Move item between lists
- `soft_delete_shopping_item` - Soft delete item
- `restore_shopping_item` - Restore soft-deleted item
- `get_aggregated_shopping_list` - Aggregate from meal plans (with unit conversion)

**Quick Lists Module:**
- `get_quick_lists` - Get all quick lists
- `create_quick_list` - Create new quick list
- `update_quick_list` - Rename quick list
- `delete_quick_list` - Delete quick list
- `add_quick_list_item` - Add item to quick list
- `update_quick_list_item` - Update item
- `remove_quick_list_item` - Remove item
- `add_quick_list_to_shopping` - Copy quick list items to shopping list

**Manual Items Module:**
- `get_manual_items` - Get manual items for week
- `create_manual_item` - Add manual item
- `update_manual_item` - Update item
- `delete_manual_item` - Delete item

#### Unit Conversion System
- Define supported units and conversion factors (cups, tbsp, tsp, oz, lb, g, kg, ml, L, etc.)
- Implement conversion logic for aggregating shopping list items
- Handle items with incompatible units gracefully (list separately)

#### TypeScript Types & IPC Wrappers
- TypeScript interfaces matching all Rust structs in `src/lib/types/`
- Tauri IPC wrapper functions in `src/lib/tauri/commands.ts`

### Out of Scope
- Recipe import from URL (deferred to Phase 5)
- Offline support / sync
- Frontend store integration (Phase 3)
- Frontend component changes (Phase 3)
- Image upload/management UI (just store path)

## Affected Areas

- **Systems/Components**:
  - `src-tauri/migrations/` - New migration files
  - `src-tauri/src/db/` - Database operation modules
  - `src-tauri/src/commands/` - Tauri command handlers
  - `src-tauri/src/lib.rs` - Command registration
  - `src/lib/types/` - TypeScript interfaces
  - `src/lib/tauri/commands.ts` - IPC wrappers

- **Data**: Creates full data model for recipes, ingredients, meal plans, shopping lists, quick lists

## Vertical Slice Analysis

**Layers involved**: Database (SQLite) → Rust DB operations → Rust Commands → TypeScript IPC wrappers

**Decomposition approach**: Feature-vertical

Implement each feature domain completely before moving to the next:
1. **Recipes + Ingredients** (foundation - other features depend on this)
2. **Meal Plans** (depends on recipes)
3. **Shopping Lists** (depends on meal plans for aggregation)
4. **Quick Lists** (independent, can parallelize)
5. **Manual Items** (independent, can parallelize)

**Rationale**:
- Recipes are the foundation - meal plans reference recipes, shopping lists aggregate from meal plans
- Feature-vertical allows testing each domain end-to-end before moving on
- Quick Lists and Manual Items are independent and can be done in parallel after Recipes

## Success Criteria

### Automated Verification
- [ ] All migrations run successfully (`pnpm test:rust` passes)
- [ ] All Rust DB operations have unit tests
- [ ] All Tauri commands have unit tests
- [ ] `cargo clippy` passes with no warnings
- [ ] TypeScript types compile (`pnpm check` passes)

### Manual Verification
- [ ] Can create/read/update/delete recipes via Tauri commands
- [ ] Can create meal plans and see aggregated shopping list
- [ ] Unit conversion works correctly when aggregating ingredients
- [ ] Quick lists can be copied to shopping lists

## Open Questions for Planning

- What's the existing pattern in `src-tauri/src/db/` for database operations? Follow it.
- What Rust structs exist in `src-tauri/src/db/`? Extend the pattern.
- How are Tauri commands registered in `src-tauri/src/lib.rs`? Follow the pattern.
- What TypeScript types exist in `src/lib/types/`? Match the naming conventions.
- How do the frontend stores model the data? Schema should align with store expectations.

## Constraints

- Follow existing code patterns in the Rust backend
- Use sqlx with compile-time query checking
- Migrations must be idempotent
- Keep Tauri commands thin - delegate to db modules
- TypeScript types must use camelCase (Rust uses snake_case with serde rename)

---

**To execute**: `/create_plan ai_docs/prompts/2025-12-15-backend-schema-and-commands.md`
