use std::{fmt, io};

pub struct FlightDataRecorder {
    writer: Box<dyn io::Write + Send + Sync>,
}

impl fmt::Debug for FlightDataRecorder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("FlightDataRecorder")
            .field("writer", &"<boxed>")
            .finish()
    }
}

impl FlightDataRecorder {
    pub fn new<W>(writer: W) -> Self
    where
        W: io::Write + Send + Sync + 'static,
    {
        FlightDataRecorder {
            writer: Box::new(writer),
        }
    }
}

impl FlightDataRecorder {
    pub fn publish<M>(&mut self, message: &M) -> Result<(), rmp_serde::encode::Error>
    where
        M: serde::Serialize,
    {
        rmp_serde::encode::write_named(&mut self.writer, message)
    }
}
