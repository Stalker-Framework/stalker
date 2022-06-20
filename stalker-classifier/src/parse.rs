use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Index;
use std::path::Path;
use strum_macros::{AsRefStr, EnumVariantNames};

#[derive(Debug, AsRefStr, EnumVariantNames)]
pub enum ParseError {
    Crashed,
    Looped,
    Internal(io::Error),
}

#[derive(Clone, PartialEq, Debug)]
pub struct Fields(Vec<String>);

impl From<io::Error> for ParseError {
    fn from(e: io::Error) -> Self {
        ParseError::Internal(e)
    }
}

impl Index<usize> for Fields {
    type Output = String;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.0[idx]
    }
}

impl Fields {
    pub fn from_file<P: AsRef<Path>>(path: P, group_size: usize) -> Result<Vec<Self>, ParseError> {
        let f = File::open(path)?;
        let mut res = Vec::new();
        let mut lines = io::BufReader::new(f).lines();

        let mut buf: Vec<String> = vec![];
        while let Some(Ok(s)) = lines.next() {
            // `timeout` may terminate the process.
            if s.contains("TERM") {
                return Err(ParseError::Looped);
            } else if s.contains("the monitored command dumped core")
                || s.contains("监视的命令已核心转储")
                || s.contains("egmentation")
                || s.contains("Warning: using insecure memory!")
            {
                return Err(ParseError::Crashed);
            } else if s.is_empty() {
                let pcd = Fields::from_text_vec_unchecked(&buf, group_size);
                res.push(pcd);
                buf.clear();
            } else {
                buf.push(s)
            }
        }
        if res.is_empty() {
            Err(ParseError::Crashed)
        } else {
            Ok(res)
        }
    }

    pub fn from_text_vec_unchecked(buf: &[String], group_size: usize) -> Self {
        if buf.is_empty() || buf.len() < group_size {
            Fields(vec![])
        } else {
            if buf.len() < group_size {
                return Fields(vec![]);
            }
            let texts = buf
                .chunks(buf.len() / group_size)
                .map(|x| x[..].concat())
                .collect::<Vec<String>>();
            Fields(texts)
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Fields(data) => data.is_empty(),
        }
    }
}
