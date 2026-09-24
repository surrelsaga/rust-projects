use crate::expense::Expense;
use std::collections::HashMap;

// build a hashmap to store the total amount paied for each category
pub fn build_total_amount(expenses: &[Expense]) -> HashMap<&String, f64> {
    let mut category_to_amount = HashMap::new();

    for expense in expenses {
        let category: &String = &expense.category;
        let total_amount = category_to_amount.entry(category).or_insert(0.0);

        *total_amount += expense.amount;
    }

    category_to_amount
}
