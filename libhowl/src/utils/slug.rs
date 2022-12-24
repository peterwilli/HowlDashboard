pub trait Slug {
    fn to_slug(&self) -> String;
}

impl Slug for str {
    fn to_slug(&self) -> String {
        let mut slug = String::new();
        let mut prev_was_whitespace = false;

        for c in self.chars() {
            if c.is_alphanumeric() {
                slug.push(c.to_ascii_lowercase());
                prev_was_whitespace = false;
            } else if !prev_was_whitespace {
                slug.push('-');
                prev_was_whitespace = true;
            }
        }

        slug
    }
}
