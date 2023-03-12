pub trait Summary {
    fn summarize(&self) -> String;
}
struct Post {
    pub title: String,   // 标题
    pub author: String,  // 作者
    pub content: String, // 内容W
}

impl Summary for Post {
    fn summarize(&self) -> String {
        format!("文章{}, 作者是{}", self.title, self.author)
    }
}

struct Weibo {
    pub username: String,
    pub content: String,
}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{}发表了微博{}", self.username, self.content)
    }
}

pub fn learn_trait_type() {
    let post = Post {
        title: "Rust语言简介".to_string(),
        author: "Sunface".to_string(),
        content: "Rust棒极了!".to_string(),
    };
    let weibo = Weibo {
        username: "sunface".to_string(),
        content: "好像微博没Tweet好用".to_string(),
    };

    println!("{}", post.summarize());
    println!("{}", weibo.summarize());

    // 默认实现
    println!("{}", post.new_summarize());
    println!("{}", weibo.new_summarize());
    println!("1 new weibo: {}", weibo.second_new_summarize());
}
pub trait NewSummary {
    fn new_summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
// 默认实现 new_summarize
impl NewSummary for Post {}
// 重载 new_summarize
impl NewSummary for Weibo {
    fn new_summarize(&self) -> String {
        format!("{}发表了微博{}", self.username, self.content)
    }
}

pub trait SecondNewSummary {
    fn summarize_author(&self) -> String;

    // 默认实现允许调用相同特征中的其他方法，哪怕这些方法没有默认实现
    fn second_new_summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
// weibo.second_new_summarize 会先调用 Summary 特征默认实现的 second_new_summarize 方法，通过该方法进而调用 Weibo 为 Summary 实现的 summarize_author 方法，最终输出：1 new weibo: (Read more from @horse_ebooks...)。
impl SecondNewSummary for Weibo {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
