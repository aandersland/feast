---
date: 2025-12-15T10:30:00-05:00
researcher: Claude
git_commit: d7efb8e
branch: main
repository: feast
topic: "Development roadmap to connect frontend with backend"
tags: [research, roadmap, backend-integration, testing]
status: complete
---

# Development Roadmap: Frontend-Backend Integration

## Question
Create a roadmap to hook the frontend up with the backend, test it, and polish/fine tune the UI.

## Summary
The Feast app has a complete mocked frontend UI with 40+ Svelte components organized across Dashboard, Recipes, Meal Planning, Shopping Lists, and Quick Lists features. The Rust backend currently only supports a basic `items` table with CRUD operations - it needs to be extended with full schema for recipes, meal plans, shopping lists, and quick lists. Zero tests exist for frontend stores or backend data operations. The roadmap is organized into 5 phases: Database Schema, Backend Commands, Frontend Integration, Testing, and UI Polish.

## Current State Analysis

### Frontend (Complete UI, Mock Data)

| Feature | Components | Store | Mock Data |
|---------|------------|-------|-----------|
| Dashboard | 4 | Yes | Derived from other stores |
| Recipes | 9 | `recipeStore` | 6 hardcoded recipes |
| Meal Planning | 3 | `mealPlanStore` | 2 mock entries |
| Shopping Lists | 10 | `weeklyShoppingListsStore`, `manualItemsStore` | Mock lists |
| Quick Lists | 5 | `quickListsStore` | 2 mock quick lists |

**Key Finding**: All stores use in-memory Svelte writables. Tauri IPC wrappers exist (`src/lib/tauri/commands.ts`) but are unused.

### Backend (Minimal Foundation)

| Current | Missing |
|---------|---------|
| `items` table | `recipes`, `ingredients`, `recipe_ingredients` tables |
| `greet`, `get_items`, `create_item`, `delete_item` | Recipe CRUD, meal plan CRUD, shopping list CRUD |
| Migration infrastructure | Full schema migrations |
| Error handling | Validation logic |

### Testing (Major Gaps)

| Area | Tests |
|------|-------|
| Rust migrations | 3 tests |
| Rust data operations | 0 tests |
| Frontend stores | 0 tests |
| Frontend components | 0 tests |

---

## Development Phases

### Phase 1: Database Schema Extension

**Goal**: Create complete SQLite schema for all data entities.

#### Tasks

1. **Design and create `recipes` table migration**
   - `id`, `name`, `description`, `prep_time`, `cook_time`, `servings`, `source_url`, `notes`, `created_at`, `updated_at`

2. **Design and create `ingredients` table migration**
   - `id`, `name`, `category` (produce, protein, dairy, pantry, etc.), `default_unit`
   - Ingredient names should be normalized for aggregation

3. **Design and create `recipe_ingredients` table migration**
   - `id`, `recipe_id`, `ingredient_id`, `quantity`, `unit`, `notes`
   - Foreign keys with CASCADE delete

4. **Design and create `meal_plans` table migration**
   - `id`, `date`, `meal_type` (breakfast, lunch, dinner, snack), `recipe_id`, `servings`, `created_at`
   - Unique constraint on (date, meal_type, recipe_id)

5. **Design and create `shopping_lists` table migration**
   - `id`, `week_start`, `name`, `created_at`
   - For weekly shopping list containers

6. **Design and create `shopping_list_items` table migration**
   - `id`, `list_id`, `ingredient_id`, `quantity`, `unit`, `is_checked`, `is_deleted`, `moved_to_list_id`
   - Support soft delete and move operations

7. **Design and create `quick_lists` table migration**
   - `id`, `name`, `created_at`, `updated_at`

8. **Design and create `quick_list_items` table migration**
   - `id`, `quick_list_id`, `name`, `category`, `default_quantity`, `default_unit`

9. **Design and create `manual_shopping_items` table migration**
   - `id`, `week_start`, `name`, `category`, `quantity`, `unit`, `is_checked`, `created_at`
   - For manually added items not from recipes

#### Deliverables
- Migration files in `src-tauri/migrations/`
- Updated `Item` struct or new structs for each entity
- Schema documentation

---

### Phase 2: Backend Command Implementation

**Goal**: Implement Tauri commands for all CRUD operations.

#### Tasks

##### Recipes Module
1. `get_recipes` - List all recipes with optional filtering
2. `get_recipe` - Get single recipe by ID with ingredients
3. `create_recipe` - Create recipe with ingredients (transaction)
4. `update_recipe` - Update recipe and ingredients (transaction)
5. `delete_recipe` - Delete recipe (cascades to ingredients)
6. `import_recipe` - Parse recipe from URL (stretch goal)

##### Ingredients Module
1. `get_ingredients` - List all ingredients
2. `create_ingredient` - Create new ingredient
3. `get_or_create_ingredient` - Find existing or create (for recipe creation)

##### Meal Plans Module
1. `get_meal_plans` - Get meal plans for date range
2. `create_meal_plan` - Add recipe to meal plan
3. `update_meal_plan` - Update servings
4. `delete_meal_plan` - Remove from meal plan

##### Shopping Lists Module
1. `get_shopping_lists` - Get lists for a week
2. `create_shopping_list` - Create new list
3. `delete_shopping_list` - Delete list
4. `add_shopping_item` - Add item to list
5. `update_shopping_item` - Update item (check, quantity)
6. `move_shopping_item` - Move item between lists
7. `soft_delete_shopping_item` - Soft delete item
8. `restore_shopping_item` - Restore soft-deleted item
9. `get_aggregated_shopping_list` - Aggregate from meal plans

##### Quick Lists Module
1. `get_quick_lists` - Get all quick lists
2. `create_quick_list` - Create new quick list
3. `update_quick_list` - Rename quick list
4. `delete_quick_list` - Delete quick list
5. `add_quick_list_item` - Add item to quick list
6. `update_quick_list_item` - Update item
7. `remove_quick_list_item` - Remove item
8. `add_quick_list_to_shopping` - Copy quick list items to shopping list

##### Manual Items Module
1. `get_manual_items` - Get manual items for week
2. `create_manual_item` - Add manual item
3. `update_manual_item` - Update item
4. `delete_manual_item` - Delete item

#### Deliverables
- Command handlers in `src-tauri/src/commands/`
- Database operations in `src-tauri/src/db/`
- TypeScript types matching Rust structs in `src/lib/types/`
- Updated Tauri IPC wrappers in `src/lib/tauri/commands.ts`

---

### Phase 3: Frontend Integration

**Goal**: Replace mock data with backend calls in all stores.

#### Tasks

##### Recipe Store Integration
1. Replace `mockRecipes` with `getRecipes()` call on mount
2. Wire `recipeStore.add()` to `createRecipe()` command
3. Wire `recipeStore.update()` to `updateRecipe()` command
4. Wire `recipeStore.remove()` to `deleteRecipe()` command
5. Add loading states to components
6. Add error handling and user feedback

##### Meal Plan Store Integration
1. Replace `mockMealPlans` with `getMealPlans()` call
2. Wire `mealPlanStore.addMeal()` to `createMealPlan()` command
3. Wire `mealPlanStore.removeMeal()` to `deleteMealPlan()` command
4. Wire `mealPlanStore.updateServings()` to `updateMealPlan()` command
5. Handle date range loading for calendar navigation

##### Shopping List Store Integration
1. Replace mock data with backend calls
2. Wire all CRUD operations to backend commands
3. Wire soft delete/restore operations
4. Wire move operations between lists
5. Integrate aggregated list from backend

##### Quick Lists Store Integration
1. Wire quick list CRUD operations
2. Wire quick list item operations
3. Implement "Add to Shopping List" feature with backend

##### Manual Items Store Integration
1. Wire manual item CRUD operations
2. Ensure proper week association

#### Component Updates
1. Add loading spinners during data fetch
2. Add error toasts/notifications
3. Handle optimistic updates where appropriate
4. Add retry logic for failed operations

#### Deliverables
- Updated stores with backend integration
- Loading and error states in UI
- Data persistence across sessions

---

### Phase 4: Testing

**Goal**: Establish comprehensive test coverage.

#### Rust Backend Tests

##### Database Operations (Priority: HIGH)
1. Test `recipes` CRUD operations
2. Test `recipe_ingredients` cascade behavior
3. Test `meal_plans` CRUD operations
4. Test `shopping_lists` and `shopping_list_items` operations
5. Test `quick_lists` and `quick_list_items` operations
6. Test aggregation query logic
7. Test constraint violations (unique, foreign key)

##### Command Handlers (Priority: MEDIUM)
1. Test each command with valid inputs
2. Test error responses for invalid inputs
3. Test transaction rollback on partial failure

#### Frontend Tests

##### Store Tests (Priority: HIGH)
1. Test `recipeStore` - add, update, remove, derived stores
2. Test `mealPlanStore` - all operations and date grouping
3. Test `shoppingListStore` - complex aggregation logic
4. Test `quickListsStore` - CRUD operations
5. Test `manualItemsStore` - all operations

##### Tauri IPC Tests (Priority: MEDIUM)
1. Test command wrappers with mocked invoke
2. Test error handling and transformation

##### Component Tests (Priority: LOW)
1. Test key user interactions
2. Test conditional rendering
3. Test form validation

#### Deliverables
- Rust tests in `src-tauri/src/` modules
- Frontend tests in `src/lib/**/*.test.ts`
- Test coverage report

---

### Phase 5: UI Polish & Refinement

**Goal**: Refine user experience based on real data behavior.

#### Tasks

##### Performance Optimization
1. Implement pagination for large recipe lists
2. Add virtual scrolling for long shopping lists
3. Optimize re-renders with proper Svelte reactivity
4. Add database indexes for common queries

##### Error Handling UX
1. Design and implement toast notification system
2. Add form validation feedback
3. Handle offline/connection errors gracefully
4. Add confirmation dialogs for destructive actions

##### Visual Polish
1. Review and refine responsive layouts
2. Ensure consistent spacing using design system
3. Add micro-interactions (hover states, transitions)
4. Review accessibility (keyboard navigation, ARIA)

##### Feature Completion
1. Implement recipe import from URL (if not done in Phase 2)
2. Add recipe image support
3. Add recipe search/filter persistence
4. Add meal plan copy/duplicate week feature
5. Add shopping list print/export feature

##### Data Seeding
1. Create seed data script for development
2. Add sample recipes for first-run experience

#### Deliverables
- Performance improvements
- Enhanced error handling
- Visual refinements
- Feature completions

---

## Code References

### Frontend Stores (Need Backend Integration)
- `src/lib/stores/recipes.ts:5-192` - Mock recipes data and store
- `src/lib/stores/mealPlan.ts:19-35` - Mock meal plan data
- `src/lib/stores/shoppingList.ts:14-174` - Mock shopping list data

### Existing Tauri Wrappers (Currently Unused)
- `src/lib/tauri/commands.ts` - Existing IPC wrapper pattern

### Current Backend
- `src-tauri/src/commands/mod.rs` - Command registration
- `src-tauri/src/db/pool.rs` - Database connection pool
- `src-tauri/migrations/` - Migration files

### Key Components
- `src/lib/components/recipes/ImportRecipe.svelte` - Has TODO for URL parsing
- `src/lib/components/shopping/ShoppingSection.svelte` - Complex aggregation display

---

## Architecture Insights

### Current Patterns to Preserve
1. **Store-based state management** - Components read from stores, stores will call backend
2. **Derived stores** - `recipeById`, `mealPlanByDate`, `aggregatedShoppingList` pattern works well
3. **Component separation** - Pure presentation components vs. connected components
4. **Error handling** - Backend has good `AppError` enum pattern

### Recommended Integration Pattern
```
Component -> Store Method -> Tauri IPC Wrapper -> Rust Command -> DB Operation
                                    |
                          (Update store on success)
```

### Migration Strategy
- Keep mock data available for development/testing
- Feature flag to switch between mock and real backend
- Gradual migration one feature at a time

---

## Open Questions

1. **Recipe Import**: Should URL parsing happen in Rust (faster, more reliable) or TypeScript (easier iteration)?
2. **Offline Support**: Should the app work offline with sync later, or require connectivity?
3. **Data Migration**: If users have mock data they want to keep, how do we handle initial migration?
4. **Image Storage**: Store recipe images in SQLite as blobs or filesystem with path references?
5. **Units/Conversions**: Should ingredient units be normalized? Support unit conversion?

---

## Phase Dependencies

```
Phase 1 (Schema) ─────┐
                      ├──> Phase 3 (Integration) ───┐
Phase 2 (Commands) ───┘                             ├──> Phase 5 (Polish)
                                                    │
Phase 4 (Testing) ──────────────────────────────────┘
```

- Phase 1 and 2 can partially overlap (implement commands as tables are ready)
- Phase 3 depends on Phase 1 and 2 completion
- Phase 4 can start alongside Phase 2 (backend tests) and Phase 3 (frontend tests)
- Phase 5 depends on Phase 3 completion but can overlap with Phase 4
