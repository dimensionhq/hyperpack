pub fn get_cache_requirements(framework: &str) -> Vec<String> {
    let mut requirements: Vec<String> = vec![String::from("node_modules/")];

    match framework {
        "next" => requirements.extend(vec![String::from(".next/cache/")]),
        "nuxt" => requirements.extend(vec![String::from(".nuxt/")]),
        "gatsby" => requirements.extend(vec![String::from(".cache/"), String::from("public/")]),
        "eleventy" => requirements.extend(vec![String::from(".cache/")]),
        "jekyll" => requirements.extend(vec![
            String::from("vendor/bin/"),
            String::from("vendor/cache"),
            String::from("vendor/bundle"),
        ]),
        "middleman" => requirements.extend(vec![
            String::from("vendor/bin/"),
            String::from("vendor/cache"),
            String::from("vendor/bundle"),
        ]),
        &_ => {}
    }

    requirements
}
