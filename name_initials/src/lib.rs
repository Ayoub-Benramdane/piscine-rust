pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut new_names: Vec<String> = Vec::new();
    for name in names {
        let sname: Vec<&str> = name.split(" ").collect();
        let Some(first_name) = sname[0].chars().next() else { todo!() };
        let Some(last_name) = sname[1].chars().next() else { todo!() };
        new_names.push(format!("{}. {}.", first_name, last_name));
    }
    new_names
}
