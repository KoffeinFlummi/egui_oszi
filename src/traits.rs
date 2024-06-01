use std::fmt::Debug;

//pub trait TimeseriesIterator<X, Y>: Iterator<Item=(X,Y)> {}
//impl<T, X, Y> TimeseriesIterator<X,Y> for T where
//    T: Iterator<Item=(X,Y)> + DoubleEndedIterator + ExactSizeIterator,
//{}

pub trait TimeseriesXAxis: Clone + Debug + PartialOrd + PartialEq + Sized {
    fn to_f64(self, origin: &mut Option<Self>) -> f64;
}

impl TimeseriesXAxis for std::time::Instant {
    fn to_f64(self, origin: &mut Option<Self>) -> f64 {
        if let Some(origin) = origin {
            (self - *origin).as_secs_f64()
        } else {
            origin.replace(self.clone());
            0.0
        }
    }
}

impl TimeseriesXAxis for f64 {
    fn to_f64(self, _origin: &mut Option<Self>) -> f64 {
        self
    }
}
