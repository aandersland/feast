---
date: 2025-12-13
status: draft
target_command: create_plan
---

# Meal Planning UI

## Goal

Build a complete, interactive UI prototype for a meal planning application. Focus is on iterating and locking down the visual design and interaction patterns before wiring up the backend.

## Background

This is a new Tauri v2 desktop app with Svelte 5 + Tailwind CSS. No UI currently exists beyond the scaffold. This prompt focuses exclusively on UI—backend integration will be handled in subsequent prompts.

## Requirements

### Must Have

**Navigation**
- Tab-based navigation at the top of the app
- Tabs: Dashboard, Recipes, Meal Plan, Shopping List

**Dashboard**
- Weekly meal calendar as the primary element
- Other dashboard widgets (TBD during implementation—suggestions welcome)

**Recipes**
- Recipe list/browser view
- Recipe detail view showing all available info:
  - Ingredients
  - Instructions
  - Prep time / Cook time
  - Servings (adjustable)
  - Nutrition info
  - Source URL
  - Notes
  - Tags/Categories
  - Image (if available)
- Create new recipe form
- Import recipe from URL interface
- Image support for recipes

**Meal Planning**
- Calendar view with flexible time periods (default: weekly)
- Click-to-add recipes onto calendar days via modal picker
- Ability to specify serving size when adding recipe (adjusts ingredient quantities)
- Support for multiple meals per day (breakfast, lunch, dinner, snacks)

**Shopping List**
- Combined list (groceries + household supplies)
- Auto-aggregates ingredients from planned recipes (e.g., "2 cups flour" + "1 cup flour" = "3 cups flour")
- Manual item additions
- Mark items as "on hand" / remove from list
- Adjust quantities needed
- "Quick lists" for common items—saved item groups that can be quickly added to the shopping list

### Out of Scope

- Backend/database integration
- Tauri commands and IPC
- Actual recipe import parsing logic
- User authentication
- Data persistence (mock data only)

## Affected Areas

- **Users/Personas**: Home cooks who want to plan meals and generate shopping lists
- **Systems/Components**:
  - `src/lib/components/` - All new UI components
  - `src/routes/` - Page structure
  - `src/lib/stores/` - UI state management with mock data
- **Data**: Mock data structures for recipes, meal plans, shopping lists

## Vertical Slice Analysis

**Layers involved**: UI only (Svelte components, stores, Tailwind styling)

**Decomposition approach**: Front-to-back (UI-first with mock data)

**Rationale**: This is explicitly a UI iteration phase. Building with realistic mock data allows full visualization and interaction testing without backend dependencies. The UI will be "dumb" but fully interactive.

## Success Criteria

### Manual Verification
- [ ] Can navigate between all tabs (Dashboard, Recipes, Meal Plan, Shopping List)
- [ ] Dashboard displays weekly meal calendar with mock planned meals
- [ ] Can browse recipe list and view recipe details
- [ ] Can access "create recipe" and "import recipe" interfaces
- [ ] Can add recipes to meal plan calendar via click-to-add
- [ ] Can adjust serving sizes when planning meals
- [ ] Shopping list shows aggregated ingredients from meal plan
- [ ] Can add/remove items from shopping list manually
- [ ] Can mark items as "on hand"
- [ ] Can add items from quick lists to shopping list
- [ ] Visual design is modern, fun, and sleek
- [ ] All screens are fully styled with Tailwind
- [ ] Interactive elements respond appropriately (hover states, focus states, etc.)

## Open Questions for Planning

- What additional dashboard widgets would complement the weekly calendar?
- How should the calendar handle days with many meals (scrolling, expansion, modal)?
- What's the visual treatment for "quick lists"?

## Constraints

- Desktop-only (no mobile responsiveness required)
- Must use Tailwind CSS v4 (already configured)
- Must use Svelte 5 patterns (runes, snippets)
- Production-ready styling (not wireframes)
- Include realistic mock data throughout

---

**To execute**: `/create_plan ai_docs/prompts/2025-12-13-meal-planning-ui.md`
