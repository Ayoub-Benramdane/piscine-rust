use std::collections::HashMap;

pub mod mall;
pub use mall::*;

pub fn biggest_store(mall: &Mall) -> (&str, &Store) {
    mall.floors
        .iter()
        .flat_map(|(_, fl)| &fl.stores)
        .max_by_key(|st| st.1.square_meters)
        .map(|(name, st)| ((*name).as_str(), st))
        .unwrap()
}

pub fn highest_paid_employee(mall: &Mall) -> Vec<(&String, &Employee)> {
    let mut highest_paid = vec![];
    let mut max_salary = 0.0;

    for floor in mall.floors.values() {
        for store in floor.stores.values() {
            for (name, employee) in &store.employees {
                if employee.salary > max_salary {
                    max_salary = employee.salary;
                    highest_paid.clear();
                    highest_paid.push((name, employee));
                } else if employee.salary == max_salary {
                    highest_paid.push((name, employee));
                }
            }
        }
    }

    highest_paid
}

pub fn nbr_of_employees(mall: &Mall) -> usize {
    mall.guards.len() +
        mall.floors
            .values()
            .flat_map(|fl| fl.stores.values())
            .map(|st| st.employees.len())
            .sum::<usize>()
}

pub fn check_for_securities(mall: &mut Mall, guards: HashMap<String, Guard>) {
    let metters: u64 = mall.floors
        .values()
        .map(|floor| floor.size_limit)
        .sum();
    let guardsneeded = ((metters as f64) / 200.0).ceil() as usize;
    let guardsnum = mall.guards.len();
    let guards_needed = guardsneeded - guardsnum;
    let mut availg = guards.into_iter();

    for _ in 0..guards_needed {
        if let Some((name, guard)) = availg.next() {
            if !mall.guards.contains_key(&name) {
                mall.hire_guard(name, guard);
            }
        } else {
            break;
        }
    }
}

pub fn cut_or_raise(mall: &mut Mall) {
    for fl in mall.floors.values_mut() {
        for st in fl.stores.values_mut() {
            for employee in st.employees.values_mut() {
                let hours = employee.working_hours.1 - employee.working_hours.0;
                let newsal = employee.salary * 0.1;
                if hours >= 10 {
                    employee.salary += newsal;
                } else {
                    employee.salary -= newsal;
                }
            }
        }
    }
}
