---
date: 2025-12-16T12:00:00Z
researcher: claude
topic: "Recipe data extraction standards and libraries"
depth: quick
sources_searched: 6
sources_cited: 10
tags: [web-research, schema.org, json-ld, recipe, web-scraping, rust, javascript]
status: complete
---

# Web Research: Recipe Data Extraction Standards

## Question

Is there a standard for presenting recipe information on websites, and what libraries exist to extract this data for a Tauri/Rust application?

## Summary

Yes, **Schema.org Recipe** is the widely-adopted standard for structured recipe data on the web. Most major recipe websites embed this data as **JSON-LD** within their HTML, making extraction reliable. For a Rust/Tauri app, the **reget** crate provides direct recipe extraction, while JavaScript alternatives like **RecipeClipper** offer broader compatibility through ML fallbacks.

## Key Findings

### The Standard: Schema.org Recipe

[Schema.org](https://schema.org/Recipe) defines a **Recipe** type that has become the de facto standard for recipe structured data. It was created in 2011 as a collaboration between Google, Bing, Yahoo, and Yandex.

**Key Properties:**

| Property | Type | Description |
|----------|------|-------------|
| `name` | Text | Recipe title |
| `image` | URL/ImageObject | Photo of the dish |
| `author` | Person/Organization | Recipe creator |
| `description` | Text | Short summary |
| `prepTime` | Duration (ISO 8601) | Preparation time |
| `cookTime` | Duration (ISO 8601) | Cooking time |
| `totalTime` | Duration (ISO 8601) | Total time |
| `recipeYield` | Text | Servings/portions |
| `recipeIngredient` | Text[] | List of ingredients |
| `recipeInstructions` | HowToStep[] | Cooking steps |
| `recipeCategory` | Text | appetizer, entrée, dessert |
| `recipeCuisine` | Text | French, Mexican, etc. |
| `nutrition` | NutritionInformation | Calories, fat, etc. |
| `aggregateRating` | AggregateRating | User ratings |
| `suitableForDiet` | RestrictedDiet | Dietary restrictions |

**All properties are optional** - websites implement varying subsets.

### Data Format: JSON-LD

**JSON-LD** (JavaScript Object Notation for Linked Data) is Google's recommended format and the most common implementation. It's embedded in HTML like this:

```html
<script type="application/ld+json">
{
  "@context": "https://schema.org",
  "@type": "Recipe",
  "name": "Chocolate Chip Cookies",
  "recipeIngredient": ["2 cups flour", "1 cup sugar"],
  "recipeInstructions": [...]
}
</script>
```

**Why JSON-LD is preferred:**
- Separate from HTML content (easy to parse)
- Google recommends it for SEO
- Structured, predictable format
- ~75% of recipe sites use it

### Adoption Reality

One developer found that **75% of recipe websites** include JSON-LD structured data that can be automatically parsed. The remaining 25% require HTML scraping or ML-based extraction.

## Library Options

### Rust Options

#### reget (Recommended for Rust)

A dedicated Rust crate for extracting schema.org recipes from JSON-LD.

```rust
use reget::parse_recipe;

let html = fetch_page_html(url);
if let Some(recipe) = parse_recipe(&html) {
    println!("Recipe: {}", recipe.name);
    println!("Ingredients: {:?}", recipe.ingredients);
}
```

**Pros:**
- Native Rust, integrates well with Tauri
- Simple API: `parse_recipe(html: &str) -> Option<Recipe>`
- Supports markdown export
- 87.5% documentation coverage

**Cons:**
- Only handles JSON-LD (not HTML fallback)
- Less mature than JS alternatives

**Dependencies:** `scraper`, `serde_json`

**Source:** [docs.rs/reget](https://docs.rs/reget/latest/reget/)

#### json-ld (General JSON-LD Processing)

Full JSON-LD processor for Rust supporting expansion, compaction, etc.

**Source:** [crates.io/crates/json-ld](https://crates.io/crates/json-ld)

### JavaScript/TypeScript Options

#### @julianpoy/recipe-clipper (Most Robust)

Powers RecipeSage with dual extraction approach:

1. **CSS Selectors**: Matches common recipe site patterns
2. **ML Fallback**: Uses machine learning for unstructured pages

```javascript
import RecipeClipper from '@julianpoy/recipe-clipper';

const recipe = await RecipeClipper.clipRecipe();
```

**Pros:**
- Highest compatibility across sites
- Works even without JSON-LD
- Active development

**Cons:**
- Requires browser/JSDOM context
- ML features need external endpoint

**Source:** [GitHub - RecipeClipper](https://github.com/julianpoy/RecipeClipper)

#### html-recipe-parser

TypeScript library with types included, focused on JSON-LD parsing.

**Source:** [npm - html-recipe-parser](https://socket.dev/npm/package/html-recipe-parser)

#### @dimfu/recipe-scraper

Supports both JSON-LD and Microdata schemas.

**Source:** npm @dimfu/recipe-scraper

### Python Options (Reference)

#### scrape-schema-recipe

Popular Python library for recipe extraction.

**Source:** [GitHub - scrape-schema-recipe](https://github.com/micahcochran/scrape-schema-recipe)

#### extruct

General structured data extraction (JSON-LD, Microdata, RDFa, Open Graph).

**Source:** [GitHub - extruct](https://github.com/scrapinghub/extruct)

## Options Comparison

| Approach | Pros | Cons | Best For |
|----------|------|------|----------|
| **reget (Rust)** | Native Rust, simple API, fast | JSON-LD only, newer crate | Tauri backend, structured sites |
| **RecipeClipper (JS)** | Highest compatibility, ML fallback | Needs browser context | Maximum site support |
| **HTML parsing (custom)** | Full control | More work, site-specific | Edge cases |

## Recommendations

### For Your Tauri App

**Recommended Architecture:**

1. **Primary: Use `reget` in Rust backend**
   - Parse JSON-LD structured data
   - Handle ~75% of recipe sites
   - Native performance

2. **Fallback: Custom HTML parsing**
   - For sites without JSON-LD
   - Use `scraper` crate to find recipe content
   - Pattern match common selectors

**Implementation Flow:**

```
URL → Fetch HTML → Try reget::parse_recipe()
                        ↓
              Found? → Extract & save to SQLite
                        ↓
              Not found? → Custom HTML parsing or error
```

### Database Schema Suggestion

Based on Schema.org Recipe properties:

```sql
CREATE TABLE recipes (
    id INTEGER PRIMARY KEY,
    source_url TEXT UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    author TEXT,
    image_url TEXT,
    prep_time_minutes INTEGER,
    cook_time_minutes INTEGER,
    total_time_minutes INTEGER,
    servings TEXT,
    cuisine TEXT,
    category TEXT,
    ingredients JSON,  -- Array of strings
    instructions JSON, -- Array of step objects
    nutrition JSON,    -- NutritionInformation object
    rating REAL,
    imported_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

## Sources

1. [Schema.org Recipe Type](https://schema.org/Recipe) — Official schema specification
2. [Google Recipe Structured Data](https://developers.google.com/search/docs/appearance/structured-data/recipe) — Google's implementation guide
3. [reget crate documentation](https://docs.rs/reget/latest/reget/) — Rust recipe extraction library
4. [json-ld crate](https://crates.io/crates/json-ld) — General Rust JSON-LD processor
5. [RecipeClipper](https://github.com/julianpoy/RecipeClipper) — JS library with ML fallback
6. [html-recipe-parser](https://socket.dev/npm/package/html-recipe-parser) — TypeScript JSON-LD parser
7. [scrape-schema-recipe](https://github.com/micahcochran/scrape-schema-recipe) — Python reference implementation
8. [extruct](https://github.com/scrapinghub/extruct) — Python structured data extraction
9. [Scraping Recipes with JSON-LD](https://www.raymondcamden.com/2024/06/12/scraping-recipes-using-nodejs-pipedream-and-json-ld) — Practical implementation guide
10. [Food Blogger Pro - Recipe Schema](https://www.foodbloggerpro.com/blog/what-is-recipe-schema/) — Schema adoption context

## Open Questions

- How to handle recipe sites that require JavaScript rendering (SPAs)?
- Should nutrition data be stored separately or as JSON blob?
- Strategy for handling sites that block automated requests?
