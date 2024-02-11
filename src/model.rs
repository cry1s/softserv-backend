use serde::Serialize;

fn get_all_soft() -> Vec<Software> {
    vec![
        Software {
            id: 0,
            name: String::from("Software 1"),
            version: String::from("1.0"),
            description: String::from("This is software 1"),
            logo: String::from("logo1.png"),
            os: String::from("Windows"),
            source: String::from("github.com/software1"),
            tags: vec![
                Tag {
                    name: String::from("tag1"),
                },
                Tag {
                    name: String::from("tag2"),
                },
            ],
        },
        Software {
            id: 1,
            name: String::from("Software 2"),
            version: String::from("2.0"),
            description: String::from("This is software 2"),
            logo: String::from("logo2.png"),
            os: String::from("Linux"),
            source: String::from("github.com/software2"),
            tags: vec![
                Tag {
                    name: String::from("tag3"),
                },
                Tag {
                    name: String::from("tag4"),
                },
            ],
        },
        Software {
            id: 2,
            name: String::from("Software 3"),
            version: String::from("3.0"),
            description: String::from("This is software 3"),
            logo: String::from("logo3.png"),
            os: String::from("MacOS"),
            source: String::from("github.com/software3"),
            tags: vec![
                Tag {
                    name: String::from("tag5"),
                },
                Tag {
                    name: String::from("tag6"),
                },
            ],
        },
        Software {
            id: 3,
            name: String::from("Software 4"),
            version: String::from("4.0"),
            description: String::from("This is software 4"),
            logo: String::from("logo4.png"),
            os: String::from("Windows"),
            source: String::from("github.com/software4"),
            tags: vec![
                Tag {
                    name: String::from("tag7"),
                },
                Tag {
                    name: String::from("tag8"),
                },
            ],
        },
        Software {
            id: 4,
            name: String::from("Software 5"),
            version: String::from("5.0"),
            description: String::from("This is software 5"),
            logo: String::from("logo5.png"),
            os: String::from("Linux"),
            source: String::from("github.com/software5"),
            tags: vec![
                Tag {
                    name: String::from("tag9"),
                },
                Tag {
                    name: String::from("tag10"),
                },
            ],
        },
    ]
}

// Структура для представления данных о софте
#[derive(Serialize, Debug)]
pub struct Software {
    id: i32,
    name: String,
    version: String,
    description: String,
    logo: String,
    os: String,
    source: String,
    tags: Vec<Tag>,
}

#[derive(Serialize, Debug)]
pub struct Tag {
    name: String,
}

pub fn get_soft_list(search: &str) -> Vec<Software> {
    get_all_soft()
        .into_iter()
        .filter(|x| x.name.contains(search) || x.description.contains(search))
        .collect()
}

pub(crate) fn get_soft_by_id(id: i32) -> Option<Software> {
    get_all_soft()
        .into_iter()
        .filter(|soft| soft.id == id)
        .next()
}

pub(crate) fn get_cart_list() -> Vec<Software> {
    get_all_soft()
        .into_iter()
        .filter(|soft| soft.id % 2 == 0)
        .collect()
}