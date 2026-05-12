use chumsky::{prelude::*};

/// Defines how the TAS should start.
#[derive(Debug, Clone)]
pub enum StartType {
    /// TAS should start immediately
    Now,
}

#[derive(Debug, Clone)]
pub struct ScriptLine {
    pub relative: bool,
    pub tick: u32,
    pub keys: Vec<char>,
    pub mouse: Option<(i32, i32)>,
}

#[derive(Debug, Clone)]
pub struct Script {
    pub version: u64,
    pub start: StartType,
    pub lines: Vec<ScriptLine>,
}

impl Script {
    fn get_parser() -> impl Parser<char, Self, Error = Simple<char>> {
        // let padding_no_newline = filter(|c: &char| c.is_inline_whitespace()).repeated();

        let comment = just("//")
            .ignore_then(text::newline().not().repeated())
            .padded();

        let version = text::keyword("version")
            .padded()
            .ignore_then(text::int(10).map(|s: String| s.parse().unwrap()))
            .padded();

        let start = text::keyword("start")
            .padded()
            .ignore_then(text::keyword("now").to(StartType::Now));

        let tick = just('+')
            .or_not()
            .then(text::int(10).map(|s: String| s.parse().unwrap()));

        let key = one_of("WASDwasd");

        let line = tick
            .then_ignore(just('>'))
            .then(key.repeated())
            .then_ignore(comment.or_not())
            .map(|((is_relative, tick), keys)| ScriptLine {
                relative: is_relative.is_some(),
                tick,
                keys,
                mouse: None,
            });

        let lines = line
            .padded_by(comment.repeated())
            .padded()
            .repeated()
            .at_least(1);

        version
            .then(start)
            .then(lines)
            .then_ignore(text::newline().repeated())
            .then_ignore(end())
            .map(|((version, start), lines)| Script {
                version,
                start,
                lines,
            })
    }

    /// Performs additionnal checks on the script.
    fn pre_process(&mut self) -> Result<(), String> {
        // Check version
        if self.version != 0 {
            return Err(format!("Invalid version {}", self.version));
        }

        // Set all relative ticks to absolute and check
        // that they are increasing
        let mut tick = self.lines[0].tick;
        for line in &mut self.lines[1..] {
            if line.relative {
                line.relative = false;
                line.tick += tick
            }

            if tick >= line.tick {
                return Err(format!("Expected tick bigger than {tick}."));
            }

            tick = line.tick;
        }

        Ok(())
    }

    pub fn try_from(src: String) -> Result<Self, Vec<String>> {
        match Self::get_parser().parse(src.clone()) {
            Err(parse_errs) => Err(parse_errs
                .iter()
                .map(|e| {
                    let line = src[..e.span().end].match_indices("\n").count() + 1;
                    format!("line {line}: {e}")
                })
                .collect()),
            Ok(mut script) => match script.pre_process() {
                Ok(_) => Ok(script),
                Err(err) => Err(vec![err]),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::script::Script;
    use chumsky::Parser;

    #[test]
    fn test_parser() {
        let script = "
        version 0
        start now
        
        1>wasd
        2>|
        3> 
        4> // test
        +5>//test
        ";

        let res = Script::get_parser().parse(script);
        assert!(res.is_ok())
    }
}
