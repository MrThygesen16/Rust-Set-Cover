use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;

struct Person{
    id: u32,
    total_groups: u32,
    group_map: HashSet<u32>,
}

impl Person {

    fn display(&self) {

        //let group_list = Vec::from_iter(self.group_map.keys());

        println!("\nPerson ID: {} | Total Groups: {} | Group Map: {:?}",
            self.id, self.total_groups, self.group_map
        );
    }
}


fn read_from_text(filename: String) -> Vec<String>  {
    let mut data = String::new();
    let mut f = File::open(filename).expect("Unable to open file");
    f.read_to_string(&mut data).expect("Unable to read string");

    let items: Vec<&str> = data.split("\r\n").collect();

    let mut data= Vec::new();

    for i in items {
        let temp = String::from(i);
        data.push(temp);
    }

    data
}


fn create_people(lst: Vec<String>) -> HashMap<u32, Person> {

    let mut result: HashMap<u32, Person> = HashMap::new();
    let total_groups = lst[0].parse::<u32>().unwrap();

    for i in 1..lst.len() {
        let items: Vec<&str> = lst[i].split(" ").collect();

        for p in 0..items.len() {
            let id = items[p].parse::<u32>().unwrap();

            // let mut map: HashMap<u32, bool> = HashMap::new();
            // map.insert(i.try_into().unwrap(), true);

            let mut hset: HashSet<u32> = HashSet::new();
            hset.insert(i.try_into().unwrap());

            let person = Person {
                id: id, total_groups: total_groups, group_map:hset
            };

            result.entry(id) .and_modify(|e| 
                {e.group_map.insert(i.try_into().unwrap());}) 
                .or_insert(person);
        }
    }
    
    result
}


fn person_most_groups( person_vec: Vec<&Person>) -> u32 {
    
    let mut most_value: usize = person_vec[0].group_map.len();
    let mut most_id: u32 = person_vec[0].id;
    
    for i in 1..person_vec.len() {
        
        if person_vec[i].group_map.len() > most_value
            {  
                // print!("{}", temp_min);
                most_value = person_vec[i].group_map.len();
                most_id = person_vec[i].id; 
            }
    }

    most_id
}


fn create_base_for_solver(person: Option<&Person>) -> (Vec<u32>, HashSet<u32>, u32) {
    match person {
        Some(person) => {(vec![person.id], person.group_map.iter().copied().collect(), person.total_groups)},
        None => (Vec::new(), HashSet::new(), 0),
    }
}


fn solver(mut people_list: HashMap<u32, Person>, mut base: (Vec<u32>, HashSet<u32>, u32)) -> (Vec<u32>, HashSet<u32>) {

        if base.1.len() == base.2.try_into().unwrap() {
           return (base.0, base.1);
        }

        if people_list.is_empty() {
           return (Vec::new(), HashSet::new());
        } 
        
        let mut person_to_add = (0,-1);

        let mut delete_list: Vec<u32> = Vec::new();

        for (id, p) in &people_list {
            
            let mut score = 0;
            
            for g in &p.group_map {
                if !base.1.contains(g) { score += 1 }
            }
            
            if score > person_to_add.1 { person_to_add = (*id, score) }
            else if score <= 0 { delete_list.push(p.id) }
        }
        
        let p = people_list.get(&person_to_add.0);
        
        let groups_to_add = match p {
            Some(person) =>  Vec::from_iter(&person.group_map),

            None => Vec::new(),
        };

        for i in groups_to_add {
            base.1.insert(*i);
        }

        base.0.push(person_to_add.0);
        
        people_list.remove(&person_to_add.0);

        for i in delete_list {
            people_list.remove(&i);
        }

        return solver(people_list, base);

}


fn main() {
    let data = read_from_text("tests/test9.txt".to_owned());

    println!("{:?}", data);

    let mut people_map = create_people(data);

    let people_vec = Vec::from_iter(people_map.values());

    let g = person_most_groups(people_vec);
    
    let p: Option<&Person> = people_map.get(&g);

    let base_person: (Vec<u32>, HashSet<u32>, u32) = create_base_for_solver(p);

    for p in &people_map {
        p.1.display();
    }

    people_map.remove(&base_person.0[0]);

    let res = solver(people_map, base_person);

    println!("\nFinal List: {:?}", res);
}
