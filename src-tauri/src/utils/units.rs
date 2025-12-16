//! Unit conversion for ingredient aggregation

use std::collections::HashMap;

/// Unit categories for grouping compatible units
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnitCategory {
    Volume,
    Weight,
    Count,
    Other,
}

/// Get the category for a unit
pub fn get_unit_category(unit: &str) -> UnitCategory {
    let unit_lower = unit.to_lowercase();
    match unit_lower.as_str() {
        // Volume units
        "cup" | "cups" | "c" |
        "tablespoon" | "tablespoons" | "tbsp" | "tbs" |
        "teaspoon" | "teaspoons" | "tsp" |
        "ml" | "milliliter" | "milliliters" |
        "l" | "liter" | "liters" |
        "fl oz" | "fluid ounce" | "fluid ounces" |
        "pint" | "pints" | "pt" |
        "quart" | "quarts" | "qt" |
        "gallon" | "gallons" | "gal" => UnitCategory::Volume,

        // Weight units
        "g" | "gram" | "grams" |
        "kg" | "kilogram" | "kilograms" |
        "oz" | "ounce" | "ounces" |
        "lb" | "lbs" | "pound" | "pounds" => UnitCategory::Weight,

        // Count units
        "" | "whole" | "piece" | "pieces" |
        "clove" | "cloves" |
        "slice" | "slices" |
        "can" | "cans" |
        "bunch" | "bunches" |
        "head" | "heads" |
        "stalk" | "stalks" |
        "sprig" | "sprigs" => UnitCategory::Count,

        _ => UnitCategory::Other,
    }
}

/// Conversion factors to a base unit within each category
/// Volume: base = ml
/// Weight: base = g
fn get_conversion_factor(unit: &str) -> Option<f64> {
    let unit_lower = unit.to_lowercase();
    match unit_lower.as_str() {
        // Volume to ml
        "ml" | "milliliter" | "milliliters" => Some(1.0),
        "l" | "liter" | "liters" => Some(1000.0),
        "tsp" | "teaspoon" | "teaspoons" => Some(4.929),
        "tbsp" | "tbs" | "tablespoon" | "tablespoons" => Some(14.787),
        "fl oz" | "fluid ounce" | "fluid ounces" => Some(29.574),
        "cup" | "cups" | "c" => Some(236.588),
        "pint" | "pints" | "pt" => Some(473.176),
        "quart" | "quarts" | "qt" => Some(946.353),
        "gallon" | "gallons" | "gal" => Some(3785.41),

        // Weight to g
        "g" | "gram" | "grams" => Some(1.0),
        "kg" | "kilogram" | "kilograms" => Some(1000.0),
        "oz" | "ounce" | "ounces" => Some(28.3495),
        "lb" | "lbs" | "pound" | "pounds" => Some(453.592),

        _ => None,
    }
}

/// Normalize a unit to its base form for display
pub fn normalize_unit(unit: &str) -> String {
    let unit_lower = unit.to_lowercase();
    match unit_lower.as_str() {
        "c" => "cup".to_string(),
        "cups" => "cup".to_string(),
        "tbs" | "tablespoons" => "tbsp".to_string(),
        "teaspoons" => "tsp".to_string(),
        "milliliters" | "milliliter" => "ml".to_string(),
        "liters" | "liter" => "L".to_string(),
        "fluid ounces" | "fluid ounce" | "fl oz" => "fl oz".to_string(),
        "pints" | "pt" => "pint".to_string(),
        "quarts" | "qt" => "quart".to_string(),
        "gallons" | "gal" => "gallon".to_string(),
        "grams" | "gram" => "g".to_string(),
        "kilograms" | "kilogram" => "kg".to_string(),
        "ounces" | "ounce" => "oz".to_string(),
        "pounds" | "pound" | "lbs" => "lb".to_string(),
        "pieces" | "piece" => "".to_string(),
        _ => unit_lower,
    }
}

/// Convert a quantity from one unit to another
/// Returns None if units are incompatible
pub fn convert_quantity(quantity: f64, from_unit: &str, to_unit: &str) -> Option<f64> {
    let from_cat = get_unit_category(from_unit);
    let to_cat = get_unit_category(to_unit);

    // Must be same category
    if from_cat != to_cat {
        return None;
    }

    // Count units don't convert
    if from_cat == UnitCategory::Count || from_cat == UnitCategory::Other {
        if normalize_unit(from_unit) == normalize_unit(to_unit) {
            return Some(quantity);
        }
        return None;
    }

    let from_factor = get_conversion_factor(from_unit)?;
    let to_factor = get_conversion_factor(to_unit)?;

    // Convert: from_unit -> base -> to_unit
    Some(quantity * from_factor / to_factor)
}

/// Result of aggregating quantities
#[derive(Debug, Clone)]
pub struct AggregatedQuantity {
    pub quantity: f64,
    pub unit: String,
    pub is_converted: bool,
}

/// Aggregate multiple quantities of the same ingredient
/// Returns the best unit and total quantity, or separate entries if incompatible
pub fn aggregate_quantities(items: &[(f64, String)]) -> Vec<AggregatedQuantity> {
    if items.is_empty() {
        return vec![];
    }

    // Group by unit category
    let mut by_category: HashMap<UnitCategory, Vec<(f64, String)>> = HashMap::new();

    for (qty, unit) in items {
        let cat = get_unit_category(unit);
        by_category.entry(cat).or_default().push((*qty, unit.clone()));
    }

    let mut results = vec![];

    for (category, group) in by_category {
        if category == UnitCategory::Count || category == UnitCategory::Other {
            // For count/other, group by normalized unit
            let mut by_unit: HashMap<String, f64> = HashMap::new();
            for (qty, unit) in group {
                let normalized = normalize_unit(&unit);
                *by_unit.entry(normalized).or_default() += qty;
            }
            for (unit, qty) in by_unit {
                results.push(AggregatedQuantity {
                    quantity: qty,
                    unit,
                    is_converted: false,
                });
            }
        } else {
            // For volume/weight, convert to most common unit
            let target_unit = find_best_target_unit(&group);
            let mut total = 0.0;
            let mut any_converted = false;

            for (qty, unit) in &group {
                if let Some(converted) = convert_quantity(*qty, unit, &target_unit) {
                    total += converted;
                    if normalize_unit(unit) != normalize_unit(&target_unit) {
                        any_converted = true;
                    }
                }
            }

            results.push(AggregatedQuantity {
                quantity: total,
                unit: normalize_unit(&target_unit),
                is_converted: any_converted,
            });
        }
    }

    results
}

/// Find the most common unit in a group (for choosing target unit)
fn find_best_target_unit(items: &[(f64, String)]) -> String {
    let mut counts: HashMap<String, usize> = HashMap::new();
    for (_, unit) in items {
        let normalized = normalize_unit(unit);
        *counts.entry(normalized).or_default() += 1;
    }

    counts
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(unit, _)| unit)
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unit_categories() {
        assert_eq!(get_unit_category("cup"), UnitCategory::Volume);
        assert_eq!(get_unit_category("CUPS"), UnitCategory::Volume);
        assert_eq!(get_unit_category("lb"), UnitCategory::Weight);
        assert_eq!(get_unit_category(""), UnitCategory::Count);
        assert_eq!(get_unit_category("pinch"), UnitCategory::Other);
    }

    #[test]
    fn test_convert_volume() {
        // 2 cups to tbsp
        let result = convert_quantity(2.0, "cup", "tbsp").unwrap();
        assert!((result - 32.0).abs() < 0.1); // 2 cups ≈ 32 tbsp

        // 1 liter to cups
        let result = convert_quantity(1.0, "L", "cup").unwrap();
        assert!((result - 4.227).abs() < 0.01);
    }

    #[test]
    fn test_convert_weight() {
        // 1 lb to oz
        let result = convert_quantity(1.0, "lb", "oz").unwrap();
        assert!((result - 16.0).abs() < 0.01);

        // 1 kg to lb
        let result = convert_quantity(1.0, "kg", "lb").unwrap();
        assert!((result - 2.205).abs() < 0.01);
    }

    #[test]
    fn test_incompatible_units() {
        // Can't convert volume to weight
        assert!(convert_quantity(1.0, "cup", "lb").is_none());
    }

    #[test]
    fn test_aggregate_same_unit() {
        let items = vec![
            (1.0, "cup".to_string()),
            (0.5, "cup".to_string()),
        ];
        let result = aggregate_quantities(&items);
        assert_eq!(result.len(), 1);
        assert!((result[0].quantity - 1.5).abs() < 0.001);
        assert_eq!(result[0].unit, "cup");
    }

    #[test]
    fn test_aggregate_different_volume_units() {
        let items = vec![
            (1.0, "cup".to_string()),
            (2.0, "tbsp".to_string()),
        ];
        let result = aggregate_quantities(&items);
        assert_eq!(result.len(), 1);
        // Result should be in cups (most common), 1 cup + 2 tbsp ≈ 1.125 cups
        assert!(result[0].is_converted);
    }

    #[test]
    fn test_aggregate_incompatible() {
        let items = vec![
            (1.0, "cup".to_string()),
            (2.0, "lb".to_string()),
        ];
        let result = aggregate_quantities(&items);
        assert_eq!(result.len(), 2); // Can't combine, separate entries
    }
}
