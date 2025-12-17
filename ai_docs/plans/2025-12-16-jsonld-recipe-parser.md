# JSON-LD Recipe Parser Module Implementation Plan

## Overview

Create a Rust module that extracts Schema.org Recipe data from HTML containing JSON-LD structured data, parses it into a `ParsedRecipe` struct compatible with `RecipeInput`, and handles the variety of real-world JSON-LD implementations found on recipe websites.

## Current State

The app has a functional recipe system with `RecipeInput` for creating recipes, but no way to import recipes from external URLs. The `ImportRecipe.svelte` component exists with a stub implementation.

**Key Discoveries**:
- `RecipeInput` at `src-tauri/src/db/recipes.rs:72-84` defines the target structure
- `IngredientInput` at `src-tauri/src/db/recipes.rs:86-94` requires `name`, `quantity` (f64), `unit`, optional `category`/`notes`
- `serde_json` already available (`Cargo.toml:21`)
- `scraper` crate needed for HTML parsing (not yet in dependencies)
- Error pattern uses `AppError` enum at `src-tauri/src/error/mod.rs:7-20`
- Tests use inline `#[cfg(test)]` modules with `include_str!()` for fixtures

## Desired End State

A `parser` module that exposes:
- `parse_recipe_from_html(html: &str) -> Result<ParsedRecipe, ParseError>` - main entry point
- `ParsedRecipe` struct with all fields needed to create a `RecipeInput`
- `ParseError` enum with specific error types for different failure modes

Verification: `cargo test parser` passes with 15+ tests covering all edge cases.

## What We're NOT Doing

- HTTP fetching (Chunk 2)
- Database storage of instructions (Chunk 3)
- Tauri command integration (Chunk 4)
- HTML scraping fallback for sites without JSON-LD
- Nutrition data extraction

## Integration Map

| Type | Location | Notes |
|------|----------|-------|
| Entry point | `src-tauri/src/parser/mod.rs` (new) | New module directory |
| Registration | `src-tauri/src/lib.rs:5` | Add `pub mod parser;` after existing modules |
| Exports | `src-tauri/src/parser/mod.rs` | Export `parse_recipe_from_html`, `ParsedRecipe`, `ParseError` |
| Consumers | Chunk 4 import command | Will call `parser::parse_recipe_from_html` |
| Dependencies | `src-tauri/Cargo.toml:17` | Add `scraper = "0.20"` |
| Events | N/A | None required |

## Implementation Approach

The parser module is organized into submodules for testability:
1. `mod.rs` - Public API, orchestrates parsing flow
2. `jsonld.rs` - JSON-LD extraction and Recipe object finding
3. `duration.rs` - ISO 8601 duration parsing
4. `ingredients.rs` - Ingredient string parsing (quantity/unit/name)

Each submodule has focused tests. HTML fixtures are embedded via `include_str!()` in a `fixtures/` subdirectory.

---

## Phase 1: Module Setup & Error Types

### Goal
Create the parser module structure, add dependencies, and define error types.

### Integration Points

**Depends on**: None
**Produces for next phase**: Module structure, `ParseError` type, `ParsedRecipe` struct

**Wiring required**:
- [ ] Add `scraper` to `src-tauri/Cargo.toml`
- [ ] Add `pub mod parser;` to `src-tauri/src/lib.rs:5`

### Changes

#### Cargo.toml

**File**: `src-tauri/Cargo.toml`

**Change**: Add scraper crate for HTML parsing

```toml
# After serde_json = "1" (line 21)
scraper = "0.20"
```

#### lib.rs Module Registration

**File**: `src-tauri/src/lib.rs`

**Change**: Add parser module declaration

```rust
// After line 8 (pub mod utils;)
pub mod parser;
```

#### Parser Module Structure

**File**: `src-tauri/src/parser/mod.rs` (new)

**Change**: Create main parser module with types and public API

```rust
//! Recipe parser for extracting Schema.org Recipe data from HTML

mod duration;
mod ingredients;
mod jsonld;

use serde::Deserialize;
use thiserror::Error;

/// Error types for recipe parsing
#[derive(Debug, Error)]
pub enum ParseError {
    #[error("No JSON-LD data found on page")]
    NoJsonLdFound,

    #[error("No Recipe found in JSON-LD data")]
    NoRecipeFound,

    #[error("Multiple recipes found on page - unable to determine which to import")]
    MultipleRecipesFound,

    #[error("Recipe data is malformed: {0}")]
    MalformedRecipe(String),
}

/// Parsed recipe ready for conversion to RecipeInput
#[derive(Debug, Clone)]
pub struct ParsedRecipe {
    pub name: String,
    pub description: String,
    pub prep_time: i64,
    pub cook_time: i64,
    pub total_time: i64,
    pub servings: i64,
    pub image_url: Option<String>,
    pub ingredients: Vec<ParsedIngredient>,
    pub instructions: Vec<String>,
    pub author: Option<String>,
    pub category: Option<String>,
    pub cuisine: Option<String>,
}

/// Parsed ingredient with quantity, unit, and name
#[derive(Debug, Clone)]
pub struct ParsedIngredient {
    pub quantity: f64,
    pub unit: String,
    pub name: String,
}

/// Parse a recipe from HTML containing JSON-LD
pub fn parse_recipe_from_html(html: &str) -> Result<ParsedRecipe, ParseError> {
    // Extract JSON-LD blocks from HTML
    let jsonld_blocks = jsonld::extract_jsonld_blocks(html)?;

    // Find Recipe object(s) in JSON-LD
    let recipe_json = jsonld::find_recipe_object(&jsonld_blocks)?;

    // Parse the Recipe JSON into ParsedRecipe
    parse_recipe_json(&recipe_json)
}

/// Parse a serde_json::Value containing a Recipe into ParsedRecipe
fn parse_recipe_json(json: &serde_json::Value) -> Result<ParsedRecipe, ParseError> {
    // Extract required name field
    let name = json
        .get("name")
        .and_then(|v| v.as_str())
        .ok_or_else(|| ParseError::MalformedRecipe("missing name".to_string()))?
        .to_string();

    // Extract ingredients (required)
    let ingredients_raw = json
        .get("recipeIngredient")
        .and_then(|v| v.as_array())
        .ok_or_else(|| ParseError::MalformedRecipe("missing ingredients".to_string()))?;

    let ingredients: Vec<ParsedIngredient> = ingredients_raw
        .iter()
        .filter_map(|v| v.as_str())
        .map(|s| ingredients::parse_ingredient(s))
        .collect();

    if ingredients.is_empty() {
        return Err(ParseError::MalformedRecipe("no valid ingredients".to_string()));
    }

    // Extract instructions (required)
    let instructions = parse_instructions(json)?;

    if instructions.is_empty() {
        return Err(ParseError::MalformedRecipe("missing instructions".to_string()));
    }

    // Extract optional fields
    let description = json
        .get("description")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    let prep_time = json
        .get("prepTime")
        .and_then(|v| v.as_str())
        .map(duration::parse_iso8601_duration)
        .unwrap_or(0);

    let cook_time = json
        .get("cookTime")
        .and_then(|v| v.as_str())
        .map(duration::parse_iso8601_duration)
        .unwrap_or(0);

    let total_time = json
        .get("totalTime")
        .and_then(|v| v.as_str())
        .map(duration::parse_iso8601_duration)
        .unwrap_or(0);

    let servings = parse_servings(json);

    let image_url = parse_image(json);

    let author = json
        .get("author")
        .and_then(|v| {
            if v.is_string() {
                v.as_str().map(|s| s.to_string())
            } else {
                v.get("name").and_then(|n| n.as_str()).map(|s| s.to_string())
            }
        });

    let category = json
        .get("recipeCategory")
        .and_then(|v| {
            if v.is_string() {
                v.as_str().map(|s| s.to_string())
            } else if v.is_array() {
                v.as_array().and_then(|arr| arr.first()).and_then(|v| v.as_str()).map(|s| s.to_string())
            } else {
                None
            }
        });

    let cuisine = json
        .get("recipeCuisine")
        .and_then(|v| {
            if v.is_string() {
                v.as_str().map(|s| s.to_string())
            } else if v.is_array() {
                v.as_array().and_then(|arr| arr.first()).and_then(|v| v.as_str()).map(|s| s.to_string())
            } else {
                None
            }
        });

    Ok(ParsedRecipe {
        name,
        description,
        prep_time,
        cook_time,
        total_time,
        servings,
        image_url,
        ingredients,
        instructions,
        author,
        category,
        cuisine,
    })
}

/// Parse recipeInstructions field (handles string[], HowToStep[], HowToSection[])
fn parse_instructions(json: &serde_json::Value) -> Result<Vec<String>, ParseError> {
    let instructions_value = json.get("recipeInstructions");

    match instructions_value {
        None => Ok(vec![]),
        Some(v) if v.is_string() => {
            // Single string - split on periods or return as-is
            let text = v.as_str().unwrap();
            Ok(vec![text.to_string()])
        }
        Some(v) if v.is_array() => {
            let arr = v.as_array().unwrap();
            let mut steps = Vec::new();

            for item in arr {
                if item.is_string() {
                    // Plain string array
                    steps.push(item.as_str().unwrap().to_string());
                } else if let Some(item_type) = item.get("@type").and_then(|t| t.as_str()) {
                    match item_type {
                        "HowToStep" => {
                            if let Some(text) = item.get("text").and_then(|t| t.as_str()) {
                                steps.push(text.to_string());
                            }
                        }
                        "HowToSection" => {
                            // Flatten section items
                            if let Some(items) = item.get("itemListElement").and_then(|i| i.as_array()) {
                                for section_item in items {
                                    if let Some(text) = section_item.get("text").and_then(|t| t.as_str()) {
                                        steps.push(text.to_string());
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }

            Ok(steps)
        }
        _ => Ok(vec![]),
    }
}

/// Parse recipeYield/servings field
fn parse_servings(json: &serde_json::Value) -> i64 {
    // Try recipeYield first
    if let Some(yield_val) = json.get("recipeYield") {
        if let Some(s) = yield_val.as_str() {
            // Extract first number from string like "4 servings" or "4-6"
            if let Some(num) = extract_first_number(s) {
                return num;
            }
        } else if let Some(n) = yield_val.as_i64() {
            return n;
        } else if let Some(arr) = yield_val.as_array() {
            // Some sites use array, take first
            if let Some(first) = arr.first() {
                if let Some(s) = first.as_str() {
                    if let Some(num) = extract_first_number(s) {
                        return num;
                    }
                }
            }
        }
    }

    // Default to 4 servings
    4
}

/// Parse image field (handles string, array, ImageObject)
fn parse_image(json: &serde_json::Value) -> Option<String> {
    let image_val = json.get("image")?;

    if let Some(url) = image_val.as_str() {
        return validate_url(url);
    }

    if let Some(arr) = image_val.as_array() {
        // Take first valid URL
        for item in arr {
            if let Some(url) = item.as_str() {
                if let Some(valid) = validate_url(url) {
                    return Some(valid);
                }
            } else if let Some(url) = item.get("url").and_then(|u| u.as_str()) {
                if let Some(valid) = validate_url(url) {
                    return Some(valid);
                }
            }
        }
    }

    // ImageObject
    if let Some(url) = image_val.get("url").and_then(|u| u.as_str()) {
        return validate_url(url);
    }

    None
}

/// Validate URL format (must be http/https)
fn validate_url(url: &str) -> Option<String> {
    if url.starts_with("http://") || url.starts_with("https://") {
        Some(url.to_string())
    } else {
        None
    }
}

/// Extract first number from a string
fn extract_first_number(s: &str) -> Option<i64> {
    let num_str: String = s.chars().take_while(|c| c.is_ascii_digit()).collect();
    if num_str.is_empty() {
        // Try finding a number anywhere in the string
        let mut num_chars = String::new();
        for c in s.chars() {
            if c.is_ascii_digit() {
                num_chars.push(c);
            } else if !num_chars.is_empty() {
                break;
            }
        }
        num_chars.parse().ok()
    } else {
        num_str.parse().ok()
    }
}
```

#### Duration Parsing Submodule

**File**: `src-tauri/src/parser/duration.rs` (new)

**Change**: Create ISO 8601 duration parser

```rust
//! ISO 8601 duration parsing

/// Parse ISO 8601 duration string (PT30M, PT1H15M) to minutes
pub fn parse_iso8601_duration(duration: &str) -> i64 {
    // Handle empty or invalid strings
    if !duration.starts_with("PT") && !duration.starts_with("P") {
        return 0;
    }

    let mut minutes: i64 = 0;
    let mut current_num = String::new();

    // Skip the P prefix
    let chars: Vec<char> = duration.chars().collect();
    let mut i = 1; // Skip 'P'

    // Skip 'T' if present (for time component)
    if i < chars.len() && chars[i] == 'T' {
        i += 1;
    }

    while i < chars.len() {
        let c = chars[i];
        if c.is_ascii_digit() {
            current_num.push(c);
        } else {
            let num: i64 = current_num.parse().unwrap_or(0);
            current_num.clear();

            match c {
                'H' => minutes += num * 60,
                'M' => minutes += num,
                'S' => {} // Ignore seconds
                _ => {}
            }
        }
        i += 1;
    }

    minutes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_minutes_only() {
        assert_eq!(parse_iso8601_duration("PT30M"), 30);
        assert_eq!(parse_iso8601_duration("PT5M"), 5);
        assert_eq!(parse_iso8601_duration("PT120M"), 120);
    }

    #[test]
    fn test_parse_hours_only() {
        assert_eq!(parse_iso8601_duration("PT1H"), 60);
        assert_eq!(parse_iso8601_duration("PT2H"), 120);
    }

    #[test]
    fn test_parse_hours_and_minutes() {
        assert_eq!(parse_iso8601_duration("PT1H30M"), 90);
        assert_eq!(parse_iso8601_duration("PT2H15M"), 135);
        assert_eq!(parse_iso8601_duration("PT1H45M"), 105);
    }

    #[test]
    fn test_parse_with_seconds() {
        // Seconds are ignored
        assert_eq!(parse_iso8601_duration("PT30M30S"), 30);
        assert_eq!(parse_iso8601_duration("PT1H30M45S"), 90);
    }

    #[test]
    fn test_invalid_duration() {
        assert_eq!(parse_iso8601_duration(""), 0);
        assert_eq!(parse_iso8601_duration("invalid"), 0);
        assert_eq!(parse_iso8601_duration("30"), 0);
    }
}
```

#### Ingredients Parsing Submodule

**File**: `src-tauri/src/parser/ingredients.rs` (new)

**Change**: Create ingredient string parser

```rust
//! Ingredient string parsing

use super::ParsedIngredient;

/// Parse an ingredient string like "2 cups flour" into components
pub fn parse_ingredient(input: &str) -> ParsedIngredient {
    let input = input.trim();

    // Try to parse quantity at the start
    let (quantity, rest) = parse_quantity(input);

    // Try to parse unit from the rest
    let (unit, name) = parse_unit(rest.trim());

    ParsedIngredient {
        quantity,
        unit: unit.to_string(),
        name: name.trim().to_string(),
    }
}

/// Parse quantity from the start of a string, handling fractions
fn parse_quantity(input: &str) -> (f64, &str) {
    let mut chars = input.char_indices().peekable();
    let mut end_idx = 0;
    let mut found_number = false;

    // Skip leading whitespace
    while let Some(&(_, c)) = chars.peek() {
        if !c.is_whitespace() {
            break;
        }
        chars.next();
    }

    // Collect number characters (digits, fractions, decimals, spaces between)
    let start_idx = chars.peek().map(|(i, _)| *i).unwrap_or(0);

    while let Some(&(idx, c)) = chars.peek() {
        if c.is_ascii_digit() || c == '/' || c == '.' || c == '-' {
            found_number = true;
            end_idx = idx + c.len_utf8();
            chars.next();
        } else if c.is_whitespace() && found_number {
            // Check if next non-space is a digit or fraction (for "1 1/2")
            let mut lookahead = chars.clone();
            lookahead.next(); // skip space
            if let Some(&(_, next_c)) = lookahead.peek() {
                if next_c.is_ascii_digit() {
                    end_idx = idx + c.len_utf8();
                    chars.next();
                } else {
                    break;
                }
            } else {
                break;
            }
        } else {
            break;
        }
    }

    if !found_number {
        return (0.0, input);
    }

    let num_str = &input[start_idx..end_idx];
    let quantity = parse_number_string(num_str);
    let rest = &input[end_idx..];

    (quantity, rest)
}

/// Parse a number string that may contain fractions
fn parse_number_string(s: &str) -> f64 {
    let s = s.trim();

    // Handle range (3-4) - take first number
    if let Some(dash_idx) = s.find('-') {
        if dash_idx > 0 {
            return parse_number_string(&s[..dash_idx]);
        }
    }

    // Handle mixed fraction (1 1/2)
    let parts: Vec<&str> = s.split_whitespace().collect();
    if parts.len() == 2 {
        let whole: f64 = parts[0].parse().unwrap_or(0.0);
        let frac = parse_fraction(parts[1]);
        return whole + frac;
    }

    // Handle simple fraction (1/2)
    if s.contains('/') {
        return parse_fraction(s);
    }

    // Handle decimal or integer
    s.parse().unwrap_or(0.0)
}

/// Parse a fraction string like "1/2" to decimal
fn parse_fraction(s: &str) -> f64 {
    let parts: Vec<&str> = s.split('/').collect();
    if parts.len() == 2 {
        let num: f64 = parts[0].trim().parse().unwrap_or(0.0);
        let den: f64 = parts[1].trim().parse().unwrap_or(1.0);
        if den != 0.0 {
            return num / den;
        }
    }
    0.0
}

/// Parse unit from the start of a string
fn parse_unit(input: &str) -> (&str, &str) {
    let input = input.trim();
    let lower = input.to_lowercase();

    // Common units to recognize
    let units = [
        // Volume
        "cups", "cup", "c",
        "tablespoons", "tablespoon", "tbsp", "tbs", "tb",
        "teaspoons", "teaspoon", "tsp", "ts",
        "fluid ounces", "fluid ounce", "fl oz",
        "milliliters", "milliliter", "ml",
        "liters", "liter", "l",
        "pints", "pint", "pt",
        "quarts", "quart", "qt",
        "gallons", "gallon", "gal",
        // Weight
        "pounds", "pound", "lbs", "lb",
        "ounces", "ounce", "oz",
        "kilograms", "kilogram", "kg",
        "grams", "gram", "g",
        // Count
        "cloves", "clove",
        "slices", "slice",
        "pieces", "piece",
        "cans", "can",
        "bunches", "bunch",
        "heads", "head",
        "stalks", "stalk",
        "sprigs", "sprig",
        "packages", "package", "pkg",
        "pinches", "pinch",
        "dashes", "dash",
        // Size modifiers that act as units
        "large", "medium", "small",
    ];

    for unit in &units {
        if lower.starts_with(unit) {
            let unit_len = unit.len();
            // Make sure it's a word boundary
            let next_char = input.chars().nth(unit_len);
            if next_char.is_none() || next_char.unwrap().is_whitespace() || next_char.unwrap() == ',' {
                return (&input[..unit_len], &input[unit_len..]);
            }
        }
    }

    // No recognized unit - check if first word looks like a unit (parenthetical)
    if input.starts_with('(') {
        // Handle "(15 oz) can" - skip parenthetical
        if let Some(close_paren) = input.find(')') {
            let after = &input[close_paren + 1..].trim_start();
            return parse_unit(after);
        }
    }

    ("", input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_ingredient() {
        let result = parse_ingredient("2 cups flour");
        assert_eq!(result.quantity, 2.0);
        assert_eq!(result.unit, "cups");
        assert_eq!(result.name, "flour");
    }

    #[test]
    fn test_fraction_ingredient() {
        let result = parse_ingredient("1/2 tsp salt");
        assert_eq!(result.quantity, 0.5);
        assert_eq!(result.unit, "tsp");
        assert_eq!(result.name, "salt");
    }

    #[test]
    fn test_mixed_fraction() {
        let result = parse_ingredient("1 1/2 cups sugar");
        assert_eq!(result.quantity, 1.5);
        assert_eq!(result.unit, "cups");
        assert_eq!(result.name, "sugar");
    }

    #[test]
    fn test_range_quantity() {
        let result = parse_ingredient("3-4 cloves garlic");
        assert_eq!(result.quantity, 3.0);
        assert_eq!(result.unit, "cloves");
        assert_eq!(result.name, "garlic");
    }

    #[test]
    fn test_no_quantity() {
        let result = parse_ingredient("salt to taste");
        assert_eq!(result.quantity, 0.0);
        assert_eq!(result.unit, "");
        assert_eq!(result.name, "salt to taste");
    }

    #[test]
    fn test_no_unit() {
        let result = parse_ingredient("2 eggs");
        assert_eq!(result.quantity, 2.0);
        assert_eq!(result.unit, "");
        assert_eq!(result.name, "eggs");
    }

    #[test]
    fn test_parenthetical_unit() {
        let result = parse_ingredient("1 (15 oz) can beans");
        assert_eq!(result.quantity, 1.0);
        assert_eq!(result.unit, "can");
        assert_eq!(result.name, "beans");
    }

    #[test]
    fn test_decimal_quantity() {
        let result = parse_ingredient("0.5 lb ground beef");
        assert_eq!(result.quantity, 0.5);
        assert_eq!(result.unit, "lb");
        assert_eq!(result.name, "ground beef");
    }
}
```

#### JSON-LD Extraction Submodule

**File**: `src-tauri/src/parser/jsonld.rs` (new)

**Change**: Create JSON-LD extraction and Recipe finding logic

```rust
//! JSON-LD extraction from HTML

use super::ParseError;
use scraper::{Html, Selector};

/// Extract all JSON-LD blocks from HTML
pub fn extract_jsonld_blocks(html: &str) -> Result<Vec<serde_json::Value>, ParseError> {
    let document = Html::parse_document(html);
    let selector = Selector::parse(r#"script[type="application/ld+json"]"#)
        .expect("Invalid selector");

    let mut blocks = Vec::new();

    for element in document.select(&selector) {
        let json_text = element.inner_html();
        if let Ok(value) = serde_json::from_str::<serde_json::Value>(&json_text) {
            blocks.push(value);
        }
    }

    if blocks.is_empty() {
        return Err(ParseError::NoJsonLdFound);
    }

    Ok(blocks)
}

/// Find Recipe object(s) in JSON-LD blocks
pub fn find_recipe_object(blocks: &[serde_json::Value]) -> Result<serde_json::Value, ParseError> {
    let mut recipes = Vec::new();

    for block in blocks {
        find_recipes_in_value(block, &mut recipes);
    }

    match recipes.len() {
        0 => Err(ParseError::NoRecipeFound),
        1 => Ok(recipes.remove(0)),
        _ => Err(ParseError::MultipleRecipesFound),
    }
}

/// Recursively find Recipe objects in a JSON value
fn find_recipes_in_value(value: &serde_json::Value, recipes: &mut Vec<serde_json::Value>) {
    match value {
        serde_json::Value::Object(obj) => {
            // Check if this object is a Recipe
            if is_recipe_type(value) {
                recipes.push(value.clone());
                return;
            }

            // Check @graph array
            if let Some(graph) = obj.get("@graph") {
                if let Some(arr) = graph.as_array() {
                    for item in arr {
                        find_recipes_in_value(item, recipes);
                    }
                }
            }
        }
        serde_json::Value::Array(arr) => {
            for item in arr {
                find_recipes_in_value(item, recipes);
            }
        }
        _ => {}
    }
}

/// Check if a JSON value has @type "Recipe" (handles array types)
fn is_recipe_type(value: &serde_json::Value) -> bool {
    if let Some(type_val) = value.get("@type") {
        if let Some(type_str) = type_val.as_str() {
            return type_str == "Recipe";
        }
        if let Some(type_arr) = type_val.as_array() {
            return type_arr.iter().any(|t| t.as_str() == Some("Recipe"));
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_simple_jsonld() {
        let html = r#"
            <html>
            <head>
                <script type="application/ld+json">
                {"@type": "Recipe", "name": "Test"}
                </script>
            </head>
            </html>
        "#;

        let blocks = extract_jsonld_blocks(html).unwrap();
        assert_eq!(blocks.len(), 1);
    }

    #[test]
    fn test_no_jsonld() {
        let html = "<html><body>No JSON-LD here</body></html>";
        let result = extract_jsonld_blocks(html);
        assert!(matches!(result, Err(ParseError::NoJsonLdFound)));
    }

    #[test]
    fn test_find_recipe_simple() {
        let blocks = vec![
            serde_json::json!({"@type": "Recipe", "name": "Test Recipe"})
        ];
        let recipe = find_recipe_object(&blocks).unwrap();
        assert_eq!(recipe.get("name").unwrap().as_str().unwrap(), "Test Recipe");
    }

    #[test]
    fn test_find_recipe_array_type() {
        let blocks = vec![
            serde_json::json!({"@type": ["Recipe", "HowTo"], "name": "Test"})
        ];
        let recipe = find_recipe_object(&blocks).unwrap();
        assert_eq!(recipe.get("name").unwrap().as_str().unwrap(), "Test");
    }

    #[test]
    fn test_find_recipe_in_graph() {
        let blocks = vec![
            serde_json::json!({
                "@context": "https://schema.org",
                "@graph": [
                    {"@type": "WebPage", "name": "Page"},
                    {"@type": "Recipe", "name": "Graph Recipe"}
                ]
            })
        ];
        let recipe = find_recipe_object(&blocks).unwrap();
        assert_eq!(recipe.get("name").unwrap().as_str().unwrap(), "Graph Recipe");
    }

    #[test]
    fn test_no_recipe_found() {
        let blocks = vec![
            serde_json::json!({"@type": "Article", "name": "Not a recipe"})
        ];
        let result = find_recipe_object(&blocks);
        assert!(matches!(result, Err(ParseError::NoRecipeFound)));
    }

    #[test]
    fn test_multiple_recipes_error() {
        let blocks = vec![
            serde_json::json!({"@type": "Recipe", "name": "Recipe 1"}),
            serde_json::json!({"@type": "Recipe", "name": "Recipe 2"})
        ];
        let result = find_recipe_object(&blocks);
        assert!(matches!(result, Err(ParseError::MultipleRecipesFound)));
    }
}
```

### Success Criteria

#### Automated Verification
- [ ] `cargo check -p feast` compiles without errors
- [ ] `cargo clippy -p feast` passes with no warnings

#### Integration Verification
- [ ] `parser` module importable from `feast_lib::parser`
- [ ] `ParseError`, `ParsedRecipe`, `ParsedIngredient` exported from module

#### Manual Verification
- [ ] None required for Phase 1

**Checkpoint**: Run `cargo check` and verify compilation before proceeding to Phase 2.

---

## Phase 2: HTML Fixtures & Integration Tests

### Goal
Create HTML test fixtures based on real recipe site structures and add integration tests for the full parsing flow.

### Integration Points

**Consumes from Phase 1**: `parse_recipe_from_html`, all types
**Produces for next phase**: Verified working parser

**Wiring required**:
- [ ] Create fixtures directory at `src-tauri/src/parser/fixtures/`
- [ ] Add `include_str!()` macros for fixture loading

### Changes

#### Fixtures Directory

**Directory**: `src-tauri/src/parser/fixtures/` (new)

Create HTML fixture files:

**File**: `src-tauri/src/parser/fixtures/minimal.html`

```html
<!DOCTYPE html>
<html>
<head>
    <script type="application/ld+json">
    {
        "@context": "https://schema.org",
        "@type": "Recipe",
        "name": "Simple Pasta",
        "recipeIngredient": [
            "1 lb pasta",
            "2 cups marinara sauce"
        ],
        "recipeInstructions": [
            "Boil pasta according to package directions.",
            "Heat sauce in a separate pan.",
            "Combine pasta and sauce."
        ]
    }
    </script>
</head>
<body></body>
</html>
```

**File**: `src-tauri/src/parser/fixtures/full_recipe.html`

```html
<!DOCTYPE html>
<html>
<head>
    <script type="application/ld+json">
    {
        "@context": "https://schema.org",
        "@type": "Recipe",
        "name": "Classic Chocolate Chip Cookies",
        "description": "The best homemade chocolate chip cookies recipe.",
        "author": {"@type": "Person", "name": "Jane Baker"},
        "image": "https://example.com/cookies.jpg",
        "prepTime": "PT15M",
        "cookTime": "PT12M",
        "totalTime": "PT27M",
        "recipeYield": "24 cookies",
        "recipeCategory": "Dessert",
        "recipeCuisine": "American",
        "recipeIngredient": [
            "2 1/4 cups all-purpose flour",
            "1 tsp baking soda",
            "1 tsp salt",
            "1 cup butter, softened",
            "3/4 cup granulated sugar",
            "3/4 cup packed brown sugar",
            "2 large eggs",
            "2 tsp vanilla extract",
            "2 cups chocolate chips"
        ],
        "recipeInstructions": [
            {"@type": "HowToStep", "text": "Preheat oven to 375°F."},
            {"@type": "HowToStep", "text": "Mix flour, baking soda, and salt in a bowl."},
            {"@type": "HowToStep", "text": "Beat butter and sugars until creamy."},
            {"@type": "HowToStep", "text": "Add eggs and vanilla to butter mixture."},
            {"@type": "HowToStep", "text": "Gradually blend in flour mixture."},
            {"@type": "HowToStep", "text": "Stir in chocolate chips."},
            {"@type": "HowToStep", "text": "Drop rounded tablespoons onto ungreased baking sheets."},
            {"@type": "HowToStep", "text": "Bake for 9 to 11 minutes or until golden brown."}
        ]
    }
    </script>
</head>
<body></body>
</html>
```

**File**: `src-tauri/src/parser/fixtures/graph_structure.html`

```html
<!DOCTYPE html>
<html>
<head>
    <script type="application/ld+json">
    {
        "@context": "https://schema.org",
        "@graph": [
            {
                "@type": "WebPage",
                "name": "Recipe Page",
                "url": "https://example.com/recipe"
            },
            {
                "@type": "Recipe",
                "name": "Garlic Bread",
                "recipeIngredient": [
                    "1 loaf French bread",
                    "1/2 cup butter",
                    "3-4 cloves garlic, minced"
                ],
                "recipeInstructions": ["Mix butter and garlic.", "Spread on bread.", "Bake at 375°F for 10 minutes."]
            }
        ]
    }
    </script>
</head>
<body></body>
</html>
```

**File**: `src-tauri/src/parser/fixtures/howto_sections.html`

```html
<!DOCTYPE html>
<html>
<head>
    <script type="application/ld+json">
    {
        "@context": "https://schema.org",
        "@type": "Recipe",
        "name": "Layered Dip",
        "recipeIngredient": ["1 can refried beans", "1 cup guacamole", "1 cup sour cream"],
        "recipeInstructions": [
            {
                "@type": "HowToSection",
                "name": "Base Layer",
                "itemListElement": [
                    {"@type": "HowToStep", "text": "Spread beans in dish."},
                    {"@type": "HowToStep", "text": "Add guacamole layer."}
                ]
            },
            {
                "@type": "HowToSection",
                "name": "Top Layer",
                "itemListElement": [
                    {"@type": "HowToStep", "text": "Spread sour cream on top."},
                    {"@type": "HowToStep", "text": "Refrigerate before serving."}
                ]
            }
        ]
    }
    </script>
</head>
<body></body>
</html>
```

**File**: `src-tauri/src/parser/fixtures/array_type.html`

```html
<!DOCTYPE html>
<html>
<head>
    <script type="application/ld+json">
    {
        "@context": "https://schema.org",
        "@type": ["Recipe", "HowTo"],
        "name": "Quick Salad",
        "recipeIngredient": ["4 cups mixed greens", "1/4 cup dressing"],
        "recipeInstructions": ["Toss greens with dressing."]
    }
    </script>
</head>
<body></body>
</html>
```

#### Integration Tests

**File**: `src-tauri/src/parser/mod.rs` (append to existing)

**Change**: Add integration tests at the bottom of the file

```rust
// Add to the bottom of mod.rs

#[cfg(test)]
mod tests {
    use super::*;

    const MINIMAL_HTML: &str = include_str!("fixtures/minimal.html");
    const FULL_RECIPE_HTML: &str = include_str!("fixtures/full_recipe.html");
    const GRAPH_HTML: &str = include_str!("fixtures/graph_structure.html");
    const SECTIONS_HTML: &str = include_str!("fixtures/howto_sections.html");
    const ARRAY_TYPE_HTML: &str = include_str!("fixtures/array_type.html");

    #[test]
    fn test_parse_minimal_recipe() {
        let result = parse_recipe_from_html(MINIMAL_HTML).unwrap();
        assert_eq!(result.name, "Simple Pasta");
        assert_eq!(result.ingredients.len(), 2);
        assert_eq!(result.instructions.len(), 3);
    }

    #[test]
    fn test_parse_full_recipe() {
        let result = parse_recipe_from_html(FULL_RECIPE_HTML).unwrap();
        assert_eq!(result.name, "Classic Chocolate Chip Cookies");
        assert_eq!(result.description, "The best homemade chocolate chip cookies recipe.");
        assert_eq!(result.prep_time, 15);
        assert_eq!(result.cook_time, 12);
        assert_eq!(result.total_time, 27);
        assert_eq!(result.servings, 24);
        assert_eq!(result.image_url, Some("https://example.com/cookies.jpg".to_string()));
        assert_eq!(result.author, Some("Jane Baker".to_string()));
        assert_eq!(result.category, Some("Dessert".to_string()));
        assert_eq!(result.cuisine, Some("American".to_string()));
        assert_eq!(result.ingredients.len(), 9);
        assert_eq!(result.instructions.len(), 8);

        // Check ingredient parsing
        let flour = &result.ingredients[0];
        assert_eq!(flour.quantity, 2.25);
        assert_eq!(flour.unit, "cups");
        assert!(flour.name.contains("flour"));
    }

    #[test]
    fn test_parse_graph_structure() {
        let result = parse_recipe_from_html(GRAPH_HTML).unwrap();
        assert_eq!(result.name, "Garlic Bread");
        assert_eq!(result.ingredients.len(), 3);

        // Check range parsing (3-4 cloves)
        let garlic = &result.ingredients[2];
        assert_eq!(garlic.quantity, 3.0);
    }

    #[test]
    fn test_parse_howto_sections() {
        let result = parse_recipe_from_html(SECTIONS_HTML).unwrap();
        assert_eq!(result.name, "Layered Dip");
        // HowToSections should be flattened
        assert_eq!(result.instructions.len(), 4);
    }

    #[test]
    fn test_parse_array_type() {
        let result = parse_recipe_from_html(ARRAY_TYPE_HTML).unwrap();
        assert_eq!(result.name, "Quick Salad");
    }

    #[test]
    fn test_error_no_jsonld() {
        let html = "<html><body>No recipe here</body></html>";
        let result = parse_recipe_from_html(html);
        assert!(matches!(result, Err(ParseError::NoJsonLdFound)));
    }

    #[test]
    fn test_error_no_recipe() {
        let html = r#"
            <html><head>
            <script type="application/ld+json">
            {"@type": "Article", "name": "Not a recipe"}
            </script>
            </head></html>
        "#;
        let result = parse_recipe_from_html(html);
        assert!(matches!(result, Err(ParseError::NoRecipeFound)));
    }

    #[test]
    fn test_error_missing_name() {
        let html = r#"
            <html><head>
            <script type="application/ld+json">
            {"@type": "Recipe", "recipeIngredient": ["1 cup flour"], "recipeInstructions": ["Mix"]}
            </script>
            </head></html>
        "#;
        let result = parse_recipe_from_html(html);
        assert!(matches!(result, Err(ParseError::MalformedRecipe(_))));
    }

    #[test]
    fn test_error_missing_ingredients() {
        let html = r#"
            <html><head>
            <script type="application/ld+json">
            {"@type": "Recipe", "name": "No Ingredients", "recipeInstructions": ["Do something"]}
            </script>
            </head></html>
        "#;
        let result = parse_recipe_from_html(html);
        assert!(matches!(result, Err(ParseError::MalformedRecipe(_))));
    }

    #[test]
    fn test_image_url_validation() {
        // Test that invalid URLs are skipped
        let html = r#"
            <html><head>
            <script type="application/ld+json">
            {
                "@type": "Recipe",
                "name": "Test",
                "image": "/relative/path.jpg",
                "recipeIngredient": ["1 egg"],
                "recipeInstructions": ["Cook"]
            }
            </script>
            </head></html>
        "#;
        let result = parse_recipe_from_html(html).unwrap();
        assert!(result.image_url.is_none()); // Relative URLs should be rejected
    }
}
```

### Success Criteria

#### Automated Verification
- [ ] `cargo test parser` passes all tests
- [ ] `cargo clippy -p feast` passes with no warnings

#### Integration Verification
- [ ] All 5 fixture files load correctly via `include_str!()`
- [ ] Tests cover: minimal recipe, full recipe, @graph, HowToSection, array @type
- [ ] Error tests cover: no JSON-LD, no recipe, missing name, missing ingredients

#### Manual Verification
- [ ] Review test output for any edge case failures

**Checkpoint**: Run `cargo test parser` and verify all tests pass before proceeding.

---

## Phase 3: Edge Case Handling & Polish

### Goal
Handle additional edge cases discovered during testing and ensure robust error messages.

### Integration Points

**Consumes from Phase 2**: Working parser with tests
**Produces**: Production-ready parser module

**Wiring required**:
- [ ] None - internal improvements only

### Changes

#### Enhanced Error Messages

**File**: `src-tauri/src/parser/mod.rs`

**Change**: Improve error message specificity

The error types already provide clear messages. Verify that:
- `NoJsonLdFound` - "No JSON-LD data found on page"
- `NoRecipeFound` - "No Recipe found in JSON-LD data"
- `MultipleRecipesFound` - "Multiple recipes found on page - unable to determine which to import"
- `MalformedRecipe(msg)` - Includes specific field that's missing/invalid

#### Additional Edge Case Fixtures

**File**: `src-tauri/src/parser/fixtures/image_variations.html` (new)

```html
<!DOCTYPE html>
<html>
<head>
    <script type="application/ld+json">
    {
        "@context": "https://schema.org",
        "@type": "Recipe",
        "name": "Image Test",
        "image": [
            {"@type": "ImageObject", "url": "https://example.com/image1.jpg"},
            "https://example.com/image2.jpg"
        ],
        "recipeIngredient": ["1 item"],
        "recipeInstructions": ["Do thing"]
    }
    </script>
</head>
<body></body>
</html>
```

**File**: `src-tauri/src/parser/fixtures/string_instructions.html` (new)

```html
<!DOCTYPE html>
<html>
<head>
    <script type="application/ld+json">
    {
        "@context": "https://schema.org",
        "@type": "Recipe",
        "name": "Single String Instructions",
        "recipeIngredient": ["2 cups rice", "4 cups water"],
        "recipeInstructions": "Combine rice and water. Bring to boil. Simmer for 20 minutes."
    }
    </script>
</head>
<body></body>
</html>
```

#### Additional Tests

**File**: `src-tauri/src/parser/mod.rs` (append to tests module)

```rust
// Add these tests to the existing tests module

const IMAGE_VARIATIONS_HTML: &str = include_str!("fixtures/image_variations.html");
const STRING_INSTRUCTIONS_HTML: &str = include_str!("fixtures/string_instructions.html");

#[test]
fn test_image_object_array() {
    let result = parse_recipe_from_html(IMAGE_VARIATIONS_HTML).unwrap();
    // Should extract first valid URL from ImageObject array
    assert_eq!(result.image_url, Some("https://example.com/image1.jpg".to_string()));
}

#[test]
fn test_string_instructions() {
    let result = parse_recipe_from_html(STRING_INSTRUCTIONS_HTML).unwrap();
    assert_eq!(result.name, "Single String Instructions");
    // Single string instruction should be kept as one item
    assert_eq!(result.instructions.len(), 1);
}

#[test]
fn test_multiple_recipes_error() {
    let html = r#"
        <html><head>
        <script type="application/ld+json">
        {"@type": "Recipe", "name": "Recipe 1", "recipeIngredient": ["1 egg"], "recipeInstructions": ["Cook"]}
        </script>
        <script type="application/ld+json">
        {"@type": "Recipe", "name": "Recipe 2", "recipeIngredient": ["1 egg"], "recipeInstructions": ["Cook"]}
        </script>
        </head></html>
    "#;
    let result = parse_recipe_from_html(html);
    assert!(matches!(result, Err(ParseError::MultipleRecipesFound)));
}

#[test]
fn test_empty_ingredients_array() {
    let html = r#"
        <html><head>
        <script type="application/ld+json">
        {"@type": "Recipe", "name": "Empty", "recipeIngredient": [], "recipeInstructions": ["Do"]}
        </script>
        </head></html>
    "#;
    let result = parse_recipe_from_html(html);
    assert!(matches!(result, Err(ParseError::MalformedRecipe(_))));
}

#[test]
fn test_servings_variations() {
    // Test "4 servings" string format
    let html = r#"
        <html><head>
        <script type="application/ld+json">
        {
            "@type": "Recipe",
            "name": "Test",
            "recipeYield": "6 servings",
            "recipeIngredient": ["1 egg"],
            "recipeInstructions": ["Cook"]
        }
        </script>
        </head></html>
    "#;
    let result = parse_recipe_from_html(html).unwrap();
    assert_eq!(result.servings, 6);
}
```

### Success Criteria

#### Automated Verification
- [ ] `cargo test parser` passes all tests (15+ tests)
- [ ] `cargo clippy -p feast` passes with no warnings
- [ ] `cargo fmt --check` passes

#### Integration Verification
- [ ] Parser handles all documented edge cases from spec
- [ ] Error messages are clear and actionable

#### Manual Verification
- [ ] Test against 2-3 saved HTML files from real recipe websites

**Checkpoint**: Run full test suite and verify coverage before marking complete.

---

## Testing Strategy

### Unit Tests
- Duration parsing (5 tests in `duration.rs`)
- Ingredient parsing (8 tests in `ingredients.rs`)
- JSON-LD extraction (6 tests in `jsonld.rs`)

### Integration Tests
- Full HTML parsing (10+ tests in `mod.rs`)
- Edge cases: minimal, full, @graph, HowToSection, array @type
- Error cases: no JSON-LD, no recipe, missing fields, multiple recipes

### Manual Testing Checklist
1. [ ] Save HTML from allrecipes.com and verify parsing
2. [ ] Save HTML from seriouseats.com and verify parsing
3. [ ] Save HTML from food.com and verify parsing
4. [ ] Test error messages display correctly in logs

## Rollback Plan

No database changes or feature flags involved.

```
Git revert to commit before Phase 1: `git revert --no-commit HEAD~N..HEAD`
```

Or simply delete the `src-tauri/src/parser/` directory and remove the module declaration from `lib.rs`.

## Migration Notes

- **Data migration**: None required
- **Feature flags**: None
- **Backwards compatibility**: Not applicable (new module)

## References

- Ticket: `ai_docs/prompts/2025-12-16-RUI-01-jsonld-recipe-parser.md`
- Parent roadmap: `ai_docs/roadmaps/2025-12-16-recipe-url-import.md`
- Research: `ai_docs/research/2025-12-16-recipe-extraction-standards.md`
- Similar patterns: `src-tauri/src/utils/units.rs` for pure function testing
