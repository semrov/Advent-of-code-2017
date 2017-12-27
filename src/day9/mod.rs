mod test;

use std::str::Chars;

#[derive(Debug,Copy,Clone)]
enum ParserState {
    Initial,
    Gerbage,
}

struct Parser<'a>
{
    stream : Chars<'a>,
    state : ParserState,
    current_level : u32,
    total_score : u32,
    removed_characters : u32
}

impl<'a> Parser<'a> {
    pub fn new(stream : &'a str) -> Parser
    {
        Parser
        {
            stream : stream.chars(), 
            state : ParserState::Initial, 
            current_level : 0, 
            total_score : 0,
            removed_characters : 0
        }
    }

    fn get_next_char(&mut self) -> Option<char>
    {
        self.stream.next()
    }

    fn skip_char(&mut self)
    {
        self.stream.next();
    }

    fn level_up(&mut self)
    {
        self.current_level += 1;
    }

    fn level_down(&mut self)
    {
        if(self.current_level == 0)
        {
            panic!("Error in nesting levels, level can't be -1");
        }
        self.current_level -= 1;
    }

    fn remove_char(&mut self)
    {
        self.removed_characters += 1;
    }

    fn error(&self, c : char)
    {
        panic!("Invalid character {} in {:?} state",c, self.state);
    }

    pub fn parse(&mut self)
    {
        while let Some(c) = self.get_next_char()
        {
            match (self.state,c)
            {
                (ParserState::Initial,'{') => self.level_up(),
                (ParserState::Initial,'}') => {
                    self.total_score += self.current_level;
                    self.level_down()
                },
                (ParserState::Initial,'<') => 
                {
                    self.state = ParserState::Gerbage;
                },
                (ParserState::Initial,',') => {},
                (ParserState::Initial,c) => self.error(c),
                (ParserState::Gerbage, '>') => self.state = ParserState::Initial,
                (ParserState::Gerbage,'!') => self.skip_char(),
                (ParserState::Gerbage,_) => self.remove_char(),
            }
        }
    }
}

pub fn run()
{
    let input = include_str!("./input.txt");
    let mut parser = Parser::new(input); 
    parser.parse();
    println!("Total score:  {}", parser.total_score);
    println!("Removed characters:  {}", parser.removed_characters);
}