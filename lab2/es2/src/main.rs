use es2::{CircularBuffer, SensorData};


fn main() 
{
    let ocb = CircularBuffer::new("buffer.bin", 10);
    match ocb
    {
        Some(mut cb) =>
        {
            let mut sd0: SensorData = SensorData{ seq: 0, values: [3.0; 10], timestamp: 0};
            let mut sd1: SensorData = SensorData{ seq: 0, values: [2.0; 10], timestamp: 18};
            let a = "Prova";
            cb.put(&sd0);
            cb.put(&sd1);
            
            sd0 = cb.get().unwrap();
            sd1 = cb.get().unwrap();
        }
        None => eprint!("Errore!")
    }
}
