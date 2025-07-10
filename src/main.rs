use std::fs::{self, read_to_string};

#[derive(Debug)]
struct Metadata {
    filename: String,
    title: String,
    desc: String,
    time: chrono::DateTime<chrono::Utc>,
}

impl Metadata {
    fn new(s: &str) -> Self {
        let mut iter_s = s.split_terminator("\\\n");
        iter_s.next();
        let filename = iter_s.next().expect("Error reading filename").to_string();
        let title = iter_s.next().expect("Error reading title").to_string();
        let desc = iter_s.next().expect("Error reading desc").to_string();
        let time = chrono::DateTime::from_timestamp(
            iter_s
                .next()
                .expect("Error reading timestamp")
                .trim()
                .parse()
                .expect("Error parsing timestamp"),
            0,
        )
        .unwrap();

        Metadata {
            filename,
            title,
            desc,
            time,
        }
    }
}

fn main() {
    fs::remove_dir_all("./website/pages/blag/").unwrap();
    fs::DirBuilder::new()
        .create("./website/pages/blag/")
        .unwrap();
    let paths = fs::read_dir("./website_templates/blags/")
        .unwrap()
        .map(|p| p.unwrap().path());
    let posts = paths.map(|p| (p.to_string_lossy().to_string(), read_to_string(p).unwrap()));

    let mut posts_metadata = Vec::new();
    for (path, post) in posts {
        let (body_md, metadata_s) = post.split_at(
            post.find("%%%\\\n")
                .unwrap_or_else(|| panic!("Metadata Delimiter not found in {}", path)),
        );
        let metadata = Metadata::new(metadata_s);
        let body = markdown::to_html_with_options(body_md, &markdown::Options::gfm())
            .expect("Error parsing Markdown");

        let post_html = format!(
            include_str!("../website_templates/blog-post.html"),
            metadata.title,
            metadata.desc,
            metadata.time.date_naive(),
            body
        );

        println!("Writing: {}", metadata.filename);

        fs::write(
            format!("./website/pages/blag/{}.html", metadata.filename),
            post_html,
        )
        .expect("Error writing");

        posts_metadata.push(metadata);
    }

    let mut blog_dir_s = String::new();

    for pm in posts_metadata {
        blog_dir_s.push_str(
            format!(
                include_str!("../website_templates/blog-entry.html"),
                pm.filename,
                pm.title,
                pm.time.date_naive(),
                pm.desc
            )
            .as_str(),
        );
        blog_dir_s.push('\n');
    }
    let blog_dir_html = format!(
        include_str!("../website_templates/blog-dir.html"),
        blog_dir_s
    );
    fs::write("./website/pages/blag.html", blog_dir_html).expect("Error writing blog index");
}
