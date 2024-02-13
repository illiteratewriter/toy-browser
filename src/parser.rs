pub trait ParserUtils {
    fn get_pos(&self) -> usize;
    fn set_pos(&mut self, pos: usize);
    fn get_input(&mut self) -> &String;

    fn next_char(&mut self) -> char {
        let pos = self.get_pos();
        self.get_input()[pos..].chars().next().unwrap()
    }

    fn starts_with(&mut self, s: &str) -> bool {
        let pos = self.get_pos();
        self.get_input()[pos..].starts_with(s)
    }

    fn eof(&mut self) -> bool {
        let pos = self.get_pos();
        pos >= self.get_input().len()
    }

    fn consume_char(&mut self) -> char {
        let pos = self.get_pos();
        let mut iter = self.get_input()[pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.set_pos(pos + next_pos);
        cur_char
    }

    fn consume_while<F>(&mut self, test: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while !self.eof() && test(self.next_char()) {
            result.push(self.consume_char());
        }
        return result;
    }

    fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }
}
