use std::io::Error;
#[derive(Default)]
pub struct Buffer {
  pub text: Vec<String>,
}

impl Buffer {
  pub fn load(&mut self, path: Option<String>) -> Result<(), Error> {
    self.text = Vec::new();
    if let Some(p) = path {
      let text = std::fs::read_to_string(p)?;
      for line in text.lines() {
        self.text.push(line.to_owned());
      }
    }
    Ok(())
  }

  pub fn is_empty(&self) -> bool {
    self.text.is_empty()
  }
}
