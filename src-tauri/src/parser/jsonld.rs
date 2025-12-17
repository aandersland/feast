//! JSON-LD extraction from HTML

use super::ParseError;
use scraper::{Html, Selector};

/// Extract all JSON-LD blocks from HTML
pub fn extract_jsonld_blocks(html: &str) -> Result<Vec<serde_json::Value>, ParseError> {
    let document = Html::parse_document(html);
    let selector =
        Selector::parse(r#"script[type="application/ld+json"]"#).expect("Invalid selector");

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
        let blocks = vec![serde_json::json!({"@type": "Recipe", "name": "Test Recipe"})];
        let recipe = find_recipe_object(&blocks).unwrap();
        assert_eq!(recipe.get("name").unwrap().as_str().unwrap(), "Test Recipe");
    }

    #[test]
    fn test_find_recipe_array_type() {
        let blocks = vec![serde_json::json!({"@type": ["Recipe", "HowTo"], "name": "Test"})];
        let recipe = find_recipe_object(&blocks).unwrap();
        assert_eq!(recipe.get("name").unwrap().as_str().unwrap(), "Test");
    }

    #[test]
    fn test_find_recipe_in_graph() {
        let blocks = vec![serde_json::json!({
            "@context": "https://schema.org",
            "@graph": [
                {"@type": "WebPage", "name": "Page"},
                {"@type": "Recipe", "name": "Graph Recipe"}
            ]
        })];
        let recipe = find_recipe_object(&blocks).unwrap();
        assert_eq!(
            recipe.get("name").unwrap().as_str().unwrap(),
            "Graph Recipe"
        );
    }

    #[test]
    fn test_no_recipe_found() {
        let blocks = vec![serde_json::json!({"@type": "Article", "name": "Not a recipe"})];
        let result = find_recipe_object(&blocks);
        assert!(matches!(result, Err(ParseError::NoRecipeFound)));
    }

    #[test]
    fn test_multiple_recipes_error() {
        let blocks = vec![
            serde_json::json!({"@type": "Recipe", "name": "Recipe 1"}),
            serde_json::json!({"@type": "Recipe", "name": "Recipe 2"}),
        ];
        let result = find_recipe_object(&blocks);
        assert!(matches!(result, Err(ParseError::MultipleRecipesFound)));
    }
}
