use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::fmt::format;
use std::io::prelude::*;
use std::path::Path;
use std::ptr::hash;
use std::string::String;
use std::sync::Arc;

use feed_rs::model::{Entry, Feed};
use html_parser::{Dom, Element, Node};
use lazy_static::lazy_static;
use lipsum::MarkovChain;
use rand::{distributions::Bernoulli, Rng, rngs::SmallRng, SeedableRng, seq::IteratorRandom};

const KEYWORDS: &str = include_str!("../data/keywords.txt");
const SYLLABLES: &str = include_str!("../data/syllables.txt");
const YEW_CONTENT: &str = include_str!("../data/yew.txt");

lazy_static! {
    static ref YEW_CHAIN: MarkovChain<'static> = {
        let mut chain = MarkovChain::new();
        chain.learn(YEW_CONTENT);
        chain
    };
}

#[tokio::test]
async fn test_display_ssr_feed() {
    let ssr_url = read_ssr_url("/data/test.txt").unwrap();
    let ret = generate_ssr_feed(ssr_url).await.unwrap();
    // println!("ret = {:?}", ret);
}

#[test]
fn test_read_ssr_url() {
    let ret = read_ssr_url("/data/test.txt");
    println!("ssr_url = {:?}", ret);
}

pub fn write_ssr_url() {
    todo!()
}

pub fn read_ssr_url(file_path: &'static str) -> std::io::Result<Vec<String>> {
    let current_dir = env::current_dir().unwrap().into_os_string().into_string().unwrap();

    let mut file_address = String::from(current_dir);
    file_address.push_str(&*file_path);
    println!("file_address = {}", file_address);

    // 创建指向所需的文件的路径
    let path = Path::new(&file_address);
    let display = path.display();

    // 以只读方式打开路径，返回 `io::Result<File>`
    let mut file = match std::fs::File::open(&path) {
        // `io::Error` 的 `description` 方法返回一个描述错误的字符串。
        Err(why) => panic!("couldn't open {}: {}", display,
                           why.description()),
        Ok(file) => file,
    };

    // 读取文件内容到一个字符串，返回 `io::Result<usize>`
    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Err(why) => panic!("couldn't read {}: {}", display,
                           why.description()),
        // Ok(_) => println!("{} contains:\n{}", display, content),
        Ok(_) => {}
    }

    // `file` 离开作用域，并且 `hello.txt` 文件将被关闭。
    let ssr_url = content.split('\n').map(|value| value.to_string()).collect::<Vec<String>>();
    let mut hash_set = HashSet::new();
    ssr_url.into_iter().for_each(|value| {
        hash_set.insert(value);
    });

    let mut result = vec![];
    hash_set.into_iter().for_each(|value| {
        result.push(value);
    });

    Ok(result)
}

async fn generate_ssr_feed(ssr_url: Vec<String>) -> Result<Vec<MirrorContent>, Box<dyn std::error::Error>> {
    println!("generate_ssr_feed start");

    // get rss_feed
    let mut result = vec![];

    for line in ssr_url {
        print!("{}  ... ", line);
        let xml = reqwest::get(&line).await?.bytes().await?;

        let ret = match feed_rs::parser::parse_with_uri(xml.as_ref(), Some(&line)) {
            Ok(feed) => {
                println!();
                // println!("mirror feed_type = {:?}", feed.feed_type);
                // println!("mirror id = {}", feed.id);
                // println!("mirror title = {:?}", feed.title);
                // println!("mirror update = {:?}", feed.updated);
                // println!("mirror authors = {:?}", feed.authors);
                // println!("mirror description = {:?}", feed.description);
                // println!("mirror links = {:?}", feed.links);
                // println!("mirror categories = {:?}", feed.categories);
                // println!("mirror contributors = {:?}", feed.contributors);
                // println!("mirror generator = {:?}", feed.generator);
                // println!("mirror icon = {:?}", feed.icon);
                // println!("mirror language = {:?}", feed.language);
                // println!("mirror logo = {:?}", feed.logo);
                // println!("mirror published = {:?}", feed.published);
                // println!("mirror rating = {:?}", feed.rating);
                // println!("mirror rights = {:?}", feed.rights);
                // println!("mirror ttl = {:?}", feed.ttl);
                let mirror_content = MirrorContent::from(feed.clone());

                mirror_content
            },
            Err(error) => return Err(Box::new(error))
        };
        result.push(ret);
    }

    println!("generate_ssr_feed end");

    Ok(result)
}

fn generate_ssr_entry(feeds: Vec<Feed>) -> Vec<Vec<Entry>> {
    feeds.into_iter().map(|value| value.entries).collect()
}

fn process_entry(content_entry: Vec<Entry>) -> Result<Vec<RssContent>, Box<dyn std::error::Error>> {
    let mut result = vec![];
    for item in content_entry {
        // println!("mirror id = {}", item.id);
        // println!("mirror title = {:?}", item.title);
        // println!("mirror update = {:?}", item.updated);
        // // println!("mirror content = {:?}", item.content);
        // let content_dom = Dom::parse(item.content.unwrap().body.unwrap().as_ref()).unwrap();
        // let content = process_dom(content_dom);
        // println!("mirror content = {:?}", content);
        // println!("mirror links = {:?}", item.links);
        // println!("mirror publish = {:?}", item.published);
        result.push(RssContent::from(item));
    }
    Ok(result)
}

fn process_dom(content_dom: Dom) -> Vec<String> {
    // println!("content_dom tree_type = {:?}", content_dom.tree_type);
    // println!("content_dom children = {:#?}", content_dom.children);
    let mut sentences = vec![];
    let content = process_element(content_dom.children);
    for sentence in content.1.iter().enumerate() {
        // println!();
        if sentence.1.is_empty() {
            continue
        } else {
            // println!("line = {}, content = {}", sentence.0, sentence.1);
            sentences.push(sentence.1.to_string());
        }
    }
    sentences
}

fn process_element(content_element: Vec<Node>) -> (String, Vec<String>) {
    let mut content = vec![];

    for element in content_element {
        match element {
            Node::Element(element) => {
                if element.name == "p" {
                    // push content
                    let mut result = String::new();
                    for index in 0..element.children.len() {
                        result.push_str(&*process_element(vec![element.children[index].clone()]).0);
                    }
                    content.push(result);
                } else if element.name == "a" {
                    let brand_name = process_element(element.children.clone()).0;
                    let mut brand_address = format!("[{}]", brand_name);
                    let link = element.attributes.get("href").unwrap().clone().unwrap().to_string();
                    brand_address.push_str(&*format!("({})", link));
                    return (brand_address, vec![]);
                } else if element.name == "hr" {
                    content.push("".to_string());
                } else if element.name == "h4" {
                    content.push(process_element(element.children.clone()).0);
                } else if element.name == "blockquote" { // TODO
                    // push content
                    let mut result = vec![];
                    for index in 0..element.children.len() {
                        result.push(process_element(vec![element.children[index].clone()]).0);
                    }
                    content.append(&mut result);
                } else if element.name == "em" { // TODO
                    content.push(process_element(element.children.clone()).0);
                }
            }
            Node::Text(text) => {
                let text = text.replace("&quot;", "").replace("&#39;", r"'").replace("\n", "").replace("&amp;", "");
                return (text, vec![]);
            }
            Node::Comment(comment) => {
                todo!()
            }
        }
    }


    (String::new(), content)
}

#[derive(Debug, Clone)]
pub enum Rss {
    Mirror(Vec<MirrorContent>),
}

#[derive(Debug, Clone)]
pub struct MirrorContent {
    pub feed_type: String,
    pub id: String,
    pub mirror_title: String,
    pub updated: String,
    pub description: String,
    pub links: String,
    pub entries: Vec<RssContent>,
}

impl From<Feed> for MirrorContent {
    fn from(feed: Feed) -> Self {
        Self {
            feed_type: format!("{:?}", feed.feed_type),
            id: feed.id.to_string(),
            mirror_title: format!("{:?}", feed.title.unwrap().content),
            updated: format!("{:?}", feed.updated.unwrap()),
            description: format!("{:?}", feed.description.unwrap().content),
            links: format!("{:?}", feed.links.into_iter().map(|value| value.href).collect::<Vec<String>>()),
            entries: feed.entries.into_iter().map(|value| RssContent::from(value)).collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RssContent {
    pub id: String,
    pub title: String,
    pub update: String,
    pub content: Vec<String>,
    pub links: Vec<String>,
    pub published: String,
}

impl From<Entry> for RssContent {
    fn from(entry: Entry) -> Self {
        let content_dom = Dom::parse(entry.content.unwrap().body.unwrap().as_ref()).unwrap();
        let content = process_dom(content_dom);
        Self {
            id: entry.id.to_string(),
            title: format!("{:?}", entry.title.unwrap().content),
            update: format!("{:?}", entry.updated.unwrap()),
            content: content,
            links: entry.links.into_iter().map(|value| value.href).collect(),
            published: format!("{:?}", entry.published.unwrap()),
        }
    }
}


#[derive(Debug, Clone)]
pub struct Generator {
    pub seed: u64,
    rng: SmallRng,
}

impl Generator {
    pub fn from_seed(seed: u64) -> Self {
        let rng = SmallRng::seed_from_u64(seed);

        Self { seed, rng }
    }
}

#[test]
fn test_generator() {
    let ssr_url = read_ssr_url("/data/test.txt").unwrap();
    println!("ssr_url = {:?}", ssr_url);

    let rt = Arc::new(tokio::runtime::Runtime::new().unwrap()).clone();

    let feeds = rt.block_on(generate_ssr_feed(ssr_url)).unwrap();
}

impl Generator {
    pub fn new_seed(&mut self) -> u64 {
        self.rng.gen()
    }

    /// [low, high)
    pub fn range(&mut self, low: usize, high: usize) -> usize {
        self.rng.gen_range(low..high)
    }

    /// `n / d` chance
    pub fn chance(&mut self, n: u32, d: u32) -> bool {
        self.rng.sample(Bernoulli::from_ratio(n, d).unwrap())
    }

    pub fn image_url(&mut self, dimension: (usize, usize), keywords: &[String]) -> String {
        let cache_buster = self.rng.gen::<u16>();
        let (width, height) = dimension;
        format!(
            "https://source.unsplash.com/random/{}x{}?{}&sig={}",
            width,
            height,
            keywords.join(","),
            cache_buster
        )
    }

    pub fn face_image_url(&mut self, dimension: (usize, usize)) -> String {
        self.image_url(dimension, &["human".to_owned(), "face".to_owned()])
    }

    pub fn human_name(&mut self) -> String {
        const SYLLABLES_MIN: usize = 1;
        const SYLLABLES_MAX: usize = 5;

        let n_syllables = self.rng.gen_range(SYLLABLES_MIN..SYLLABLES_MAX);
        let first_name = SYLLABLES
            .split_whitespace()
            .choose_multiple(&mut self.rng, n_syllables)
            .join("");

        let n_syllables = self.rng.gen_range(SYLLABLES_MIN..SYLLABLES_MAX);
        let last_name = SYLLABLES
            .split_whitespace()
            .choose_multiple(&mut self.rng, n_syllables)
            .join("");

        format!("{} {}", title_case(&first_name), title_case(&last_name))
    }

    pub fn keywords(&mut self) -> Vec<String> {
        const KEYWORDS_MIN: usize = 1;
        const KEYWORDS_MAX: usize = 4;

        let n_keywords = self.rng.gen_range(KEYWORDS_MIN..KEYWORDS_MAX);
        KEYWORDS
            .split_whitespace()
            .map(ToOwned::to_owned)
            .choose_multiple(&mut self.rng, n_keywords)
    }

    pub fn title(&mut self) -> String {
        const WORDS_MIN: usize = 3;
        const WORDS_MAX: usize = 8;
        const SMALL_WORD_LEN: usize = 3;

        let n_words = self.rng.gen_range(WORDS_MIN..WORDS_MAX);
        let mut title = String::new();

        let words = YEW_CHAIN
            .iter_with_rng(&mut self.rng)
            .map(|word| word.trim_matches(|c: char| c.is_ascii_punctuation()))
            .filter(|word| !word.is_empty())
            .take(n_words);

        for (i, word) in words.enumerate() {
            if i > 0 {
                title.push(' ');
            }

            // Capitalize the first word and all long words.
            if i == 0 || word.len() > SMALL_WORD_LEN {
                title.push_str(&title_case(word));
            } else {
                title.push_str(word);
            }
        }
        title
    }

    pub fn sentence(&mut self) -> String {
        const WORDS_MIN: usize = 7;
        const WORDS_MAX: usize = 25;

        let n_words = self.rng.gen_range(WORDS_MIN..WORDS_MAX);
        YEW_CHAIN.generate_with_rng(&mut self.rng, n_words)
    }

    pub fn paragraph(&mut self) -> String {
        const SENTENCES_MIN: usize = 3;
        const SENTENCES_MAX: usize = 20;

        let n_sentences = self.rng.gen_range(SENTENCES_MIN..SENTENCES_MAX);
        let mut paragraph = String::new();
        for i in 0..n_sentences {
            if i > 0 {
                paragraph.push(' ');
            }

            paragraph.push_str(&self.sentence());
        }
        paragraph
    }
}

fn title_case(word: &str) -> String {
    let idx = match word.chars().next() {
        Some(c) => c.len_utf8(),
        None => 0,
    };

    let mut result = String::with_capacity(word.len());
    result.push_str(&word[..idx].to_uppercase());
    result.push_str(&word[idx..]);
    result
}

pub trait Generated: Sized {
    fn generate(gen: &mut Generator) -> Self;
    fn generate_from_seed(seed: u64) -> Self {
        Self::generate(&mut Generator::from_seed(seed))
    }
}
