//! Recipe parser for extracting Schema.org Recipe data from HTML

mod duration;
mod ingredients;
mod jsonld;

use html_escape::decode_html_entities;
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
        .ok_or_else(|| ParseError::MalformedRecipe("missing name".to_string()))?;
    let name = decode_html_entities(name).to_string();

    // Extract ingredients (required)
    let ingredients_raw = json
        .get("recipeIngredient")
        .and_then(|v| v.as_array())
        .ok_or_else(|| ParseError::MalformedRecipe("missing ingredients".to_string()))?;

    let ingredients: Vec<ParsedIngredient> = ingredients_raw
        .iter()
        .filter_map(|v| v.as_str())
        .map(ingredients::parse_ingredient)
        .collect();

    if ingredients.is_empty() {
        return Err(ParseError::MalformedRecipe(
            "no valid ingredients".to_string(),
        ));
    }

    // Extract instructions (required)
    let instructions = parse_instructions(json)?;

    if instructions.is_empty() {
        return Err(ParseError::MalformedRecipe(
            "missing instructions".to_string(),
        ));
    }

    // Extract optional fields
    let description = json
        .get("description")
        .and_then(|v| v.as_str())
        .map(|s| decode_html_entities(s).to_string())
        .unwrap_or_default();

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

    let author = json.get("author").and_then(|v| {
        if v.is_string() {
            v.as_str().map(|s| s.to_string())
        } else {
            v.get("name")
                .and_then(|n| n.as_str())
                .map(|s| s.to_string())
        }
    });

    let category = json.get("recipeCategory").and_then(|v| {
        if v.is_string() {
            v.as_str().map(|s| s.to_string())
        } else if v.is_array() {
            v.as_array()
                .and_then(|arr| arr.first())
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
        } else {
            None
        }
    });

    let cuisine = json.get("recipeCuisine").and_then(|v| {
        if v.is_string() {
            v.as_str().map(|s| s.to_string())
        } else if v.is_array() {
            v.as_array()
                .and_then(|arr| arr.first())
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
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

/// Decode HTML entities and convert to String
fn decode_text(s: &str) -> String {
    decode_html_entities(s).to_string()
}

/// Parse recipeInstructions field (handles string[], HowToStep[], HowToSection[])
fn parse_instructions(json: &serde_json::Value) -> Result<Vec<String>, ParseError> {
    let instructions_value = json.get("recipeInstructions");

    match instructions_value {
        None => Ok(vec![]),
        Some(v) if v.is_string() => {
            // Single string - split on periods or return as-is
            let text = v.as_str().unwrap();
            Ok(vec![decode_text(text)])
        }
        Some(v) if v.is_array() => {
            let arr = v.as_array().unwrap();
            let mut steps = Vec::new();

            for item in arr {
                if item.is_string() {
                    // Plain string array
                    steps.push(decode_text(item.as_str().unwrap()));
                } else if let Some(item_type) = item.get("@type").and_then(|t| t.as_str()) {
                    match item_type {
                        "HowToStep" => {
                            if let Some(text) = item.get("text").and_then(|t| t.as_str()) {
                                steps.push(decode_text(text));
                            }
                        }
                        "HowToSection" => {
                            // Flatten section items
                            if let Some(items) =
                                item.get("itemListElement").and_then(|i| i.as_array())
                            {
                                for section_item in items {
                                    if let Some(text) =
                                        section_item.get("text").and_then(|t| t.as_str())
                                    {
                                        steps.push(decode_text(text));
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
        assert_eq!(
            result.description,
            "The best homemade chocolate chip cookies recipe."
        );
        assert_eq!(result.prep_time, 15);
        assert_eq!(result.cook_time, 12);
        assert_eq!(result.total_time, 27);
        assert_eq!(result.servings, 24);
        assert_eq!(
            result.image_url,
            Some("https://example.com/cookies.jpg".to_string())
        );
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

    const IMAGE_VARIATIONS_HTML: &str = include_str!("fixtures/image_variations.html");
    const STRING_INSTRUCTIONS_HTML: &str = include_str!("fixtures/string_instructions.html");

    #[test]
    fn test_image_object_array() {
        let result = parse_recipe_from_html(IMAGE_VARIATIONS_HTML).unwrap();
        // Should extract first valid URL from ImageObject array
        assert_eq!(
            result.image_url,
            Some("https://example.com/image1.jpg".to_string())
        );
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
}

#[cfg(test)]
mod allrecipes_test {
    use super::*;

    #[test]
    fn test_allrecipes_html() {
        let html = include_str!("../../../ai_docs/recipes/No-Peek Chicken Recipe.html");
        let result = parse_recipe_from_html(html).unwrap();
        println!("Name: {}", result.name);
        println!("Image URL: {:?}", result.image_url);
        println!("Ingredients count: {}", result.ingredients.len());
        assert!(result.image_url.is_some(), "Image should be present");
    }
}
